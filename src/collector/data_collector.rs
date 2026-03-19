use anyhow::Result;
use crate::parser::history_parser::{HistoryParser, GitParser, ParsedCommand, ParsedCommit};
use crate::analyzer::context_inference::{ContextInferenceEngine, WorkContext};
use crate::collector;

/// Helper for collecting and parsing all necessary data for analysis
/// Reserved for Phase 3b refactoring to reduce boilerplate across handlers
#[allow(dead_code)]
pub struct DataCollector;

#[allow(dead_code)]
impl DataCollector {
    /// Collect all data needed for analysis (history + commits + context)
    /// This consolidates the repeated code pattern across 14 handlers
    pub fn collect_and_analyze() -> Result<(Vec<ParsedCommand>, Vec<ParsedCommit>, WorkContext)> {
        // Parallel collection
        let history_handle = std::thread::spawn(|| {
            let collector = collector::history::HistoryCollector::new();
            collector.read_history()
        });

        let git_handle = std::thread::spawn(|| {
            let collector = collector::git::GitCollector::new();
            if collector.is_git_repo() {
                collector.recent_commits(100)
            } else {
                Ok(Vec::new())
            }
        });

        // Collect results with proper error handling
        let history_commands = history_handle
            .join()
            .map_err(|_| anyhow::anyhow!("History collection thread panicked"))?
            .map_err(|e| anyhow::anyhow!("Failed to read history: {}", e))?;

        let git_commits = git_handle
            .join()
            .map_err(|_| anyhow::anyhow!("Git collection thread panicked"))?
            .map_err(|e| anyhow::anyhow!("Failed to collect git commits: {}", e))?;

        // Parse data
        let parsed_commands = HistoryParser::new().parse_commands(history_commands);
        let parsed_commits = GitParser::new().parse_commits(git_commits);

        // Infer context
        let context = ContextInferenceEngine::new().infer_context();

        Ok((parsed_commands, parsed_commits, context))
    }

    /// Collect data with custom commit limit (for weekly reports, etc.)
    pub fn collect_with_limit(commit_limit: usize) -> Result<(Vec<ParsedCommand>, Vec<ParsedCommit>, WorkContext)> {
        let history_handle = std::thread::spawn(|| {
            let collector = collector::history::HistoryCollector::new();
            collector.read_history()
        });

        let git_handle = std::thread::spawn(move || {
            let collector = collector::git::GitCollector::new();
            if collector.is_git_repo() {
                collector.recent_commits(commit_limit)
            } else {
                Ok(Vec::new())
            }
        });

        let history_commands = history_handle
            .join()
            .map_err(|_| anyhow::anyhow!("History collection thread panicked"))?
            .map_err(|e| anyhow::anyhow!("Failed to read history: {}", e))?;

        let git_commits = git_handle
            .join()
            .map_err(|_| anyhow::anyhow!("Git collection thread panicked"))?
            .map_err(|e| anyhow::anyhow!("Failed to collect git commits: {}", e))?;

        let parsed_commands = HistoryParser::new().parse_commands(history_commands);
        let parsed_commits = GitParser::new().parse_commits(git_commits);
        let context = ContextInferenceEngine::new().infer_context();

        Ok((parsed_commands, parsed_commits, context))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_collector_creation() {
        // Just verify the collector exists and can be instantiated
        let _collector = DataCollector;
    }
}
