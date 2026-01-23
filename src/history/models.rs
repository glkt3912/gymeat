use serde::{Deserialize, Serialize};

use crate::models::{DailyPlan, Goal, MonthlyPlan, WeeklyPlan};

/// プランの種類
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PlanType {
    Daily,
    Weekly,
    Monthly,
}

impl std::fmt::Display for PlanType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlanType::Daily => write!(f, "daily"),
            PlanType::Weekly => write!(f, "weekly"),
            PlanType::Monthly => write!(f, "monthly"),
        }
    }
}

/// 保存するプランデータ
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum PlanData {
    Daily(DailyPlan),
    Weekly(WeeklyPlan),
    Monthly(MonthlyPlan),
}

impl PlanData {
    pub fn plan_type(&self) -> PlanType {
        match self {
            PlanData::Daily(_) => PlanType::Daily,
            PlanData::Weekly(_) => PlanType::Weekly,
            PlanData::Monthly(_) => PlanType::Monthly,
        }
    }

    pub fn date_range(&self) -> (String, String) {
        match self {
            PlanData::Daily(plan) => {
                let date = plan.date.clone().unwrap_or_else(|| "unknown".to_string());
                (date.clone(), date)
            }
            PlanData::Weekly(plan) => {
                let start = plan.start_date.clone();
                // 7日後を計算
                if let Ok(start_date) = chrono::NaiveDate::parse_from_str(&start, "%Y-%m-%d") {
                    let end_date = start_date + chrono::Duration::days(6);
                    (start, end_date.format("%Y-%m-%d").to_string())
                } else {
                    (start.clone(), start)
                }
            }
            PlanData::Monthly(plan) => {
                let start = plan.start_date.clone();
                // 30日後を計算
                if let Ok(start_date) = chrono::NaiveDate::parse_from_str(&start, "%Y-%m-%d") {
                    let end_date = start_date + chrono::Duration::days(29);
                    (start, end_date.format("%Y-%m-%d").to_string())
                } else {
                    (start.clone(), start)
                }
            }
        }
    }
}

/// 履歴エントリ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    /// 一意なID (UUID v4)
    pub id: String,
    /// 作成日時 (ISO 8601)
    pub created_at: String,
    /// プランの種類
    pub plan_type: PlanType,
    /// 目的 (bulk/cut/maintain)
    pub goal: Goal,
    /// 目標カロリー
    pub target_calories: f32,
    /// プランデータ
    pub plan: PlanData,
}

impl HistoryEntry {
    pub fn new(goal: Goal, target_calories: f32, plan: PlanData) -> Self {
        let id = uuid::Uuid::new_v4().to_string();
        let created_at = chrono::Local::now().to_rfc3339();
        let plan_type = plan.plan_type();

        Self {
            id,
            created_at,
            plan_type,
            goal,
            target_calories,
            plan,
        }
    }

    /// IDの短縮表示 (先頭8文字)
    pub fn short_id(&self) -> &str {
        if self.id.len() >= 8 {
            &self.id[..8]
        } else {
            &self.id
        }
    }

    /// 作成日 (日付のみ)
    pub fn created_date(&self) -> String {
        if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&self.created_at) {
            dt.format("%Y-%m-%d").to_string()
        } else {
            self.created_at.clone()
        }
    }

    /// 作成時刻 (時刻のみ)
    pub fn created_time(&self) -> String {
        if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&self.created_at) {
            dt.format("%H:%M").to_string()
        } else {
            "".to_string()
        }
    }
}

/// 履歴インデックス (メタデータのみ)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryIndex {
    pub version: String,
    pub entries: Vec<HistoryIndexEntry>,
}

impl Default for HistoryIndex {
    fn default() -> Self {
        Self {
            version: "1.0".to_string(),
            entries: Vec::new(),
        }
    }
}

impl HistoryIndex {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_entry(&mut self, entry: &HistoryEntry) {
        let (start_date, end_date) = entry.plan.date_range();
        let index_entry = HistoryIndexEntry {
            id: entry.id.clone(),
            created_at: entry.created_at.clone(),
            plan_type: entry.plan_type,
            goal: entry.goal,
            target_calories: entry.target_calories,
            start_date,
            end_date,
        };
        self.entries.push(index_entry);
    }

