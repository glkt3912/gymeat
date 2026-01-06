use clap::Parser;
use gymeat::{
    cli::CliArgs, config::PlanConfig, data::MealDatabase, models::MealType,
    output::TerminalOutput, planner::DailyPlanner, Result,
};

fn main() -> Result<()> {
    // CLI引数をパース
    let args = CliArgs::parse();

    // 設定を作成
    let config = create_config(&args);

    // 初回実行時のガイダンス
    if args.weight.is_none() && args.calories.is_none() && !args.verbose {
        eprintln!("💡 ヒント: より正確なカロリー計算のために体組成情報を指定できます");
        eprintln!("   例: gymeat --goal bulk --weight 70 --height 175 --age 25 --gender male");
        eprintln!();
        eprintln!("   デフォルトカロリーで生成します ({}kcal)...", config.default_calories());
        eprintln!();
    }

    // 設定を検証
    if let Err(e) = config.validate() {
        eprintln!("❌ エラー: {}", e);
        eprintln!("\n💡 ヒント: 体組成情報 (体重、身長、年齢、性別) をすべて指定するか、");
        eprintln!("        --calories オプションでカロリーを直接指定してください。");
        eprintln!("\n📖 詳しくは --help をご覧ください");
        std::process::exit(1);
    }

    // メニューデータベースをロード
    let database = MealDatabase::new_embedded()?;

    if args.verbose {
        println!("✅ メニューデータベース読み込み完了: {} 種類", database.count());
        println!("   - 朝食: {} 種類", database.count_by_type(MealType::Breakfast));
        println!("   - 昼食: {} 種類", database.count_by_type(MealType::Lunch));
        println!("   - 夕食: {} 種類", database.count_by_type(MealType::Dinner));
        println!("   - 間食: {} 種類", database.count_by_type(MealType::Snack));
        println!();
    }

    // プランナーを作成して1日プランを生成
    let planner = DailyPlanner::new(&database, &config);
    let plan = planner.generate()?;

    // 出力
    let enable_color = !args.no_color && atty::is(atty::Stream::Stdout);
    let output = TerminalOutput::new(enable_color);
    output.print_daily_plan(&plan, &database, args.recipe);

    println!(); // 最後に改行

    Ok(())
}

fn create_config(args: &CliArgs) -> PlanConfig {
    PlanConfig {
        goal: args.goal.into(),
        weight: args.weight,
        height: args.height,
        age: args.age,
        gender: args.gender.map(|g| g.into()),
        activity_level: args.activity.into(),
        custom_calories: args.calories,
    }
}
