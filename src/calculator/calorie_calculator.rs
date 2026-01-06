use crate::config::{ActivityLevel, Gender};
use crate::models::Goal;

/// カロリー計算機
pub struct CalorieCalculator;

impl CalorieCalculator {
    /// Harris-Benedict式でBMR (基礎代謝量) を計算
    ///
    /// # Arguments
    /// * `weight` - 体重 (kg)
    /// * `height` - 身長 (cm)
    /// * `age` - 年齢 (歳)
    /// * `gender` - 性別
    ///
    /// # Returns
    /// BMR (kcal/日)
    pub fn calculate_bmr(weight: f32, height: f32, age: u32, gender: Gender) -> f32 {
        match gender {
            Gender::Male => {
                // 男性: 88.362 + (13.397 × 体重kg) + (4.799 × 身長cm) - (5.677 × 年齢)
                88.362 + (13.397 * weight) + (4.799 * height) - (5.677 * age as f32)
            }
            Gender::Female => {
                // 女性: 447.593 + (9.247 × 体重kg) + (3.098 × 身長cm) - (4.330 × 年齢)
                447.593 + (9.247 * weight) + (3.098 * height) - (4.330 * age as f32)
            }
        }
    }

    /// TDEE (1日の総消費カロリー) を計算
    ///
    /// # Arguments
    /// * `bmr` - 基礎代謝量 (kcal/日)
    /// * `activity_level` - 活動レベル
    ///
    /// # Returns
    /// TDEE (kcal/日)
    pub fn calculate_tdee(bmr: f32, activity_level: ActivityLevel) -> f32 {
        bmr * activity_level.multiplier()
    }

    /// 目的別のカロリー目標を計算
    ///
    /// # Arguments
    /// * `tdee` - 1日の総消費カロリー (kcal/日)
    /// * `goal` - トレーニング目的
    ///
    /// # Returns
    /// カロリー目標 (kcal/日)
    pub fn calculate_target_calories(tdee: f32, goal: Goal) -> f32 {
        match goal {
            Goal::Bulk => tdee + 300.0,     // 増量: +300kcal
            Goal::Cut => tdee - 500.0,      // 減量: -500kcal
            Goal::Maintain => tdee,         // 維持: そのまま
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bmr_male() {
        // 男性: 25歳, 70kg, 175cm
        let bmr = CalorieCalculator::calculate_bmr(70.0, 175.0, 25, Gender::Male);

        // 期待値: 88.362 + (13.397 * 70) + (4.799 * 175) - (5.677 * 25)
        //       = 88.362 + 937.79 + 839.825 - 141.925
        //       = 1724.052
        assert!((bmr - 1724.0).abs() < 1.0);
    }

    #[test]
    fn test_bmr_female() {
        // 女性: 28歳, 55kg, 160cm
        let bmr = CalorieCalculator::calculate_bmr(55.0, 160.0, 28, Gender::Female);

        // 期待値: 447.593 + (9.247 * 55) + (3.098 * 160) - (4.330 * 28)
        //       = 447.593 + 508.585 + 495.68 - 121.24
        //       = 1330.618
        assert!((bmr - 1330.0).abs() < 1.0);
    }

    #[test]
    fn test_tdee_moderate() {
        let bmr = 1700.0;
        let tdee = CalorieCalculator::calculate_tdee(bmr, ActivityLevel::Moderate);

        // 期待値: 1700 * 1.55 = 2635
        assert!((tdee - 2635.0).abs() < 1.0);
    }

    #[test]
    fn test_target_calories_bulk() {
        let tdee = 2500.0;
        let target = CalorieCalculator::calculate_target_calories(tdee, Goal::Bulk);

        // 期待値: 2500 + 300 = 2800
        assert_eq!(target, 2800.0);
    }

    #[test]
    fn test_target_calories_cut() {
        let tdee = 2500.0;
        let target = CalorieCalculator::calculate_target_calories(tdee, Goal::Cut);

        // 期待値: 2500 - 500 = 2000
        assert_eq!(target, 2000.0);
    }

    #[test]
    fn test_target_calories_maintain() {
        let tdee = 2500.0;
        let target = CalorieCalculator::calculate_target_calories(tdee, Goal::Maintain);

        // 期待値: 2500
        assert_eq!(target, 2500.0);
    }
}
