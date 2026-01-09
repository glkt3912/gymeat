use crate::calculator::{CalorieCalculator, MacroCalculator};
use crate::config::PlanConfig;
use crate::data::MealDatabase;
use crate::error::{MealPlannerError, Result};
use crate::models::{DailyPlan, Meal, MealType, MacroTarget, Nutrition, WeeklyPlan};
use chrono::{Duration, NaiveDate};
use rand::seq::SliceRandom;
use std::collections::HashSet;

/// 既に使用したメニューを追跡するトラッカー
struct UsedMealsTracker {
    used_ids: HashSet<String>,
}

impl UsedMealsTracker {
    fn new() -> Self {
        Self {
            used_ids: HashSet::new(),
        }
    }

    /// メニューが既に使用済みかチェック
    fn is_used(&self, meal_id: &str) -> bool {
        self.used_ids.contains(meal_id)
    }

    /// メニューを使用済みとしてマーク
    fn mark_used(&mut self, meal_id: String) {
        self.used_ids.insert(meal_id);
    }
}

/// 週間プランナー
pub struct WeeklyPlanner<'a> {
    database: &'a MealDatabase,
    config: &'a PlanConfig,
}

impl<'a> WeeklyPlanner<'a> {
    /// 新しいWeeklyPlannerを作成
    pub fn new(database: &'a MealDatabase, config: &'a PlanConfig) -> Self {
        Self { database, config }
    }

    /// 7日間の食事プランを生成
    pub fn generate(&self, start_date: Option<NaiveDate>) -> Result<WeeklyPlan> {
        // 1. 開始日を決定 (指定がなければ今日)
        let start = start_date.unwrap_or_else(|| chrono::Local::now().naive_local().date());

        // 2. 目標カロリーとマクロを計算 (全日共通)
        let target = self.calculate_target()?;

        // 3. 使用済みメニュートラッカーを初期化
        let mut tracker = UsedMealsTracker::new();

        // 4. 7日分のプランを生成
        let mut daily_plans = Vec::with_capacity(7);
        for day_offset in 0..7 {
            let current_date = start + Duration::days(day_offset);
            let plan = self.generate_daily_plan(&target, &current_date, &mut tracker)?;
            daily_plans.push(plan);
        }

        Ok(WeeklyPlan::new(
            start.format("%Y-%m-%d").to_string(),
            daily_plans,
            target,
        ))
    }

    /// 1日分のプランを生成 (デデュプリケーション対応)
    fn generate_daily_plan(
        &self,
        target: &MacroTarget,
        date: &NaiveDate,
        tracker: &mut UsedMealsTracker,
    ) -> Result<DailyPlan> {
        // 朝食: 23%, 昼食: 33%, 夕食: 34%, 間食: 10%
        let breakfast = self.select_meal_with_dedup(
            MealType::Breakfast,
            target.daily_calories * 0.23,
            tracker,
        )?;
        let lunch = self.select_meal_with_dedup(
            MealType::Lunch,
            target.daily_calories * 0.33,
            tracker,
        )?;
        let dinner = self.select_meal_with_dedup(
            MealType::Dinner,
            target.daily_calories * 0.34,
            tracker,
        )?;
        let snack = self.select_meal_with_dedup(
            MealType::Snack,
            target.daily_calories * 0.10,
            tracker,
        )?;

        let meals = vec![breakfast, lunch, dinner, snack];
        let total_nutrition = Nutrition::sum(&meals.iter().map(|m| &m.nutrition).collect::<Vec<_>>());

        Ok(DailyPlan::new(
            Some(date.format("%Y-%m-%d").to_string()),
            meals,
            total_nutrition,
            target.clone(),
        ))
    }

    /// デデュプリケーション対応のメニュー選択
    fn select_meal_with_dedup(
        &self,
        meal_type: MealType,
        target_calories: f32,
        tracker: &mut UsedMealsTracker,
    ) -> Result<Meal> {
        // 1. 未使用のメニューのみをフィルタ
        let available_candidates: Vec<_> = self
            .database
            .filter_by_type(meal_type)
            .into_iter()
            .filter(|m| !tracker.is_used(&m.id))
            .collect();

        // 2. 候補がない場合の処理
        if available_candidates.is_empty() {
            // フォールバック: 全メニューから選択 (重複を許可)
            return self.select_meal_fallback(meal_type, target_calories);
        }

        // 3. 目標カロリーとの差分を計算してソート
        let mut candidates = available_candidates
            .into_iter()
            .map(|m| {
                let diff = (m.nutrition.calories - target_calories).abs();
                (m, diff)
            })
            .collect::<Vec<_>>();

        candidates.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        // 4. 上位3つ (または候補数の最小値) からランダムに選ぶ
        let top_n = candidates.len().min(3);
        let top_candidates: Vec<_> = candidates.iter().take(top_n).map(|(m, _)| m).collect();

        let mut rng = rand::thread_rng();
        let selected = (**top_candidates.choose(&mut rng).unwrap()).clone();

        // 5. 使用済みとしてマーク
        tracker.mark_used(selected.id.clone());

        Ok(selected)
    }

