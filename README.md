# Gymeat - 筋トレ用食事メニュー生成CLI

gym + eat = gymeat 🏋️💪

目的別（増量・減量・維持）に最適化された食事プランを生成するRust製CLIツールです。

## 特徴

- **目的別メニュー**: 増量(Bulk)、減量(Cut)、維持(Maintain)の3つのモードに対応
- **週間プラン生成**: 7日分のメニューを一度に生成、メニューの重複を自動回避
- **精密なカロリー計算**: Harris-Benedict式を使用したBMR/TDEE計算
- **性別対応**: 男性・女性それぞれに最適化されたカロリー計算
- **24種類の筋トレ向けメニュー**: 高タンパクで栄養バランスの取れた実際の料理
- **最適化されたメニュー選択**: 目標カロリーに最も近いメニューを優先的に選択（±10%以内の精度）
- **レシピ表示**: 調理手順も確認可能
- **カラフルな出力**: 見やすいターミナル表示
- **初心者にも優しい**: 初回実行時のガイダンス、詳細なエラーメッセージ、達成度評価とアドバイス
- **多様な出力形式**: JSON、CSV、Markdown、PDF形式での出力に対応（データ分析や印刷に便利）

## インストール

```bash
# リポジトリをクローン
git clone https://github.com/yourusername/gymeat
cd gymeat

# ビルド
cargo build --release

# インストール (オプション)
cargo install --path .
```

## 使い方

### 基本的な使用

```bash
# デフォルト (維持モード、デフォルトカロリー)
gymeat

# 増量モード + 体組成情報を指定
gymeat --goal bulk --weight 70 --height 175 --age 25 --gender male

# 減量モード + カスタムカロリー指定
gymeat --goal cut --calories 1800

# 女性の場合
gymeat --goal maintain --weight 55 --height 160 --age 28 --gender female --activity light

# レシピを表示
gymeat --goal bulk --calories 2800 --recipe

# 週間プラン生成（今日から7日間）
gymeat --weekly --goal bulk

# 週間プラン生成（指定日から7日間）
gymeat --weekly --start-date 2026-01-13 --goal cut

# JSON形式で出力
gymeat --output json-pretty --output-file plan.json

# CSV形式で週間プランを出力
gymeat --weekly --output csv --output-file weekly.csv

# Markdown形式で出力（レシピ付き）
gymeat --recipe --output markdown --output-file plan.md

# PDF形式で出力（印刷用、要pandoc）
gymeat --output pdf --output-file plan.pdf
```

### 出力フォーマット

プランを様々な形式で出力できます:

**JSON形式** - APIやデータ分析に最適
```bash
# 標準出力にJSON出力
gymeat --output json

# ファイルに整形済みJSON出力
gymeat --output json-pretty --output-file plan.json
```

**CSV形式** - スプレッドシートで編集・分析
```bash
gymeat --weekly --output csv --output-file weekly.csv
# ExcelやGoogleスプレッドシートで開ける
```

**Markdown形式** - ドキュメント作成や共有に便利
```bash
gymeat --recipe --output markdown --output-file plan.md
# GitHubやNotionなどで美しく表示
```

**PDF形式** - 印刷やレポート作成に
```bash
gymeat --weekly --output pdf --output-file weekly_plan.pdf
# 要pandocインストール: brew install pandoc basictex
```

### 週間プラン

週間プランを生成すると、7日分のメニューが一度に表示されます。メニューの重複は自動的に回避されます（データベースの容量に応じて）。

```bash
# 今日から7日間の増量プランを生成
gymeat --weekly --goal bulk --weight 70 --height 175 --age 25 --gender male

# 特定の日から開始
gymeat --weekly --start-date 2026-02-01 --goal cut --calories 2000
```

**週間プランの特徴:**
- 7日分のメニューを一括生成
- 同じメニューが週内で重複しないよう自動調整
- 1日平均と週間合計の栄養統計を表示
- 開始日を指定可能（デフォルトは今日）
```

### 履歴管理

生成したプランを履歴に保存・管理できます。履歴は `~/.gymeat/history/` に保存されます。

```bash
# プラン生成と同時に履歴に保存
gymeat --goal bulk --save
gymeat --weekly --goal cut --save

# 履歴一覧を表示（最新10件）
gymeat history list

# フィルタを使って履歴を検索
gymeat history list --goal bulk           # 増量プランのみ
gymeat history list --plan-type weekly    # 週間プランのみ
gymeat history list --last 7d             # 直近7日間

# 履歴の詳細を表示
gymeat history show <ID>
gymeat history show --latest              # 最新の履歴を表示

