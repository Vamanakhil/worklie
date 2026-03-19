use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use crate::cache_manager::CacheManager;
use anyhow::Result;

/// Metrics for a single repository
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoMetrics {
    pub name: String,
    pub path: String,
    pub total_commits: usize,
    pub total_activities: usize,
    pub average_focus: f64,
    pub work_types: HashMap<String, usize>,
}

/// Analysis across a whole workspace
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceAnalysis {
    pub total_repos: usize,
    pub repos_active: usize,
    pub total_commits: usize,
    pub total_activities: usize,
    pub average_focus: f64,
    pub activity_by_repo: HashMap<String, RepoMetrics>,
    pub context_switches_between_repos: usize,
}

/// Analyzes work across multiple repositories
pub struct MultiRepoAnalyzer;

impl MultiRepoAnalyzer {
    /// Analyze work across multiple repositories using cache
    pub fn analyze_workspace(
        repos: Vec<PathBuf>,
        cache_manager: &CacheManager,
    ) -> Result<WorkspaceAnalysis> {
        let mut activity_by_repo: HashMap<String, RepoMetrics> = HashMap::new();
        let mut total_commits = 0;
        let mut total_activities = 0;
        let mut total_focus = 0.0;
        let mut repos_active = 0;

        for repo_path in &repos {
            let repo_name = repo_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_string();

            let repo_path_str = repo_path.to_string_lossy().to_string();

            // Load historical data for this repo
            match cache_manager.load_repository_history(&repo_path_str, 7) {
                Ok(snapshots) if !snapshots.is_empty() => {
                    repos_active += 1;

                    let mut repo_commits = 0;
                    let mut repo_activities = 0;
                    let mut repo_focus = 0.0;
                    let mut work_types: HashMap<String, usize> = HashMap::new();

                    for snapshot in snapshots {
                        repo_commits += snapshot.parsed_commits.len();
                        repo_activities += snapshot.activities.len();
                        repo_focus += 0.3; // Simplified

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

                            *work_types.entry(work_type.to_string()).or_insert(0) += 1;
                        }
                    }

                    let avg_focus = if repo_activities > 0 {
                        repo_focus / repo_activities.max(1) as f64
                    } else {
                        0.0
                    };

                    total_commits += repo_commits;
                    total_activities += repo_activities;
                    total_focus += avg_focus;

                    activity_by_repo.insert(
                        repo_name.clone(),
                        RepoMetrics {
                            name: repo_name,
                            path: repo_path_str,
                            total_commits: repo_commits,
                            total_activities: repo_activities,
                            average_focus: avg_focus,
                            work_types,
                        },
                    );
                }
                _ => {
                    // No data for this repo yet
                }
            }
        }

        let avg_focus = if repos_active > 0 {
            total_focus / repos_active as f64
        } else {
            0.0
        };

        Ok(WorkspaceAnalysis {
            total_repos: repos.len(),
            repos_active,
            total_commits,
            total_activities,
            average_focus: avg_focus,
            activity_by_repo,
            context_switches_between_repos: (repos_active as f64 * 0.5) as usize,
        })
    }

    /// Generate human-readable workspace report
    pub fn generate_report(analysis: &WorkspaceAnalysis) -> String {
        let mut report = String::new();

        report.push_str("🏢 Workspace Analysis\n");
        report.push_str("====================\n\n");

        report.push_str(&format!("Repository Count: {}/{} active\n\n", analysis.repos_active, analysis.total_repos));

        report.push_str("Summary\n");
        report.push_str("-------\n");
        report.push_str(&format!("  • Total Commits: {}\n", analysis.total_commits));
        report.push_str(&format!("  • Total Activities: {}\n", analysis.total_activities));
        report.push_str(&format!("  • Average Focus: {:.1}/1.0\n", analysis.average_focus));
        report.push_str(&format!(
            "  • Context Switches: {}\n\n",
            analysis.context_switches_between_repos
        ));

        report.push_str("Repository Breakdown\n");
        report.push_str("-------------------\n");

        let mut sorted_repos: Vec<_> = analysis.activity_by_repo.values().collect();
        sorted_repos.sort_by(|a, b| b.total_commits.cmp(&a.total_commits));

        for repo in sorted_repos {
            report.push_str(&format!(
                "  {} - {} commits, {} activities, focus: {:.1}\n",
                repo.name, repo.total_commits, repo.total_activities, repo.average_focus
            ));

            if !repo.work_types.is_empty() {
                let mut types: Vec<_> = repo.work_types.iter().collect();
                types.sort_by(|a, b| b.1.cmp(a.1));
                for (work_type, count) in types.iter().take(3) {
                    report.push_str(&format!("      • {}: {}\n", work_type, count));
                }
            }
        }

        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multi_repo_analyzer_creation() {
        let _analyzer = MultiRepoAnalyzer;
    }

    #[test]
    fn test_workspace_analysis_generation() {
        let mut activity_by_repo = HashMap::new();
        activity_by_repo.insert(
            "repo1".to_string(),
            RepoMetrics {
                name: "repo1".to_string(),
                path: "/path/to/repo1".to_string(),
                total_commits: 10,
                total_activities: 5,
                average_focus: 0.7,
                work_types: HashMap::new(),
            },
        );

        let analysis = WorkspaceAnalysis {
            total_repos: 1,
            repos_active: 1,
            total_commits: 10,
            total_activities: 5,
            average_focus: 0.7,
            activity_by_repo,
            context_switches_between_repos: 0,
        };

        let report = MultiRepoAnalyzer::generate_report(&analysis);
        assert!(report.contains("Workspace Analysis"));
        assert!(report.contains("repo1"));
    }
}
