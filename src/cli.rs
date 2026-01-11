use clap::{Parser, ValueEnum};

/// 筋トレ用食事メニュー生成ツール
#[derive(Parser, Debug)]
#[command(
    name = "gymeat",
    version = "0.1.0",
    about = "筋トレ用食事メニュー生成CLI (gym + eat)",
    long_about = "目的別（増量・減量・維持）に最適化された食事プランを生成します

使用例:
  # デフォルト (維持モード、デフォルトカロリー)
  gymeat

  # 増量モード + 体組成情報を指定
  gymeat --goal bulk --weight 70 --height 175 --age 25 --gender male

  # 減量モード + カスタムカロリー指定
  gymeat --goal cut --calories 1800

  # レシピ付きで表示
  gymeat --recipe --verbose

  # 週間プラン生成 (今日から7日間)
  gymeat --weekly --goal bulk

  # 週間プラン生成 (指定日から7日間)
  gymeat --weekly --start-date 2026-01-13 --goal cut

  # 月間プラン生成 (今日から30日間)
  gymeat --monthly --goal maintain

  # 月間プラン生成 (指定日から30日間)
  gymeat --monthly --start-date 2026-02-01 --goal bulk

  # JSON形式で出力
  gymeat --output json --output-file plan.json

  # CSV形式で週間プラン出力
  gymeat --weekly --output csv --output-file weekly.csv

  # PDF形式で出力 (要pandoc)
  gymeat --output pdf --output-file plan.pdf"
)]
pub struct CliArgs {
    /// トレーニング目的: bulk (増量), cut (減量), maintain (維持)
    #[arg(short, long, default_value = "maintain")]
    pub goal: GoalArg,

    /// 体重 (kg) - カロリー計算に使用
    #[arg(short, long)]
    pub weight: Option<f32>,

    /// 身長 (cm)
    #[arg(long)]
    pub height: Option<f32>,

    /// 年齢 (歳)
    #[arg(long)]
    pub age: Option<u32>,

    /// 性別: male (男性), female (女性)
    #[arg(long)]
    pub gender: Option<GenderArg>,

    /// 活動レベル: sedentary (運動なし), light (週1-3日), moderate (週3-5日), active (週6-7日), very-active (1日2回以上)
    #[arg(short, long, default_value = "moderate")]
    pub activity: ActivityArg,

    /// カスタムカロリー目標 (kcal/日) - 指定した場合、体組成情報は不要
    #[arg(short, long)]
    pub calories: Option<f32>,

    /// レシピ (調理手順) を表示
    #[arg(short, long)]
    pub recipe: bool,

    /// カラー出力を無効化
    #[arg(long)]
    pub no_color: bool,

    /// 詳細情報を表示
    #[arg(short, long)]
    pub verbose: bool,

    /// 7日間の週間プランを生成
    #[arg(long, conflicts_with = "monthly")]
    pub weekly: bool,

    /// 30日間の月間プランを生成
    #[arg(long, conflicts_with = "weekly")]
    pub monthly: bool,

    /// プラン開始日 (YYYY-MM-DD形式, デフォルト: 今日)
    #[arg(long)]
    pub start_date: Option<String>,

    /// 出力フォーマット: terminal (ターミナル), json, json-pretty, csv, markdown, pdf
    #[arg(short = 'o', long, default_value = "terminal")]
    pub output: OutputFormatArg,

    /// 出力先ファイルパス (指定しない場合は標準出力、PDF出力時は必須)
    #[arg(long)]
    pub output_file: Option<String>,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum GoalArg {
    /// 増量
    Bulk,
    /// 減量
    Cut,
    /// 維持
    Maintain,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum GenderArg {
    /// 男性
    Male,
    /// 女性
    Female,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ActivityArg {
    /// ほぼ運動なし
    Sedentary,
    /// 週1-3日
    Light,
    /// 週3-5日
    Moderate,
    /// 週6-7日
    Active,
    /// 1日2回以上
    #[value(name = "very-active")]
    VeryActive,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum OutputFormatArg {
    /// ターミナル出力 (デフォルト)
    Terminal,
    /// JSON形式
    Json,
    /// JSON形式 (整形済み)
    #[value(name = "json-pretty")]
    JsonPretty,
    /// CSV形式
    Csv,
    /// Markdown形式
    Markdown,
    /// PDF形式 (要pandoc)
    Pdf,
}

impl From<GoalArg> for crate::models::Goal {
    fn from(arg: GoalArg) -> Self {
        match arg {
            GoalArg::Bulk => crate::models::Goal::Bulk,
            GoalArg::Cut => crate::models::Goal::Cut,
            GoalArg::Maintain => crate::models::Goal::Maintain,
        }
    }
}

impl From<GenderArg> for crate::config::Gender {
    fn from(arg: GenderArg) -> Self {
        match arg {
            GenderArg::Male => crate::config::Gender::Male,
            GenderArg::Female => crate::config::Gender::Female,
        }
    }
}

impl From<ActivityArg> for crate::config::ActivityLevel {
    fn from(arg: ActivityArg) -> Self {
        match arg {
            ActivityArg::Sedentary => crate::config::ActivityLevel::Sedentary,
            ActivityArg::Light => crate::config::ActivityLevel::Light,
            ActivityArg::Moderate => crate::config::ActivityLevel::Moderate,
            ActivityArg::Active => crate::config::ActivityLevel::Active,
            ActivityArg::VeryActive => crate::config::ActivityLevel::VeryActive,
        }
    }
}