# 履歴を削除
gymeat history delete <ID>
```

**履歴機能の特徴:**
- プラン生成時に `--save` オプションで履歴に保存
- 履歴はJSON形式で保存され、ユーザーが直接確認可能
- 目的（bulk/cut/maintain）やプランタイプでフィルタ可能
- IDの先頭8文字で検索可能（完全なUUIDを入力する必要なし）

### オプション

```
Options:
  -g, --goal <GOAL>
          トレーニング目的 [default: maintain] [possible values: bulk, cut, maintain]

  -w, --weight <WEIGHT>
          体重 (kg) - カロリー計算に使用

      --height <HEIGHT>
          身長 (cm)

      --age <AGE>
          年齢 (歳)

      --gender <GENDER>
          性別 [possible values: male, female]

  -a, --activity <ACTIVITY>
          活動レベル [default: moderate]
          [possible values: sedentary, light, moderate, active, very-active]

  -c, --calories <CALORIES>
          カスタムカロリー目標 (kcal/日)

  -r, --recipe
          レシピ (調理手順) を表示

      --no-color
          カラー出力を無効化

  -v, --verbose
          詳細情報を表示

      --weekly
          7日間の週間プランを生成

      --start-date <START_DATE>
          週間プラン開始日 (YYYY-MM-DD形式, デフォルト: 今日)
          ※--weeklyと併用

  -o, --output <FORMAT>
          出力フォーマット [default: terminal]
          [possible values: terminal, json, json-pretty, csv, markdown, pdf]

      --output-file <PATH>
          出力先ファイルパス (指定しない場合は標準出力、PDF出力時は必須)

      --save
          生成したプランを履歴に保存

  -h, --help
          ヘルプを表示
```

## カロリー計算について

### 体組成情報から自動計算

体重、身長、年齢、性別を指定すると、Harris-Benedict式で基礎代謝量(BMR)を計算し、活動レベルを考慮してTDEE(1日の総消費カロリー)を算出します。

- **増量モード**: TDEE + 300kcal
- **減量モード**: TDEE - 500kcal
- **維持モード**: TDEE

### マクロ栄養素の配分

| 目的 | タンパク質 | 脂質 | 炭水化物 |
|------|-----------|------|----------|
| 増量 | 25% | 25% | 50% |
| 減量 | 40% | 30% | 30% |
| 維持 | 30% | 25% | 45% |

### デフォルトカロリー

体組成情報を指定しない場合:

- 増量: 2800kcal
- 減量: 2000kcal
- 維持: 2400kcal

## メニュー例

以下のような実際の料理が24種類用意されています:

**朝食 (6種類)**

- オートミール + プロテイン
- 卵3個のスクランブルエッグ + 全粒粉トースト
- ギリシャヨーグルト + グラノーラ
- プロテインパンケーキ
- 鮭おにぎり + 納豆 + 味噌汁
- アボカドトースト + ゆで卵

**昼食 (7種類)**

- 鶏胸肉のグリル定食
- 牛もも肉ステーキ + 焼き野菜
- 豚ヒレ肉の生姜焼き定食
- マグロのポケ丼
- 等

**夕食 (7種類)**

- サーモンのムニエル
- 鶏もも肉の照り焼き
- 白身魚のホイル焼き
- 等

**間食 (4種類)**

- プロテインバー
- プロテインシェイク + バナナ
- ナッツとドライフルーツ
- カッテージチーズ + フルーツ

## 実装例

```bash
# 男性、25歳、70kg、175cm、週3-5日トレーニング、増量目的
$ gymeat --goal bulk --weight 70 --height 175 --age 25 --gender male --activity moderate

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
     筋トレ用食事メニュー (増量モード)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
日付: 2026-01-05

目標栄養素:
  カロリー: 2972 kcal
  タンパク質: 186g (25%)
  脂質: 83g (25%)
  炭水化物: 372g (50%)
...
```

## 技術スタック

- **言語**: Rust (Edition 2021)
- **CLI**: clap v4.5
- **カラー出力**: colored v2.1
- **乱択**: rand v0.8
- **エラーハンドリング**: thiserror v1.0
- **日付**: chrono v0.4

## プロジェクト構造

```
gymeat/
├── src/
│   ├── main.rs              # エントリポイント
│   ├── lib.rs               # ライブラリルート
│   ├── cli.rs               # CLI引数定義
│   ├── config.rs            # 設定
│   ├── error.rs             # エラー型
│   ├── models/              # データモデル
│   ├── data/                # メニューデータ
│   ├── calculator/          # カロリー・栄養計算
│   ├── planner/             # プランニングロジック
│   └── output/              # 出力処理
└── tests/                   # テスト
```

## 今後の拡張予定

- [x] 週間プラン生成
- [x] JSON/CSV/Markdown/PDF出力
- [x] 月間プラン生成
- [x] 履歴管理機能
- [ ] 除外食材フィルタ
- [ ] 外部ファイルからのカスタムメニュー読み込み

## ライセンス

MIT

## 参考

- 栄養計算: Harris-Benedict式
- 栄養データ: 文部科学省 日本食品標準成分表を参考
