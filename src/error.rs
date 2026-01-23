use thiserror::Error;

/// meal-plannerのエラー型
#[derive(Debug, Error)]
pub enum MealPlannerError {
    #[error("無効な体重です: {0}kg (1-300kgの範囲で指定してください)")]
    InvalidWeight(f32),

    #[error("無効な身長です: {0}cm (100-250cmの範囲で指定してください)")]
    InvalidHeight(f32),

    #[error("無効な年齢です: {0}歳 (10-100歳の範囲で指定してください)")]
    InvalidAge(u32),

    #[error("無効なカロリー目標です: {0}kcal (500-10000kcalの範囲で指定してください)")]
    InvalidCalories(f32),

    #[error("設定の検証に失敗しました: {0}")]
    ConfigValidationError(String),

    #[error("適切な{0}が見つかりません。これは通常、メニューデータの不足が原因です。--verbose で詳細を確認できます")]
    NoSuitableMealFound(String),

    #[error("メニューが不足しています。最低4食分のメニューが必要です")]
    InsufficientMeals,

    #[error("データの読み込みに失敗しました: {0}")]
    DataLoadError(String),

    #[error("データのパースに失敗しました: {0}")]
    DataParseError(String),

    #[error("出力エラー: {0}")]
    OutputError(String),

    #[error("ファイル書き込みエラー: {0}")]
    FileWriteError(String),

    #[error("フォーマット変換エラー: {0}")]
    FormatError(String),

    #[error("PDF生成エラー: {0}\n💡 ヒント: pandocとTeX環境がインストールされているか確認してください\nインストール方法:\n  macOS: brew install pandoc basictex\n  Ubuntu: apt-get install pandoc texlive-xetex\n  Windows: https://pandoc.org/installing.html")]
    PdfGenerationError(String),

    #[error("IO エラー: {0}")]
    IoError(#[from] std::io::Error),

    #[error("無効な日付形式です: {0} (YYYY-MM-DD形式で指定してください)")]
    InvalidDate(String),

    #[error("履歴が見つかりません: {0}")]
    HistoryNotFound(String),

    #[error("履歴エラー: {0}")]
    HistoryError(String),
}

/// Resultのエイリアス型
pub type Result<T> = std::result::Result<T, MealPlannerError>;
