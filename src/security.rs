/// Security module for input validation and safe handling
use std::path::{Path, PathBuf};

/// Safe validation for commands
pub struct SafeValidator;

impl SafeValidator {
    /// Validate a command string is safe to process
    /// - No null bytes
    /// - Reasonable length
    /// - No control characters that could cause issues
    pub fn validate_command(cmd: &str) -> Result<(), String> {
        // Check for null bytes
        if cmd.contains('\0') {
            return Err("Command contains null bytes".to_string());
        }

        // Check length (max 10KB per command to prevent DoS)
        if cmd.len() > 10240 {
            return Err("Command exceeds maximum length (10KB)".to_string());
        }

        // Check for control characters (except newline/tab which are allowed in history)
        for ch in cmd.chars() {
            if ch.is_control() && ch != '\n' && ch != '\t' && ch != '\r' {
                return Err(format!("Command contains invalid control character: {:?}", ch));
            }
        }

        Ok(())
    }

    /// Validate a file path is safe to read
    /// Reserved for Phase 3b: Enhanced file monitoring and validation
    #[allow(dead_code)]
    pub fn validate_file_path(path: &Path) -> Result<(), String> {
        // Check if path exists
        if !path.exists() {
            return Ok(()); // File doesn't exist is OK (not all users have history files)
        }

        // Check if it's a file
        if !path.is_file() {
            return Err(format!("Path is not a file: {}", path.display()));
        }

        // Check if it's readable by trying to get metadata
        match std::fs::metadata(path) {
            Ok(metadata) => {
                // Check permissions - we need read access
                if metadata.permissions().readonly() {
                    // Readonly is OK, we just need read access
                    Ok(())
                } else {
                    Ok(())
                }
            }
            Err(e) => Err(format!("Cannot access file: {}", e)),
        }
    }

    /// Validate a timestamp is reasonable
    pub fn validate_timestamp(ts: u64) -> Result<(), String> {
        // Timestamp should be somewhat recent (not year 2100+)
        // Allow timestamps from year 1970 (Unix epoch) to 2100
        let year_2100_timestamp = 4102444800u64;

        if ts > year_2100_timestamp {
            return Err("Timestamp is in the future (beyond year 2100)".to_string());
        }

        Ok(())
    }

    /// Validate git output format
    /// Expected: hash:timestamp:author:subject
    /// Reserved for Phase 3b: Enhanced git output validation
    #[allow(dead_code)]
    pub fn validate_git_output_line(line: &str) -> Result<(), String> {
        let parts: Vec<&str> = line.splitn(4, ':').collect();

        if parts.len() < 4 {
            return Err("Git output line missing required fields".to_string());
        }

        let hash = parts[0];
        let timestamp_str = parts[1];
        let author = parts[2];
        let subject = parts[3];

        // Validate hash (should be hex characters)
        if !hash.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err("Invalid git hash format".to_string());
        }

        if hash.len() > 40 {
            // SHA1 is 40 chars, SHA256 is 64 chars
            return Err("Git hash too long".to_string());
        }

        // Validate timestamp is a number
        if timestamp_str.parse::<u64>().is_err() {
            return Err("Invalid timestamp format".to_string());
        }

        // Check author and subject lengths (reasonable bounds)
        if author.len() > 256 {
            return Err("Author name too long".to_string());
        }

        if subject.len() > 1024 {
            return Err("Commit subject too long".to_string());
        }

        Ok(())
    }
}

/// Safe path handling
/// Reserved for Phase 3b: Path validation and secure file operations
#[allow(dead_code)]
pub struct SafePath;

#[allow(dead_code)]
impl SafePath {
    /// Get home directory safely
    pub fn get_home_dir() -> Result<PathBuf, String> {
        dirs::home_dir().ok_or_else(|| "Could not determine home directory".to_string())
    }

    /// Get a safe path for history file (prevents directory traversal)
    pub fn get_history_file(filename: &str) -> Result<PathBuf, String> {
        // Only allow specific filenames to prevent directory traversal
        match filename {
            ".bash_history" | ".zsh_history" => {
                let home = Self::get_home_dir()?;
                Ok(home.join(filename))
            }
            _ => Err("Invalid history filename".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_command() {
        assert!(SafeValidator::validate_command("git status").is_ok());
        assert!(SafeValidator::validate_command("npm test").is_ok());

        // Should reject null bytes
        assert!(SafeValidator::validate_command("git\0status").is_err());

        // Should reject extremely long commands
        let long_cmd = "a".repeat(20000);
        assert!(SafeValidator::validate_command(&long_cmd).is_err());
    }

    #[test]
    fn test_validate_timestamp() {
        assert!(SafeValidator::validate_timestamp(1609459200).is_ok()); // 2021-01-01
        assert!(SafeValidator::validate_timestamp(0).is_ok());           // Unix epoch

        // Should reject future timestamps
        let future = 4102444800u64 + 1000;
        assert!(SafeValidator::validate_timestamp(future).is_err());
    }

    #[test]
    fn test_validate_git_output() {
        let valid = "abc123:1609459200:John Doe:fix: bug in parser";
        assert!(SafeValidator::validate_git_output_line(valid).is_ok());

        // Should reject invalid formats
        assert!(SafeValidator::validate_git_output_line("incomplete").is_err());
        assert!(SafeValidator::validate_git_output_line("xyz:notanumber:author:subject").is_err());
    }

    #[test]
    fn test_safe_path_history_files() {
        let result = SafePath::get_history_file(".bash_history");
        assert!(result.is_ok());

        // Should reject invalid filenames
        assert!(SafePath::get_history_file("../../etc/passwd").is_err());
        assert!(SafePath::get_history_file("random_file").is_err());
    }
}
