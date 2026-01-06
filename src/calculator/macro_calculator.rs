use crate::models::{Goal, MacroTarget};

/// マクロ栄養素計算機
pub struct MacroCalculator;

impl MacroCalculator {
    /// 目的別のマクロ栄養素配分を計算
    ///
    /// # Arguments
    /// * `calories` - 1日のカロリー目標 (kcal)
    /// * `goal` - トレーニング目的
    ///
    /// # Returns
    /// マクロ栄養素の目標値
    pub fn calculate_macros(calories: f32, goal: Goal) -> MacroTarget {
        let (protein_ratio, fat_ratio, carb_ratio) = Self::get_macro_ratios(goal);

        MacroTarget::new(
            goal,
            calories,
            Self::calories_to_protein_grams(calories * protein_ratio),
            Self::calories_to_fat_grams(calories * fat_ratio),
            Self::calories_to_carbs_grams(calories * carb_ratio),
        )
    }

    /// 目的別のマクロ栄養素比率を取得
    fn get_macro_ratios(goal: Goal) -> (f32, f32, f32) {
        match goal {
            // 増量: タンパク質25%, 脂質25%, 炭水化物50%
            Goal::Bulk => (0.25, 0.25, 0.50),

            // 減量: タンパク質40%, 脂質30%, 炭水化物30%
            Goal::Cut => (0.40, 0.30, 0.30),

            // 維持: タンパク質30%, 脂質25%, 炭水化物45%
            Goal::Maintain => (0.30, 0.25, 0.45),
        }
    }

    /// カロリーをタンパク質のグラム数に変換
    /// タンパク質: 4kcal/g
    fn calories_to_protein_grams(calories: f32) -> f32 {
        calories / 4.0
    }

    /// カロリーを脂質のグラム数に変換
    /// 脂質: 9kcal/g
    fn calories_to_fat_grams(calories: f32) -> f32 {
        calories / 9.0
    }

    /// カロリーを炭水化物のグラム数に変換
    /// 炭水化物: 4kcal/g
    fn calories_to_carbs_grams(calories: f32) -> f32 {
        calories / 4.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macro_calculator_bulk() {
        let calories = 2800.0;
        let target = MacroCalculator::calculate_macros(calories, Goal::Bulk);

        assert_eq!(target.daily_calories, 2800.0);
        assert_eq!(target.goal, Goal::Bulk);

        // タンパク質: 2800 * 0.25 / 4 = 175g
        assert!((target.protein_grams - 175.0).abs() < 0.1);

        // 脂質: 2800 * 0.25 / 9 = 77.78g
        assert!((target.fat_grams - 77.78).abs() < 0.1);

        // 炭水化物: 2800 * 0.50 / 4 = 350g
        assert!((target.carbs_grams - 350.0).abs() < 0.1);
    }

    #[test]
    fn test_macro_calculator_cut() {
        let calories = 2000.0;
        let target = MacroCalculator::calculate_macros(calories, Goal::Cut);

        assert_eq!(target.daily_calories, 2000.0);
        assert_eq!(target.goal, Goal::Cut);

        // タンパク質: 2000 * 0.40 / 4 = 200g
        assert!((target.protein_grams - 200.0).abs() < 0.1);

        // 脂質: 2000 * 0.30 / 9 = 66.67g
        assert!((target.fat_grams - 66.67).abs() < 0.1);

        // 炭水化物: 2000 * 0.30 / 4 = 150g
        assert!((target.carbs_grams - 150.0).abs() < 0.1);
    }

    #[test]
    fn test_macro_calculator_maintain() {
        let calories = 2400.0;
        let target = MacroCalculator::calculate_macros(calories, Goal::Maintain);

        assert_eq!(target.daily_calories, 2400.0);
        assert_eq!(target.goal, Goal::Maintain);

        // タンパク質: 2400 * 0.30 / 4 = 180g
        assert!((target.protein_grams - 180.0).abs() < 0.1);

        // 脂質: 2400 * 0.25 / 9 = 66.67g
        assert!((target.fat_grams - 66.67).abs() < 0.1);

        // 炭水化物: 2400 * 0.45 / 4 = 270g
        assert!((target.carbs_grams - 270.0).abs() < 0.1);
    }
}
