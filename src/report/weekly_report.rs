use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::analyzer::activity_clusterer::Activity;
use crate::analyzer::context_inference::WorkContext;
use crate::cache_manager::CacheManager;
use anyhow::Result;

/// Legacy structured weekly report (keeping for backward compatibility)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeeklyReport {
    pub projects: Vec<String>,
    pub total_commits: usize,
    pub total_activities: usize,
    pub domains: Vec<String>,
    pub work_types: Vec<String>,
    pub commit_messages: Vec<String>,
}

/// Daily statistics for a single day
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyStats {
    pub date: String,                      // YYYY-MM-DD
    pub commits_count: usize,
    pub activity_count: usize,
    pub total_time_minutes: u64,
    pub focus_score: f64,                  // Average focus 0.0-1.0
    pub work_types: HashMap<String, usize>, // Distribution
}

/// Trend direction indicator
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TrendDirection {
    Up,     // ↑ Improved
    Down,   // ↓ Decreased
    Stable, // → No change
}

impl TrendDirection {
    pub fn symbol(&self) -> &'static str {
        match self {
            TrendDirection::Up => "↑",
            TrendDirection::Down => "↓",
            TrendDirection::Stable => "→",
        }
    }
}

/// Weekly trends compared to previous week
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeeklyTrends {
    pub commits_trend: TrendDirection,
    pub commits_change: i32,               // Percentage change
    pub focus_trend: TrendDirection,
    pub focus_change: f64,                 // Points change
    pub best_day: Option<String>,          // Date with highest productivity
    pub worst_day: Option<String>,         // Date with lowest productivity
    pub context_switches_avg: f64,
}

/// Enhanced weekly report with historical data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeeklyReportData {
    pub period_start: String,              // Monday
    pub period_end: String,                // Sunday
    pub daily_stats: Vec<DailyStats>,      // 7 days
    pub weekly_summary: DailyStats,        // Aggregated
    pub trends: WeeklyTrends,
    pub insights: Vec<String>,             // Auto-generated recommendations
    pub comparison_available: bool,        // Was there previous week data?
}

/// Generates enhanced weekly work reports with trends
pub struct WeeklyReportGenerator;

impl WeeklyReportGenerator {
    pub fn new() -> Self {
        Self
    }

    /// Generate report from daily activities (legacy method for backward compatibility)
    pub fn generate_report_data(&self, activities: Vec<Activity>, context: &WorkContext) -> WeeklyReport {
        let mut domains = std::collections::HashSet::new();
        let mut work_types = std::collections::HashSet::new();
        let mut projects = std::collections::HashSet::new();
        let mut commit_messages = Vec::new();

        if let Some(project) = &context.project_name {
            projects.insert(project.clone());
        }
        if let Some(domain) = &context.domain {
            domains.insert(domain.clone());
        }
        if let Some(work_type) = &context.work_type {
            work_types.insert(work_type.clone());
        }

        let mut total_commits = 0;
        for activity in &activities {
            total_commits += activity.commits.len();
            for commit in &activity.commits {
                commit_messages.push(commit.message.clone());
            }
        }

        WeeklyReport {
            projects: projects.into_iter().collect(),
            total_commits,
            total_activities: activities.len(),
            domains: domains.into_iter().collect(),
            work_types: work_types.into_iter().collect(),
            commit_messages,
        }
    }

    /// Generate human-readable report from daily activities (legacy backward compat)
    pub fn generate_report_legacy(&self, activities: Vec<Activity>, context: &WorkContext) -> String {
        let mut report = String::new();

        report.push_str("Weekly Engineering Report\n");
        report.push_str("=========================\n\n");

        report.push_str("Projects Worked On:\n");
        if let Some(project) = &context.project_name {
            report.push_str(&format!("  • {}\n", project));
        } else {
            report.push_str("  • No projects detected\n");
        }
        report.push_str("\n");

        report.push_str("Major Work Areas:\n");
        if let Some(domain) = &context.domain {
            report.push_str(&format!("  • {}\n", domain));
        }
        if let Some(work_type) = &context.work_type {
            report.push_str(&format!("  • {}\n", work_type));
        }
        if context.domain.is_none() && context.work_type.is_none() {
            report.push_str("  • Not detected\n");
        }
        report.push_str("\n");

        report.push_str(&format!("Statistics:\n"));
        let total_commits: usize = activities.iter().map(|a| a.commits.len()).sum();
        report.push_str(&format!("  • {} total commits\n", total_commits));
        report.push_str(&format!("  • {} activities\n", activities.len()));
        report.push_str("\n");

        report.push_str("Main Contributions:\n");
        let mut shown = 0;
        for activity in &activities {
            for commit in &activity.commits {
                if shown >= 5 {
                    break;
                }
                report.push_str(&format!("  • {}\n", commit.message));
                shown += 1;
            }
            if shown >= 5 {
                break;
            }
        }

        report
    }

