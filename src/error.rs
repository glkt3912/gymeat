use std::path::PathBuf;
use thiserror::Error;

/// meal-plannerのエラー型
#[derive(Debug, Error)]
pub enum MealPlannerError {
    // ===========================================
    // バリデーションエラー
    // ===========================================
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

    #[error("無効な日付形式です: {0} (YYYY-MM-DD形式で指定してください)")]
    InvalidDate(String),

    // ===========================================
    // プランナーエラー
    // ===========================================
    #[error("適切な{0}が見つかりません。これは通常、メニューデータの不足が原因です。--verbose で詳細を確認できます")]
    NoSuitableMealFound(String),

    #[error("メニューが不足しています。最低4食分のメニューが必要です")]
    InsufficientMeals,

    // ===========================================
    // 出力エラー
    // ===========================================
    #[error("出力エラー: {0}")]
    OutputError(String),

    #[error("ファイル書き込みエラー: {path}")]
    FileWriteError {
        path: String,
        #[source]
        source: std::io::Error,
    },

    #[error("フォーマット変換エラー: {context}")]
    FormatError {
        context: String,
        #[source]
        source: serde_json::Error,
    },

    // ===========================================
    // PDF生成エラー
    // ===========================================
    #[error("pandocコマンドが見つかりません\n💡 ヒント: pandocとTeX環境がインストールされているか確認してください\nインストール方法:\n  macOS: brew install pandoc basictex\n  Ubuntu: apt-get install pandoc texlive-xetex\n  Windows: https://pandoc.org/installing.html")]
    PandocNotFound,

    #[error("pandocの実行に失敗しました")]
    PandocExecutionFailed {
        #[source]
        source: std::io::Error,
    },

    #[error("PDF生成に失敗しました: {stderr}")]
    PdfGenerationFailed { stderr: String },

    // ===========================================
    // I/Oエラー
    // ===========================================
    #[error("IO エラー: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSONエラー: {0}")]
    JsonError(#[from] serde_json::Error),

    // ===========================================
    // 履歴エラー
    // ===========================================
    #[error("履歴が見つかりません: {0}")]
    HistoryNotFound(String),

    #[error("ホームディレクトリを取得できません")]
    HistoryHomeDirNotFound,

    #[error("履歴ディレクトリの作成に失敗しました: {}", path.display())]
    HistoryDirCreationFailed {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("履歴ファイルの読み込みに失敗しました: {}", path.display())]
    HistoryReadFailed {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("履歴ファイルの保存に失敗しました: {}", path.display())]
    HistoryWriteFailed {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("履歴ファイルの削除に失敗しました: {}", path.display())]
    HistoryDeleteFailed {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("履歴のパースに失敗しました: {context}")]
    HistoryParseFailed {
        context: String,
        #[source]
        source: serde_json::Error,
    },

    #[error("履歴のシリアライズに失敗しました: {context}")]
    HistorySerializeFailed {
        context: String,
        #[source]
        source: serde_json::Error,
    },

    #[error("履歴エラー: {0}")]
    HistoryArgumentError(String),
}

impl MealPlannerError {
    /// エラーの種類に応じた終了コードを返す
    pub fn exit_code(&self) -> i32 {
        match self {
            // 入力バリデーションエラー
            Self::InvalidWeight(_)
            | Self::InvalidHeight(_)
            | Self::InvalidAge(_)
            | Self::InvalidCalories(_)
            | Self::InvalidDate(_)
            | Self::ConfigValidationError(_) => 2,

            // I/Oエラー
            Self::IoError(_)
            | Self::FileWriteError { .. }
            | Self::HistoryReadFailed { .. }
            | Self::HistoryWriteFailed { .. }
            | Self::HistoryDeleteFailed { .. }
            | Self::HistoryDirCreationFailed { .. } => 3,

            // PDF生成エラー
            Self::PandocNotFound
            | Self::PandocExecutionFailed { .. }
            | Self::PdfGenerationFailed { .. } => 4,

            // 履歴エラー
            Self::HistoryNotFound(_)
            | Self::HistoryHomeDirNotFound
            | Self::HistoryParseFailed { .. }
            | Self::HistorySerializeFailed { .. }
            | Self::HistoryArgumentError(_) => 5,

            // その他
            _ => 1,
        }
    }
}

/// Resultのエイリアス型
pub type Result<T> = std::result::Result<T, MealPlannerError>;
