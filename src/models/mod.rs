pub mod goal;
pub mod meal;
pub mod nutrition;
pub mod recipe;

pub use goal::{Goal, MacroTarget};
pub use meal::{Meal, MealType};
pub use nutrition::Nutrition;
pub use recipe::Recipe;

/// 1日の食事プラン
#[derive(Debug, Clone)]
pub struct DailyPlan {
    pub date: Option<String>,
    pub meals: Vec<Meal>,           // 朝昼夕+間食の4食
    pub total_nutrition: Nutrition, // 合計栄養
    pub target: MacroTarget,        // 目標値
}

impl DailyPlan {
    pub fn new(
        date: Option<String>,
        meals: Vec<Meal>,
        total_nutrition: Nutrition,
        target: MacroTarget,
    ) -> Self {
        Self {
            date,
            meals,
            total_nutrition,
            target,
        }
    }
}

/// 7日間の食事プラン
#[derive(Debug, Clone)]
pub struct WeeklyPlan {
    pub start_date: String,          // 開始日 (YYYY-MM-DD)
    pub daily_plans: Vec<DailyPlan>, // 7日分のDailyPlan
    pub target: MacroTarget,         // 共通の目標値
}

impl WeeklyPlan {
    pub fn new(start_date: String, daily_plans: Vec<DailyPlan>, target: MacroTarget) -> Self {
        Self {
            start_date,
            daily_plans,
            target,
        }
    }

    /// 週間平均栄養を計算
    pub fn average_nutrition(&self) -> Nutrition {
        let total = Nutrition::sum(
            &self
                .daily_plans
                .iter()
                .map(|p| &p.total_nutrition)
                .collect::<Vec<_>>(),
        );

        Nutrition {
            calories: total.calories / 7.0,
            protein: total.protein / 7.0,
            fat: total.fat / 7.0,
            carbohydrates: total.carbohydrates / 7.0,
        }
    }

    /// 週間合計栄養を計算
    pub fn total_nutrition(&self) -> Nutrition {
        Nutrition::sum(
            &self
                .daily_plans
                .iter()
                .map(|p| &p.total_nutrition)
                .collect::<Vec<_>>(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weekly_plan_average_nutrition() {
        // 7日分のモックデータ
        let target = MacroTarget {
            goal: Goal::Maintain,
            daily_calories: 2400.0,
            protein_grams: 180.0,
            fat_grams: 67.0,
            carbs_grams: 270.0,
        };

        let daily_plans: Vec<DailyPlan> = (0..7)
            .map(|i| DailyPlan {
                date: Some(format!("2026-01-{:02}", i + 6)),
                meals: vec![],
                total_nutrition: Nutrition {
                    calories: 2400.0,
                    protein: 180.0,
                    fat: 67.0,
                    carbohydrates: 270.0,
                },
                target: target.clone(),
            })
            .collect();

        let weekly_plan = WeeklyPlan::new("2026-01-06".to_string(), daily_plans, target);

        let avg = weekly_plan.average_nutrition();
        assert!((avg.calories - 2400.0).abs() < 0.1);
        assert!((avg.protein - 180.0).abs() < 0.1);
        assert!((avg.fat - 67.0).abs() < 0.1);
        assert!((avg.carbohydrates - 270.0).abs() < 0.1);
    }

    #[test]
    fn test_weekly_plan_total_nutrition() {
        let target = MacroTarget {
            goal: Goal::Maintain,
            daily_calories: 2400.0,
            protein_grams: 180.0,
            fat_grams: 67.0,
            carbs_grams: 270.0,
        };

        let daily_plans: Vec<DailyPlan> = (0..7)
            .map(|i| DailyPlan {
                date: Some(format!("2026-01-{:02}", i + 6)),
                meals: vec![],
                total_nutrition: Nutrition {
                    calories: 2400.0,
                    protein: 180.0,
                    fat: 67.0,
                    carbohydrates: 270.0,
                },
                target: target.clone(),
            })
            .collect();

        let weekly_plan = WeeklyPlan::new("2026-01-06".to_string(), daily_plans, target);

        let total = weekly_plan.total_nutrition();
        assert!((total.calories - 16800.0).abs() < 0.1);
        assert!((total.protein - 1260.0).abs() < 0.1);
        assert!((total.fat - 469.0).abs() < 0.1);
        assert!((total.carbohydrates - 1890.0).abs() < 0.1);
    }
}