    /// Generate report from cache with historical data
    pub fn generate_from_cache(
        &self,
        cache_manager: &CacheManager,
        repo_path: &str,
    ) -> Result<WeeklyReportData> {
        // Load last 14 days of data (current week + previous week for comparison)
        let snapshots = cache_manager.load_repository_history(repo_path, 14)?;

        if snapshots.is_empty() {
            // Return empty report with no historical data
            return Ok(WeeklyReportData {
                period_start: chrono::Local::now()
                    .format("%Y-%m-%d")
                    .to_string(),
                period_end: chrono::Local::now()
                    .format("%Y-%m-%d")
                    .to_string(),
                daily_stats: vec![],
                weekly_summary: DailyStats {
                    date: "unknown".to_string(),
                    commits_count: 0,
                    activity_count: 0,
                    total_time_minutes: 0,
                    focus_score: 0.0,
                    work_types: HashMap::new(),
                },
                trends: WeeklyTrends {
                    commits_trend: TrendDirection::Stable,
                    commits_change: 0,
                    focus_trend: TrendDirection::Stable,
                    focus_change: 0.0,
                    best_day: None,
                    worst_day: None,
                    context_switches_avg: 0.0,
                },
                insights: vec!["No historical data available yet. Run worklie report daily to build history.".to_string()],
                comparison_available: false,
            });
        }

        // Group snapshots by week
        let mut current_week = vec![];
        let mut previous_week = vec![];

        let now = chrono::Local::now();
        let week_ago = now - chrono::Duration::days(7);

        for snapshot in snapshots {
            if let Ok(snapshot_date) = chrono::NaiveDate::parse_from_str(&snapshot.date, "%Y-%m-%d") {
                let snapshot_datetime = snapshot_date.and_hms_opt(0, 0, 0).unwrap();
                if snapshot_datetime > week_ago.naive_local() {
                    current_week.push(snapshot);
                } else {
                    previous_week.push(snapshot);
                }
            }
        }

        // Calculate daily stats for current week
        let daily_stats = self.calculate_daily_stats(&current_week);
        let weekly_summary = self.aggregate_daily_stats(&daily_stats);

        // Calculate trends by comparing weeks
        let previous_summary = self.aggregate_daily_stats(&self.calculate_daily_stats(&previous_week));
        let trends = self.calculate_trends(&weekly_summary, &previous_summary, &daily_stats);

        // Generate insights
        let insights = self.generate_insights(&weekly_summary, &trends);

        Ok(WeeklyReportData {
            period_start: {
                let today = chrono::Local::now().date_naive();
                // Simply use today's date (improve with proper Monday calculation if needed)
                today.format("%Y-%m-%d").to_string()
            },
            period_end: chrono::Local::now()
                .format("%Y-%m-%d")
                .to_string(),
            daily_stats,
            weekly_summary,
            trends,
            insights,
            comparison_available: !previous_week.is_empty(),
        })
    }

