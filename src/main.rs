use chrono::NaiveDate;
use clap::Parser;
use gymeat::{
    cli::{CliArgs, OutputFormatArg},
    config::PlanConfig,
    data::MealDatabase,
    error::MealPlannerError,
    models::MealType,
    output::{
        write_daily_plan_to_pdf, write_monthly_plan_to_pdf, write_output,
        write_weekly_plan_to_pdf, CsvFormatter, JsonFormatter, MarkdownFormatter,
        OutputDestination, OutputFormatter, TerminalOutput,
    },
    planner::{DailyPlanner, MonthlyPlanner, WeeklyPlanner},
    Result,
};
use std::path::PathBuf;

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
        eprintln!(
            "   デフォルトカロリーで生成します ({}kcal)...",
            config.default_calories()
        );
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
        println!(
            "✅ メニューデータベース読み込み完了: {} 種類",
            database.count()
        );
        println!(
            "   - 朝食: {} 種類",
            database.count_by_type(MealType::Breakfast)
        );
        println!(
            "   - 昼食: {} 種類",
            database.count_by_type(MealType::Lunch)
        );
        println!(
            "   - 夕食: {} 種類",
            database.count_by_type(MealType::Dinner)
        );
        println!(
            "   - 間食: {} 種類",
            database.count_by_type(MealType::Snack)
        );
        println!();
    }

    // 出力先の決定
    let destination = if let Some(path) = &args.output_file {
        OutputDestination::File(PathBuf::from(path))
    } else {
        OutputDestination::Stdout
    };

    // 開始日のパース (週間/月間プラン共通)
    let start_date = if let Some(date_str) = &args.start_date {
        Some(
            NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
                .map_err(|_| MealPlannerError::InvalidDate(date_str.clone()))?,
        )
    } else {
        None
    };

    // 月間プラン、週間プラン、または日次プラン
    if args.monthly {
        // 月間プラン生成
        let planner = MonthlyPlanner::new(&database, &config);
        let plan = planner.generate(start_date)?;

        // フォーマット別出力
        match args.output {
            OutputFormatArg::Terminal => {
                let enable_color = !args.no_color && atty::is(atty::Stream::Stdout);
                let output = TerminalOutput::new(enable_color);
                output.print_monthly_plan(&plan, &database, args.recipe);
                println!(); // 最後に改行
            }
            OutputFormatArg::Json => {
                let formatter = JsonFormatter::new(false);
                let content = formatter.format_monthly_plan(&plan, &database, args.recipe)?;
                write_output(&content, destination)?;
                if args.output_file.is_some() {
                    println!("✅ JSON出力が完了しました: {}", args.output_file.unwrap());
                }
            }
            OutputFormatArg::JsonPretty => {
                let formatter = JsonFormatter::new(true);
                let content = formatter.format_monthly_plan(&plan, &database, args.recipe)?;
                write_output(&content, destination)?;
                if args.output_file.is_some() {
                    println!("✅ JSON出力が完了しました: {}", args.output_file.unwrap());
                }
            }
            OutputFormatArg::Csv => {
                let formatter = CsvFormatter::new();
                let content = formatter.format_monthly_plan(&plan, &database, args.recipe)?;
                write_output(&content, destination)?;
                if args.output_file.is_some() {
                    println!("✅ CSV出力が完了しました: {}", args.output_file.unwrap());
                }
            }
            OutputFormatArg::Markdown => {
                let formatter = MarkdownFormatter::new();
                let content = formatter.format_monthly_plan(&plan, &database, args.recipe)?;
                write_output(&content, destination)?;
                if args.output_file.is_some() {
                    println!(
                        "✅ Markdown出力が完了しました: {}",
                        args.output_file.unwrap()
                    );
                }
            }
            OutputFormatArg::Pdf => {
                if let Some(path_str) = &args.output_file {
                    let path = PathBuf::from(path_str);
                    write_monthly_plan_to_pdf(&plan, &database, args.recipe, &path)?;
                    println!("✅ PDF出力が完了しました: {}", path_str);
                } else {
                    return Err(MealPlannerError::OutputError(
                        "PDF出力には--output-fileオプションが必要です".to_string(),
                    ));
                }
            }
        }
    } else if args.weekly {
        // 週間プラン生成
        let planner = WeeklyPlanner::new(&database, &config);
        let plan = planner.generate(start_date)?;

        // フォーマット別出力
        match args.output {
            OutputFormatArg::Terminal => {
                let enable_color = !args.no_color && atty::is(atty::Stream::Stdout);
                let output = TerminalOutput::new(enable_color);
                output.print_weekly_plan(&plan, &database, args.recipe);
                println!(); // 最後に改行
            }
            OutputFormatArg::Json => {
                let formatter = JsonFormatter::new(false);
                let content = formatter.format_weekly_plan(&plan, &database, args.recipe)?;
                write_output(&content, destination)?;
                if args.output_file.is_some() {
                    println!("✅ JSON出力が完了しました: {}", args.output_file.unwrap());
                }
            }
            OutputFormatArg::JsonPretty => {
                let formatter = JsonFormatter::new(true);
                let content = formatter.format_weekly_plan(&plan, &database, args.recipe)?;
                write_output(&content, destination)?;
                if args.output_file.is_some() {
                    println!("✅ JSON出力が完了しました: {}", args.output_file.unwrap());
                }
            }
            OutputFormatArg::Csv => {
                let formatter = CsvFormatter::new();
                let content = formatter.format_weekly_plan(&plan, &database, args.recipe)?;
                write_output(&content, destination)?;
                if args.output_file.is_some() {
                    println!("✅ CSV出力が完了しました: {}", args.output_file.unwrap());
                }
            }
            OutputFormatArg::Markdown => {
                let formatter = MarkdownFormatter::new();
                let content = formatter.format_weekly_plan(&plan, &database, args.recipe)?;
                write_output(&content, destination)?;
                if args.output_file.is_some() {
                    println!(
                        "✅ Markdown出力が完了しました: {}",
                        args.output_file.unwrap()
                    );
                }
            }
            OutputFormatArg::Pdf => {
                if let Some(path_str) = &args.output_file {
                    let path = PathBuf::from(path_str);
                    write_weekly_plan_to_pdf(&plan, &database, args.recipe, &path)?;
                    println!("✅ PDF出力が完了しました: {}", path_str);
                } else {
                    return Err(MealPlannerError::OutputError(
                        "PDF出力には--output-fileオプションが必要です".to_string(),
                    ));
                }
            }
        }
    } else {
        // 日次プラン生成
        let planner = DailyPlanner::new(&database, &config);
        let plan = planner.generate()?;

        // フォーマット別出力
        match args.output {
            OutputFormatArg::Terminal => {
                let enable_color = !args.no_color && atty::is(atty::Stream::Stdout);
                let output = TerminalOutput::new(enable_color);
                output.print_daily_plan(&plan, &database, args.recipe);
                println!(); // 最後に改行
            }
            OutputFormatArg::Json => {
                let formatter = JsonFormatter::new(false);
                let content = formatter.format_daily_plan(&plan, &database, args.recipe)?;
                write_output(&content, destination)?;
                if args.output_file.is_some() {
                    println!("✅ JSON出力が完了しました: {}", args.output_file.unwrap());
                }
            }
            OutputFormatArg::JsonPretty => {
                let formatter = JsonFormatter::new(true);
                let content = formatter.format_daily_plan(&plan, &database, args.recipe)?;
                write_output(&content, destination)?;
                if args.output_file.is_some() {
                    println!("✅ JSON出力が完了しました: {}", args.output_file.unwrap());
                }
            }
            OutputFormatArg::Csv => {
                let formatter = CsvFormatter::new();
                let content = formatter.format_daily_plan(&plan, &database, args.recipe)?;
                write_output(&content, destination)?;
                if args.output_file.is_some() {
                    println!("✅ CSV出力が完了しました: {}", args.output_file.unwrap());
                }
            }
            OutputFormatArg::Markdown => {
                let formatter = MarkdownFormatter::new();
                let content = formatter.format_daily_plan(&plan, &database, args.recipe)?;
                write_output(&content, destination)?;
                if args.output_file.is_some() {
                    println!(
                        "✅ Markdown出力が完了しました: {}",
                        args.output_file.unwrap()
                    );
                }
            }
            OutputFormatArg::Pdf => {
                if let Some(path_str) = &args.output_file {
                    let path = PathBuf::from(path_str);
                    write_daily_plan_to_pdf(&plan, &database, args.recipe, &path)?;
                    println!("✅ PDF出力が完了しました: {}", path_str);
                } else {
                    return Err(MealPlannerError::OutputError(
                        "PDF出力には--output-fileオプションが必要です".to_string(),
                    ));
                }
            }
        }
    }

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
