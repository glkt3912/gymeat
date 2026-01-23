use chrono::NaiveDate;
use clap::Parser;
use gymeat::{
    cli::{CliArgs, Commands, HistoryCommands, OutputFormatArg},
    config::PlanConfig,
    data::MealDatabase,
    error::MealPlannerError,
    history::{HistoryEntry, HistoryStorage, PlanData},
    models::MealType,
    output::{
        write_daily_plan_to_pdf, write_monthly_plan_to_pdf, write_output, write_weekly_plan_to_pdf,
        CsvFormatter, JsonFormatter, MarkdownFormatter, OutputDestination, OutputFormatter,
        TerminalOutput,
    },
    planner::{DailyPlanner, MonthlyPlanner, WeeklyPlanner},
    Result,
};
use std::path::PathBuf;

fn main() -> Result<()> {
    // CLI引数をパース
    let args = CliArgs::parse();

    // サブコマンドの処理
    if let Some(command) = &args.command {
        return handle_command(command);
    }

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
                if let Some(path) = &args.output_file {
                    println!("✅ JSON出力が完了しました: {}", path);
                }
            }
            OutputFormatArg::JsonPretty => {
                let formatter = JsonFormatter::new(true);
                let content = formatter.format_monthly_plan(&plan, &database, args.recipe)?;
                write_output(&content, destination)?;
                if let Some(path) = &args.output_file {
                    println!("✅ JSON出力が完了しました: {}", path);
                }
            }
            OutputFormatArg::Csv => {
                let formatter = CsvFormatter::new();
                let content = formatter.format_monthly_plan(&plan, &database, args.recipe)?;
                write_output(&content, destination)?;
                if let Some(path) = &args.output_file {
                    println!("✅ CSV出力が完了しました: {}", path);
                }
            }
            OutputFormatArg::Markdown => {
                let formatter = MarkdownFormatter::new();
                let content = formatter.format_monthly_plan(&plan, &database, args.recipe)?;
                write_output(&content, destination)?;
                if let Some(path) = &args.output_file {
                    println!("✅ Markdown出力が完了しました: {}", path);
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

        // 履歴に保存
        if args.save {
            let target_calories = config
                .custom_calories
                .unwrap_or_else(|| config.default_calories());
            save_to_history(config.goal, target_calories, PlanData::Monthly(plan))?;
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
                if let Some(path) = &args.output_file {
                    println!("✅ JSON出力が完了しました: {}", path);
                }
            }
            OutputFormatArg::JsonPretty => {
                let formatter = JsonFormatter::new(true);
                let content = formatter.format_weekly_plan(&plan, &database, args.recipe)?;
                write_output(&content, destination)?;
                if let Some(path) = &args.output_file {
                    println!("✅ JSON出力が完了しました: {}", path);
                }
            }
            OutputFormatArg::Csv => {
                let formatter = CsvFormatter::new();
                let content = formatter.format_weekly_plan(&plan, &database, args.recipe)?;
                write_output(&content, destination)?;
                if let Some(path) = &args.output_file {
                    println!("✅ CSV出力が完了しました: {}", path);
                }
            }
            OutputFormatArg::Markdown => {
                let formatter = MarkdownFormatter::new();
                let content = formatter.format_weekly_plan(&plan, &database, args.recipe)?;
                write_output(&content, destination)?;
                if let Some(path) = &args.output_file {
                    println!("✅ Markdown出力が完了しました: {}", path);
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

        // 履歴に保存
        if args.save {
            let target_calories = config
                .custom_calories
                .unwrap_or_else(|| config.default_calories());
            save_to_history(config.goal, target_calories, PlanData::Weekly(plan))?;
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
                if let Some(path) = &args.output_file {
                    println!("✅ JSON出力が完了しました: {}", path);
                }
            }
            OutputFormatArg::JsonPretty => {
                let formatter = JsonFormatter::new(true);
                let content = formatter.format_daily_plan(&plan, &database, args.recipe)?;
                write_output(&content, destination)?;
                if let Some(path) = &args.output_file {
                    println!("✅ JSON出力が完了しました: {}", path);
                }
            }
            OutputFormatArg::Csv => {
                let formatter = CsvFormatter::new();
                let content = formatter.format_daily_plan(&plan, &database, args.recipe)?;
                write_output(&content, destination)?;
                if let Some(path) = &args.output_file {
                    println!("✅ CSV出力が完了しました: {}", path);
                }
            }
            OutputFormatArg::Markdown => {
                let formatter = MarkdownFormatter::new();
                let content = formatter.format_daily_plan(&plan, &database, args.recipe)?;
                write_output(&content, destination)?;
                if let Some(path) = &args.output_file {
                    println!("✅ Markdown出力が完了しました: {}", path);
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

        // 履歴に保存
        if args.save {
            let target_calories = config
                .custom_calories
                .unwrap_or_else(|| config.default_calories());
            save_to_history(config.goal, target_calories, PlanData::Daily(plan))?;
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

/// サブコマンドを処理
fn handle_command(command: &Commands) -> Result<()> {
    match command {
        Commands::History(history_args) => handle_history_command(&history_args.command),
    }
}

/// 履歴コマンドを処理
fn handle_history_command(command: &HistoryCommands) -> Result<()> {
    let storage = HistoryStorage::new()?;

    match command {
        HistoryCommands::List(args) => handle_history_list(&storage, args),
        HistoryCommands::Show(args) => handle_history_show(&storage, args),
        HistoryCommands::Delete(args) => handle_history_delete(&storage, args),
    }
}

/// 履歴一覧を表示
fn handle_history_list(
    storage: &HistoryStorage,
    args: &gymeat::cli::HistoryListArgs,
) -> Result<()> {
    let index = storage.load_index()?;

    if index.is_empty() {
        println!("履歴がありません");
        println!();
        println!("💡 ヒント: --save オプションでプランを保存できます");
        println!("   例: gymeat --goal bulk --save");
        return Ok(());
    }

    // フィルタリング
    let mut entries: Vec<_> = index.entries.iter().collect();

    // 目的でフィルタ
    if let Some(goal_arg) = &args.goal {
        let goal: gymeat::models::Goal = (*goal_arg).into();
        entries.retain(|e| e.goal == goal);
    }

    // プランタイプでフィルタ
    if let Some(plan_type_arg) = &args.plan_type {
        let plan_type: gymeat::history::PlanType = (*plan_type_arg).into();
        entries.retain(|e| e.plan_type == plan_type);
    }

    // 日付範囲でフィルタ
    if let Some(from) = &args.from {
        entries.retain(|e| e.created_at.as_str() >= from.as_str());
    }
    if let Some(to) = &args.to {
        entries.retain(|e| e.created_at.as_str() <= to.as_str());
    }

    // 直近の期間でフィルタ
    if let Some(last) = &args.last {
        if let Some(days) = parse_duration(last) {
            let threshold = chrono::Local::now() - chrono::Duration::days(days);
            let threshold_str = threshold.to_rfc3339();
            entries.retain(|e| e.created_at.as_str() >= threshold_str.as_str());
        }
    }

    // 新しい順に並べ替え
    entries.reverse();

    // 件数制限
    let entries: Vec<_> = entries.into_iter().take(args.limit).collect();

    if entries.is_empty() {
        println!("条件に一致する履歴がありません");
        return Ok(());
    }

    // ヘッダー
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!(
        "{:<10} {:<12} {:<10} {:<10} {:<8} {:<24}",
        "ID", "作成日", "時刻", "タイプ", "目的", "期間"
    );
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    for entry in &entries {
        let goal_str = match entry.goal {
            gymeat::models::Goal::Bulk => "増量",
            gymeat::models::Goal::Cut => "減量",
            gymeat::models::Goal::Maintain => "維持",
        };
        let type_str = match entry.plan_type {
            gymeat::history::PlanType::Daily => "日次",
            gymeat::history::PlanType::Weekly => "週間",
            gymeat::history::PlanType::Monthly => "月間",
        };
        let date_range = if entry.start_date == entry.end_date {
            entry.start_date.clone()
        } else {
            format!("{} ~ {}", entry.start_date, entry.end_date)
        };

        println!(
            "{:<10} {:<12} {:<10} {:<10} {:<8} {:<24}",
            entry.short_id(),
            entry.created_date(),
            entry.created_time(),
            type_str,
            goal_str,
            date_range
        );
    }

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("合計: {} 件", entries.len());

    Ok(())
}

/// 履歴詳細を表示
fn handle_history_show(
    storage: &HistoryStorage,
    args: &gymeat::cli::HistoryShowArgs,
) -> Result<()> {
    let entry = if args.latest {
        storage
            .load_latest()?
            .ok_or_else(|| MealPlannerError::HistoryError("履歴がありません".to_string()))?
    } else if let Some(id) = &args.id {
        storage.load_entry(id)?
    } else {
        return Err(MealPlannerError::HistoryError(
            "IDまたは--latestを指定してください".to_string(),
        ));
    };

    // 基本情報を表示
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("履歴詳細");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("ID: {}", entry.id);
    println!("作成日時: {}", entry.created_at);
    println!(
        "プランタイプ: {}",
        match entry.plan_type {
            gymeat::history::PlanType::Daily => "日次",
            gymeat::history::PlanType::Weekly => "週間",
            gymeat::history::PlanType::Monthly => "月間",
        }
    );
    println!(
        "目的: {}",
        match entry.goal {
            gymeat::models::Goal::Bulk => "増量",
            gymeat::models::Goal::Cut => "減量",
            gymeat::models::Goal::Maintain => "維持",
        }
    );
    println!("目標カロリー: {:.0} kcal", entry.target_calories);
    println!();

    // プランデータをJSON形式で表示
    let json = serde_json::to_string_pretty(&entry.plan)
        .map_err(|e| MealPlannerError::HistoryError(format!("JSONシリアライズエラー: {}", e)))?;
    println!("プランデータ:");
    println!("{}", json);

    Ok(())
}

/// 履歴を削除
fn handle_history_delete(
    storage: &HistoryStorage,
    args: &gymeat::cli::HistoryDeleteArgs,
) -> Result<()> {
    // 削除前に確認
    let index = storage.load_index()?;
    let entry = index
        .find_entry(&args.id)
        .ok_or_else(|| MealPlannerError::HistoryNotFound(args.id.clone()))?;

    println!("以下の履歴を削除します:");
    println!("  ID: {}", entry.short_id());
    println!("  作成日: {}", entry.created_date());
    println!(
        "  タイプ: {}",
        match entry.plan_type {
            gymeat::history::PlanType::Daily => "日次",
            gymeat::history::PlanType::Weekly => "週間",
            gymeat::history::PlanType::Monthly => "月間",
        }
    );

    storage.delete_entry(&args.id)?;
    println!();
    println!("✅ 履歴を削除しました");

    Ok(())
}

/// 期間文字列をパース (例: "7d" -> 7, "30d" -> 30)
fn parse_duration(s: &str) -> Option<i64> {
    let s = s.trim().to_lowercase();
    if s.ends_with('d') {
        s[..s.len() - 1].parse().ok()
    } else {
        s.parse().ok()
    }
}

/// プランを履歴に保存
fn save_to_history(goal: gymeat::models::Goal, target_calories: f32, plan: PlanData) -> Result<()> {
    let storage = HistoryStorage::new()?;
    let entry = HistoryEntry::new(goal, target_calories, plan);
    storage.save_entry(&entry)?;
    println!();
    println!("✅ 履歴に保存しました (ID: {})", entry.short_id());
    Ok(())
}