    pub fn remove_entry(&mut self, id: &str) -> bool {
        let original_len = self.entries.len();
        self.entries.retain(|e| e.id != id);
        self.entries.len() != original_len
    }

    pub fn find_entry(&self, id: &str) -> Option<&HistoryIndexEntry> {
        // 完全一致または前方一致
        self.entries
            .iter()
            .find(|e| e.id == id || e.id.starts_with(id))
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

/// インデックスエントリ (メタデータのみ)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryIndexEntry {
    pub id: String,
    pub created_at: String,
    pub plan_type: PlanType,
    pub goal: Goal,
    pub target_calories: f32,
    pub start_date: String,
    pub end_date: String,
}

impl HistoryIndexEntry {
    /// IDの短縮表示 (先頭8文字)
    pub fn short_id(&self) -> &str {
        if self.id.len() >= 8 {
            &self.id[..8]
        } else {
            &self.id
        }
    }

    /// 作成日 (日付のみ)
    pub fn created_date(&self) -> String {
        if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&self.created_at) {
            dt.format("%Y-%m-%d").to_string()
        } else {
            self.created_at.clone()
        }
    }

    /// 作成時刻 (時刻のみ)
    pub fn created_time(&self) -> String {
        if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&self.created_at) {
            dt.format("%H:%M").to_string()
        } else {
            "".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{MacroTarget, Nutrition};

    fn create_test_daily_plan() -> DailyPlan {
        DailyPlan {
            date: Some("2026-01-23".to_string()),
            meals: vec![],
            total_nutrition: Nutrition {
                calories: 2400.0,
                protein: 150.0,
                fat: 80.0,
                carbohydrates: 300.0,
            },
            target: MacroTarget {
                goal: Goal::Bulk,
                daily_calories: 2500.0,
                protein_grams: 150.0,
                fat_grams: 80.0,
                carbs_grams: 300.0,
            },
        }
    }

    #[test]
    fn test_history_entry_creation() {
        let plan = create_test_daily_plan();
        let entry = HistoryEntry::new(Goal::Bulk, 2500.0, PlanData::Daily(plan));

        assert!(!entry.id.is_empty());
        assert!(!entry.created_at.is_empty());
        assert_eq!(entry.plan_type, PlanType::Daily);
        assert_eq!(entry.goal, Goal::Bulk);
        assert_eq!(entry.target_calories, 2500.0);
    }

    #[test]
    fn test_short_id() {
        let plan = create_test_daily_plan();
        let entry = HistoryEntry::new(Goal::Bulk, 2500.0, PlanData::Daily(plan));

        assert_eq!(entry.short_id().len(), 8);
    }

    #[test]
    fn test_history_index() {
        let mut index = HistoryIndex::new();
        assert!(index.is_empty());

        let plan = create_test_daily_plan();
        let entry = HistoryEntry::new(Goal::Bulk, 2500.0, PlanData::Daily(plan));
        let entry_id = entry.id.clone();

        index.add_entry(&entry);
        assert_eq!(index.len(), 1);

        // 検索テスト
        assert!(index.find_entry(&entry_id).is_some());
        assert!(index.find_entry(&entry_id[..8]).is_some());
        assert!(index.find_entry("nonexistent").is_none());

        // 削除テスト
        assert!(index.remove_entry(&entry_id));
        assert!(index.is_empty());
    }

    #[test]
    fn test_plan_type_display() {
        assert_eq!(PlanType::Daily.to_string(), "daily");
        assert_eq!(PlanType::Weekly.to_string(), "weekly");
        assert_eq!(PlanType::Monthly.to_string(), "monthly");
    }

    #[test]
    fn test_serialization() {
        let plan = create_test_daily_plan();
        let entry = HistoryEntry::new(Goal::Bulk, 2500.0, PlanData::Daily(plan));

        let json = serde_json::to_string(&entry).unwrap();
        let deserialized: HistoryEntry = serde_json::from_str(&json).unwrap();

        assert_eq!(entry.id, deserialized.id);
        assert_eq!(entry.goal, deserialized.goal);
    }
}
