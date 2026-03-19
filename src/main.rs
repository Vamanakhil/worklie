mod collector;
mod parser;
mod analyzer;
mod report;
mod cache;
mod cache_manager;
mod security;
mod error;
mod workspace;
mod output_formatter;
mod config;
mod reliability;
mod logging;

use clap::{Parser, Subcommand};
use std::process;
use parser::history_parser::HistoryParser;
use chrono;
use tracing::{info, debug, error as log_error};

/// Worklie - Developer Activity Intelligence Engine
#[derive(Parser)]
#[command(name = "worklie")]
#[command(about = "Local-first developer activity intelligence CLI", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate daily work report
    Report {
        /// Output in JSON format
        #[arg(long)]
        json: bool,
    },
    /// Generate weekly work report
    Weekly {
        /// Output in JSON format
        #[arg(long)]
        json: bool,
    },
    /// Generate standup report
    Standup {
        /// Output in JSON format
        #[arg(long)]
        json: bool,
    },
    /// Show statistics
    Stats {
        /// Output in JSON format
        #[arg(long)]
        json: bool,
    },
    /// Analyze work sessions
    Sessions {
        /// Session gap threshold in minutes
        #[arg(long, default_value = "30")]
        gap: u64,
    },
    /// Detect deep work/focus time
    Focus,
    /// Classify work by type
    Classify,
    /// Analyze context switching
    Switches,
    /// Generate portfolio/resume
    Portfolio {
        /// Output format
        #[arg(long, default_value = "text")]
        format: String,
    },
    /// Show productivity metrics
    Metrics,
    /// Analyze focus areas
    FocusAreas,
    /// Generate changelog for release
    Changelog,
    /// Generate PR description
    Pr {
        /// Branch name for comparison
        #[arg(short, long)]
        from: Option<String>,
        /// Output in JSON format
        #[arg(long)]
        json: bool,
    },
    /// Generate resume bullets
    Resume,
    /// Analyze work across multiple repositories
    MultiRepo {
        /// Workspace root directory (default: current directory)
        #[arg(long)]
        workspace: Option<String>,
        /// Output in JSON format
        #[arg(long)]
        json: bool,
    },
    /// Visualize commit activity as time-based heatmap
    Heatmap {
        /// Number of days of history to analyze
        #[arg(long, default_value = "90")]
        days: u32,
        /// Output in JSON format
        #[arg(long)]
        json: bool,
    },
    /// Manage Worklie configuration
    Config {
        /// Show current configuration
        #[arg(long)]
        show: bool,
        /// Reset to defaults
        #[arg(long)]
        reset: bool,
    },
    /// Run system diagnostics and health checks
    Diagnose,
    /// Show version
    Version,
}

