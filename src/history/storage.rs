use std::fs;
use std::path::PathBuf;

use crate::constants::{APP_DIR_NAME, HISTORY_DIR_NAME, INDEX_FILE_NAME, PLANS_DIR_NAME};
use crate::error::{MealPlannerError, Result};
use crate::history::models::{HistoryEntry, HistoryIndex};

/// 履歴ストレージ
pub struct HistoryStorage {
    base_dir: PathBuf,
    history_dir: PathBuf,
    plans_dir: PathBuf,
    index_path: PathBuf,
}

impl HistoryStorage {
    /// 新しいストレージインスタンスを作成
    pub fn new() -> Result<Self> {
        let base_dir = Self::get_base_dir()?;
        let history_dir = base_dir.join(HISTORY_DIR_NAME);
        let plans_dir = history_dir.join(PLANS_DIR_NAME);
        let index_path = history_dir.join(INDEX_FILE_NAME);

        Ok(Self {
            base_dir,
            history_dir,
            plans_dir,
            index_path,
        })
    }

    /// ベースディレクトリを取得 (~/.gymeat)
    fn get_base_dir() -> Result<PathBuf> {
        dirs::home_dir()
            .map(|home| home.join(APP_DIR_NAME))
            .ok_or_else(|| {
                MealPlannerError::HistoryError("ホームディレクトリを取得できません".to_string())
            })
    }

    /// 必要なディレクトリを初期化
    pub fn initialize(&self) -> Result<()> {
        // ディレクトリ作成
        fs::create_dir_all(&self.plans_dir).map_err(|e| {
            MealPlannerError::HistoryError(format!("履歴ディレクトリの作成に失敗しました: {}", e))
        })?;

        // インデックスファイルが存在しなければ作成
        if !self.index_path.exists() {
            let index = HistoryIndex::new();
            self.save_index(&index)?;
        }

        Ok(())
    }

    /// インデックスを読み込み
    pub fn load_index(&self) -> Result<HistoryIndex> {
        if !self.index_path.exists() {
            return Ok(HistoryIndex::new());
        }

        let content = fs::read_to_string(&self.index_path).map_err(|e| {
            MealPlannerError::HistoryError(format!("インデックスの読み込みに失敗しました: {}", e))
        })?;

        serde_json::from_str(&content).map_err(|e| {
            MealPlannerError::HistoryError(format!("インデックスのパースに失敗しました: {}", e))
        })
    }

    /// インデックスを保存
    pub fn save_index(&self, index: &HistoryIndex) -> Result<()> {
        let content = serde_json::to_string_pretty(index).map_err(|e| {
            MealPlannerError::HistoryError(format!(
                "インデックスのシリアライズに失敗しました: {}",
                e
            ))
        })?;

        fs::write(&self.index_path, content).map_err(|e| {
            MealPlannerError::HistoryError(format!("インデックスの保存に失敗しました: {}", e))
        })
    }

    /// 履歴エントリを保存
    pub fn save_entry(&self, entry: &HistoryEntry) -> Result<()> {
        // ストレージを初期化
        self.initialize()?;

        // プランデータを保存
        let plan_path = self.plans_dir.join(format!("{}.json", entry.id));
        let content = serde_json::to_string_pretty(entry).map_err(|e| {
            MealPlannerError::HistoryError(format!("プランのシリアライズに失敗しました: {}", e))
        })?;

        fs::write(&plan_path, content).map_err(|e| {
            MealPlannerError::HistoryError(format!("プランの保存に失敗しました: {}", e))
        })?;

        // インデックスを更新
        let mut index = self.load_index()?;
        index.add_entry(entry);
        self.save_index(&index)?;

        Ok(())
    }

    /// 履歴エントリを読み込み
    pub fn load_entry(&self, id: &str) -> Result<HistoryEntry> {
        // インデックスから完全なIDを取得
        let index = self.load_index()?;
        let index_entry = index
            .find_entry(id)
            .ok_or_else(|| MealPlannerError::HistoryNotFound(id.to_string()))?;

        let plan_path = self.plans_dir.join(format!("{}.json", index_entry.id));

        if !plan_path.exists() {
            return Err(MealPlannerError::HistoryNotFound(id.to_string()));
        }

        let content = fs::read_to_string(&plan_path).map_err(|e| {
            MealPlannerError::HistoryError(format!("プランの読み込みに失敗しました: {}", e))
        })?;

        serde_json::from_str(&content).map_err(|e| {
            MealPlannerError::HistoryError(format!("プランのパースに失敗しました: {}", e))
        })
    }

