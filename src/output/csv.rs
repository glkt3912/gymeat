use super::formatter::OutputFormatter;
use crate::data::MealDatabase;
use crate::error::Result;
use crate::models::{DailyPlan, MealType, MonthlyPlan, WeeklyPlan};

/// CSV出力フォーマッター
#[derive(Default)]
pub struct CsvFormatter;

impl CsvFormatter {
    pub fn new() -> Self {
        Self
    }
}

impl OutputFormatter for CsvFormatter {
    fn format_daily_plan(
        &self,
        plan: &DailyPlan,
        _database: &MealDatabase,
        _show_recipe: bool,
    ) -> Result<String> {
        let mut csv = String::new();

        // ヘッダー行
        csv.push_str("Date,MealType,MealName,Calories,Protein,Fat,Carbs,PrepTime,Tags\n");

        // データ行
        let date_str = plan.date.as_deref().unwrap_or("");
        for meal in &plan.meals {
            csv.push_str(&format!(
                "{},{},{},{:.1},{:.1},{:.1},{:.1},{},{}\n",
                escape_csv(date_str),
                meal_type_label(meal.meal_type),
                escape_csv(&meal.name),
                meal.nutrition.calories,
                meal.nutrition.protein,
                meal.nutrition.fat,
                meal.nutrition.carbohydrates,
                meal.prep_time,
                escape_csv(&meal.tags.join("; "))
            ));
        }

        // サマリー行
        csv.push_str(&format!(
            "{},TOTAL,Daily Total,{:.1},{:.1},{:.1},{:.1},,\n",
            escape_csv(date_str),
            plan.total_nutrition.calories,
            plan.total_nutrition.protein,
            plan.total_nutrition.fat,
            plan.total_nutrition.carbohydrates
        ));

        // 目標値サマリー
        csv.push_str(&format!(
            "{},TARGET,Daily Target,{:.1},{:.1},{:.1},{:.1},,\n",
            escape_csv(date_str),
            plan.target.daily_calories,
            plan.target.protein_grams,
            plan.target.fat_grams,
            plan.target.carbs_grams
        ));

        Ok(csv)
    }

    fn format_weekly_plan(
        &self,
        plan: &WeeklyPlan,
        _database: &MealDatabase,
        _show_recipe: bool,
    ) -> Result<String> {
        let mut csv = String::new();

        // ヘッダー行
        csv.push_str("Date,MealType,MealName,Calories,Protein,Fat,Carbs,PrepTime,Tags\n");

        // 各日のプランを順次追加
        for daily_plan in &plan.daily_plans {
            let date_str = daily_plan.date.as_deref().unwrap_or("");

            for meal in &daily_plan.meals {
                csv.push_str(&format!(
                    "{},{},{},{:.1},{:.1},{:.1},{:.1},{},{}\n",
                    escape_csv(date_str),
                    meal_type_label(meal.meal_type),
                    escape_csv(&meal.name),
                    meal.nutrition.calories,
                    meal.nutrition.protein,
                    meal.nutrition.fat,
                    meal.nutrition.carbohydrates,
                    meal.prep_time,
                    escape_csv(&meal.tags.join("; "))
                ));
            }

            // 日次サマリー
            csv.push_str(&format!(
                "{},DAILY_TOTAL,Daily Total,{:.1},{:.1},{:.1},{:.1},,\n",
                escape_csv(date_str),
                daily_plan.total_nutrition.calories,
                daily_plan.total_nutrition.protein,
                daily_plan.total_nutrition.fat,
                daily_plan.total_nutrition.carbohydrates
            ));
        }

        // 週間サマリー
        let weekly_total = plan.total_nutrition();
        csv.push_str(&format!(
            "{},WEEKLY_TOTAL,Weekly Total,{:.1},{:.1},{:.1},{:.1},,\n",
            escape_csv(&plan.start_date),
            weekly_total.calories,
            weekly_total.protein,
            weekly_total.fat,
            weekly_total.carbohydrates
        ));

        // 週間平均
        let daily_avg = plan.average_nutrition();
        csv.push_str(&format!(
            "{},DAILY_AVERAGE,Daily Average,{:.1},{:.1},{:.1},{:.1},,\n",
            escape_csv(&plan.start_date),
            daily_avg.calories,
            daily_avg.protein,
            daily_avg.fat,
            daily_avg.carbohydrates
        ));

        // 目標値
        csv.push_str(&format!(
            "{},TARGET,Daily Target,{:.1},{:.1},{:.1},{:.1},,\n",
            escape_csv(&plan.start_date),
            plan.target.daily_calories,
            plan.target.protein_grams,
            plan.target.fat_grams,
            plan.target.carbs_grams
        ));

        Ok(csv)
    }

