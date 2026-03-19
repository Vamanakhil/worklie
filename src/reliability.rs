/// Enhanced error handling with graceful degradation
use std::path::PathBuf;
use crate::error::WorklieError;
use crate::output_formatter::Colors;

/// Safe operations that degrade gracefully on failure
pub struct SafeOperations;

impl SafeOperations {
    /// Safely read history with fallback
    pub fn safe_read_history<F>(operation: F) -> Vec<String>
    where
        F: FnOnce() -> Result<Vec<String>, WorklieError>,
    {
        match operation() {
            Ok(commands) => commands,
            Err(e) => {
                eprintln!("⚠ Warning: History collection failed: {}", e);
                eprintln!("  Continuing with empty history for this session");
                Vec::new()
            }
        }
    }

    /// Safely execute git operations with error classification
    pub fn safe_git_operation<F, T>(operation: F) -> Result<T, WorklieError>
    where
        F: FnOnce() -> Result<T, std::io::Error>,
    {
        operation().map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                WorklieError::GitCollectionError("Git not found or not in a git repository".to_string())
            } else if e.kind() == std::io::ErrorKind::PermissionDenied {
                WorklieError::GitCollectionError("Permission denied accessing git".to_string())
            } else {
                WorklieError::GitCollectionError(format!("Git operation failed: {}", e))
            }
        })
    }

    /// Sanitize command strings to prevent issues
    pub fn sanitize_command(command: &str) -> String {
        command
            .chars()
            .filter(|c| !matches!(c, '\x00'..='\x08' | '\x0b'..='\x0c' | '\x0e'..='\x1f'))
            .collect::<String>()
            .trim()
            .to_string()
    }

    /// Safely access cache with logging
    pub fn safe_cache_operation<F, T>(
        operation: F,
        context: &str,
    ) -> Option<T>
    where
        F: FnOnce() -> Result<T, WorklieError>,
    {
        match operation() {
            Ok(result) => Some(result),
            Err(e) => {
                eprintln!("⚠ Cache operation failed ({}): {}", context, e);
                None
            }
        }
    }

    /// Validate file path accessibility
    pub fn validate_file_access(path: &PathBuf) -> Result<(), WorklieError> {
        use std::fs;

        if !path.exists() {
            return Err(WorklieError::IoError(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("File not found: {}", path.display()),
            )));
        }

        if !path.is_file() {
            return Err(WorklieError::IoError(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Path is not a file: {}", path.display()),
            )));
        }

        // Try to read metadata to verify accessibility
        fs::metadata(path).map_err(|e| {
            WorklieError::IoError(std::io::Error::new(
                e.kind(),
                format!("Cannot access file {}: {}", path.display(), e),
            ))
        })?;

        Ok(())
    }
}

/// Diagnostics for system readiness
#[derive(Debug, Clone)]
pub struct DiagnosticsReport {
    pub history_accessible: bool,
    pub git_accessible: bool,
    pub cache_writable: bool,
    pub config_valid: bool,
    pub issues: Vec<String>,
    pub warnings: Vec<String>,
}

impl DiagnosticsReport {
    pub fn is_healthy(&self) -> bool {
        self.history_accessible && self.git_accessible && self.cache_writable
    }

    pub fn print(&self) {
        println!("{}╔════════════════════════════════════════╗{}", Colors::GREEN, Colors::RESET);
        println!("{}║  Worklie System Diagnostics            ║{}", Colors::GREEN, Colors::RESET);
        println!("{}╚════════════════════════════════════════╝{}", Colors::GREEN, Colors::RESET);

        println!("\n{}📋 Status Summary:{}", Colors::BOLD, Colors::RESET);
        println!(
            "  History Files:  {}",
            if self.history_accessible {
                format!("{}✓ OK{}", Colors::GREEN, Colors::RESET)
            } else {
                format!("{}✗ Issues{}", Colors::BRIGHT_YELLOW, Colors::RESET)
            }
        );
        println!(
            "  Git Access:     {}",
            if self.git_accessible {
                format!("{}✓ OK{}", Colors::GREEN, Colors::RESET)
            } else {
                format!("{}✗ Issues{}", Colors::BRIGHT_YELLOW, Colors::RESET)
            }
        );
        println!(
            "  Cache Directory: {}",
            if self.cache_writable {
                format!("{}✓ OK{}", Colors::GREEN, Colors::RESET)
            } else {
                format!("{}✗ Issues{}", Colors::BRIGHT_YELLOW, Colors::RESET)
            }
        );
        println!(
            "  Configuration:  {}",
            if self.config_valid {
                format!("{}✓ OK{}", Colors::GREEN, Colors::RESET)
            } else {
                format!("{}✗ Issues{}", Colors::BRIGHT_YELLOW, Colors::RESET)
            }
        );

        if !self.warnings.is_empty() {
            println!("\n{}⚠ Warnings:{}", Colors::YELLOW, Colors::RESET);
            for warning in &self.warnings {
                println!("  • {}", warning);
            }
        }

        if !self.issues.is_empty() {
            println!("\n{}❌ Issues:{}", Colors::BRIGHT_YELLOW, Colors::RESET);
            for issue in &self.issues {
                println!("  • {}", issue);
            }
        }

        if self.is_healthy() {
            println!("\n{}✓ System is healthy and ready to use{}", Colors::GREEN, Colors::RESET);
        } else {
            println!(
                "\n{}⚠ Some issues detected. See above for details.{}",
                Colors::YELLOW, Colors::RESET
            );
        }
    }
}

