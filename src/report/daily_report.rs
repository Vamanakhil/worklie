use serde::{Serialize, Deserialize};
use crate::analyzer::activity_clusterer::Activity;
use crate::analyzer::context_inference::WorkContext;

/// Structured daily report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyReport {
    pub project: Option<String>,
    pub domain: Option<String>,
    pub work_type: Option<String>,
    pub branch: Option<String>,
    pub activities: Vec<u32>,
    pub total_commits: usize,
    pub commit_messages: Vec<String>,
    pub files_modified: Vec<String>,
}

/// Generates daily work reports
pub struct DailyReportGenerator;

impl DailyReportGenerator {
    /// Create a new DailyReportGenerator
    pub fn new() -> Self {
        Self
    }

    /// Generate a structured daily report
    pub fn generate_report_data(&self, activities: Vec<Activity>, context: &WorkContext) -> DailyReport {
        let mut all_files = std::collections::HashSet::new();
        let mut total_commits = 0;
        let mut commit_messages = Vec::new();

        for activity in &activities {
            total_commits += activity.commits.len();
            for commit in &activity.commits {
                commit_messages.push(commit.message.clone());
            }
            for file in &activity.files {
                all_files.insert(file.clone());
            }
        }

        DailyReport {
            project: context.project_name.clone(),
            domain: context.domain.clone(),
            work_type: context.work_type.clone(),
            branch: context.branch_name.clone(),
            activities: (0..activities.len() as u32).collect(),
            total_commits,
            commit_messages,
            files_modified: all_files.into_iter().collect(),
        }
    }

    /// Generate a human-readable daily work report
    pub fn generate_report(&self, activities: Vec<Activity>, context: &WorkContext) -> String {
        let mut report = String::new();

        report.push_str("Daily Work Summary\n");
        report.push_str("==================\n\n");

        // Project information
        if let Some(ref project) = context.project_name {
            report.push_str(&format!("Project: {}\n\n", project));
        } else {
            report.push_str("Project: Not detected\n\n");
        }

        // Activities
        report.push_str("Activities Detected:\n");
        if activities.is_empty() {
            report.push_str("  • No activities detected\n");
        } else {
            for activity in &activities {
                report.push_str(&format!("  • {}\n", activity.title));
            }
        }
        report.push_str("\n");

        // Commits
        report.push_str("Commits:\n");
        let mut total_commits = 0;
        for activity in &activities {
            total_commits += activity.commits.len();
        }
        if total_commits == 0 {
            report.push_str("  • No commits detected\n");
        } else {
            report.push_str(&format!("  • {} commits\n", total_commits));
            let mut shown = 0;
            for activity in &activities {
                for commit in &activity.commits {
                    if shown >= 3 {
                        break;
                    }
                    report.push_str(&format!("    • {}\n", commit.message));
                    shown += 1;
                }
                if shown >= 3 {
                    break;
                }
            }
        }
        report.push_str("\n");

        // Files modified
        report.push_str("Files Modified:\n");
        let mut all_files = std::collections::HashSet::new();
        for activity in &activities {
            for file in &activity.files {
                all_files.insert(file);
            }
        }
        if all_files.is_empty() {
            report.push_str("  • No files modified\n");
        } else {
            let mut shown = 0;
            for file in all_files.iter() {
                if shown >= 5 {
                    break;
                }
                report.push_str(&format!("  • {}\n", file));
                shown += 1;
            }
            if all_files.len() > 5 {
                report.push_str(&format!("  • ... and {} more files\n", all_files.len() - 5));
            }
        }
        report.push_str("\n");

        // Focus areas
        report.push_str("Focus Areas:\n");
        if let Some(ref domain) = context.domain {
            report.push_str(&format!("  • {}\n", domain));
        }
        if let Some(ref work_type) = context.work_type {
            report.push_str(&format!("  • {}\n", work_type));
        }
        if context.domain.is_none() && context.work_type.is_none() {
            report.push_str("  • Not detected\n");
        }

        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::history_parser::{ParsedCommand, ParsedCommit};

    #[test]
    fn test_generate_empty_report() {
        let generator = DailyReportGenerator::new();
        let context = WorkContext {
            project_name: None,
            domain: None,
            work_type: None,
            repository_path: None,
            branch_name: None,
        };

        let report = generator.generate_report(vec![], &context);
        assert!(report.contains("Daily Work Summary"));
        assert!(report.contains("No activities detected"));
    }

    #[test]
    fn test_generate_report_with_data() {
        let generator = DailyReportGenerator::new();
        let context = WorkContext {
            project_name: Some("test-project".to_string()),
            domain: Some("authentication".to_string()),
            work_type: Some("debugging".to_string()),
            repository_path: Some("/path/to/repo".to_string()),
            branch_name: Some("feature-auth".to_string()),
        };

        let activity = Activity {
            id: "test-activity".to_string(),
            title: "Fixed authentication bug".to_string(),
            description: "Fixed validation issue".to_string(),
            start_time: 1000,
            end_time: 2000,
            commands: vec![],
            commits: vec![ParsedCommit {
                hash: "abc123".to_string(),
                message: "fix(auth): token validation".to_string(),
                author: None,
                timestamp: 1500,
            }],
            files: vec!["auth.js".to_string()],
            directory: None,
        };

        let report = generator.generate_report(vec![activity], &context);
        assert!(report.contains("Project: test-project"));
        assert!(report.contains("Fixed authentication bug"));
    }
}
