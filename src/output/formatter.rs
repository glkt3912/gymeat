use crate::data::MealDatabase;
use crate::error::Result;
use crate::models::{DailyPlan, MonthlyPlan, WeeklyPlan};
use std::path::PathBuf;

/// 出力フォーマッターの共通trait
pub trait OutputFormatter {
    /// 1日プランを文字列に変換
    fn format_daily_plan(
        &self,
        plan: &DailyPlan,
        database: &MealDatabase,
        show_recipe: bool,
    ) -> Result<String>;

    /// 週間プランを文字列に変換
    fn format_weekly_plan(
        &self,
        plan: &WeeklyPlan,
        database: &MealDatabase,
        show_recipe: bool,
    ) -> Result<String>;

    /// 月間プランを文字列に変換
    fn format_monthly_plan(
        &self,
        plan: &MonthlyPlan,
        database: &MealDatabase,
        show_recipe: bool,
    ) -> Result<String>;

    /// フォーマット名を返す
    fn format_name(&self) -> &'static str;
}

/// 出力先の列挙型
pub enum OutputDestination {
    Stdout,
    File(PathBuf),
}

/// 出力を実行するヘルパー関数
pub fn write_output(content: &str, destination: OutputDestination) -> Result<()> {
    match destination {
        OutputDestination::Stdout => {
            println!("{}", content);
            Ok(())
        }
        OutputDestination::File(path) => {
            std::fs::write(&path, content).map_err(|e| {
                crate::error::MealPlannerError::FileWriteError(format!(
                    "ファイル書き込み失敗 ({}): {}",
                    path.display(),
                    e
                ))
            })?;
            Ok(())
        }
    }
}