fn main() {
    // Initialize structured logging before parsing arguments
    logging::init_logging();

    let args = Args::parse();
    info!("Starting worklie with command: {}", std::env::args().collect::<Vec<_>>().join(" "));

    if let Err(e) = run(args) {
        log_error!("Error: {}", e);
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn run(args: Args) -> anyhow::Result<()> {
    match args.command {
        Commands::Report { json } => handle_report_command(json),
        Commands::Weekly { json } => handle_weekly_command(json),
        Commands::Standup { json } => handle_standup_command(json),
        Commands::Stats { json } => handle_stats_command(json),
        Commands::Sessions { gap } => handle_sessions_command(gap * 60),
        Commands::Focus => handle_focus_detection(),
        Commands::Classify => handle_classify_command(),
        Commands::Switches => handle_switches_command(),
        Commands::Portfolio { format } => handle_portfolio_command(&format),
        Commands::Metrics => handle_metrics_command(),
        Commands::FocusAreas => handle_focus_command(),
        Commands::Changelog => handle_changelog_command(),
        Commands::Pr { from, json } => handle_pr_command(from, json),
        Commands::Resume => handle_resume_command(),
        Commands::MultiRepo { workspace, json } => handle_multi_repo_command(workspace, json),
        Commands::Heatmap { days, json } => handle_heatmap_command(days, json),
        Commands::Config { show, reset } => handle_config_command(show, reset),
        Commands::Diagnose => handle_diagnose_command(),
        Commands::Version => {
            println!("worklie {}", env!("CARGO_PKG_VERSION"));
            Ok(())
        }
    }
}

fn handle_report_command(json: bool) -> anyhow::Result<()> {
    debug!("Executing report command with json={}", json);

    // Use threads to collect git and history data in parallel
    let history_handle = std::thread::spawn(|| {
        let collector = collector::history::HistoryCollector::new();
        collector.read_history()
    });

    let git_handle = std::thread::spawn(|| {
        let collector = collector::git::GitCollector::new();
        if collector.is_git_repo() {
            collector.recent_commits(20)
        } else {
            Ok(Vec::new())
        }
    });

    // Wait for both to complete
    let history_commands = history_handle
        .join()
        .map_err(|_| anyhow::anyhow!(error::WorklieError::ThreadError("History collection thread panicked".to_string())))?
        .map_err(|e| anyhow::anyhow!(error::WorklieError::HistoryCollectionError(e.to_string())))?;

    let git_commits = git_handle
        .join()
        .map_err(|_| anyhow::anyhow!(error::WorklieError::ThreadError("Git collection thread panicked".to_string())))?
        .map_err(|e| anyhow::anyhow!(error::WorklieError::GitCollectionError(e.to_string())))?;

    debug!("Collected {} history commands and {} git commits", history_commands.len(), git_commits.len());

    // Parse data
    let history_parser = HistoryParser::new();
    let git_parser = parser::history_parser::GitParser::new();

    let parsed_commands = history_parser.parse_commands(history_commands);
    let parsed_commits = git_parser.parse_commits(git_commits);

    debug!("Parsed {} commands and {} commits", parsed_commands.len(), parsed_commits.len());

    // Analyze data
    let context_engine = analyzer::context_inference::ContextInferenceEngine::new();
    let context = context_engine.infer_context();

    let clusterer = analyzer::activity_clusterer::ActivityClusterer::new();
    let activities = clusterer.cluster_activities(parsed_commands, parsed_commits.clone());

    debug!("Clustered into {} activities", activities.len());

    // Save to cache
    if let Ok(mut cache_manager) = cache_manager::CacheManager::new() {
        let git_collector = collector::git::GitCollector::new();
        if let Some(repo_path) = git_collector.repo_root() {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();

            let today = chrono::Local::now().format("%Y-%m-%d").to_string();

            let snapshot = cache_manager::DailySnapshot {
                timestamp: now,
                date: today,
                repo_path,
                activities: activities.clone(),
                parsed_commits,
            };

            match cache_manager.save_day_snapshot(snapshot) {
                Ok(_) => debug!("Saved snapshot to cache"),
                Err(e) => debug!("Failed to save snapshot: {}", e),
            }
        }
    }

    // Generate report
    let report_generator = report::daily_report::DailyReportGenerator::new();

    if json {
        let report_data = report_generator.generate_report_data(activities, &context);
        let json_output = serde_json::to_string_pretty(&report_data)?;
        println!("{}", json_output);
    } else {
        let report = report_generator.generate_report(activities, &context);
        print!("{}", report);
    }

    info!("Report generation completed successfully");
    Ok(())
}

fn handle_weekly_command(json: bool) -> anyhow::Result<()> {
    let git_collector = collector::git::GitCollector::new();

    // Try to use cache-based trends if in a git repo
    if let Some(repo_path) = git_collector.repo_root() {
        if let Ok(cache_manager) = cache_manager::CacheManager::new() {
            match report::weekly_report::WeeklyReportGenerator::new()
                .generate_from_cache(&cache_manager, &repo_path) {
                Ok(report_data) => {
                    if json {
                        let json_output = serde_json::to_string_pretty(&report_data)?;
                        println!("{}", json_output);
                    } else {
                        let report = report::weekly_report::WeeklyReportGenerator::new()
                            .generate_report(&report_data);
                        print!("{}", report);
                    }
                    return Ok(());
                }
                Err(_) => {
                    // Fall through to current week analysis if cache is empty
                }
            }
        }
    }

    // Fallback: Analyze current week without historical trends
    let history_collector = collector::history::HistoryCollector::new();
    let git_collector = collector::git::GitCollector::new();

    let history_commands = history_collector.read_history()?;
    let git_commits = if git_collector.is_git_repo() {
        git_collector.recent_commits(100)?
    } else {
        Vec::new()
    };

    let history_parser = HistoryParser::new();
    let git_parser = parser::history_parser::GitParser::new();

    let parsed_commands = history_parser.parse_commands(history_commands);
    let parsed_commits = git_parser.parse_commits(git_commits);

    let context_engine = analyzer::context_inference::ContextInferenceEngine::new();
    let context = context_engine.infer_context();

    let clusterer = analyzer::activity_clusterer::ActivityClusterer::new();
    let activities = clusterer.cluster_activities(parsed_commands, parsed_commits);

    let report_generator = report::weekly_report::WeeklyReportGenerator::new();

    if json {
        let report_data = report_generator.generate_report_data(activities.clone(), &context);
        let json_output = serde_json::to_string_pretty(&report_data)?;
        println!("{}", json_output);
    } else {
        let report = report_generator.generate_report_legacy(activities, &context);
        print!("{}", report);
    }

    Ok(())
}

fn handle_standup_command(json: bool) -> anyhow::Result<()> {
    let history_collector = collector::history::HistoryCollector::new();
    let git_collector = collector::git::GitCollector::new();

    let history_commands = history_collector.read_history()?;
    let git_commits = if git_collector.is_git_repo() {
        git_collector.recent_commits(20)?
    } else {
        Vec::new()
    };

    let history_parser = HistoryParser::new();
    let git_parser = parser::history_parser::GitParser::new();

    let parsed_commands = history_parser.parse_commands(history_commands);
    let parsed_commits = git_parser.parse_commits(git_commits);

    let context_engine = analyzer::context_inference::ContextInferenceEngine::new();
    let context = context_engine.infer_context();

    let clusterer = analyzer::activity_clusterer::ActivityClusterer::new();
    let activities = clusterer.cluster_activities(parsed_commands, parsed_commits);

    let report_generator = report::standup_report::StandupReportGenerator::new();

    if json {
        let report_data = report_generator.generate_report_data(activities, &context);
        let json_output = serde_json::to_string_pretty(&report_data)?;
        println!("{}", json_output);
    } else {
        let report = report_generator.generate_report(activities, &context);
        print!("{}", report);
    }

    Ok(())
}

fn handle_stats_command(json: bool) -> anyhow::Result<()> {
    let history_collector = collector::history::HistoryCollector::new();
    let git_collector = collector::git::GitCollector::new();

    let history_commands = history_collector.read_history()?;
    let git_commits = if git_collector.is_git_repo() {
        git_collector.recent_commits(100)?
    } else {
        Vec::new()
    };

    let history_parser = HistoryParser::new();
    let git_parser = parser::history_parser::GitParser::new();

    let parsed_commands = history_parser.parse_commands(history_commands);
    let parsed_commits = git_parser.parse_commits(git_commits);

    if json {
        let stats = serde_json::json!({
            "commands_executed": parsed_commands.len(),
            "commits_made": parsed_commits.len(),
            "unique_commands": parsed_commands.iter().map(|c| &c.command).collect::<std::collections::HashSet<_>>().len(),
        });
        println!("{}", serde_json::to_string_pretty(&stats)?);
    } else {
        println!("Worklie Statistics");
        println!("==================\n");
        println!("Commands executed: {}", parsed_commands.len());
        println!("Commits made: {}", parsed_commits.len());
        println!(
            "Unique commands: {}",
            parsed_commands
                .iter()
                .map(|c| &c.command)
                .collect::<std::collections::HashSet<_>>()
                .len()
        );
    }

    Ok(())
}

fn handle_focus_command() -> anyhow::Result<()> {
    let history_collector = collector::history::HistoryCollector::new();
    let git_collector = collector::git::GitCollector::new();

    let history_commands = history_collector.read_history()?;
    let git_commits = if git_collector.is_git_repo() {
        git_collector.recent_commits(50)?
    } else {
        Vec::new()
    };

    let history_parser = HistoryParser::new();
    let git_parser = parser::history_parser::GitParser::new();

    let parsed_commands = history_parser.parse_commands(history_commands);
    let _parsed_commits = git_parser.parse_commits(git_commits);

    let context_engine = analyzer::context_inference::ContextInferenceEngine::new();
    let context = context_engine.infer_context();

    println!("Focus Analysis");
    println!("==============\n");

    if let Some(domain) = &context.domain {
        println!("Primary Domain: {}", domain);
    }
    if let Some(work_type) = &context.work_type {
        println!("Work Type: {}", work_type);
    }

    // Analyze command patterns
    let mut tool_usage = std::collections::HashMap::new();
    for cmd in &parsed_commands {
        let tool = cmd.command.split_whitespace().next().unwrap_or("unknown");
        *tool_usage.entry(tool).or_insert(0) += 1;
    }

    if !tool_usage.is_empty() {
        println!("\nMost Used Tools:");
        let mut tools: Vec<_> = tool_usage.iter().collect();
        tools.sort_by_key(|&(_, count)| std::cmp::Reverse(*count));
        for (tool, count) in tools.iter().take(5) {
            println!("  • {} ({}x)", tool, count);
        }
    }

    Ok(())
}

fn handle_changelog_command() -> anyhow::Result<()> {
    let git_collector = collector::git::GitCollector::new();

    if !git_collector.is_git_repo() {
        println!("Not in a git repository");
        return Ok(());
    }

    println!("Changelog");
    println!("=========\n");

    let commits = git_collector.recent_commits(50)?;
    let git_parser = parser::history_parser::GitParser::new();
    let parsed_commits = git_parser.parse_commits(commits);

    let mut features = Vec::new();
    let mut fixes = Vec::new();
    let mut other = Vec::new();

    for commit in &parsed_commits {
        let msg_lower = commit.message.to_lowercase();
        if msg_lower.starts_with("feat") || msg_lower.starts_with("feature") {
            features.push(&commit.message);
        } else if msg_lower.starts_with("fix") || msg_lower.starts_with("bug") {
            fixes.push(&commit.message);
        } else {
            other.push(&commit.message);
        }
    }

    if !features.is_empty() {
        println!("### Features");
        for msg in features {
            println!("- {}", msg);
        }
        println!();
    }

    if !fixes.is_empty() {
        println!("### Bug Fixes");
        for msg in fixes {
            println!("- {}", msg);
        }
        println!();
    }

    if !other.is_empty() {
        println!("### Other Changes");
        for msg in other {
            println!("- {}", msg);
        }
    }

    Ok(())
}

fn handle_pr_command(_from: Option<String>, json: bool) -> anyhow::Result<()> {
    let history_collector = collector::history::HistoryCollector::new();
    let git_collector = collector::git::GitCollector::new();

    let history_commands = history_collector.read_history()?;
    let git_commits = if git_collector.is_git_repo() {
        git_collector.recent_commits(20)?
    } else {
        Vec::new()
    };

    let history_parser = HistoryParser::new();
    let git_parser = parser::history_parser::GitParser::new();

    let parsed_commands = history_parser.parse_commands(history_commands);
    let parsed_commits = git_parser.parse_commits(git_commits);

    let context_engine = analyzer::context_inference::ContextInferenceEngine::new();
    let context = context_engine.infer_context();

    let clusterer = analyzer::activity_clusterer::ActivityClusterer::new();
    let activities = clusterer.cluster_activities(parsed_commands, parsed_commits);

    let pr_summary = format!(
        "## Summary\n\n{} commits, {} activities\n\n## Changes\n\n{}",
        activities.iter().map(|a| a.commits.len()).sum::<usize>(),
        activities.len(),
        activities
            .iter()
            .take(5)
            .map(|a| format!("- {}", a.title))
            .collect::<Vec<_>>()
            .join("\n")
    );

    if json {
        let pr_data = serde_json::json!({
            "summary": pr_summary,
            "branch": context.branch_name,
            "project": context.project_name,
        });
        println!("{}", serde_json::to_string_pretty(&pr_data)?);
    } else {
        println!("PR Description");
        println!("===============\n");
        println!("{}", pr_summary);
    }

    Ok(())
}

fn handle_resume_command() -> anyhow::Result<()> {
    let history_collector = collector::history::HistoryCollector::new();
    let git_collector = collector::git::GitCollector::new();

    let history_commands = history_collector.read_history()?;
    let git_commits = if git_collector.is_git_repo() {
        git_collector.recent_commits(50)?
    } else {
        Vec::new()
    };

    let history_parser = HistoryParser::new();
    let git_parser = parser::history_parser::GitParser::new();

    let parsed_commands = history_parser.parse_commands(history_commands);
    let parsed_commits = git_parser.parse_commits(git_commits);

    let context_engine = analyzer::context_inference::ContextInferenceEngine::new();
    let context = context_engine.infer_context();

    let clusterer = analyzer::activity_clusterer::ActivityClusterer::new();
    let activities = clusterer.cluster_activities(parsed_commands, parsed_commits);

    println!("Resume Bullets");
    println!("==============\n");

    if let Some(project) = &context.project_name {
        println!("Project: {}", project);
    }

    for activity in activities.iter().take(10) {
        println!("• {}", activity.title);
        if !activity.commits.is_empty() {
            println!("  Commits: {}", activity.commits.len());
        }
    }

    Ok(())
}

fn handle_sessions_command(gap_threshold: u64) -> anyhow::Result<()> {
    let history_collector = collector::history::HistoryCollector::new();
    let git_collector = collector::git::GitCollector::new();

    let history_commands = history_collector.read_history()?;
    let git_commits = if git_collector.is_git_repo() {
        git_collector.recent_commits(50)?
    } else {
        Vec::new()
    };

    let history_parser = HistoryParser::new();
    let git_parser = parser::history_parser::GitParser::new();

    let parsed_commands = history_parser.parse_commands(history_commands);
    let parsed_commits = git_parser.parse_commits(git_commits);

    let detector = analyzer::session::SessionDetector::new()
        .with_gap_threshold(gap_threshold);
    let sessions = detector.detect_sessions(parsed_commands, &parsed_commits);

    println!("Work Sessions Today");
    println!("===================\n");

    if sessions.is_empty() {
        println!("No sessions detected yet");
        return Ok(());
    }

    let mut total_duration: u64 = 0;
    let mut total_focus: f32 = 0.0;

    for session in &sessions {
        total_duration += session.duration_minutes;
        total_focus += session.focus_score;

        println!("Session {} ({} UTC) - {}m - {}",
            session.id + 1,
            session.start_time,
            session.duration_minutes,
            session.work_type.display_name()
        );
        println!("  Commands: {}", session.commands_count);
        println!("  Commits: {}", session.commits.len());
        println!("  Focus Score: {:.2} / 1.0", session.focus_score);
        if !session.commits.is_empty() {
            println!("  Work: {}", session.commits[0].message);
        }
        println!();
    }

    let avg_focus = total_focus / sessions.len() as f32;
    println!("SUMMARY:");
    println!("  Total work time: {}h {}", total_duration / 60, total_duration % 60);
    println!("  Sessions: {}", sessions.len());
    println!("  Average duration: {}m", total_duration / sessions.len() as u64);
    println!("  Average focus: {:.2} / 1.0", avg_focus);

    Ok(())
}

fn handle_focus_detection() -> anyhow::Result<()> {
    let history_collector = collector::history::HistoryCollector::new();
    let git_collector = collector::git::GitCollector::new();

    let history_commands = history_collector.read_history()?;
    let git_commits = if git_collector.is_git_repo() {
        git_collector.recent_commits(50)?
    } else {
        Vec::new()
    };

    let history_parser = HistoryParser::new();
    let git_parser = parser::history_parser::GitParser::new();

    let parsed_commands = history_parser.parse_commands(history_commands);
    let parsed_commits = git_parser.parse_commits(git_commits);

    let detector = analyzer::session::SessionDetector::new();
    let sessions = detector.detect_sessions(parsed_commands, &parsed_commits);

    println!("Deep Work Sessions");
    println!("==================\n");

    let mut deep_work_sessions: Vec<_> = sessions
        .iter()
        .filter(|s| s.duration_minutes >= 120) // 2+ hours
        .collect();

    deep_work_sessions.sort_by_key(|s| std::cmp::Reverse(s.duration_minutes));

    if deep_work_sessions.is_empty() {
        println!("No deep work sessions (2+ hours) detected");
        return Ok(());
    }

    for session in &deep_work_sessions {
        println!("{}m - {}",
            session.duration_minutes,
            session.work_type.display_name()
        );
        if !session.commits.is_empty() {
            println!("  {}", session.commits[0].message);
        }
    }

    let total_deep: u64 = deep_work_sessions.iter().map(|s| s.duration_minutes).sum();
    println!("\nTotal deep work: {}h {}m", total_deep / 60, total_deep % 60);

    Ok(())
}

fn handle_classify_command() -> anyhow::Result<()> {
    let history_collector = collector::history::HistoryCollector::new();
    let git_collector = collector::git::GitCollector::new();

    let history_commands = history_collector.read_history()?;
    let git_commits = if git_collector.is_git_repo() {
        git_collector.recent_commits(50)?
    } else {
        Vec::new()
    };

    let history_parser = HistoryParser::new();
    let git_parser = parser::history_parser::GitParser::new();

    let parsed_commands = history_parser.parse_commands(history_commands);
    let parsed_commits = git_parser.parse_commits(git_commits);

    let detector = analyzer::session::SessionDetector::new();
    let sessions = detector.detect_sessions(parsed_commands, &parsed_commits);

    println!("Work Classification");
    println!("===================\n");

    let mut type_counts = std::collections::HashMap::new();
    for session in &sessions {
        *type_counts.entry(session.work_type.clone()).or_insert(0) += 1;
    }

    let mut types: Vec<_> = type_counts.iter().collect();
    types.sort_by_key(|&(_, count)| std::cmp::Reverse(*count));

    for (work_type, count) in types {
        let percentage = (*count as f32 / sessions.len() as f32) * 100.0;
        println!("  • {}: {:.0}% ({} sessions)", work_type.display_name(), percentage, count);
    }

    Ok(())
}

fn handle_switches_command() -> anyhow::Result<()> {
    let history_collector = collector::history::HistoryCollector::new();
    let git_collector = collector::git::GitCollector::new();

    let history_commands = history_collector.read_history()?;
    let git_commits = if git_collector.is_git_repo() {
        git_collector.recent_commits(50)?
    } else {
        Vec::new()
    };

    let history_parser = HistoryParser::new();
    let git_parser = parser::history_parser::GitParser::new();

    let parsed_commands = history_parser.parse_commands(history_commands);
    let parsed_commits = git_parser.parse_commits(git_commits);

    let detector = analyzer::session::SessionDetector::new();
    let sessions = detector.detect_sessions(parsed_commands, &parsed_commits);

    println!("Context Switching Analysis");
    println!("==========================\n");

    let switches = sessions.len().saturating_sub(1);
    let recovery_per_switch = 15; // minutes
    let estimated_loss = switches * recovery_per_switch;

    println!("Total switches: {}", switches);
    println!("Estimated recovery time: ~{} minutes per switch", recovery_per_switch);
    println!("Estimated lost productivity: ~{}m ({:.1}h)\n", estimated_loss, estimated_loss as f32 / 60.0);

    if estimated_loss > 60 {
        println!("⚠️  Context switching cost is significant!");
        println!("Recommendation: Batch your work by task type");
    } else {
        println!("✓ Good context switching discipline");
    }

    Ok(())
}

fn handle_portfolio_command(format: &str) -> anyhow::Result<()> {
    let history_collector = collector::history::HistoryCollector::new();
    let git_collector = collector::git::GitCollector::new();

    let history_commands = history_collector.read_history()?;
    let git_commits = if git_collector.is_git_repo() {
        git_collector.recent_commits(100)?
    } else {
        Vec::new()
    };

    let history_parser = HistoryParser::new();
    let git_parser = parser::history_parser::GitParser::new();

    let parsed_commands = history_parser.parse_commands(history_commands);
    let parsed_commits = git_parser.parse_commits(git_commits);

    let context_engine = analyzer::context_inference::ContextInferenceEngine::new();
    let context = context_engine.infer_context();

    let detector = analyzer::session::SessionDetector::new();
    let sessions = detector.detect_sessions(parsed_commands, &parsed_commits);

    if format.to_lowercase() == "markdown" {
        println!("## Engineering Work Summary\n");
        if let Some(project) = &context.project_name {
            println!("### {}\n", project);
        }

        for (i, session) in sessions.iter().enumerate().take(5) {
            println!("#### Work Session {}\n", i + 1);
            if !session.commits.is_empty() {
                println!("{}\n", session.commits[0].message);
            }
            println!("- Duration: {}m", session.duration_minutes);
            println!("- Type: {}", session.work_type.display_name());
            println!("- Commits: {}", session.commits.len());
            println!();
        }
    } else {
        println!("Engineering Work Summary");
        println!("========================\n");
        if let Some(project) = &context.project_name {
            println!("Project: {}", project);
        }
        for (i, session) in sessions.iter().enumerate().take(5) {
            println!("\n[{}] {}", i + 1, session.work_type.display_name());
            if !session.commits.is_empty() {
                println!("    {}", session.commits[0].message);
            }
        }
    }

    Ok(())
}

fn handle_metrics_command() -> anyhow::Result<()> {
    let history_collector = collector::history::HistoryCollector::new();
    let git_collector = collector::git::GitCollector::new();

    let history_commands = history_collector.read_history()?;
    let git_commits = if git_collector.is_git_repo() {
        git_collector.recent_commits(100)?
    } else {
        Vec::new()
    };

    let history_parser = HistoryParser::new();
    let git_parser = parser::history_parser::GitParser::new();

    let parsed_commands = history_parser.parse_commands(history_commands);
    let parsed_commits = git_parser.parse_commits(git_commits);

    let detector = analyzer::session::SessionDetector::new();
    let sessions = detector.detect_sessions(parsed_commands, &parsed_commits);

    println!("Worklie Productivity Metrics");
    println!("=============================\n");

    let total_duration: u64 = sessions.iter().map(|s| s.duration_minutes).sum();
    let deep_duration: u64 = sessions
        .iter()
        .filter(|s| s.duration_minutes >= 120)
        .map(|s| s.duration_minutes)
        .sum();

    println!("TIME ANALYSIS:");
    println!("  Total work time: {}h {}m", total_duration / 60, total_duration % 60);
    println!("  Deep work: {}h {}m ({:.0}%)", 
        deep_duration / 60, 
        deep_duration % 60,
        (deep_duration as f32 / total_duration.max(1) as f32) * 100.0
    );
    println!("  Sessions: {}", sessions.len());
    if !sessions.is_empty() {
        println!("  Average session: {}m", total_duration / sessions.len() as u64);
    }

    println!("\nCOMMITS & CODE:");
    println!("  Total commits: {}", parsed_commits.len());
    println!("  Avg commits/day: {}", (parsed_commits.len() as f32 / sessions.len().max(1) as f32) as usize);

    println!("\nFOCUS PATTERNS:");
    let avg_focus: f32 = sessions.iter().map(|s| s.focus_score).sum::<f32>() / sessions.len().max(1) as f32;
    println!("  Average focus: {:.2} / 1.0", avg_focus);

    println!("\nRECOMMENDATIONS:");
    if avg_focus > 0.7 {
        println!("✓ Your focus is strong - maintain this");
    } else {
        println!("→ Try to reduce context switching");
    }

    Ok(())
}

fn handle_multi_repo_command(workspace_path: Option<String>, json: bool) -> anyhow::Result<()> {
    use std::path::PathBuf;

    let root = if let Some(path) = workspace_path {
        PathBuf::from(path)
    } else {
        std::env::current_dir()?
    };

    // Find repositories in workspace
    let repos = workspace::WorkspaceManager::find_repositories(&root)?;

    if repos.is_empty() {
        if json {
            println!("{{}}");
        } else {
            println!("No git repositories found in workspace");
        }
        return Ok(());
    }

    // Initialize cache manager
    let cache_manager = cache_manager::CacheManager::new()?;

    // Analyze workspace
    let analysis = analyzer::multi_repo_analyzer::MultiRepoAnalyzer::analyze_workspace(repos, &cache_manager)?;

    if json {
        println!("{}", serde_json::to_string_pretty(&analysis)?);
    } else {
        let report = analyzer::multi_repo_analyzer::MultiRepoAnalyzer::generate_report(&analysis);
        println!("{}", report);
    }

    Ok(())
}

fn handle_heatmap_command(days: u32, json: bool) -> anyhow::Result<()> {
    let git_collector = collector::git::GitCollector::new();

    // Get recent commits
    let git_commits_raw = if git_collector.is_git_repo() {
        let num_commits = (days * 20).max(100) as usize; // Estimate ~20 commits per day
        git_collector.recent_commits(num_commits)?
    } else {
        eprintln!("Not in a git repository");
        return Ok(());
    };

    // Parse commits
    let git_parser = parser::history_parser::GitParser::new();
    let git_commits = git_parser.parse_commits(git_commits_raw);

    // Generate heatmap
    let heatmap = analyzer::commit_heatmap::CommitHeatmap::from_commits(&git_commits);

    if json {
        // Return JSON with heatmap data
        let mut slots_data = Vec::new();
        for (slot, count) in &heatmap.slots {
            slots_data.push(serde_json::json!({
                "day": slot.day_of_week,
                "hour": slot.hour,
                "commits": count,
            }));
        }
        let peak_hours = heatmap.peak_hours();
        let peak_days = heatmap.peak_days();

        let json_output = serde_json::json!({
            "total_commits": git_commits.len(),
            "heatmap_slots": slots_data,
            "max_activity": heatmap.max_activity,
            "peak_hours": peak_hours.iter()
                .map(|(h, c)| serde_json::json!({"hour": h, "commits": c}))
                .collect::<Vec<_>>(),
            "peak_days": peak_days.iter()
                .map(|(d, c)| serde_json::json!({"day": d, "commits": c}))
                .collect::<Vec<_>>(),
        });

        println!("{}", serde_json::to_string_pretty(&json_output)?);
    } else {
        // Render visual heatmap
        println!("{}", heatmap.render());

        // Show insights
        let peak_hours = heatmap.peak_hours();
        let peak_days = heatmap.peak_days();

        if !peak_hours.is_empty() {
            println!(
                "{}Peak Hours:{}\n",
                output_formatter::Colors::BOLD,
                output_formatter::Colors::RESET
            );
            for (hour, commits) in peak_hours {
                println!("  {:02}:00 - {:02}:59: {} commits", hour, hour, commits);
            }
        }

        if !peak_days.is_empty() {
            println!(
                "\n{}Peak Days:{}\n",
                output_formatter::Colors::BOLD,
                output_formatter::Colors::RESET
            );
            for (day, commits) in peak_days {
                println!("  {}: {} commits", day, commits);
            }
        }

        println!(
            "\n{}Total commits analyzed: {}{}",
            output_formatter::Colors::CYAN,
            git_commits.len(),
            output_formatter::Colors::RESET
        );
    }

    Ok(())
}

fn handle_config_command(show: bool, reset: bool) -> anyhow::Result<()> {
    if reset {
        debug!("Resetting configuration to defaults");
        config::WorklieConfig::reset_to_defaults()?;
        info!("Configuration has been reset to defaults");
        println!("\nConfiguration has been reset to defaults");
        println!("Edit {}", config::WorklieConfig::config_file().display());
        println!("to customize settings");
        return Ok(());
    }

    // Load and display current configuration
    debug!("Loading configuration from file");
    let cfg = config::WorklieConfig::load()?;
    debug!("Configuration loaded successfully");

    if show || (!show && !reset) {
        cfg.print();
    }

    Ok(())
}

fn handle_diagnose_command() -> anyhow::Result<()> {
    debug!("Running system diagnostics");
    let report = reliability::run_diagnostics();
    info!("Diagnostics completed: healthy={}", report.is_healthy());
    report.print();
    Ok(())
}
