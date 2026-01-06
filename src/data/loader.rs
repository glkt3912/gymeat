use crate::error::Result;
use crate::models::{Meal, MealType, Recipe};
use std::collections::HashMap;

/// メニューデータベース
pub struct MealDatabase {
    meals: Vec<Meal>,
    recipes: HashMap<String, Recipe>,
}

impl MealDatabase {
    /// 組み込みデータで新しいMealDatabaseを作成
    pub fn new_embedded() -> Result<Self> {
        let meals = super::meals_data::get_meals();

        // レシピデータを読み込み
        let mut recipes = HashMap::new();
        for meal in &meals {
            if let Some(recipe) = super::meals_data::get_recipe(&meal.id) {
                recipes.insert(meal.id.clone(), recipe);
            }
        }

        Ok(Self { meals, recipes })
    }

    /// 指定した食事タイプのメニューをフィルタ
    pub fn filter_by_type(&self, meal_type: MealType) -> Vec<&Meal> {
        self.meals
            .iter()
            .filter(|m| m.meal_type == meal_type)
            .collect()
    }

    /// レシピを取得
    pub fn get_recipe(&self, meal_id: &str) -> Option<&Recipe> {
        self.recipes.get(meal_id)
    }

    /// すべてのメニューを取得
    pub fn get_all_meals(&self) -> &[Meal] {
        &self.meals
    }

    /// メニュー数を取得
    pub fn count(&self) -> usize {
        self.meals.len()
    }

    /// 食事タイプ別のメニュー数を取得
    pub fn count_by_type(&self, meal_type: MealType) -> usize {
        self.meals
            .iter()
            .filter(|m| m.meal_type == meal_type)
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meal_database_creation() {
        let db = MealDatabase::new_embedded().unwrap();
        assert!(db.count() >= 20); // 最低20種類
    }

    #[test]
    fn test_filter_by_type() {
        let db = MealDatabase::new_embedded().unwrap();

        let breakfasts = db.filter_by_type(MealType::Breakfast);
        assert!(breakfasts.len() >= 5);

        let lunches = db.filter_by_type(MealType::Lunch);
        assert!(lunches.len() >= 5);

        let dinners = db.filter_by_type(MealType::Dinner);
        assert!(dinners.len() >= 5);

        let snacks = db.filter_by_type(MealType::Snack);
        assert!(snacks.len() >= 3);
    }

    #[test]
    fn test_get_recipe() {
        let db = MealDatabase::new_embedded().unwrap();

        let recipe = db.get_recipe("lunch_001");
        assert!(recipe.is_some());

        let recipe = db.get_recipe("nonexistent");
        assert!(recipe.is_none());
    }
}
