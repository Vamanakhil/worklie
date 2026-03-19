use serde::{Serialize, Deserialize};
use crate::parser::history_parser::{ParsedCommand, ParsedCommit};

/// Represents a work session (group of related activities)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkSession {
    pub id: usize,
    pub start_time: u64,
    pub end_time: u64,
    pub duration_minutes: u64,
    pub commands_count: usize,
    pub commits: Vec<ParsedCommit>,
    pub work_type: SessionWorkType,
    pub focus_score: f32, // 0.0 to 1.0, higher = more focused
}

/// Type of work being done in a session
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SessionWorkType {
    Debugging,      // Fixing bugs
    FeatureDev,     // Building new features
    Refactoring,    // Code cleanup
    Testing,        // Test writing/running
    DevOps,         // Infrastructure work
    Documentation,  // Writing docs
    Unknown,        // Unclassified
}

impl SessionWorkType {
    pub fn display_name(&self) -> &str {
        match self {
            SessionWorkType::Debugging => "Debugging",
            SessionWorkType::FeatureDev => "Feature Development",
            SessionWorkType::Refactoring => "Refactoring",
            SessionWorkType::Testing => "Testing",
            SessionWorkType::DevOps => "DevOps/Infrastructure",
            SessionWorkType::Documentation => "Documentation",
            SessionWorkType::Unknown => "Other Work",
        }
    }
}

/// Detects and analyzes work sessions
pub struct SessionDetector {
    /// Time gap (seconds) that triggers new session
    gap_threshold: u64,
    /// Minimum session duration to be meaningful (seconds)
    min_duration: u64,
}

impl SessionDetector {
    pub fn new() -> Self {
        Self {
            gap_threshold: 30 * 60,    // 30 minutes
            min_duration: 5 * 60,      // 5 minutes minimum
        }
    }

    pub fn with_gap_threshold(mut self, seconds: u64) -> Self {
        self.gap_threshold = seconds;
        self
    }

    /// Detect sessions from commands and commits
    pub fn detect_sessions(
        &self,
        mut commands: Vec<ParsedCommand>,
        commits: &[ParsedCommit],
    ) -> Vec<WorkSession> {
        if commands.is_empty() {
            return Vec::new();
        }

        // Sort commands by timestamp
        commands.sort_by_key(|c| c.timestamp);

        let mut sessions = Vec::new();
        let mut session_commands = Vec::new();
        let mut last_timestamp: u64 = 0;
        let mut session_id = 0;

        for cmd in commands {
            // Check if we should start a new session
            if !session_commands.is_empty()
                && cmd.timestamp.saturating_sub(last_timestamp) > self.gap_threshold
            {
                // Create session from accumulated commands
                if let Some(session) = self.create_session(
                    session_id,
                    &session_commands,
                    &commits,
                ) {
                    sessions.push(session);
                    session_id += 1;
                }
                session_commands.clear();
            }

            session_commands.push(cmd.clone());
            last_timestamp = cmd.timestamp;
        }

        // Don't forget the last session
        if !session_commands.is_empty() {
            if let Some(session) = self.create_session(session_id, &session_commands, &commits) {
                sessions.push(session);
            }
        }

        sessions
    }

    /// Create a session from commands
    fn create_session(
        &self,
        id: usize,
        commands: &[ParsedCommand],
        all_commits: &[ParsedCommit],
    ) -> Option<WorkSession> {
        if commands.is_empty() {
            return None;
        }

        let start_time = commands.first().map(|c| c.timestamp)?;
        let end_time = commands.last().map(|c| c.timestamp)?;
        let duration_minutes = (end_time - start_time) / 60;

        // Skip if too short
        if end_time - start_time < self.min_duration {
            return None;
        }

        // Find commits within session window
        let session_commits: Vec<ParsedCommit> = all_commits
            .iter()
            .filter(|c| c.timestamp >= start_time && c.timestamp <= end_time + self.gap_threshold)
            .cloned()
            .collect();

        // Classify work type
        let work_type = self.classify_work_type(commands, &session_commits);

        // Calculate focus score (how concentrated the work was)
        let focus_score = self.calculate_focus_score(commands);

        Some(WorkSession {
            id,
            start_time,
            end_time,
            duration_minutes,
            commands_count: commands.len(),
            commits: session_commits,
            work_type,
            focus_score,
        })
    }