pub fn run_diagnostics() -> DiagnosticsReport {
    use std::fs;

    let mut report = DiagnosticsReport {
        history_accessible: false,
        git_accessible: false,
        cache_writable: false,
        config_valid: false,
        issues: Vec::new(),
        warnings: Vec::new(),
    };

    // Check history files
    if let Ok(home) = std::env::var("HOME") {
        let history_files = vec![
            format!("{}/.zsh_history", home),
            format!("{}/.bash_history", home),
        ];

        let mut history_found = false;
        for path in history_files {
            if PathBuf::from(&path).exists() {
                history_found = true;
                report.history_accessible = true;
                break;
            }
        }

        if !history_found {
            report
                .warnings
                .push("No history files found (.zsh_history or .bash_history)".to_string());
        }
    } else {
        report
            .issues
            .push("Could not determine home directory".to_string());
    }

    // Check git availability
    if let Ok(output) = std::process::Command::new("git")
        .arg("--version")
        .output()
    {
        if output.status.success() {
            report.git_accessible = true;
        } else {
            report.issues.push("Git command failed".to_string());
        }
    } else {
        report
            .issues
            .push("Git not found in PATH. Install git to enable commit analysis.".to_string());
    }

    // Check cache directory
    let cache_dir = PathBuf::from(format!(
        "{}/.worklie",
        std::env::var("HOME").unwrap_or_default()
    ));

    if let Ok(_) = fs::create_dir_all(&cache_dir) {
        // Try to write test file
        let test_file = cache_dir.join(".diagnostic_test");
        if let Ok(_) = fs::write(&test_file, b"test") {
            let _ = fs::remove_file(&test_file);
            report.cache_writable = true;
        } else {
            report
                .issues
                .push(format!("Cannot write to cache directory: {}", cache_dir.display()));
        }
    } else {
        report.issues.push(format!(
            "Cannot create cache directory: {}",
            cache_dir.display()
        ));
    }

    // Check configuration
    if let Ok(config) = crate::config::WorklieConfig::load() {
        if let Ok(_) = config.validate() {
            report.config_valid = true;
        } else {
            report.issues.push("Configuration validation failed".to_string());
        }
    } else {
        report.warnings.push("Using default configuration".to_string());
        report.config_valid = true; // Defaults are always valid
    }

    report
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_command() {
        let dirty = "git status\x00bad\x08worse";
        let clean = SafeOperations::sanitize_command(dirty);
        assert!(!clean.contains('\x00'));
        assert!(!clean.contains('\x08'));
    }

    #[test]
    fn test_safe_read_history_fallback() {
        let result = SafeOperations::safe_read_history(|| {
            Err(WorklieError::HistoryCollectionError("test error".to_string()))
        });
        assert_eq!(result.len(), 0); // Returns empty vec on error
    }

    #[test]
    fn test_safe_read_history_success() {
        let test_data = vec!["cmd1".to_string(), "cmd2".to_string()];
        let data_clone = test_data.clone();
        let result = SafeOperations::safe_read_history(|| Ok(data_clone));
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_diagnostics_report_healthy() {
        let mut report = DiagnosticsReport {
            history_accessible: true,
            git_accessible: true,
            cache_writable: true,
            config_valid: true,
            issues: Vec::new(),
            warnings: Vec::new(),
        };
        assert!(report.is_healthy());

        report.git_accessible = false;
        assert!(!report.is_healthy());
    }
}
