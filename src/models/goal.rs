use serde::{Deserialize, Serialize};

/// トレーニング目的
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Goal {
    Bulk,     // 増量
    Cut,      // 減量
    Maintain, // 維持
}

/// マクロ栄養素の目標値
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacroTarget {
    pub goal: Goal,
    pub daily_calories: f32,
    pub protein_grams: f32,
    pub fat_grams: f32,
    pub carbs_grams: f32,
}

impl MacroTarget {
    pub fn new(
        goal: Goal,
        daily_calories: f32,
        protein_grams: f32,
        fat_grams: f32,
        carbs_grams: f32,
    ) -> Self {
        Self {
            goal,
            daily_calories,
            protein_grams,
            fat_grams,
            carbs_grams,
        }
    }
}
