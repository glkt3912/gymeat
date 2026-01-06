# Gymeat - 筋トレ用食事メニュー生成CLI

gym + eat = gymeat 🏋️💪

目的別（増量・減量・維持）に最適化された食事プランを生成するRust製CLIツールです。

## 特徴

- **目的別メニュー**: 増量(Bulk)、減量(Cut)、維持(Maintain)の3つのモードに対応
- **精密なカロリー計算**: Harris-Benedict式を使用したBMR/TDEE計算
- **性別対応**: 男性・女性それぞれに最適化されたカロリー計算
- **24種類の筋トレ向けメニュー**: 高タンパクで栄養バランスの取れた実際の料理
- **最適化されたメニュー選択**: 目標カロリーに最も近いメニューを優先的に選択（±10%以内の精度）
- **レシピ表示**: 調理手順も確認可能
- **カラフルな出力**: 見やすいターミナル表示
- **初心者にも優しい**: 初回実行時のガイダンス、詳細なエラーメッセージ、達成度評価とアドバイス

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
```

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

- [ ] 週間プラン生成
- [ ] 月間プラン生成
- [ ] JSON/CSV出力
- [ ] 除外食材フィルタ
- [ ] 外部ファイルからのカスタムメニュー読み込み
- [ ] 履歴管理機能

## ライセンス

MIT

## 参考

- 栄養計算: Harris-Benedict式
- 栄養データ: 文部科学省 日本食品標準成分表を参考
