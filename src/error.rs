use std::fmt;
use std::io;

/// Custom error type for Worklie operations
#[derive(Debug)]
pub enum WorklieError {
    /// Failed to collect shell history
    HistoryCollectionError(String),

    /// Failed to collect git data
    GitCollectionError(String),

    /// Failed to perform cache operation
    CacheError(String),

    /// Failed to parse data
    /// Reserved for Phase 3b: Advanced parsing with error recovery
    #[allow(dead_code)]
    ParseError(String),

    /// Failed to cluster activities
    /// Reserved for Phase 3b: Clustering algorithm errors
    #[allow(dead_code)]
    ClusterError(String),

    /// Thread panicked or failed to join
    ThreadError(String),

    /// Configuration or setup error
    /// Reserved for Phase 3b: Configuration system
    #[allow(dead_code)]
    ConfigError(String),

    /// I/O operation failed
    IoError(io::Error),
}

impl fmt::Display for WorklieError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WorklieError::HistoryCollectionError(msg) => {
                write!(f, "Failed to collect shell history: {}", msg)
            }
            WorklieError::GitCollectionError(msg) => {
                write!(f, "Failed to collect git data: {}", msg)
            }
            WorklieError::CacheError(msg) => {
                write!(f, "Cache operation failed: {}", msg)
            }
            WorklieError::ParseError(msg) => {
                write!(f, "Failed to parse data: {}", msg)
            }
            WorklieError::ClusterError(msg) => {
                write!(f, "Failed to cluster activities: {}", msg)
            }
            WorklieError::ThreadError(msg) => {
                write!(f, "Thread error: {}", msg)
            }
            WorklieError::ConfigError(msg) => {
                write!(f, "Configuration error: {}", msg)
            }
            WorklieError::IoError(err) => {
                write!(f, "I/O error: {}", err)
            }
        }
    }
}

impl std::error::Error for WorklieError {}

impl From<io::Error> for WorklieError {
    fn from(err: io::Error) -> Self {
        WorklieError::IoError(err)
    }
}

/// Helper to convert Result<T> to actionable error message
impl WorklieError {
    /// Get a user-friendly error message with suggestions
    /// Reserved for Phase 3b: Enhanced error handling with context propagation
    #[allow(dead_code)]
    pub fn with_context(&self, context: &str) -> String {
        match self {
            WorklieError::HistoryCollectionError(_) => {
                format!(
                    "{}\n\nContext: {}\n\nTip: Ensure ~/.zsh_history or ~/.bash_history exists and is readable",
                    self, context
                )
            }
            WorklieError::GitCollectionError(_) => {
                format!(
                    "{}\n\nContext: {}\n\nTip: Make sure you're in a git repository and git is installed",
                    self, context
                )
            }
            WorklieError::CacheError(_) => {
                format!(
                    "{}\n\nContext: {}\n\nTip: Try removing ~/.worklie/cache/ and running again",
                    self, context
                )
            }
            WorklieError::ThreadError(_) => {
                format!(
                    "{}\n\nContext: {}\n\nTip: This is an internal error. Please report it",
                    self, context
                )
            }
            _ => format!("{}\n\nContext: {}", self, context),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = WorklieError::HistoryCollectionError("file not found".to_string());
        let msg = err.to_string();
        assert!(msg.contains("shell history"));
        assert!(msg.contains("file not found"));
    }

    #[test]
    fn test_error_with_context() {
        let err = WorklieError::GitCollectionError("command failed".to_string());
        let msg = err.with_context("git log");
        assert!(msg.contains("git data"));
        assert!(msg.contains("git repository"));
        assert!(msg.contains("git log"));
    }

    #[test]
    fn test_io_error_conversion() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "test");
        let err: WorklieError = io_err.into();
        let msg = err.to_string();
        assert!(msg.contains("I/O error"));
    }
}
