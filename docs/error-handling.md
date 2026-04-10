# エラーハンドリング仕様

本ドキュメントでは、gymeat のエラー型・終了コード・バリデーション範囲を説明します。

## 目次

1. [エラー型一覧](#エラー型一覧)
2. [終了コード](#終了コード)
3. [バリデーション範囲](#バリデーション範囲)

---

## エラー型一覧

`MealPlannerError` は `thiserror` クレートで定義されています（`src/error.rs`）。

### バリデーションエラー（終了コード 2）

| バリアント | エラーメッセージ |
|---|---|
| `InvalidWeight(f32)` | 無効な体重: {0}kg (1-300kgの範囲で指定) |
| `InvalidHeight(f32)` | 無効な身長: {0}cm (100-250cmの範囲で指定) |
| `InvalidAge(u32)` | 無効な年齢: {0}歳 (10-100歳の範囲で指定) |
| `InvalidCalories(f32)` | 無効なカロリー目標: {0}kcal (500-10000kcalの範囲で指定) |
| `ConfigValidationError(String)` | 設定の検証に失敗: {0} |
| `InvalidDate(String)` | 無効な日付形式: {0} (YYYY-MM-DD形式で指定) |

### プランナーエラー（終了コード 1）

| バリアント | エラーメッセージ |
|---|---|
| `NoSuitableMealFound(String)` | 適切な{0}が見つかりません |
| `InsufficientMeals` | メニューが不足しています（最低4食分必要）|

### 出力エラー（終了コード 1）

| バリアント | エラーメッセージ |
|---|---|
| `OutputError(String)` | 出力エラー: {0} |
| `FileWriteError { path, source }` | ファイル書き込みエラー: {path} |
| `FormatError { context, source }` | フォーマット変換エラー: {context} |

### PDF生成エラー（終了コード 4）

| バリアント | 説明 |
|---|---|
| `PandocNotFound` | pandocコマンドが見つからない（インストール手順をメッセージに含む）|
| `PandocExecutionFailed { source }` | pandocの実行に失敗 |
| `PdfGenerationFailed { stderr }` | PDF生成失敗（pandocのstderrを含む）|

### I/Oエラー（終了コード 3）

| バリアント | 説明 |
|---|---|
| `IoError(std::io::Error)` | 汎用I/Oエラー（`#[from]` による自動変換）|
| `JsonError(serde_json::Error)` | JSONエラー（`#[from]` による自動変換）|
| `FileWriteError { path, source }` | ファイル書き込み失敗（終了コード3） |
| `HistoryReadFailed { path, source }` | 履歴ファイル読み込み失敗（終了コード3） |
| `HistoryWriteFailed { path, source }` | 履歴ファイル保存失敗（終了コード3） |
| `HistoryDeleteFailed { path, source }` | 履歴ファイル削除失敗（終了コード3） |
| `HistoryDirCreationFailed { path, source }` | 履歴ディレクトリ作成失敗（終了コード3） |

### 履歴エラー（終了コード 5）

| バリアント | 説明 |
|---|---|
| `HistoryNotFound(String)` | 指定IDの履歴が存在しない |
| `HistoryHomeDirNotFound` | ホームディレクトリが取得できない |
| `HistoryParseFailed { context, source }` | 履歴JSONのパース失敗 |
| `HistorySerializeFailed { context, source }` | 履歴JSONのシリアライズ失敗 |
| `HistoryArgumentError(String)` | 履歴コマンドの引数エラー |

---

## 終了コード

| コード | カテゴリ | 主な原因 |
|---|---|---|
| `0` | 正常終了 | — |
| `1` | その他エラー | プランナーエラー・出力エラー |
| `2` | バリデーションエラー | 体重・身長・年齢・カロリー・日付の範囲外 |
| `3` | I/Oエラー | ファイル書き込み・読み込み・ディレクトリ作成失敗 |
| `4` | PDF生成エラー | pandoc未インストール・PDF変換失敗 |
| `5` | 履歴エラー | 履歴未発見・ホームディレクトリ取得失敗・パース失敗 |

---

## バリデーション範囲

`PlanConfig::validate()` で検証される値の許容範囲:

| フィールド | 定数 | 最小値 | 最大値 |
|---|---|---|---|
| 体重 (`--weight`) | `MAX_WEIGHT_KG` | 0超 | 300 kg |
| 身長 (`--height`) | `MIN_HEIGHT_CM`, `MAX_HEIGHT_CM` | 100 cm | 250 cm |
| 年齢 (`--age`) | — | 10歳 | 100歳 |
| カロリー (`--calories`) | `MIN_CALORIES`, `MAX_CALORIES` | 500 kcal | 10000 kcal |

### 体組成情報の整合性チェック

体重・身長・年齢のいずれか1つでも指定した場合、4つすべて（体重・身長・年齢・性別）が必要です。

```
weight, height, age, gender のうち1つでも Some → 4つすべて必須
いずれも None → デフォルトカロリーを使用
custom_calories が Some → 体組成情報なしでも可
```

---

## 関連ソース

| ファイル | 役割 |
|---|---|
| `src/error.rs` | MealPlannerError 定義・終了コードマッピング |
| `src/config.rs` | PlanConfig::validate() バリデーション実装 |
| `src/constants.rs` | バリデーション範囲の定数定義 |
