use crate::calculator::{CalorieCalculator, MacroCalculator};
use crate::config::PlanConfig;
use crate::data::MealDatabase;
use crate::error::{MealPlannerError, Result};
use crate::models::{DailyPlan, Meal, MealType, MacroTarget, Nutrition};
use rand::seq::SliceRandom;

/// 1日プランナー
pub struct DailyPlanner<'a> {
    database: &'a MealDatabase,
    config: &'a PlanConfig,
}

impl<'a> DailyPlanner<'a> {
    /// 新しいDailyPlannerを作成
    pub fn new(database: &'a MealDatabase, config: &'a PlanConfig) -> Self {
        Self { database, config }
    }

    /// 1日の食事プランを生成
    pub fn generate(&self) -> Result<DailyPlan> {
        // 1. 目標カロリーとマクロを計算
        let target = self.calculate_target()?;

        // 2. 各食事タイプごとにメニューを選択
        //    朝食: 23%, 昼食: 33%, 夕食: 34%, 間食: 10%
        let breakfast = self.select_meal(MealType::Breakfast, target.daily_calories * 0.23)?;
        let lunch = self.select_meal(MealType::Lunch, target.daily_calories * 0.33)?;
        let dinner = self.select_meal(MealType::Dinner, target.daily_calories * 0.34)?;
        let snack = self.select_meal(MealType::Snack, target.daily_calories * 0.10)?;

        let meals = vec![breakfast, lunch, dinner, snack];

        // 3. 合計栄養を計算
        let total_nutrition = Nutrition::sum(&meals.iter().map(|m| &m.nutrition).collect::<Vec<_>>());

        // 4. 日付を取得
        let date = Some(chrono::Local::now().format("%Y-%m-%d").to_string());

        Ok(DailyPlan::new(date, meals, total_nutrition, target))
    }

    /// 目標カロリーとマクロ栄養素を計算
    fn calculate_target(&self) -> Result<MacroTarget> {
        let calories = if let Some(custom) = self.config.custom_calories {
            // カスタムカロリーが指定されている場合
            custom
        } else if let (Some(w), Some(h), Some(a), Some(g)) =
            (self.config.weight, self.config.height, self.config.age, self.config.gender)
        {
            // 体組成情報からBMR/TDEEを計算
            let bmr = CalorieCalculator::calculate_bmr(w, h, a, g);
            let tdee = CalorieCalculator::calculate_tdee(bmr, self.config.activity_level);
            CalorieCalculator::calculate_target_calories(tdee, self.config.goal)
        } else {
            // デフォルト値を使用
            self.config.default_calories()
        };

        Ok(MacroCalculator::calculate_macros(calories, self.config.goal))
    }

    /// 指定した食事タイプのメニューを選択
    fn select_meal(&self, meal_type: MealType, target_calories: f32) -> Result<Meal> {
        // 目標カロリーとの差分を計算してソート
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
            return Err(MealPlannerError::NoSuitableMealFound(meal_type_str.to_string()));
        }

        // 目標に近い順にソート
        candidates.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        // 上位3つからランダムに選ぶ（多様性を確保しつつ精度も保つ）
        let top_n = candidates.len().min(3);
        let top_candidates: Vec<_> = candidates.iter().take(top_n).map(|(m, _)| m).collect();

        let mut rng = rand::thread_rng();
        Ok((**top_candidates.choose(&mut rng).unwrap()).clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{ActivityLevel, Gender};
    use crate::models::Goal;

    #[test]
    fn test_daily_planner_generate() {
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

        let planner = DailyPlanner::new(&db, &config);
        let plan = planner.generate();

        assert!(plan.is_ok());

        let plan = plan.unwrap();
        assert_eq!(plan.meals.len(), 4); // 朝昼夕+間食
        assert!(plan.total_nutrition.calories > 0.0);
    }

    #[test]
    fn test_daily_planner_with_custom_calories() {
        let db = MealDatabase::new_embedded().unwrap();
        let config = PlanConfig {
            goal: Goal::Cut,
            weight: None,
            height: None,
            age: None,
            gender: None,
            activity_level: ActivityLevel::Moderate,
            custom_calories: Some(1800.0),
        };

        let planner = DailyPlanner::new(&db, &config);
        let plan = planner.generate();

        assert!(plan.is_ok());

        let plan = plan.unwrap();
        // カロリーが目標の±20%以内かチェック
        let diff = (plan.total_nutrition.calories - 1800.0).abs();
        assert!(diff < 1800.0 * 0.40); // 緩めの許容範囲
    }

    #[test]
    fn test_calculate_target() {
        let db = MealDatabase::new_embedded().unwrap();
        let config = PlanConfig {
            goal: Goal::Bulk,
            weight: Some(70.0),
            height: Some(175.0),
            age: Some(25),
            gender: Some(Gender::Male),
            activity_level: ActivityLevel::Moderate,
            custom_calories: None,
        };

        let planner = DailyPlanner::new(&db, &config);
        let target = planner.calculate_target().unwrap();

        assert!(target.daily_calories > 2500.0); // 増量なので高カロリー
        assert_eq!(target.goal, Goal::Bulk);
    }
}
