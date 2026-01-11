use super::formatter::OutputFormatter;
use crate::data::MealDatabase;
use crate::error::Result;
use crate::models::{DailyPlan, Goal, MealType, MonthlyPlan, WeeklyPlan};

/// Markdown出力フォーマッター
#[derive(Default)]
pub struct MarkdownFormatter;

impl MarkdownFormatter {
    pub fn new() -> Self {
        Self
    }
}

impl OutputFormatter for MarkdownFormatter {
    fn format_daily_plan(
        &self,
        plan: &DailyPlan,
        database: &MealDatabase,
        show_recipe: bool,
    ) -> Result<String> {
        let mut md = String::new();

        // ヘッダー
        md.push_str(&format!(
            "# 筋トレ用食事メニュー ({})\n\n",
            goal_label(plan.target.goal)
        ));

        if let Some(date) = &plan.date {
            md.push_str(&format!("**日付:** {}\n\n", date));
        }

        // 目標栄養素テーブル
        md.push_str("## 目標栄養素\n\n");
        md.push_str("| 項目 | 目標値 |\n");
        md.push_str("|------|--------|\n");
        md.push_str(&format!(
            "| カロリー | {:.0} kcal |\n",
            plan.target.daily_calories
        ));
        md.push_str(&format!(
            "| タンパク質 | {:.0}g |\n",
            plan.target.protein_grams
        ));
        md.push_str(&format!("| 脂質 | {:.0}g |\n", plan.target.fat_grams));
        md.push_str(&format!(
            "| 炭水化物 | {:.0}g |\n\n",
            plan.target.carbs_grams
        ));

        // メニュー一覧
        md.push_str("## 本日のメニュー\n\n");
        for meal in &plan.meals {
            md.push_str(&format!(
                "### {} - {}\n\n",
                meal_type_label(meal.meal_type),
                meal.name
            ));

            // 栄養情報テーブル
            md.push_str("| カロリー | タンパク質 | 脂質 | 炭水化物 | 調理時間 |\n");
            md.push_str("|----------|------------|------|----------|----------|\n");
            md.push_str(&format!(
                "| {:.0} kcal | {:.0}g | {:.0}g | {:.0}g | {}分 |\n\n",
                meal.nutrition.calories,
                meal.nutrition.protein,
                meal.nutrition.fat,
                meal.nutrition.carbohydrates,
                meal.prep_time
            ));

            // 食材リスト
            if !meal.ingredients.is_empty() {
                md.push_str("**食材:**\n\n");
                for ingredient in &meal.ingredients {
                    md.push_str(&format!("- {}\n", ingredient));
                }
                md.push('\n');
            }

            // タグ
            if !meal.tags.is_empty() {
                md.push_str(&format!("**タグ:** {}\n\n", meal.tags.join(", ")));
            }

            // レシピ
            if show_recipe {
                if let Some(recipe) = database.get_recipe(&meal.id) {
                    md.push_str("**調理手順:**\n\n");
                    for (i, step) in recipe.steps.iter().enumerate() {
                        md.push_str(&format!("{}. {}\n", i + 1, step));
                    }
                    md.push('\n');
                }
            }
        }

        // サマリー
        md.push_str("## 合計栄養\n\n");
        md.push_str("| 項目 | 実績 | 目標 | 達成率 |\n");
        md.push_str("|------|------|------|--------|\n");

        let cal_achievement = (plan.total_nutrition.calories / plan.target.daily_calories) * 100.0;
        md.push_str(&format!(
            "| カロリー | {:.0} kcal | {:.0} kcal | {:.1}% |\n",
            plan.total_nutrition.calories, plan.target.daily_calories, cal_achievement
        ));

        let protein_achievement =
            (plan.total_nutrition.protein / plan.target.protein_grams) * 100.0;
        md.push_str(&format!(
            "| タンパク質 | {:.0}g | {:.0}g | {:.1}% |\n",
            plan.total_nutrition.protein, plan.target.protein_grams, protein_achievement
        ));

        let fat_achievement = (plan.total_nutrition.fat / plan.target.fat_grams) * 100.0;
        md.push_str(&format!(
            "| 脂質 | {:.0}g | {:.0}g | {:.1}% |\n",
            plan.total_nutrition.fat, plan.target.fat_grams, fat_achievement
        ));

        let carbs_achievement =
            (plan.total_nutrition.carbohydrates / plan.target.carbs_grams) * 100.0;
        md.push_str(&format!(
            "| 炭水化物 | {:.0}g | {:.0}g | {:.1}% |\n\n",
            plan.total_nutrition.carbohydrates, plan.target.carbs_grams, carbs_achievement
        ));

        Ok(md)
    }

