use serde::{Deserialize, Serialize};

/// レシピ (調理手順)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recipe {
    pub meal_id: String,
    pub steps: Vec<String>, // 調理手順
}

impl Recipe {
    pub fn new(meal_id: impl Into<String>, steps: Vec<String>) -> Self {
        Self {
            meal_id: meal_id.into(),
            steps,
        }
    }
}
