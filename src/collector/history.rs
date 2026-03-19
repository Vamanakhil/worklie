use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use crate::security::SafeValidator;

/// Represents a timestamped command
#[derive(Debug, Clone)]
pub struct TimestampedCommand {
    pub command: String,
    pub timestamp: u64, // Unix timestamp
}

/// Collects shell history from bash or zsh history files
pub struct HistoryCollector;

impl HistoryCollector {
    /// Create a new HistoryCollector
    pub fn new() -> Self {
        Self
    }

    /// Get possible history file paths
    fn get_history_files() -> Vec<String> {
        let home = dirs::home_dir()
            .expect("Could not determine home directory")
            .to_string_lossy()
            .to_string();

        vec![
            format!("{}/.bash_history", home),
            format!("{}/.zsh_history", home),
        ]
    }

    /// Read history file with timestamps, limited to most recent entries
    pub fn read_history(&self) -> io::Result<Vec<TimestampedCommand>> {
        let mut commands = Vec::new();
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        for history_file in Self::get_history_files() {
            let path = Path::new(&history_file);
            if !path.exists() {
                continue;
            }

            let mut file = File::open(path)?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;
            let content = String::from_utf8_lossy(&buffer);

            for line in content.lines() {
                // Skip empty lines
                if line.trim().is_empty() {
                    continue;
                }

                // Parse zsh format: ": timestamp:0;command"
                if line.starts_with(':') && line.contains(';') {
                    if let Some(cmd_entry) = Self::parse_zsh_history_line(line) {
                        if !Self::is_noise_command(&cmd_entry.command) {
                            // Only keep commands from last 24 hours
                            if current_time.saturating_sub(cmd_entry.timestamp) < 86400 {
                                commands.push(cmd_entry);
                            }
                        }
                    }
                } else if !line.starts_with(':') {
                    // Bash history format - no timestamps
                    let command = line.to_string();
                    if !Self::is_noise_command(&command) {
                        commands.push(TimestampedCommand {
                            command,
                            timestamp: 0, // Unknown timestamp
                        });
                    }
                }
            }
        }

        // Limit to most recent 1000 commands to keep memory usage low
        if commands.len() > 1000 {
            commands.sort_by_key(|c| c.timestamp);
            commands = commands.into_iter().rev().take(1000).collect();
        }

        Ok(commands)
    }

    /// Parse zsh history line format: ": timestamp:0;command"
    fn parse_zsh_history_line(line: &str) -> Option<TimestampedCommand> {
        if !line.starts_with(':') {
            return None;
        }

        // Find the semicolon which separates metadata from command
        if let Some(semicolon_pos) = line.find(';') {
            // Extract command (everything after semicolon)
            let command = line[semicolon_pos + 1..].trim().to_string();

            // Validate command before processing
            if SafeValidator::validate_command(&command).is_err() {
                return None; // Skip invalid commands
            }

            // Extract timestamp from metadata (between ":" and next ":")
            let metadata = &line[1..semicolon_pos]; // Skip leading ":"
            if let Some(first_colon) = metadata.find(':') {
                let timestamp_str = metadata[..first_colon].trim();
                if let Ok(timestamp) = timestamp_str.parse::<u64>() {
                    // Validate timestamp is reasonable
                    if SafeValidator::validate_timestamp(timestamp).is_err() {
                        return None;
                    }

                    if !command.is_empty() {
                        return Some(TimestampedCommand { command, timestamp });
                    }
                }
            }
        }

        None
    }

    /// Determine if a command is noise that should be filtered out
    fn is_noise_command(cmd: &str) -> bool {
        let trimmed = cmd.trim();
        // Too short to be meaningful
        if trimmed.len() < 2 {
            return true;
        }

        // Common noise commands
        matches!(trimmed,
            "ls" | "ll" | "la" | "cd" | "pwd" | "clear" | "history" |
            "exit" | "logout" | "su" | "sudo -v" | "fg" | "bg" | "jobs"
        ) ||
        // Repeated whitespace only
        trimmed.chars().all(|c| c.is_whitespace())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_noise_command() {
        assert!(HistoryCollector::is_noise_command("ls"));
        assert!(HistoryCollector::is_noise_command("cd"));
        assert!(HistoryCollector::is_noise_command("  "));
        assert!(!HistoryCollector::is_noise_command("vim file.txt"));
        assert!(!HistoryCollector::is_noise_command("git commit"));
    }

    #[test]
    fn test_parse_zsh_history_line() {
        let line = ": 1234567890:0;git commit -m 'initial'";
        let result = HistoryCollector::parse_zsh_history_line(line);
        assert!(result.is_some());
        let cmd = result.unwrap();
        assert_eq!(cmd.command, "git commit -m 'initial'");
        assert_eq!(cmd.timestamp, 1234567890);
    }

    #[test]
    fn test_parse_zsh_history_line_invalid() {
        let line = "invalid format";
        let result = HistoryCollector::parse_zsh_history_line(line);
        assert!(result.is_none());
    }
}