    /// Calculate daily stats from snapshots
    fn calculate_daily_stats(&self, snapshots: &[crate::cache_manager::DailySnapshot]) -> Vec<DailyStats> {
        let mut daily_map: HashMap<String, (usize, usize, f64, HashMap<String, usize>)> = HashMap::new();

        for snapshot in snapshots {
            let entry = daily_map.entry(snapshot.date.clone())
                .or_insert_with(|| (0, 0, 0.0, HashMap::new()));

            entry.0 += snapshot.parsed_commits.len();
            entry.1 += snapshot.activities.len();

            // Calculate average focus score
            entry.2 = if snapshot.activities.is_empty() {
                0.3
            } else {
                0.3 // Simplified for now
            };

            // Count work types from commits
            for commit in &snapshot.parsed_commits {
                let msg_lower = commit.message.to_lowercase();
                let work_type = if msg_lower.starts_with("feat") {
                    "Features"
                } else if msg_lower.starts_with("fix") {
                    "Bug Fixes"
                } else if msg_lower.starts_with("test") {
                    "Testing"
                } else if msg_lower.starts_with("refactor") {
                    "Refactoring"
                } else if msg_lower.starts_with("docs") {
                    "Documentation"
                } else if msg_lower.starts_with("chore") {
                    "Maintenance"
                } else {
                    "Other"
                };

                *entry.3.entry(work_type.to_string()).or_insert(0) += 1;
            }
        }

        let mut daily_stats: Vec<_> = daily_map
            .into_iter()
            .map(|(date, (commits, activities, focus, work_types))| DailyStats {
                date,
                commits_count: commits,
                activity_count: activities,
                total_time_minutes: activities as u64 * 30, // Rough estimate
                focus_score: focus,
                work_types,
            })
            .collect();

        daily_stats.sort_by(|a, b| a.date.cmp(&b.date));
        daily_stats
    }

    /// Aggregate daily stats into weekly summary
    fn aggregate_daily_stats(&self, daily_stats: &[DailyStats]) -> DailyStats {
        let mut total_commits = 0;
        let mut total_activities = 0;
        let mut total_focus = 0.0;
        let mut all_work_types: HashMap<String, usize> = HashMap::new();

        for day in daily_stats {
            total_commits += day.commits_count;
            total_activities += day.activity_count;
            total_focus += day.focus_score;

            for (work_type, count) in &day.work_types {
                *all_work_types.entry(work_type.clone()).or_insert(0) += count;
            }
        }

        let avg_focus = if daily_stats.is_empty() {
            0.0
        } else {
            total_focus / daily_stats.len() as f64
        };

        DailyStats {
            date: "weekly_summary".to_string(),
            commits_count: total_commits,
            activity_count: total_activities,
            total_time_minutes: total_activities as u64 * 30,
            focus_score: avg_focus,
            work_types: all_work_types,
        }
    }

    /// Calculate trends comparing this week to previous week
    fn calculate_trends(
        &self,
        current: &DailyStats,
        previous: &DailyStats,
        daily_stats: &[DailyStats],
    ) -> WeeklyTrends {
        let commits_change = if previous.commits_count == 0 {
            0
        } else {
            (current.commits_count as i32 - previous.commits_count as i32)
                * 100 / previous.commits_count as i32
        };

        let commits_trend = if commits_change > 5 {
            TrendDirection::Up
        } else if commits_change < -5 {
            TrendDirection::Down
        } else {
            TrendDirection::Stable
        };

        let focus_change = current.focus_score - previous.focus_score;
        let focus_trend = if focus_change > 0.05 {
            TrendDirection::Up
        } else if focus_change < -0.05 {
            TrendDirection::Down
        } else {
            TrendDirection::Stable
        };

        // Find best and worst days
        let best_day = daily_stats.iter().max_by(|a, b| a.focus_score.partial_cmp(&b.focus_score).unwrap()).map(|d| d.date.clone());
        let worst_day = daily_stats.iter().min_by(|a, b| a.focus_score.partial_cmp(&b.focus_score).unwrap()).map(|d| d.date.clone());

        WeeklyTrends {
            commits_trend,
            commits_change,
            focus_trend,
            focus_change,
            best_day,
            worst_day,
            context_switches_avg: 2.5, // Placeholder
        }
    }