    /// 履歴エントリを削除
    pub fn delete_entry(&self, id: &str) -> Result<()> {
        // インデックスから完全なIDを取得
        let mut index = self.load_index()?;
        let index_entry = index
            .find_entry(id)
            .ok_or_else(|| MealPlannerError::HistoryNotFound(id.to_string()))?;
        let full_id = index_entry.id.clone();

        // プランファイルを削除
        let plan_path = self.plans_dir.join(format!("{}.json", full_id));
        if plan_path.exists() {
            fs::remove_file(&plan_path).map_err(|e| {
                MealPlannerError::HistoryError(format!("プランの削除に失敗しました: {}", e))
            })?;
        }

        // インデックスから削除
        index.remove_entry(&full_id);
        self.save_index(&index)?;

        Ok(())
    }

    /// 最新の履歴エントリを取得
    pub fn load_latest(&self) -> Result<Option<HistoryEntry>> {
        let index = self.load_index()?;
        if let Some(entry) = index.entries.last() {
            Ok(Some(self.load_entry(&entry.id)?))
        } else {
            Ok(None)
        }
    }

    /// ストレージのベースディレクトリを取得
    pub fn base_dir(&self) -> &PathBuf {
        &self.base_dir
    }

    /// 履歴ディレクトリを取得
    pub fn history_dir(&self) -> &PathBuf {
        &self.history_dir
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::history::models::{PlanData, PlanType};
    use crate::models::{DailyPlan, Goal, MacroTarget, Nutrition};
    use std::env;
    use std::fs;

    fn create_test_storage() -> (HistoryStorage, PathBuf) {
        let temp_dir = env::temp_dir().join(format!("gymeat_test_{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&temp_dir).unwrap();

        // テスト用にHOMEを一時的に変更
        let storage = HistoryStorage {
            base_dir: temp_dir.clone(),
            history_dir: temp_dir.join("history"),
            plans_dir: temp_dir.join("history").join("plans"),
            index_path: temp_dir.join("history").join("index.json"),
        };

        (storage, temp_dir)
    }

    fn cleanup_test_dir(path: &PathBuf) {
        let _ = fs::remove_dir_all(path);
    }

    fn create_test_entry() -> HistoryEntry {
        let plan = DailyPlan {
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
        };
        HistoryEntry::new(Goal::Bulk, 2500.0, PlanData::Daily(plan))
    }

    #[test]
    fn test_initialize() {
        let (storage, temp_dir) = create_test_storage();
        storage.initialize().unwrap();

        assert!(storage.plans_dir.exists());
        assert!(storage.index_path.exists());

        cleanup_test_dir(&temp_dir);
    }

    #[test]
    fn test_save_and_load_entry() {
        let (storage, temp_dir) = create_test_storage();
        storage.initialize().unwrap();

        let entry = create_test_entry();
        let entry_id = entry.id.clone();

        // 保存
        storage.save_entry(&entry).unwrap();

        // 読み込み
        let loaded = storage.load_entry(&entry_id).unwrap();
        assert_eq!(loaded.id, entry_id);
        assert_eq!(loaded.goal, Goal::Bulk);
        assert_eq!(loaded.plan_type, PlanType::Daily);

        // 短縮IDでも検索可能
        let loaded_short = storage.load_entry(&entry_id[..8]).unwrap();
        assert_eq!(loaded_short.id, entry_id);

        cleanup_test_dir(&temp_dir);
    }

    #[test]
    fn test_delete_entry() {
        let (storage, temp_dir) = create_test_storage();
        storage.initialize().unwrap();

        let entry = create_test_entry();
        let entry_id = entry.id.clone();

        // 保存
        storage.save_entry(&entry).unwrap();

        // 削除
        storage.delete_entry(&entry_id).unwrap();

        // 読み込みに失敗することを確認
        assert!(storage.load_entry(&entry_id).is_err());

        cleanup_test_dir(&temp_dir);
    }

    #[test]
    fn test_load_latest() {
        let (storage, temp_dir) = create_test_storage();
        storage.initialize().unwrap();

        // 空の場合
        assert!(storage.load_latest().unwrap().is_none());

        // エントリを追加
        let entry = create_test_entry();
        let entry_id = entry.id.clone();
        storage.save_entry(&entry).unwrap();

        // 最新を取得
        let latest = storage.load_latest().unwrap().unwrap();
        assert_eq!(latest.id, entry_id);

        cleanup_test_dir(&temp_dir);
    }

    #[test]
    fn test_index_operations() {
        let (storage, temp_dir) = create_test_storage();
        storage.initialize().unwrap();

        let entry1 = create_test_entry();
        let entry2 = create_test_entry();

        storage.save_entry(&entry1).unwrap();
        storage.save_entry(&entry2).unwrap();

        let index = storage.load_index().unwrap();
        assert_eq!(index.len(), 2);

        cleanup_test_dir(&temp_dir);
    }
}
