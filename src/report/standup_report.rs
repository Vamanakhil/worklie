use serde::{Serialize, Deserialize};
use crate::analyzer::activity_clusterer::Activity;
use crate::analyzer::context_inference::WorkContext;

/// Structured standup report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StandupReport {
    pub yesterday_activities: Vec<String>,
    pub today_plan: Vec<String>,
    pub blockers: Vec<String>,
}

/// Generates standup reports
pub struct StandupReportGenerator;

impl StandupReportGenerator {
    pub fn new() -> Self {
        Self
    }

    pub fn generate_report_data(&self, activities: Vec<Activity>, _context: &WorkContext) -> StandupReport {
        let mut yesterday_activities = Vec::new();

        for activity in &activities {
            yesterday_activities.push(activity.title.clone());
        }

        StandupReport {
            yesterday_activities,
            today_plan: vec!["Continue working on remaining tasks".to_string()],
            blockers: vec![],
        }
    }

    pub fn generate_report(&self, activities: Vec<Activity>, _context: &WorkContext) -> String {
        let mut report = String::new();

        report.push_str("Standup Report\n");
        report.push_str("==============\n\n");

        report.push_str("Yesterday\n");
        report.push_str("---------\n");
        if activities.is_empty() {
            report.push_str("• No activities recorded\n");
        } else {
            for activity in &activities {
                report.push_str(&format!("• {}\n", activity.title));
            }
        }
        report.push_str("\n");

        report.push_str("Today\n");
        report.push_str("-----\n");
        report.push_str("• Continue with ongoing tasks\n");
        report.push_str("\n");

        report.push_str("Blockers\n");
        report.push_str("--------\n");
        report.push_str("None detected\n");

        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standup_report_generation() {
        let generator = StandupReportGenerator::new();
        let context = WorkContext {
            project_name: Some("test-project".to_string()),
            domain: None,
            work_type: None,
            repository_path: None,
            branch_name: None,
        };
        let report = generator.generate_report(vec![], &context);
        assert!(report.contains("Standup Report"));
    }
}
