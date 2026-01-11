use super::formatter::OutputFormatter;
use crate::data::MealDatabase;
use crate::error::{MealPlannerError, Result};
use crate::models::{DailyPlan, Meal, WeeklyPlan};
use serde::Serialize;

/// JSON出力フォーマッター
pub struct JsonFormatter {
    pretty: bool, // インデント付き整形の有無
}

impl JsonFormatter {
    /// 新しいJsonFormatterを作成 (pretty: 整形出力するか)
    pub fn new(pretty: bool) -> Self {
        Self { pretty }
    }
}

impl OutputFormatter for JsonFormatter {
    fn format_daily_plan(
        &self,
        plan: &DailyPlan,
        database: &MealDatabase,
        show_recipe: bool,
    ) -> Result<String> {
        let serializable = SerializableDailyPlan::from_plan(plan, database, show_recipe);

        let json = if self.pretty {
            serde_json::to_string_pretty(&serializable)
        } else {
            serde_json::to_string(&serializable)
        };

        json.map_err(|e| MealPlannerError::FormatError(format!("JSON変換失敗: {}", e)))
    }

    fn format_weekly_plan(
        &self,
        plan: &WeeklyPlan,
        database: &MealDatabase,
        show_recipe: bool,
    ) -> Result<String> {
        let serializable = SerializableWeeklyPlan::from_plan(plan, database, show_recipe);

        let json = if self.pretty {
            serde_json::to_string_pretty(&serializable)
        } else {
            serde_json::to_string(&serializable)
        };

        json.map_err(|e| MealPlannerError::FormatError(format!("JSON変換失敗: {}", e)))
    }

    fn format_name(&self) -> &'static str {
        if self.pretty {
            "json-pretty"
        } else {
            "json"
        }
    }
}

/// シリアライズ可能な1日プラン (レシピ情報を含む可能性あり)
#[derive(Serialize)]
struct SerializableDailyPlan {
    date: Option<String>,
    meals: Vec<SerializableMeal>,
    total_nutrition: crate::models::Nutrition,
    target: crate::models::MacroTarget,
}

impl SerializableDailyPlan {
    fn from_plan(plan: &DailyPlan, database: &MealDatabase, show_recipe: bool) -> Self {
        let meals = plan
            .meals
            .iter()
            .map(|m| SerializableMeal::from_meal(m, database, show_recipe))
            .collect();

        Self {
            date: plan.date.clone(),
            meals,
            total_nutrition: plan.total_nutrition.clone(),
            target: plan.target.clone(),
        }
    }
}

/// シリアライズ可能な週間プラン
#[derive(Serialize)]
struct SerializableWeeklyPlan {
    start_date: String,
    daily_plans: Vec<SerializableDailyPlan>,
    target: crate::models::MacroTarget,
    weekly_total: crate::models::Nutrition,
    daily_average: crate::models::Nutrition,
}

impl SerializableWeeklyPlan {
    fn from_plan(plan: &WeeklyPlan, database: &MealDatabase, show_recipe: bool) -> Self {
        let daily_plans = plan
            .daily_plans
            .iter()
            .map(|p| SerializableDailyPlan::from_plan(p, database, show_recipe))
            .collect();

        Self {
            start_date: plan.start_date.clone(),
            daily_plans,
            target: plan.target.clone(),
            weekly_total: plan.total_nutrition(),
            daily_average: plan.average_nutrition(),
        }
    }
}

/// シリアライズ可能なMeal (レシピを含む可能性あり)
#[derive(Serialize)]
struct SerializableMeal {
    id: String,
    name: String,
    meal_type: crate::models::MealType,
    nutrition: crate::models::Nutrition,
    ingredients: Vec<String>,
    tags: Vec<String>,
    prep_time: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    recipe: Option<crate::models::Recipe>,
}

impl SerializableMeal {
    fn from_meal(meal: &Meal, database: &MealDatabase, show_recipe: bool) -> Self {
        let recipe = if show_recipe {
            database.get_recipe(&meal.id).cloned()
        } else {
            None
        };

        Self {
            id: meal.id.clone(),
            name: meal.name.clone(),
            meal_type: meal.meal_type,
            nutrition: meal.nutrition.clone(),
            ingredients: meal.ingredients.clone(),
            tags: meal.tags.clone(),
            prep_time: meal.prep_time,
            recipe,
        }
    }
}