    fn format_weekly_plan(
        &self,
        plan: &WeeklyPlan,
        database: &MealDatabase,
        show_recipe: bool,
    ) -> Result<String> {
        let mut md = String::new();

        // ヘッダー
        md.push_str(&format!(
            "# 週間食事プラン ({})\n\n",
            goal_label(plan.target.goal)
        ));
        md.push_str(&format!("**期間:** {} から7日間\n\n", plan.start_date));

        // 目標栄養素テーブル
        md.push_str("## 1日あたりの目標栄養素\n\n");
        md.push_str("| 項目 | 目標値 |\n");
        md.push_str("|------|--------|\n");
        md.push_str(&format!(
            "| カロリー | {:.0} kcal |\n",
            plan.target.daily_calories
        ));
        md.push_str(&format!(
            "| タンパク質 | {:.0}g |\n",
            plan.target.protein_grams
        ));
        md.push_str(&format!("| 脂質 | {:.0}g |\n", plan.target.fat_grams));
        md.push_str(&format!(
            "| 炭水化物 | {:.0}g |\n\n",
            plan.target.carbs_grams
        ));

        // 各日のプラン
        for (i, daily_plan) in plan.daily_plans.iter().enumerate() {
            md.push_str(&format!(
                "\n---\n\n## Day {} - {}\n\n",
                i + 1,
                daily_plan.date.as_deref().unwrap_or("")
            ));

            // 各食事
            for meal in &daily_plan.meals {
                md.push_str(&format!(
                    "### {} - {}\n\n",
                    meal_type_label(meal.meal_type),
                    meal.name
                ));

                // 栄養情報テーブル
                md.push_str("| カロリー | タンパク質 | 脂質 | 炭水化物 | 調理時間 |\n");
                md.push_str("|----------|------------|------|----------|----------|\n");
                md.push_str(&format!(
                    "| {:.0} kcal | {:.0}g | {:.0}g | {:.0}g | {}分 |\n\n",
                    meal.nutrition.calories,
                    meal.nutrition.protein,
                    meal.nutrition.fat,
                    meal.nutrition.carbohydrates,
                    meal.prep_time
                ));

                // 食材リスト
                if !meal.ingredients.is_empty() {
                    md.push_str("**食材:**\n\n");
                    for ingredient in &meal.ingredients {
                        md.push_str(&format!("- {}\n", ingredient));
                    }
                    md.push('\n');
                }

                // レシピ
                if show_recipe {
                    if let Some(recipe) = database.get_recipe(&meal.id) {
                        md.push_str("**調理手順:**\n\n");
                        for (j, step) in recipe.steps.iter().enumerate() {
                            md.push_str(&format!("{}. {}\n", j + 1, step));
                        }
                        md.push('\n');
                    }
                }
            }

            // 日次サマリー
            md.push_str("### 1日の合計\n\n");
            md.push_str("| カロリー | タンパク質 | 脂質 | 炭水化物 |\n");
            md.push_str("|----------|------------|------|----------|\n");
            md.push_str(&format!(
                "| {:.0} kcal | {:.0}g | {:.0}g | {:.0}g |\n\n",
                daily_plan.total_nutrition.calories,
                daily_plan.total_nutrition.protein,
                daily_plan.total_nutrition.fat,
                daily_plan.total_nutrition.carbohydrates
            ));
        }

        // 週間サマリー
        md.push_str("\n---\n\n## 週間サマリー\n\n");

        let weekly_total = plan.total_nutrition();
        let daily_avg = plan.average_nutrition();

        md.push_str("### 週間合計\n\n");
        md.push_str("| カロリー | タンパク質 | 脂質 | 炭水化物 |\n");
        md.push_str("|----------|------------|------|----------|\n");
        md.push_str(&format!(
            "| {:.0} kcal | {:.0}g | {:.0}g | {:.0}g |\n\n",
            weekly_total.calories,
            weekly_total.protein,
            weekly_total.fat,
            weekly_total.carbohydrates
        ));

        md.push_str("### 1日平均\n\n");
        md.push_str("| カロリー | タンパク質 | 脂質 | 炭水化物 |\n");
        md.push_str("|----------|------------|------|----------|\n");
        md.push_str(&format!(
            "| {:.0} kcal | {:.0}g | {:.0}g | {:.0}g |\n\n",
            daily_avg.calories, daily_avg.protein, daily_avg.fat, daily_avg.carbohydrates
        ));

        md.push_str("### 目標との比較 (1日平均)\n\n");
        md.push_str("| 項目 | 平均実績 | 目標 | 達成率 |\n");
        md.push_str("|------|----------|------|--------|\n");

        let cal_achievement = (daily_avg.calories / plan.target.daily_calories) * 100.0;
        md.push_str(&format!(
            "| カロリー | {:.0} kcal | {:.0} kcal | {:.1}% |\n",
            daily_avg.calories, plan.target.daily_calories, cal_achievement
        ));

        let protein_achievement = (daily_avg.protein / plan.target.protein_grams) * 100.0;
        md.push_str(&format!(
            "| タンパク質 | {:.0}g | {:.0}g | {:.1}% |\n",
            daily_avg.protein, plan.target.protein_grams, protein_achievement
        ));

        let fat_achievement = (daily_avg.fat / plan.target.fat_grams) * 100.0;
        md.push_str(&format!(
            "| 脂質 | {:.0}g | {:.0}g | {:.1}% |\n",
            daily_avg.fat, plan.target.fat_grams, fat_achievement
        ));

        let carbs_achievement = (daily_avg.carbohydrates / plan.target.carbs_grams) * 100.0;
        md.push_str(&format!(
            "| 炭水化物 | {:.0}g | {:.0}g | {:.1}% |\n\n",
            daily_avg.carbohydrates, plan.target.carbs_grams, carbs_achievement
        ));

        Ok(md)
    }

