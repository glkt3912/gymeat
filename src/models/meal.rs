use serde::{Deserialize, Serialize};
use super::nutrition::Nutrition;

/// 食事タイプ
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MealType {
    Breakfast,  // 朝食
    Lunch,      // 昼食
    Dinner,     // 夕食
    Snack,      // 間食
}

/// 食事データ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meal {
    pub id: String,
    pub name: String,              // 料理名 (例: "鶏胸肉のグリル")
    pub ingredients: Vec<String>,  // 食材リスト
    pub nutrition: Nutrition,      // 栄養情報
    pub meal_type: MealType,       // 朝食/昼食/夕食/間食
    pub tags: Vec<String>,         // タグ (例: ["高タンパク", "低脂肪"])
    pub prep_time: u32,            // 調理時間 (分)
}

impl Meal {
    /// 新しいMealを作成するヘルパー関数
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        ingredients: Vec<String>,
        nutrition: Nutrition,
        meal_type: MealType,
        tags: Vec<String>,
        prep_time: u32,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            ingredients,
            nutrition,
            meal_type,
            tags,
            prep_time,
        }
    }
}
