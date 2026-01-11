use super::formatter::OutputFormatter;
use super::markdown::MarkdownFormatter;
use crate::data::MealDatabase;
use crate::error::{MealPlannerError, Result};
use crate::models::{DailyPlan, WeeklyPlan};
use std::path::Path;
use std::process::Command;

/// PDF出力フォーマッター (pandoc依存)
#[derive(Default)]
pub struct PdfFormatter;

impl PdfFormatter {
    pub fn new() -> Self {
        Self
    }

    /// pandocがインストールされているか確認
    pub fn check_pandoc_available() -> bool {
        Command::new("pandoc").arg("--version").output().is_ok()
    }
}

impl OutputFormatter for PdfFormatter {
    fn format_daily_plan(
        &self,
        _plan: &DailyPlan,
        _database: &MealDatabase,
        _show_recipe: bool,
    ) -> Result<String> {
        // PDF出力は直接文字列を返せないため、エラーメッセージを返す
        Err(MealPlannerError::OutputError(
            "PDF出力には--output-fileオプションが必要です".to_string(),
        ))
    }

    fn format_weekly_plan(
        &self,
        _plan: &WeeklyPlan,
        _database: &MealDatabase,
        _show_recipe: bool,
    ) -> Result<String> {
        // PDF出力は直接文字列を返せないため、エラーメッセージを返す
        Err(MealPlannerError::OutputError(
            "PDF出力には--output-fileオプションが必要です".to_string(),
        ))
    }

    fn format_name(&self) -> &'static str {
        "pdf"
    }
}

/// 1日プランをPDFファイルとして出力
pub fn write_daily_plan_to_pdf(
    plan: &DailyPlan,
    database: &MealDatabase,
    show_recipe: bool,
    output_path: &Path,
) -> Result<()> {
    // pandocが利用可能か確認
    if !PdfFormatter::check_pandoc_available() {
        return Err(MealPlannerError::PdfGenerationError(
            "pandocコマンドが見つかりません".to_string(),
        ));
    }

    // Markdown生成
    let md_formatter = MarkdownFormatter::new();
    let markdown = md_formatter.format_daily_plan(plan, database, show_recipe)?;

    // 一時ファイルに保存
    let temp_md = std::env::temp_dir().join("gymeat_temp.md");
    std::fs::write(&temp_md, &markdown)
        .map_err(|e| MealPlannerError::FileWriteError(format!("一時ファイル作成失敗: {}", e)))?;

    // pandocでPDF生成
    let output = Command::new("pandoc")
        .arg(&temp_md)
        .arg("-o")
        .arg(output_path)
        .arg("--pdf-engine=xelatex") // 日本語対応
        .arg("-V")
        .arg("CJKmainfont=Hiragino Sans") // macOS用フォント (環境により調整が必要)
        .arg("-V")
        .arg("geometry:margin=2cm") // マージン設定
        .output()
        .map_err(|e| MealPlannerError::PdfGenerationError(format!("pandoc実行失敗: {}", e)))?;

    // 一時ファイル削除
    let _ = std::fs::remove_file(&temp_md);

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(MealPlannerError::PdfGenerationError(format!(
            "PDF生成失敗: {}",
            stderr
        )));
    }

    Ok(())
}

/// 週間プランをPDFファイルとして出力
pub fn write_weekly_plan_to_pdf(
    plan: &WeeklyPlan,
    database: &MealDatabase,
    show_recipe: bool,
    output_path: &Path,
) -> Result<()> {
    // pandocが利用可能か確認
    if !PdfFormatter::check_pandoc_available() {
        return Err(MealPlannerError::PdfGenerationError(
            "pandocコマンドが見つかりません".to_string(),
        ));
    }

    // Markdown生成
    let md_formatter = MarkdownFormatter::new();
    let markdown = md_formatter.format_weekly_plan(plan, database, show_recipe)?;

    // 一時ファイルに保存
    let temp_md = std::env::temp_dir().join("gymeat_temp_weekly.md");
    std::fs::write(&temp_md, &markdown)
        .map_err(|e| MealPlannerError::FileWriteError(format!("一時ファイル作成失敗: {}", e)))?;

    // pandocでPDF生成
    let output = Command::new("pandoc")
        .arg(&temp_md)
        .arg("-o")
        .arg(output_path)
        .arg("--pdf-engine=xelatex") // 日本語対応
        .arg("-V")
        .arg("CJKmainfont=Hiragino Sans") // macOS用フォント
        .arg("-V")
        .arg("geometry:margin=2cm") // マージン設定
        .output()
        .map_err(|e| MealPlannerError::PdfGenerationError(format!("pandoc実行失敗: {}", e)))?;

    // 一時ファイル削除
    let _ = std::fs::remove_file(&temp_md);

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(MealPlannerError::PdfGenerationError(format!(
            "PDF生成失敗: {}",
            stderr
        )));
    }

    Ok(())
}
