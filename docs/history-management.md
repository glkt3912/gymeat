# 履歴管理仕様

本ドキュメントでは、gymeat の履歴保存・検索・削除機能の仕様を説明します。

## 目次

1. [ディレクトリ構造](#ディレクトリ構造)
2. [データ構造](#データ構造)
3. [操作一覧](#操作一覧)
4. [ID検索の仕組み](#id検索の仕組み)
5. [フィルタ仕様](#フィルタ仕様)

---

## ディレクトリ構造

履歴データはホームディレクトリ以下に保存されます。

```
~/.gymeat/
└── history/
    ├── index.json          # メタデータインデックス（全履歴の概要）
    └── plans/
        ├── <UUID>.json     # 各プランの詳細データ
        ├── <UUID>.json
        └── ...
```

| 定数 | 値 |
|---|---|
| `APP_DIR_NAME` | `.gymeat` |
| `HISTORY_DIR_NAME` | `history` |
| `PLANS_DIR_NAME` | `plans` |
| `INDEX_FILE_NAME` | `index.json` |

---

## データ構造

### HistoryEntry（プランファイル）

`~/.gymeat/history/plans/<UUID>.json` に保存される完全なエントリ。

```json
{
  "id": "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx",
  "created_at": "2026-04-10T12:00:00+09:00",
  "plan_type": "daily",
  "goal": "bulk",
  "target_calories": 2800.0,
  "plan": {
    "type": "daily",
    ...DailyPlan / WeeklyPlan / MonthlyPlan
  }
}
```

### HistoryIndex（インデックスファイル）

`~/.gymeat/history/index.json` に保存されるメタデータのみの一覧。

```json
{
  "version": "1.0",
  "entries": [
    {
      "id": "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx",
      "created_at": "2026-04-10T12:00:00+09:00",
      "plan_type": "daily",
      "goal": "bulk",
      "target_calories": 2800.0,
      "start_date": "2026-04-10",
      "end_date": "2026-04-10"
    }
  ]
}
```

### PlanType

| 値（JSON） | 内容 |
|---|---|
| `"daily"` | 1日プラン |
| `"weekly"` | 7日プラン（start_date〜start_date+6日）|
| `"monthly"` | 30日プラン（start_date〜start_date+29日）|

---

## 操作一覧

### 保存（`--save` オプション）

```bash
gymeat --goal bulk --save
gymeat --weekly --goal cut --save
```

内部処理:
1. `~/.gymeat/history/plans/` ディレクトリを初期化（未作成の場合）
2. プランデータを `<UUID>.json` として書き込み
3. `index.json` にメタデータを追記

### 一覧表示

```bash
gymeat history list                    # 最新10件
gymeat history list --goal bulk        # 増量プランのみ
gymeat history list --plan-type weekly # 週間プランのみ
gymeat history list --last 7d          # 直近7日間
```

### 詳細表示

```bash
gymeat history show <ID>       # IDの先頭8文字でも可
gymeat history show --latest   # 最新を表示
```

### 削除

```bash
gymeat history delete <ID>     # IDの先頭8文字でも可
```

---

## ID検索の仕組み

IDはUUID v4（例: `a1b2c3d4-e5f6-...`）で生成されます。

表示・検索時は先頭8文字の短縮IDが使えます（`SHORT_ID_LENGTH = 8`）。

```
検索クエリ例: "a1b2c3d4"

HistoryIndex::find_entry() が以下を試みる:
  1. e.id == "a1b2c3d4"          → 完全一致
  2. e.id.starts_with("a1b2c3d4") → 前方一致
```

完全なUUIDを入力する必要はありません。

---

## フィルタ仕様

`history list` コマンドで使用できるフィルタ:

| オプション | 型 | 説明 |
|---|---|---|
| `--goal <GOAL>` | `bulk \| cut \| maintain` | 目的でフィルタ |
| `--plan-type <TYPE>` | `daily \| weekly \| monthly` | プランタイプでフィルタ |
| `--last <DURATION>` | `7d`, `30d` など | 直近N日間でフィルタ |

---

## 関連ソース

| ファイル | 役割 |
|---|---|
| `src/history/models.rs` | HistoryEntry, HistoryIndex, PlanType 型定義 |
| `src/history/storage.rs` | HistoryStorage 実装（保存・読込・削除）|
| `src/constants.rs` | パス定数・SHORT_ID_LENGTH |