    /// フォールバック: 重複を許可してメニューを選択
    fn select_meal_fallback(&self, meal_type: MealType, target_calories: f32) -> Result<Meal> {
        let mut candidates = self
            .database
            .filter_by_type(meal_type)
            .into_iter()
            .map(|m| {
                let diff = (m.nutrition.calories - target_calories).abs();
                (m, diff)
            })
            .collect::<Vec<_>>();

        if candidates.is_empty() {
            let meal_type_str = match meal_type {
                MealType::Breakfast => "朝食",
                MealType::Lunch => "昼食",
                MealType::Dinner => "夕食",
                MealType::Snack => "間食",
            };
            return Err(MealPlannerError::NoSuitableMealFound(
                meal_type_str.to_string(),
            ));
        }

        candidates.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        let top_n = candidates.len().min(3);
        let top_candidates: Vec<_> = candidates.iter().take(top_n).map(|(m, _)| m).collect();

        let mut rng = rand::thread_rng();
        Ok((**top_candidates.choose(&mut rng).unwrap()).clone())
    }

    /// 目標カロリーとマクロ栄養素を計算 (DailyPlannerと同じロジック)
    fn calculate_target(&self) -> Result<MacroTarget> {
        let calories = if let Some(custom) = self.config.custom_calories {
            custom
        } else if let (Some(w), Some(h), Some(a), Some(g)) = (
            self.config.weight,
            self.config.height,
            self.config.age,
            self.config.gender,
        ) {
            let bmr = CalorieCalculator::calculate_bmr(w, h, a, g);
            let tdee = CalorieCalculator::calculate_tdee(bmr, self.config.activity_level);
            CalorieCalculator::calculate_target_calories(tdee, self.config.goal)
        } else {
            self.config.default_calories()
        };

        Ok(MacroCalculator::calculate_macros(calories, self.config.goal))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{ActivityLevel, Gender};
    use crate::models::Goal;

    #[test]
    fn test_weekly_planner_generate() {
        let db = MealDatabase::new_embedded().unwrap();
        let config = PlanConfig {
            goal: Goal::Maintain,
            weight: Some(70.0),
            height: Some(175.0),
            age: Some(25),
            gender: Some(Gender::Male),
            activity_level: ActivityLevel::Moderate,
            custom_calories: None,
        };

        let planner = WeeklyPlanner::new(&db, &config);
        let plan = planner.generate(None).unwrap();

        assert_eq!(plan.daily_plans.len(), 7);

        // 各日に4食あることを確認
        for daily_plan in &plan.daily_plans {
            assert_eq!(daily_plan.meals.len(), 4);
        }
    }

    #[test]
    fn test_weekly_planner_with_custom_start_date() {
        let db = MealDatabase::new_embedded().unwrap();
        let config = PlanConfig {
            goal: Goal::Bulk,
            weight: None,
            height: None,
            age: None,
            gender: None,
            activity_level: ActivityLevel::Moderate,
            custom_calories: Some(2800.0),
        };

        let start = NaiveDate::from_ymd_opt(2026, 1, 6).unwrap();
        let planner = WeeklyPlanner::new(&db, &config);
        let plan = planner.generate(Some(start)).unwrap();

        assert_eq!(plan.start_date, "2026-01-06");
        assert_eq!(plan.daily_plans[0].date.as_ref().unwrap(), "2026-01-06");
        assert_eq!(plan.daily_plans[6].date.as_ref().unwrap(), "2026-01-12");
    }

    #[test]
    fn test_deduplication() {
        let db = MealDatabase::new_embedded().unwrap();
        let config = PlanConfig {
            goal: Goal::Cut,
            weight: Some(65.0),
            height: Some(170.0),
            age: Some(30),
            gender: Some(Gender::Female),
            activity_level: ActivityLevel::Light,
            custom_calories: None,
        };

        let planner = WeeklyPlanner::new(&db, &config);
        let plan = planner.generate(None).unwrap();

        // 各食事タイプごとに重複をチェック
        for meal_type in [MealType::Breakfast, MealType::Lunch, MealType::Dinner, MealType::Snack] {
            let meal_ids: Vec<String> = plan
                .daily_plans
                .iter()
                .flat_map(|dp| &dp.meals)
                .filter(|m| m.meal_type == meal_type)
                .map(|m| m.id.clone())
                .collect();

            let unique_count = meal_ids.iter().collect::<HashSet<_>>().len();

            // データベースサイズを考慮
            let db_count = db.count_by_type(meal_type);
            let expected_min_unique = 7.min(db_count);

            // 少なくともデータベースが許す限りユニークであるべき
            assert!(unique_count >= expected_min_unique.saturating_sub(1));
        }
    }
}