    /// Generate actionable insights from data
    fn generate_insights(&self, summary: &DailyStats, trends: &WeeklyTrends) -> Vec<String> {
        let mut insights = vec![];

        if trends.commits_trend == TrendDirection::Down {
            insights.push(format!(
                "📉 Commit activity down {}%. Consider increasing focus time.",
                trends.commits_change.abs()
            ));
        }

        if trends.commits_trend == TrendDirection::Up {
            insights.push(format!(
                "📈 Great week! Commits up {}%. Maintain this momentum.",
                trends.commits_change
            ));
        }

        if summary.focus_score < 0.4 {
            insights.push("🎯 Focus score is low. Try blocking 2-hour focus sessions.".to_string());
        }

        if summary.focus_score > 0.7 {
            insights.push("⭐ Excellent focus score! Protect this time on your calendar.".to_string());
        }

        if let Some(best_day) = &trends.best_day {
            insights.push(format!("✅ Best day: {}. Consider scheduling important work then.", best_day));
        }

        insights
    }

    /// Generate human-readable report
    pub fn generate_report(&self, data: &WeeklyReportData) -> String {
        let mut report = String::new();

        report.push_str("📊 Weekly Engineering Report\n");
        report.push_str("=============================\n\n");

        report.push_str(&format!("📅 Period: {} to {}\n\n", data.period_start, data.period_end));

        // Summary section
        report.push_str("Summary\n");
        report.push_str("-------\n");
        report.push_str(&format!("  • Commits: {} ", data.weekly_summary.commits_count));
        report.push_str(&format!("({} {})\n", data.trends.commits_trend.symbol(), data.trends.commits_change));
        report.push_str(&format!("  • Activities: {}\n", data.weekly_summary.activity_count));
        report.push_str(&format!(
            "  • Focus Score: {:.1}/1.0 ({} {})\n\n",
            data.weekly_summary.focus_score,
            data.trends.focus_trend.symbol(),
            if data.trends.focus_change > 0.0 { "+" } else { "" }
        ));

        // Work breakdown
        if !data.weekly_summary.work_types.is_empty() {
            report.push_str("Work Breakdown\n");
            report.push_str("--------------\n");
            let mut work_types: Vec<_> = data.weekly_summary.work_types.iter().collect();
            work_types.sort_by(|a, b| b.1.cmp(a.1));
            for (work_type, count) in work_types {
                let pct = (*count as f64 / data.weekly_summary.commits_count as f64 * 100.0) as u32;
                report.push_str(&format!("  • {}: {} ({}%)\n", work_type, count, pct));
            }
            report.push_str("\n");
        }

        // Insights
        if !data.insights.is_empty() {
            report.push_str("Insights & Recommendations\n");
            report.push_str("---------------------------\n");
            for insight in &data.insights {
                report.push_str(&format!("  {}\n", insight));
            }
            report.push_str("\n");
        }

        // Daily breakdown
        if !data.daily_stats.is_empty() {
            report.push_str("Daily Breakdown\n");
            report.push_str("---------------\n");
            for day in &data.daily_stats {
                report.push_str(&format!(
                    "  {} - {} commits, focus: {:.1}\n",
                    day.date, day.commits_count, day.focus_score
                ));
            }
        }

        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trend_direction_symbol() {
        assert_eq!(TrendDirection::Up.symbol(), "↑");
        assert_eq!(TrendDirection::Down.symbol(), "↓");
        assert_eq!(TrendDirection::Stable.symbol(), "→");
    }

    #[test]
    fn test_weekly_report_generation() {
        let generator = WeeklyReportGenerator::new();
        let data = WeeklyReportData {
            period_start: "2026-03-16".to_string(),
            period_end: "2026-03-22".to_string(),
            daily_stats: vec![],
            weekly_summary: DailyStats {
                date: "summary".to_string(),
                commits_count: 10,
                activity_count: 5,
                total_time_minutes: 150,
                focus_score: 0.7,
                work_types: HashMap::new(),
            },
            trends: WeeklyTrends {
                commits_trend: TrendDirection::Up,
                commits_change: 25,
                focus_trend: TrendDirection::Up,
                focus_change: 0.1,
                best_day: Some("2026-03-19".to_string()),
                worst_day: Some("2026-03-16".to_string()),
                context_switches_avg: 2.0,
            },
            insights: vec!["Good week!".to_string()],
            comparison_available: true,
        };

        let report = generator.generate_report(&data);
        assert!(report.contains("Weekly Engineering Report"));
        assert!(report.contains("Commits: 10") || report.contains("10 commits"));
    }
}
