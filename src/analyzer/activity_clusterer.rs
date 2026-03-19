use serde::{Serialize, Deserialize};
use crate::parser::history_parser::{ParsedCommand, ParsedCommit};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    /// Regex to find filenames with common development extensions
    static ref FILENAME_PATTERN: Regex = Regex::new(
        r"\b([a-zA-Z_][a-zA-Z0-9_]*\.(js|ts|py|rs|go|java|jsx|tsx))\b"
    ).unwrap();
}

/// Represents a clustered activity/task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity {
    pub id: String,
    pub title: String,
    pub description: String,
    pub start_time: u64,
    pub end_time: u64,
    pub commands: Vec<ParsedCommand>,
    pub commits: Vec<ParsedCommit>,
    pub files: Vec<String>,
    pub directory: Option<String>,
}

/// Clusters raw signals into meaningful activities
pub struct ActivityClusterer {
    /// Time window for clustering activities (in seconds)
    time_window_seconds: u64,
}

impl ActivityClusterer {
    /// Create a new ActivityClusterer with default time window (30 minutes)
    pub fn new() -> Self {
        Self {
            time_window_seconds: 30 * 60, // 30 minutes
        }
    }

    /// Create a new ActivityClusterer with custom time window (in minutes)
    /// Reserved for Phase 3b configurable clustering parameters
    #[allow(dead_code)]
    pub fn with_time_window_minutes(minutes: u64) -> Self {
        Self {
            time_window_seconds: minutes * 60,
        }
    }

    /// Cluster commands and commits into activities based on time proximity
    pub fn cluster_activities(
        &self,
        mut commands: Vec<ParsedCommand>,
        commits: Vec<ParsedCommit>,
    ) -> Vec<Activity> {
        if commands.is_empty() && commits.is_empty() {
            return Vec::new();
        }

        // Sort by timestamp
        commands.sort_by_key(|c| c.timestamp);

        // Create clusters based on time windows (avoid cloning by consuming iterator)
        let mut clusters: Vec<Vec<ParsedCommand>> = Vec::new();
        let mut current_cluster: Vec<ParsedCommand> = Vec::new();
        let mut last_timestamp: u64 = 0;

        for cmd in commands.into_iter() {
            let cmd_timestamp = cmd.timestamp;

            if !current_cluster.is_empty()
                && cmd_timestamp.saturating_sub(last_timestamp) > self.time_window_seconds
            {
                // Start a new cluster (move, don't clone)
                clusters.push(current_cluster);
                current_cluster = vec![cmd];
            } else {
                current_cluster.push(cmd);
            }
            last_timestamp = cmd_timestamp;
        }

        if !current_cluster.is_empty() {
            clusters.push(current_cluster);
        }

        // Convert clusters to activities
        let mut activities = Vec::new();
        for (i, cmd_group) in clusters.into_iter().enumerate() {
            if cmd_group.is_empty() {
                continue;
            }

            // Find commits within this time window
            let start_time = cmd_group.first().map(|c| c.timestamp).unwrap_or(0);
            let end_time = cmd_group.last().map(|c| c.timestamp).unwrap_or(0);

            let window_commits: Vec<ParsedCommit> = commits
                .iter()
                .filter(|c| c.timestamp >= start_time && c.timestamp <= end_time + self.time_window_seconds)
                .cloned()
                .collect();

            let files = Self::extract_files_from_activity(&cmd_group, &window_commits);
            let title = self.generate_activity_title(&cmd_group, &window_commits);

            activities.push(Activity {
                id: format!("activity-{}", i),
                title,
                description: format!(
                    "Activity with {} commands and {} commits",
                    cmd_group.len(),
                    window_commits.len()
                ),
                start_time,
                end_time,
                commands: cmd_group,
                commits: window_commits,
                files,
                directory: None,
            });
        }

        activities
    }

    /// Extract files mentioned in commands and commits using regex
    fn extract_files_from_activity(_commands: &[ParsedCommand], commits: &[ParsedCommit]) -> Vec<String> {
        let mut files = std::collections::HashSet::new();

        // Extract files from commit messages using compiled regex (single pass per commit)
        for commit in commits {
            for capture in FILENAME_PATTERN.captures_iter(&commit.message) {
                if let Some(filename) = capture.get(1) {
                    files.insert(filename.as_str().to_string());
                }
            }
        }

        files.into_iter().collect()
    }

    /// Generate a title for an activity based on its contents
    fn generate_activity_title(&self, commands: &[ParsedCommand], commits: &[ParsedCommit]) -> String {
        // Look for work type hints in commits first
        for commit in commits {
            let msg_lower = commit.message.to_lowercase();
            if msg_lower.starts_with("fix") || msg_lower.starts_with("bug") {
                return format!("Bug Fix: {}", &commit.message[..std::cmp::min(50, commit.message.len())]);
            }
            if msg_lower.starts_with("feat") || msg_lower.starts_with("feature") {
                return format!("Feature: {}", &commit.message[..std::cmp::min(50, commit.message.len())]);
            }
        }

        // Analyze commands
        let mut has_git = false;
        let mut has_edit = false;
        let mut git_cmd = String::new();

        for cmd in commands {
            let lower_cmd = cmd.command.to_lowercase();
            if lower_cmd.starts_with("git ") {
                has_git = true;
                git_cmd = cmd.command.clone();
            }
            if lower_cmd.contains("vim ") || lower_cmd.contains("nvim ") || lower_cmd.contains("code ") {
                has_edit = true;
            }
        }

        if has_git && has_edit {
            return "Code Changes".to_string();
        } else if has_git {
            return format!("Git Operations: {}", git_cmd);
        } else if has_edit {
            return "File Editing".to_string();
        }

        "General Activity".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cluster_activities() {
        let clusterer = ActivityClusterer::new();
        let commands = vec![
            ParsedCommand {
                command: "git status".to_string(),
                timestamp: 1000,
                directory: None,
            },
            ParsedCommand {
                command: "vim file.rs".to_string(),
                timestamp: 1010,
                directory: None,
            },
        ];

        let commits = vec![ParsedCommit {
            hash: "abc123".to_string(),
            message: "fix: bug in file.rs".to_string(),
            author: Some("Test Author".to_string()),
            timestamp: 1005,
        }];

        let activities = clusterer.cluster_activities(commands, commits);
        assert!(!activities.is_empty());
        assert_eq!(activities[0].commits.len(), 1);
    }
}
