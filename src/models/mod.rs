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
