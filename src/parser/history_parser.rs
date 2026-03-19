use serde::{Serialize, Deserialize};

/// Represents a parsed command from shell history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedCommand {
    pub command: String,
    pub timestamp: u64, // Unix timestamp
    pub directory: Option<String>,
}

/// Parses shell history entries into structured data
pub struct HistoryParser;

impl HistoryParser {
    /// Create a new HistoryParser
    pub fn new() -> Self {
        Self
    }

    /// Parse a raw history line into a ParsedCommand
    /// Sanitizes command input to remove control characters and ensure safety
    pub fn parse_command(&self, command: String, timestamp: u64) -> ParsedCommand {
        let sanitized = crate::reliability::SafeOperations::sanitize_command(&command);
        ParsedCommand {
            command: sanitized,
            timestamp,
            directory: None,
        }
    }

    /// Parse multiple history commands
    pub fn parse_commands(&self, commands: Vec<crate::collector::history::TimestampedCommand>) -> Vec<ParsedCommand> {
        commands
            .into_iter()
            .map(|cmd| self.parse_command(cmd.command, cmd.timestamp))
            .collect()
    }
}

/// Represents a parsed git commit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedCommit {
    pub hash: String,
    pub message: String,
    pub author: Option<String>,
    pub timestamp: u64, // Unix timestamp
}

/// Parses git log entries into structured data
pub struct GitParser;

impl GitParser {
    /// Create a new GitParser
    pub fn new() -> Self {
        Self
    }

    /// Parse a raw git log line into a ParsedCommit
    /// Expected format: "hash:timestamp:author:subject"
    /// Sanitizes author and message fields to remove control characters
    pub fn parse_commit(&self, commit_line: String) -> Option<ParsedCommit> {
        let mut parts = commit_line.splitn(4, ':');
        let hash = parts.next()?.to_string();
        let timestamp = parts.next()?.parse::<u64>().ok()?;
        let author = parts.next()
            .map(|s| crate::reliability::SafeOperations::sanitize_command(s));
        let message = parts.next().unwrap_or("")
            .to_string();
        let sanitized_message = crate::reliability::SafeOperations::sanitize_command(&message);

        if hash.is_empty() || sanitized_message.is_empty() {
            return None;
        }

        Some(ParsedCommit {
            hash,
            message: sanitized_message,
            author,
            timestamp,
        })
    }

    /// Parse multiple git log lines
    pub fn parse_commits(&self, commit_lines: Vec<String>) -> Vec<ParsedCommit> {
        commit_lines
            .into_iter()
            .filter_map(|line| self.parse_commit(line))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_command() {
        let parser = HistoryParser::new();
        let parsed = parser.parse_command("git commit -m \"fix bug\"".to_string(), 1234567890);
        assert_eq!(parsed.command, "git commit -m \"fix bug\"");
        assert_eq!(parsed.timestamp, 1234567890);
    }

    #[test]
    fn test_parse_command_with_control_characters() {
        let parser = HistoryParser::new();
        let dirty_command = "git status\x00bad\x08worse".to_string();
        let parsed = parser.parse_command(dirty_command, 1234567890);
        // Control characters should be filtered out
        assert!(!parsed.command.contains('\x00'));
        assert!(!parsed.command.contains('\x08'));
    }

    #[test]
    fn test_parse_commit() {
        let parser = GitParser::new();
        let parsed = parser.parse_commit("abc123:1234567890:Engineer Name:fix authentication bug".to_string());
        assert!(parsed.is_some());
        let commit = parsed.unwrap();
        assert_eq!(commit.hash, "abc123");
        assert_eq!(commit.timestamp, 1234567890);
        assert_eq!(commit.author, Some("Engineer Name".to_string()));
        assert_eq!(commit.message, "fix authentication bug");
    }

    #[test]
    fn test_parse_commit_with_control_chars() {
        let parser = GitParser::new();
        let dirty_commit = "abc123:1234567890:Engineer\x00Name:fix bug\x1fmessage".to_string();
        let parsed = parser.parse_commit(dirty_commit);
        assert!(parsed.is_some());
        let commit = parsed.unwrap();
        // Author and message should be sanitized
        assert!(!commit.message.contains('\x1f'));
        if let Some(author) = &commit.author {
            assert!(!author.contains('\x00'));
        }
    }

    #[test]
    fn test_parse_commit_invalid() {
        let parser = GitParser::new();
        let parsed = parser.parse_commit("invalid".to_string());
        assert!(parsed.is_none());
    }
}
