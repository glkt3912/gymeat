use crate::error::{MealPlannerError, Result};
use crate::models::Goal;

/// 性別
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gender {
    Male,
    Female,
}

/// 活動レベル
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivityLevel {
    Sedentary,  // 1.2  (ほぼ運動なし)
    Light,      // 1.375 (週1-3日)
    Moderate,   // 1.55 (週3-5日)
    Active,     // 1.725 (週6-7日)
    VeryActive, // 1.9  (1日2回以上)
}

impl ActivityLevel {
    /// 活動レベルの乗数を取得
    pub fn multiplier(&self) -> f32 {
        match self {
            ActivityLevel::Sedentary => 1.2,
            ActivityLevel::Light => 1.375,
            ActivityLevel::Moderate => 1.55,
            ActivityLevel::Active => 1.725,
            ActivityLevel::VeryActive => 1.9,
        }
    }
}

/// プラン生成の設定
#[derive(Debug, Clone)]
pub struct PlanConfig {
    pub goal: Goal,
    pub weight: Option<f32>,    // 体重 (kg)
    pub height: Option<f32>,    // 身長 (cm)
    pub age: Option<u32>,       // 年齢
    pub gender: Option<Gender>, // 性別
    pub activity_level: ActivityLevel,
    pub custom_calories: Option<f32>, // カスタムカロリー目標
}

impl PlanConfig {
    /// 新しいPlanConfigを作成
    pub fn new(goal: Goal) -> Self {
        Self {
            goal,
            weight: None,
            height: None,
            age: None,
            gender: None,
            activity_level: ActivityLevel::Moderate,
            custom_calories: None,
        }
    }

    /// 設定を検証
    pub fn validate(&self) -> Result<()> {
        // 体重の検証
        if let Some(w) = self.weight {
            if w <= 0.0 || w > 300.0 {
                return Err(MealPlannerError::InvalidWeight(w));
            }
        }

        // 身長の検証
        if let Some(h) = self.height {
            if !(100.0..=250.0).contains(&h) {
                return Err(MealPlannerError::InvalidHeight(h));
            }
        }

        // 年齢の検証
        if let Some(a) = self.age {
            if !(10..=100).contains(&a) {
                return Err(MealPlannerError::InvalidAge(a));
            }
        }

        // カスタムカロリーの検証
        if let Some(c) = self.custom_calories {
            if !(500.0..=10000.0).contains(&c) {
                return Err(MealPlannerError::InvalidCalories(c));
            }
        }

        // BMR計算に必要な情報が揃っているか確認
        if self.custom_calories.is_none()
            && (self.weight.is_some() || self.height.is_some() || self.age.is_some())
        {
            // いずれかが指定されている場合、すべて必要
            if self.weight.is_none()
                || self.height.is_none()
                || self.age.is_none()
                || self.gender.is_none()
            {
                return Err(MealPlannerError::ConfigValidationError(
                    "カロリー計算には体重、身長、年齢、性別のすべてが必要です。\
                     または --calories オプションでカロリーを直接指定してください。"
                        .to_string(),
                ));
            }
        }

        Ok(())
    }

    /// デフォルトのカロリー目標を取得 (体組成情報がない場合)
    pub fn default_calories(&self) -> f32 {
        match self.goal {
            Goal::Bulk => 2800.0,
            Goal::Cut => 2000.0,
            Goal::Maintain => 2400.0,
        }
    }
}

impl Default for PlanConfig {
    fn default() -> Self {
        Self::new(Goal::Maintain)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_config() {
        let config = PlanConfig {
            goal: Goal::Maintain,
            weight: Some(70.0),
            height: Some(175.0),
            age: Some(25),
            gender: Some(Gender::Male),
            activity_level: ActivityLevel::Moderate,
            custom_calories: None,
        };

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_invalid_weight() {
        let config = PlanConfig {
            goal: Goal::Maintain,
            weight: Some(400.0), // 無効
            height: Some(175.0),
            age: Some(25),
            gender: Some(Gender::Male),
            activity_level: ActivityLevel::Moderate,
            custom_calories: None,
        };

        assert!(config.validate().is_err());
    }

    #[test]
    fn test_custom_calories_only() {
        let config = PlanConfig {
            goal: Goal::Maintain,
            weight: None,
            height: None,
            age: None,
            gender: None,
            activity_level: ActivityLevel::Moderate,
            custom_calories: Some(2400.0),
        };

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_incomplete_body_stats() {
        let config = PlanConfig {
            goal: Goal::Maintain,
            weight: Some(70.0), // 体重だけ指定
            height: None,
            age: None,
            gender: None,
            activity_level: ActivityLevel::Moderate,
            custom_calories: None,
        };

        // 体重だけでは不十分
        assert!(config.validate().is_err());
    }
}
