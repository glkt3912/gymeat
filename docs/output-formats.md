# 出力フォーマット仕様

本ドキュメントでは、gymeat がサポートする出力フォーマットの仕様と使い方を説明します。

## 目次

1. [フォーマット一覧](#フォーマット一覧)
2. [出力先](#出力先)
3. [各フォーマット詳細](#各フォーマット詳細)
4. [OutputFormatter trait](#outputformatter-trait)

---

## フォーマット一覧

`--output` オプションで指定する値とその対応:

| 値 | フォーマッター | 用途 |
|---|---|---|
| `terminal`（デフォルト）| `TerminalOutput` | カラー付きターミナル表示 |
| `json` | `JsonFormatter` | 改行なしのコンパクトJSON |
| `json-pretty` | `JsonFormatter` | インデント付き整形JSON |
| `csv` | `CsvFormatter` | スプレッドシート向けCSV |
| `markdown` | `MarkdownFormatter` | GitHub/Notion向けMarkdown |
| `pdf` | `PdfFormatter` | pandoc経由でPDF生成 |

---

## 出力先

`--output-file <PATH>` を指定するとファイルに書き込み、指定しない場合は標準出力に出力されます。

```
OutputDestination::Stdout  → println! で標準出力
OutputDestination::File    → fs::write でファイル書き込み
```

**制約**: PDF出力（`--output pdf`）は `--output-file` の指定が必須です。

---

## 各フォーマット詳細

### terminal

- カラー出力（`colored` クレート使用）
- `--no-color` オプションで無効化可能
- 達成度（目標カロリー比）と評価コメントを表示
- `--verbose` オプションで追加情報を表示

### json / json-pretty

- `serde_json` でシリアライズ
- `json`: `serde_json::to_string()`
- `json-pretty`: `serde_json::to_string_pretty()`
- プラン全体（栄養情報・メニュー・目標値）が含まれる

### csv

週次・月次プランでは複数行になります。

| カラム構成 |
|---|
| 日付, 食事タイプ, 料理名, カロリー, タンパク質(g), 脂質(g), 炭水化物(g) |

ExcelやGoogleスプレッドシートでそのまま開けます。

### markdown

- `# タイトル`, `## 日付`, テーブル形式でメニューを列挙
- `--recipe` オプション併用でレシピ手順も含まれる
- GitHub・Notionでレンダリング可能

### pdf

pandoc を使って Markdown → PDF 変換します。

**必要な外部コマンド:**

```bash
# macOS
brew install pandoc basictex

# Ubuntu
apt-get install pandoc texlive-xetex

# Windows
# https://pandoc.org/installing.html
```

**エラー時の終了コード:**

| エラー | 終了コード |
|---|---|
| `PandocNotFound` | 4 |
| `PandocExecutionFailed` | 4 |
| `PdfGenerationFailed` | 4 |

---

## OutputFormatter trait

すべてのフォーマッターが実装する共通インターフェース:

```rust
pub trait OutputFormatter {
    fn format_daily_plan(
        &self, plan: &DailyPlan, database: &MealDatabase, show_recipe: bool,
    ) -> Result<String>;

    fn format_weekly_plan(
        &self, plan: &WeeklyPlan, database: &MealDatabase, show_recipe: bool,
    ) -> Result<String>;

    fn format_monthly_plan(
        &self, plan: &MonthlyPlan, database: &MealDatabase, show_recipe: bool,
    ) -> Result<String>;

    fn format_name(&self) -> &'static str;
}
```

---

## 関連ソース

| ファイル | 役割 |
|---|---|
| `src/output/formatter.rs` | OutputFormatter trait・OutputDestination・write_output |
| `src/output/terminal.rs` | ターミナル出力 |
| `src/output/json.rs` | JSON出力 |
| `src/output/csv.rs` | CSV出力 |
| `src/output/markdown.rs` | Markdown出力 |
| `src/output/pdf.rs` | PDF生成（pandoc経由）|