    fn format_monthly_plan(
        &self,
        plan: &MonthlyPlan,
        _database: &MealDatabase,
        _show_recipe: bool,
    ) -> Result<String> {
        let mut csv = String::new();

        // ヘッダー行
        csv.push_str("Date,MealType,MealName,Calories,Protein,Fat,Carbs,PrepTime,Tags\n");

        // 各日のプランを順次追加
        for daily_plan in &plan.daily_plans {
            let date_str = daily_plan.date.as_deref().unwrap_or("");

            for meal in &daily_plan.meals {
                csv.push_str(&format!(
                    "{},{},{},{:.1},{:.1},{:.1},{:.1},{},{}\n",
                    escape_csv(date_str),
                    meal_type_label(meal.meal_type),
                    escape_csv(&meal.name),
                    meal.nutrition.calories,
                    meal.nutrition.protein,
                    meal.nutrition.fat,
                    meal.nutrition.carbohydrates,
                    meal.prep_time,
                    escape_csv(&meal.tags.join("; "))
                ));
            }

            // 日次サマリー
            csv.push_str(&format!(
                "{},DAILY_TOTAL,Daily Total,{:.1},{:.1},{:.1},{:.1},,\n",
                escape_csv(date_str),
                daily_plan.total_nutrition.calories,
                daily_plan.total_nutrition.protein,
                daily_plan.total_nutrition.fat,
                daily_plan.total_nutrition.carbohydrates
            ));
        }

        // 月間サマリー
        let monthly_total = plan.total_nutrition();
        csv.push_str(&format!(
            "{},MONTHLY_TOTAL,Monthly Total,{:.1},{:.1},{:.1},{:.1},,\n",
            escape_csv(&plan.start_date),
            monthly_total.calories,
            monthly_total.protein,
            monthly_total.fat,
            monthly_total.carbohydrates
        ));

        // 月間平均
        let daily_avg = plan.average_nutrition();
        csv.push_str(&format!(
            "{},DAILY_AVERAGE,Daily Average,{:.1},{:.1},{:.1},{:.1},,\n",
            escape_csv(&plan.start_date),
            daily_avg.calories,
            daily_avg.protein,
            daily_avg.fat,
            daily_avg.carbohydrates
        ));

        // 目標値
        csv.push_str(&format!(
            "{},TARGET,Daily Target,{:.1},{:.1},{:.1},{:.1},,\n",
            escape_csv(&plan.start_date),
            plan.target.daily_calories,
            plan.target.protein_grams,
            plan.target.fat_grams,
            plan.target.carbs_grams
        ));

        Ok(csv)
    }

    fn format_name(&self) -> &'static str {
        "csv"
    }
}

/// CSV用の文字列エスケープ
fn escape_csv(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}

/// 食事タイプのラベル
fn meal_type_label(meal_type: MealType) -> &'static str {
    match meal_type {
        MealType::Breakfast => "Breakfast",
        MealType::Lunch => "Lunch",
        MealType::Dinner => "Dinner",
        MealType::Snack => "Snack",
    }
}
