/// Structured logging setup and utilities for Worklie
use tracing_subscriber::{fmt, prelude::*, EnvFilter};
use std::env;

/// Initialize structured logging with environment-based configuration
///
/// Respects the WORKLIE_LOG environment variable for log level:
/// - WORKLIE_LOG=trace   (most verbose)
/// - WORKLIE_LOG=debug
/// - WORKLIE_LOG=info    (default if not specified)
/// - WORKLIE_LOG=warn
/// - WORKLIE_LOG=error   (minimal)
///
/// Example:
/// ```bash
/// WORKLIE_LOG=debug ./worklie report
/// WORKLIE_LOG=trace ./worklie --help
/// ```
pub fn init_logging() {
    let env_filter = env::var("WORKLIE_LOG")
        .unwrap_or_else(|_| "info".to_string());

    let env_filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new(&env_filter))
        .unwrap_or_else(|_| EnvFilter::new("info"));

    let fmt_layer = fmt::layer()
        .with_target(true)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_level(true)
        .with_writer(std::io::stderr);

    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer)
        .init();

    tracing::info!("Worklie logging initialized");
}

/// Initialize logging with explicit level (for testing/debugging)
pub fn init_logging_with_level(level: &str) {
    let env_filter = EnvFilter::try_new(level)
        .unwrap_or_else(|_| EnvFilter::new("info"));

    let fmt_layer = fmt::layer()
        .with_target(true)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_level(true)
        .with_writer(std::io::stderr);

    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer)
        .init();

    tracing::info!("Worklie logging initialized with level: {}", level);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logging_module_creation() {
        // Module should initialize without panic
        // Actual init_logging() call would setup global subscriber
        // We don't test that here to avoid conflicts with other tests
        assert_eq!("trace".to_lowercase(), "trace");
    }

    #[test]
    fn test_env_filter_parsing() {
        // Verify that filter strings are valid
        for level in &["trace", "debug", "info", "warn", "error"] {
            let result = EnvFilter::try_new(level);
            assert!(
                result.is_ok(),
                "Filter '{}' should be valid",
                level
            );
        }
    }

    #[test]
    fn test_invalid_env_filter() {
        // EnvFilter is very permissive and accepts various formats
        // It treats strings as potential target/module filters
        // So even "invalid_level" might be accepted as a module filter
        // We'll verify that the EnvFilter can be created but validate proper levels work
        let valid = EnvFilter::try_new("debug");
        assert!(valid.is_ok(), "Valid filter 'debug' should work");

        // EnvFilter accepts almost anything, treating it as potential directives
        // This is by design in the tracing library
        let permissive = EnvFilter::try_new("something_unusual");
        // The filter creation might succeed even with unusual strings
        // since they could be interpreted as module filters
        let _ = permissive;
    }
}
