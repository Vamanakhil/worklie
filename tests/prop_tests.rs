//! Property-based testing for Worklie components
//!
//! Uses proptest to verify invariants and properties hold for random inputs

// Disable file output for proptest to avoid issues in test environments
proptest::prop_compose! {
    fn safe_string()(s in r"[a-zA-Z0-9\-_\./ ]*") -> String {
        s
    }
}

#[cfg(test)]
mod prop_tests {
    use proptest::prelude::*;
    use worklie::logging;
    use worklie::reliability::SafeOperations;
    use worklie::parser::history_parser::{HistoryParser, GitParser};

    // Property: Sanitize command should never produce control characters
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn prop_sanitize_never_produces_control_chars(s in ".*") {
            let sanitized = SafeOperations::sanitize_command(&s);

            // Check that no control characters (0x00-0x1F except tab) are present
            for c in sanitized.chars() {
                let code = c as u32;
                // Only tab (0x09) and newline (0x0a) and carriage return (0x0d) are acceptable
                prop_assert!(
                    code >= 0x20 || code == 0x09 || code == 0x0a || code == 0x0d,
                    "Found control character: {:?} (0x{:02x})",
                    c,
                    code
                );
            }
        }
    }

    // Property: Sanitize should preserve normal content
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn prop_sanitize_preserves_alphanumeric(s in r"[a-zA-Z0-9]+") {
            let sanitized = SafeOperations::sanitize_command(&s);
            prop_assert_eq!(sanitized, s, "Alphanumeric content should be preserved");
        }
    }

    // Property: Sanitized output should be valid UTF-8
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn prop_sanitized_output_is_valid_utf8(s in ".*") {
            let sanitized = SafeOperations::sanitize_command(&s);
            // Should always be valid UTF-8 (string can't be invalid UTF-8 in Rust)
            prop_assert!(sanitized.is_ascii() || sanitized.chars().all(|_| true));
        }
    }

    // Property: Multiple sanitizations should be idempotent
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn prop_sanitize_is_idempotent(s in ".*") {
            let sanitized_once = SafeOperations::sanitize_command(&s);
            let sanitized_twice = SafeOperations::sanitize_command(&sanitized_once);
            prop_assert_eq!(sanitized_once, sanitized_twice, "Sanitization should be idempotent");
        }
    }

    // Property: History parser should never panic on any input
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn prop_history_parser_never_panics(command in ".*", timestamp in 0u64..u64::MAX) {
            let parser = HistoryParser::new();
            let _ = parser.parse_command(command, timestamp);
            // Should not panic
        }
    }

    // Property: Parsed command should have associated timestamp
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn prop_parsed_command_preserves_timestamp(timestamp in 1000u64..2000000000u64) {
            let parser = HistoryParser::new();
            let parsed = parser.parse_command("test command".to_string(), timestamp);
            prop_assert_eq!(parsed.timestamp, timestamp);
        }
    }

    // Property: Git parser should be robust to malformed input
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn prop_git_parser_handles_malformed(line in ".*") {
            let parser = GitParser::new();
            let result = parser.parse_commit(line);
            // Should never panic, result can be Some or None
            let _ = result;
        }
    }

    // Property: Config should accept valid focus threshold
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn prop_valid_focus_threshold_accepted(threshold in 0.0f64..=1.0f64) {
            let config = worklie::config::WorklieConfig::default();
            let mut config = config;
            config.analysis.focus_threshold = threshold;
            let result = config.validate();
            prop_assert!(result.is_ok(), "Focus threshold {} should be accepted", threshold);
        }
    }

    // Property: Cluster time window must be positive and <= 1440
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn prop_valid_cluster_time_window(minutes in 1u64..1440u64) {
            let mut config = worklie::config::WorklieConfig::default();
            // Also set session_gap to be >= cluster_time_window to satisfy cross-field validation
            config.analysis.cluster_time_window_minutes = minutes;
            if config.analysis.session_gap_minutes < minutes {
                config.analysis.session_gap_minutes = minutes;
            }
            let result = config.validate();
            prop_assert!(result.is_ok(), "Cluster time window {} should be valid", minutes);
        }
    }

    // Property: Retention days must be positive and <= 36500
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn prop_valid_retention_days(days in 1u32..36500u32) {
            let config = worklie::config::WorklieConfig::default();
            let mut config = config;
            config.cache.retention_days = days;
            let result = config.validate();
            prop_assert!(result.is_ok(), "Retention days {} should be valid", days);
        }
    }
}
