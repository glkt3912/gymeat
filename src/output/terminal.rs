use crate::constants::{ACHIEVEMENT_LOWER_BOUND, ACHIEVEMENT_UPPER_BOUND};
use crate::data::MealDatabase;
use crate::models::{DailyPlan, Goal, Meal, MealType, MonthlyPlan, Nutrition, WeeklyPlan};
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
        println!(
            "  カロリー: {} kcal",
            self.colorize_value(&format!("{:.0}", target.daily_calories))
        );
        println!(
            "  タンパク質: {}g ({}%)",
            self.colorize_value(&format!("{:.0}", target.protein_grams)),
            self.calculate_percentage(target.protein_grams * 4.0, target.daily_calories)
        );
        println!(
            "  脂質: {}g ({}%)",
            self.colorize_value(&format!("{:.0}", target.fat_grams)),
            self.calculate_percentage(target.fat_grams * 9.0, target.daily_calories)
        );
        println!(
            "  炭水化物: {}g ({}%)",
            self.colorize_value(&format!("{:.0}", target.carbs_grams)),
            self.calculate_percentage(target.carbs_grams * 4.0, target.daily_calories)
        );
    }

    fn print_meals(&self, meals: &[Meal], database: &MealDatabase, show_recipe: bool) {
        for meal in meals {
            let meal_type_label = match meal.meal_type {
                MealType::Breakfast => "朝食",
                MealType::Lunch => "昼食",
                MealType::Dinner => "夕食",
                MealType::Snack => "間食",
            };

            println!(
                "\n【{}】{}",
                self.colorize_label(meal_type_label),
                self.colorize_meal_name(&meal.name)
            );
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
        println!(
            "  \n  栄養: {} kcal | P: {}g | F: {}g | C: {}g",
            self.colorize_value(&format!("{:.0}", nutrition.calories)),
            self.colorize_value(&format!("{:.0}", nutrition.protein)),
            self.colorize_value(&format!("{:.0}", nutrition.fat)),
            self.colorize_value(&format!("{:.0}", nutrition.carbohydrates))
        );
    }

    fn print_summary(&self, actual: &Nutrition, target: &crate::models::MacroTarget) {
        println!("\n合計栄養:");

        let cal_pct = (actual.calories / target.daily_calories) * 100.0;
        let pro_pct = (actual.protein / target.protein_grams) * 100.0;
        let fat_pct = (actual.fat / target.fat_grams) * 100.0;
        let carb_pct = (actual.carbohydrates / target.carbs_grams) * 100.0;

        println!(
            "  カロリー: {} kcal ({}%)",
            self.colorize_value(&format!("{:.0}", actual.calories)),
            self.colorize_percentage(cal_pct)
        );

        println!(
            "  タンパク質: {} g ({}%)",
            self.colorize_value(&format!("{:.0}", actual.protein)),
            self.colorize_percentage(pro_pct)
        );

        println!(
            "  脂質: {} g ({}%)",
            self.colorize_value(&format!("{:.0}", actual.fat)),
            self.colorize_percentage(fat_pct)
        );

        println!(
            "  炭水化物: {} g ({}%)",
            self.colorize_value(&format!("{:.0}", actual.carbohydrates)),
            self.colorize_percentage(carb_pct)
        );

        // アドバイス表示
        self.print_advice(cal_pct, pro_pct, &target.goal);
    }

    fn print_advice(&self, cal_pct: f32, pro_pct: f32, goal: &Goal) {
        println!("\n📊 評価:");

        // カロリー達成度
        if (ACHIEVEMENT_LOWER_BOUND as f32..=ACHIEVEMENT_UPPER_BOUND as f32).contains(&cal_pct) {
            println!("  ✅ カロリー目標を達成しています!");
        } else if cal_pct < ACHIEVEMENT_LOWER_BOUND as f32 {
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
        if pro_pct >= ACHIEVEMENT_LOWER_BOUND as f32 {
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

        // 達成度範囲内: 緑、それ以外: 黄色
        if (ACHIEVEMENT_LOWER_BOUND as f32..=ACHIEVEMENT_UPPER_BOUND as f32).contains(&pct) {
            pct_str.green().to_string()
        } else {
            pct_str.yellow().to_string()
        }
    }

    fn colorize_day_header(&self, text: &str) -> String {
        if self.enable_color {
            text.bright_cyan().bold().to_string()
        } else {
            text.to_string()
        }
    }

    /// 週間プランを表示
    pub fn print_weekly_plan(
        &self,
        plan: &WeeklyPlan,
        _database: &MealDatabase,
        _show_recipe: bool,
    ) {
        self.print_weekly_header(&plan.target.goal, &plan.start_date);

        println!("\n{}", "━".repeat(80));
        self.print_target(&plan.target);
        println!("{}\n", "━".repeat(80));

        // 各日のプランを表示
        for (idx, daily_plan) in plan.daily_plans.iter().enumerate() {
            let day_label = format!("Day {} ({})", idx + 1, daily_plan.date.as_ref().unwrap());
            println!("\n{}", self.colorize_day_header(&day_label));
            println!("{}", "─".repeat(80));

            self.print_meals_compact(&daily_plan.meals);
            self.print_daily_summary(&daily_plan.total_nutrition, &daily_plan.target);

            if idx < plan.daily_plans.len() - 1 {
                println!("\n{}", "━".repeat(80));
            }
        }

        // 週間サマリー
        println!("\n{}", "━".repeat(80));
        self.print_weekly_summary(plan);
    }

    fn print_weekly_header(&self, goal: &Goal, start_date: &str) {
        let title = match goal {
            Goal::Bulk => "7日間食事プラン (増量モード)",
            Goal::Cut => "7日間食事プラン (減量モード)",
            Goal::Maintain => "7日間食事プラン (維持モード)",
        };

        println!("\n{}", "━".repeat(80));
        println!("     {}", self.colorize_title(title));
        println!("     開始日: {}", start_date);
        println!("{}", "━".repeat(80));
    }

    /// コンパクトな食事表示 (週間ビュー用)
    fn print_meals_compact(&self, meals: &[Meal]) {
        for meal in meals {
            let meal_type_label = match meal.meal_type {
                MealType::Breakfast => "朝",
                MealType::Lunch => "昼",
                MealType::Dinner => "夕",
                MealType::Snack => "間",
            };

            println!(
                "  [{}] {} ({:.0} kcal | P:{:.0}g F:{:.0}g C:{:.0}g)",
                self.colorize_label(meal_type_label),
                self.colorize_meal_name(&meal.name),
                meal.nutrition.calories,
                meal.nutrition.protein,
                meal.nutrition.fat,
                meal.nutrition.carbohydrates,
            );
        }
    }

    /// 日次サマリー (簡易版)
    fn print_daily_summary(&self, actual: &Nutrition, target: &crate::models::MacroTarget) {
        let cal_pct = (actual.calories / target.daily_calories) * 100.0;
        println!(
            "  合計: {} kcal ({}%)",
            self.colorize_value(&format!("{:.0}", actual.calories)),
            self.colorize_percentage(cal_pct),
        );
    }

    /// 週間サマリー
    fn print_weekly_summary(&self, plan: &WeeklyPlan) {
        println!("\n週間統計:");

        let avg = plan.average_nutrition();
        let total = plan.total_nutrition();

        println!("  1日平均:");
        println!(
            "    カロリー: {} kcal",
            self.colorize_value(&format!("{:.0}", avg.calories))
        );
        println!(
            "    タンパク質: {} g",
            self.colorize_value(&format!("{:.0}", avg.protein))
        );
        println!(
            "    脂質: {} g",
            self.colorize_value(&format!("{:.0}", avg.fat))
        );
        println!(
            "    炭水化物: {} g",
            self.colorize_value(&format!("{:.0}", avg.carbohydrates))
        );

        println!("\n  週間合計:");
        println!(
            "    カロリー: {} kcal",
            self.colorize_value(&format!("{:.0}", total.calories))
        );

        // 目標達成率
        let target_weekly_cal = plan.target.daily_calories * 7.0;
        let cal_pct = (total.calories / target_weekly_cal) * 100.0;
        println!("    目標達成率: {}%", self.colorize_percentage(cal_pct));
    }

    /// 月間プランを表示
    pub fn print_monthly_plan(
        &self,
        plan: &MonthlyPlan,
        _database: &MealDatabase,
        _show_recipe: bool,
    ) {
        self.print_monthly_header(&plan.target.goal, &plan.start_date);

        println!("\n{}", "━".repeat(80));
        self.print_target(&plan.target);
        println!("{}\n", "━".repeat(80));

        // 各日のプランを表示
        for (idx, daily_plan) in plan.daily_plans.iter().enumerate() {
            let day_label = format!("Day {} ({})", idx + 1, daily_plan.date.as_ref().unwrap());
            println!("\n{}", self.colorize_day_header(&day_label));
            println!("{}", "─".repeat(80));

            self.print_meals_compact(&daily_plan.meals);
            self.print_daily_summary(&daily_plan.total_nutrition, &daily_plan.target);

            if idx < plan.daily_plans.len() - 1 {
                println!("\n{}", "━".repeat(80));
            }
        }

        // 月間サマリー
        println!("\n{}", "━".repeat(80));
        self.print_monthly_summary(plan);
    }

    fn print_monthly_header(&self, goal: &Goal, start_date: &str) {
        let title = match goal {
            Goal::Bulk => "30日間食事プラン (増量モード)",
            Goal::Cut => "30日間食事プラン (減量モード)",
            Goal::Maintain => "30日間食事プラン (維持モード)",
        };

        println!("\n{}", "━".repeat(80));
        println!("     {}", self.colorize_title(title));
        println!("     開始日: {}", start_date);
        println!("{}", "━".repeat(80));
    }

    /// 月間サマリー
    fn print_monthly_summary(&self, plan: &MonthlyPlan) {
        println!("\n月間統計:");

        let avg = plan.average_nutrition();
        let total = plan.total_nutrition();

        println!("  1日平均:");
        println!(
            "    カロリー: {} kcal",
            self.colorize_value(&format!("{:.0}", avg.calories))
        );
        println!(
            "    タンパク質: {} g",
            self.colorize_value(&format!("{:.0}", avg.protein))
        );
        println!(
            "    脂質: {} g",
            self.colorize_value(&format!("{:.0}", avg.fat))
        );
        println!(
            "    炭水化物: {} g",
            self.colorize_value(&format!("{:.0}", avg.carbohydrates))
        );

        println!("\n  月間合計:");
        println!(
            "    カロリー: {} kcal",
            self.colorize_value(&format!("{:.0}", total.calories))
        );

        // 目標達成率
        let days = plan.daily_plans.len() as f32;
        let target_monthly_cal = plan.target.daily_calories * days;
        let cal_pct = (total.calories / target_monthly_cal) * 100.0;
        println!("    目標達成率: {}%", self.colorize_percentage(cal_pct));
    }
}
