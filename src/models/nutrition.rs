use serde::{Deserialize, Serialize};

/// 栄養情報
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Nutrition {
    pub calories: f32,       // kcal
    pub protein: f32,        // g
    pub fat: f32,            // g
    pub carbohydrates: f32,  // g
}

impl Nutrition {
    /// 複数の栄養情報を合計
    pub fn sum(nutritions: &[&Nutrition]) -> Self {
        let mut total = Nutrition {
            calories: 0.0,
            protein: 0.0,
            fat: 0.0,
            carbohydrates: 0.0,
        };

        for nutrition in nutritions {
            total.calories += nutrition.calories;
            total.protein += nutrition.protein;
            total.fat += nutrition.fat;
            total.carbohydrates += nutrition.carbohydrates;
        }

        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nutrition_sum() {
        let n1 = Nutrition {
            calories: 100.0,
            protein: 10.0,
            fat: 5.0,
            carbohydrates: 15.0,
        };
        let n2 = Nutrition {
            calories: 200.0,
            protein: 20.0,
            fat: 10.0,
            carbohydrates: 30.0,
        };

        let total = Nutrition::sum(&[&n1, &n2]);

        assert_eq!(total.calories, 300.0);
        assert_eq!(total.protein, 30.0);
        assert_eq!(total.fat, 15.0);
        assert_eq!(total.carbohydrates, 45.0);
    }
}