    fn format_monthly_plan(
        &self,
        plan: &MonthlyPlan,
        database: &MealDatabase,
        show_recipe: bool,
    ) -> Result<String> {
        let mut md = String::new();

        // ヘッダー
        md.push_str(&format!(
            "# 月間食事プラン ({})\n\n",
            goal_label(plan.target.goal)
        ));
        md.push_str(&format!("**期間:** {} から30日間\n\n", plan.start_date));

        // 目標栄養素テーブル
        md.push_str("## 1日あたりの目標栄養素\n\n");
        md.push_str("| 項目 | 目標値 |\n");
        md.push_str("|------|--------|\n");
        md.push_str(&format!(
            "| カロリー | {:.0} kcal |\n",
            plan.target.daily_calories
        ));
        md.push_str(&format!(
            "| タンパク質 | {:.0}g |\n",
            plan.target.protein_grams
        ));
        md.push_str(&format!("| 脂質 | {:.0}g |\n", plan.target.fat_grams));
        md.push_str(&format!(
            "| 炭水化物 | {:.0}g |\n\n",
            plan.target.carbs_grams
        ));

        // 各日のプラン
        for (i, daily_plan) in plan.daily_plans.iter().enumerate() {
            md.push_str(&format!(
                "\n---\n\n## Day {} - {}\n\n",
                i + 1,
                daily_plan.date.as_deref().unwrap_or("")
            ));

            // 各食事
            for meal in &daily_plan.meals {
                md.push_str(&format!(
                    "### {} - {}\n\n",
                    meal_type_label(meal.meal_type),
                    meal.name
                ));

                // 栄養情報テーブル
                md.push_str("| カロリー | タンパク質 | 脂質 | 炭水化物 | 調理時間 |\n");
                md.push_str("|----------|------------|------|----------|----------|\n");
                md.push_str(&format!(
                    "| {:.0} kcal | {:.0}g | {:.0}g | {:.0}g | {}分 |\n\n",
                    meal.nutrition.calories,
                    meal.nutrition.protein,
                    meal.nutrition.fat,
                    meal.nutrition.carbohydrates,
                    meal.prep_time
                ));

                // 食材リスト
                if !meal.ingredients.is_empty() {
                    md.push_str("**食材:**\n\n");
                    for ingredient in &meal.ingredients {
                        md.push_str(&format!("- {}\n", ingredient));
                    }
                    md.push('\n');
                }

                // レシピ
                if show_recipe {
                    if let Some(recipe) = database.get_recipe(&meal.id) {
                        md.push_str("**調理手順:**\n\n");
                        for (j, step) in recipe.steps.iter().enumerate() {
                            md.push_str(&format!("{}. {}\n", j + 1, step));
                        }
                        md.push('\n');
                    }
                }
            }

            // 日次サマリー
            md.push_str("### 1日の合計\n\n");
            md.push_str("| カロリー | タンパク質 | 脂質 | 炭水化物 |\n");
            md.push_str("|----------|------------|------|----------|\n");
            md.push_str(&format!(
                "| {:.0} kcal | {:.0}g | {:.0}g | {:.0}g |\n\n",
                daily_plan.total_nutrition.calories,
                daily_plan.total_nutrition.protein,
                daily_plan.total_nutrition.fat,
                daily_plan.total_nutrition.carbohydrates
            ));
        }

        // 月間サマリー
        md.push_str("\n---\n\n## 月間サマリー\n\n");

        let monthly_total = plan.total_nutrition();
        let daily_avg = plan.average_nutrition();

        md.push_str("### 月間合計\n\n");
        md.push_str("| カロリー | タンパク質 | 脂質 | 炭水化物 |\n");
        md.push_str("|----------|------------|------|----------|\n");
        md.push_str(&format!(
            "| {:.0} kcal | {:.0}g | {:.0}g | {:.0}g |\n\n",
            monthly_total.calories,
            monthly_total.protein,
            monthly_total.fat,
            monthly_total.carbohydrates
        ));

        md.push_str("### 1日平均\n\n");
        md.push_str("| カロリー | タンパク質 | 脂質 | 炭水化物 |\n");
        md.push_str("|----------|------------|------|----------|\n");
        md.push_str(&format!(
            "| {:.0} kcal | {:.0}g | {:.0}g | {:.0}g |\n\n",
            daily_avg.calories, daily_avg.protein, daily_avg.fat, daily_avg.carbohydrates
        ));

        md.push_str("### 目標との比較 (1日平均)\n\n");
        md.push_str("| 項目 | 平均実績 | 目標 | 達成率 |\n");
        md.push_str("|------|----------|------|--------|\n");

        let cal_achievement = (daily_avg.calories / plan.target.daily_calories) * 100.0;
        md.push_str(&format!(
            "| カロリー | {:.0} kcal | {:.0} kcal | {:.1}% |\n",
            daily_avg.calories, plan.target.daily_calories, cal_achievement
        ));

        let protein_achievement = (daily_avg.protein / plan.target.protein_grams) * 100.0;
        md.push_str(&format!(
            "| タンパク質 | {:.0}g | {:.0}g | {:.1}% |\n",
            daily_avg.protein, plan.target.protein_grams, protein_achievement
        ));

        let fat_achievement = (daily_avg.fat / plan.target.fat_grams) * 100.0;
        md.push_str(&format!(
            "| 脂質 | {:.0}g | {:.0}g | {:.1}% |\n",
            daily_avg.fat, plan.target.fat_grams, fat_achievement
        ));

        let carbs_achievement = (daily_avg.carbohydrates / plan.target.carbs_grams) * 100.0;
        md.push_str(&format!(
            "| 炭水化物 | {:.0}g | {:.0}g | {:.1}% |\n\n",
            daily_avg.carbohydrates, plan.target.carbs_grams, carbs_achievement
        ));

        Ok(md)
    }

    fn format_name(&self) -> &'static str {
        "markdown"
    }
}

/// 目標のラベル
fn goal_label(goal: Goal) -> &'static str {
    match goal {
        Goal::Bulk => "増量モード",
        Goal::Cut => "減量モード",
        Goal::Maintain => "維持モード",
    }
}

/// 食事タイプのラベル
fn meal_type_label(meal_type: MealType) -> &'static str {
    match meal_type {
        MealType::Breakfast => "朝食",
        MealType::Lunch => "昼食",
        MealType::Dinner => "夕食",
        MealType::Snack => "間食",
    }
}
