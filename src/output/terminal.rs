use crate::data::MealDatabase;
use crate::models::{DailyPlan, Goal, Meal, MealType, Nutrition};
use colored::*;

/// ターミナル出力
pub struct TerminalOutput {
    enable_color: bool,
}

impl TerminalOutput {
    pub fn new(enable_color: bool) -> Self {
        Self { enable_color }
    }

    /// 1日プランを表示
    pub fn print_daily_plan(&self, plan: &DailyPlan, database: &MealDatabase, show_recipe: bool) {
        self.print_header(&plan.target.goal);

        if let Some(date) = &plan.date {
            println!("日付: {}\n", date);
        }

        self.print_target(&plan.target);
        println!("\n{}", "━".repeat(50));

        self.print_meals(&plan.meals, database, show_recipe);

        println!("\n{}", "━".repeat(50));
        self.print_summary(&plan.total_nutrition, &plan.target);
    }

    fn print_header(&self, goal: &Goal) {
        let title = match goal {
            Goal::Bulk => "筋トレ用食事メニュー (増量モード)",
            Goal::Cut => "筋トレ用食事メニュー (減量モード)",
            Goal::Maintain => "筋トレ用食事メニュー (維持モード)",
        };

        println!("\n{}", "━".repeat(50));
        println!("     {}", self.colorize_title(title));
        println!("{}", "━".repeat(50));
    }

    fn print_target(&self, target: &crate::models::MacroTarget) {
        println!("\n目標栄養素:");
        println!("  カロリー: {} kcal", self.colorize_value(&format!("{:.0}", target.daily_calories)));
        println!("  タンパク質: {}g ({}%)",
                 self.colorize_value(&format!("{:.0}", target.protein_grams)),
                 self.calculate_percentage(target.protein_grams * 4.0, target.daily_calories));
        println!("  脂質: {}g ({}%)",
                 self.colorize_value(&format!("{:.0}", target.fat_grams)),
                 self.calculate_percentage(target.fat_grams * 9.0, target.daily_calories));
        println!("  炭水化物: {}g ({}%)",
                 self.colorize_value(&format!("{:.0}", target.carbs_grams)),
                 self.calculate_percentage(target.carbs_grams * 4.0, target.daily_calories));
    }

    fn print_meals(&self, meals: &[Meal], database: &MealDatabase, show_recipe: bool) {
        for meal in meals {
            let meal_type_label = match meal.meal_type {
                MealType::Breakfast => "朝食",
                MealType::Lunch => "昼食",
                MealType::Dinner => "夕食",
                MealType::Snack => "間食",
            };

            println!("\n【{}】{}",
                     self.colorize_label(meal_type_label),
                     self.colorize_meal_name(&meal.name));
            println!("  調理時間: {}分", meal.prep_time);

            if !meal.ingredients.is_empty() {
                println!("  食材:");
                for ingredient in &meal.ingredients {
                    println!("    - {}", ingredient);
                }
            }

            self.print_nutrition(&meal.nutrition);

            if !meal.tags.is_empty() {
                let tags = meal.tags.join(", ");
                println!("  タグ: {}", self.colorize_tags(&tags));
            }

            // レシピを表示
            if show_recipe {
                if let Some(recipe) = database.get_recipe(&meal.id) {
                    println!("\n  調理手順:");
                    for step in &recipe.steps {
                        println!("    {}", step);
                    }
                }
            }
        }
    }

    fn print_nutrition(&self, nutrition: &Nutrition) {
        println!("  \n  栄養: {} kcal | P: {}g | F: {}g | C: {}g",
                 self.colorize_value(&format!("{:.0}", nutrition.calories)),
                 self.colorize_value(&format!("{:.0}", nutrition.protein)),
                 self.colorize_value(&format!("{:.0}", nutrition.fat)),
                 self.colorize_value(&format!("{:.0}", nutrition.carbohydrates)));
    }

    fn print_summary(&self, actual: &Nutrition, target: &crate::models::MacroTarget) {
        println!("\n合計栄養:");

        let cal_pct = (actual.calories / target.daily_calories) * 100.0;
        let pro_pct = (actual.protein / target.protein_grams) * 100.0;
        let fat_pct = (actual.fat / target.fat_grams) * 100.0;
        let carb_pct = (actual.carbohydrates / target.carbs_grams) * 100.0;

        println!("  カロリー: {} kcal ({}%)",
                 self.colorize_value(&format!("{:.0}", actual.calories)),
                 self.colorize_percentage(cal_pct));

        println!("  タンパク質: {} g ({}%)",
                 self.colorize_value(&format!("{:.0}", actual.protein)),
                 self.colorize_percentage(pro_pct));

        println!("  脂質: {} g ({}%)",
                 self.colorize_value(&format!("{:.0}", actual.fat)),
                 self.colorize_percentage(fat_pct));

        println!("  炭水化物: {} g ({}%)",
                 self.colorize_value(&format!("{:.0}", actual.carbohydrates)),
                 self.colorize_percentage(carb_pct));

        // アドバイス表示
        self.print_advice(cal_pct, pro_pct, &target.goal);
    }

    fn print_advice(&self, cal_pct: f32, pro_pct: f32, goal: &Goal) {
        println!("\n📊 評価:");

        // カロリー達成度
        if (90.0..=110.0).contains(&cal_pct) {
            println!("  ✅ カロリー目標を達成しています!");
        } else if cal_pct < 90.0 {
            let diff = ((100.0 - cal_pct) as i32).abs();
            println!("  ⚠️  カロリーが目標より{}%少ないです", diff);
            println!("     💡 間食を追加するか、主食の量を増やしてみましょう");
        } else {
            let diff = (cal_pct - 100.0) as i32;
            println!("  ⚠️  カロリーが目標より{}%多いです", diff);
            if *goal == Goal::Cut {
                println!("     💡 減量中は目標カロリーを守ることが重要です");
            }
        }

        // タンパク質達成度
        if pro_pct >= 90.0 {
            println!("  ✅ タンパク質も十分に摂取できています");
        } else {
            println!("  ⚠️  タンパク質が不足しています ({}%)", pro_pct as i32);
            println!("     💡 プロテインシェイクや鶏胸肉を追加しましょう");
        }
    }

    fn calculate_percentage(&self, part: f32, total: f32) -> u32 {
        ((part / total) * 100.0).round() as u32
    }

    // カラー装飾メソッド
    fn colorize_title(&self, text: &str) -> String {
        if self.enable_color {
            text.bright_cyan().bold().to_string()
        } else {
            text.to_string()
        }
    }

    fn colorize_label(&self, text: &str) -> String {
        if self.enable_color {
            text.yellow().bold().to_string()
        } else {
            text.to_string()
        }
    }

    fn colorize_meal_name(&self, text: &str) -> String {
        if self.enable_color {
            text.bright_white().bold().to_string()
        } else {
            text.to_string()
        }
    }

    fn colorize_value(&self, text: &str) -> String {
        if self.enable_color {
            text.green().bold().to_string()
        } else {
            text.to_string()
        }
    }

    fn colorize_tags(&self, text: &str) -> String {
        if self.enable_color {
            text.bright_black().to_string()
        } else {
            text.to_string()
        }
    }

    fn colorize_percentage(&self, pct: f32) -> String {
        let pct_str = format!("{:.0}", pct);
        if !self.enable_color {
            return pct_str;
        }

        // 90-110%: 緑、それ以外: 黄色
        if (90.0..=110.0).contains(&pct) {
            pct_str.green().to_string()
        } else {
            pct_str.yellow().to_string()
        }
    }
}