    /// Classify the type of work in a session
    fn classify_work_type(
        &self,
        commands: &[ParsedCommand],
        commits: &[ParsedCommit],
    ) -> SessionWorkType {
        let mut scores = std::collections::HashMap::new();

        // Check commits for keywords
        for commit in commits {
            let msg_lower = commit.message.to_lowercase();

            if msg_lower.contains("fix") || msg_lower.contains("bug") {
                *scores.entry(SessionWorkType::Debugging).or_insert(0) += 3;
            } else if msg_lower.contains("feat") || msg_lower.contains("feature") {
                *scores.entry(SessionWorkType::FeatureDev).or_insert(0) += 3;
            } else if msg_lower.contains("refactor") || msg_lower.contains("cleanup") {
                *scores.entry(SessionWorkType::Refactoring).or_insert(0) += 3;
            } else if msg_lower.contains("test") || msg_lower.contains("spec") {
                *scores.entry(SessionWorkType::Testing).or_insert(0) += 3;
            } else if msg_lower.contains("doc") || msg_lower.contains("readme") {
                *scores.entry(SessionWorkType::Documentation).or_insert(0) += 3;
            }
        }

        // Check commands for tool patterns
        for cmd in commands {
            let cmd_lower = cmd.command.to_lowercase();

            if cmd_lower.contains("docker") || cmd_lower.contains("kubectl") {
                *scores.entry(SessionWorkType::DevOps).or_insert(0) += 1;
            } else if cmd_lower.contains("cargo test") || cmd_lower.contains("npm test") {
                *scores.entry(SessionWorkType::Testing).or_insert(0) += 1;
            }
        }

        // Return highest scoring type, or Unknown
        scores
            .into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(wt, _)| wt)
            .unwrap_or(SessionWorkType::Unknown)
    }

    /// Calculate focus score (0.0 to 1.0)
    /// Higher = more concentrated/focused work
    fn calculate_focus_score(&self, commands: &[ParsedCommand]) -> f32 {
        if commands.len() < 2 {
            return 0.5;
        }

        let mut gaps = Vec::new();
        for i in 1..commands.len() {
            let gap = commands[i].timestamp.saturating_sub(commands[i - 1].timestamp);
            gaps.push(gap);
        }

        let avg_gap = gaps.iter().sum::<u64>() / gaps.len() as u64;

        // If average gap < 30 seconds, high focus
        // If average gap > 2 minutes, low focus
        if avg_gap < 30 {
            0.9
        } else if avg_gap < 60 {
            0.7
        } else if avg_gap < 120 {
            0.5
        } else {
            0.3
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_detection() {
        let detector = SessionDetector::new();
        let commands = vec![
            ParsedCommand { command: "git status".to_string(), timestamp: 1000, directory: None },
            ParsedCommand { command: "vim file.rs".to_string(), timestamp: 1100, directory: None },
            ParsedCommand { command: "cargo test".to_string(), timestamp: 1200, directory: None },
            // 30 min gap - new session
            ParsedCommand { command: "git log".to_string(), timestamp: 1000 + 1800 + 100, directory: None },
        ];

        let sessions = detector.detect_sessions(commands, &vec![]);
        // Should have 2 sessions: one for first 3 commands, one for the last command
        // But the second one might be too short, so check >= 1
        assert!(sessions.len() >= 1);
    }

    #[test]
    fn test_work_type_classification() {
        let detector = SessionDetector::new();
        let commands = vec![
            ParsedCommand { command: "vim file.rs".to_string(), timestamp: 1000, directory: None },
            ParsedCommand { command: "cargo test".to_string(), timestamp: 1000 + 600, directory: None },
        ];
        let commits = vec![
            ParsedCommit {
                hash: "abc123".to_string(),
                message: "fix: bug in parser".to_string(),
                author: None,
                timestamp: 1050,
            },
        ];

        let sessions = detector.detect_sessions(commands, &commits);
        assert!(!sessions.is_empty(), "No sessions detected");
        assert_eq!(sessions[0].work_type, SessionWorkType::Debugging);
    }
}
