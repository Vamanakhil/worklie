use serde::{Serialize, Deserialize};
use crate::collector::git::GitCollector;

/// Infers context about the work being done (project, domain, work type)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkContext {
    pub project_name: Option<String>,
    pub domain: Option<String>,
    pub work_type: Option<String>,
    pub repository_path: Option<String>,
    pub branch_name: Option<String>,
}

/// Infers context from various signals
pub struct ContextInferenceEngine {
    git_collector: GitCollector,
}

impl ContextInferenceEngine {
    /// Create a new ContextInferenceEngine
    pub fn new() -> Self {
        Self {
            git_collector: GitCollector::new(),
        }
    }

    /// Infer work context from git information and recent activity
    pub fn infer_context(&self) -> WorkContext {
        let mut context = WorkContext {
            project_name: None,
            domain: None,
            work_type: None,
            repository_path: None,
            branch_name: None,
        };

        // Get git information if available
        if self.git_collector.is_git_repo() {
            context.repository_path = self.git_collector.repo_root();
            context.branch_name = self.git_collector.current_branch();

            // Infer project name from repository path
            if let Some(path) = &context.repository_path {
                context.project_name = Some(
                    path.split(std::path::MAIN_SEPARATOR)
                        .last()
                        .unwrap_or("unknown")
                        .to_string(),
                );
            }

            // Infer domain from branch name and recent commits
            context.domain = self.infer_domain();
            context.work_type = self.infer_work_type();
        }

        context
    }

    /// Infer domain (auth, api, docker, etc.) from branch names and commit messages
    fn infer_domain(&self) -> Option<String> {
        // Check branch name for domain hints
        if let Some(branch) = &self.git_collector.current_branch() {
            let branch_lower = branch.to_lowercase();

            let domain_keywords = vec![
                ("auth", "authentication"),
                ("api", "api"),
                ("docker", "docker"),
                ("db", "database"),
                ("ui", "frontend"),
                ("frontend", "frontend"),
                ("backend", "backend"),
                ("infra", "infrastructure"),
                ("devops", "devops"),
                ("test", "testing"),
                ("deploy", "deployment"),
            ];

            for (keyword, domain) in domain_keywords {
                if branch_lower.contains(keyword) {
                    return Some(domain.to_string());
                }
            }
        }

        None
    }

    /// Infer work type (debugging, feature dev, testing, etc.)
    fn infer_work_type(&self) -> Option<String> {
        if let Ok(commits) = self.git_collector.recent_commits(10) {
            for commit in commits {
                let commit_lower = commit.to_lowercase();

                if commit_lower.contains("fix") || commit_lower.contains("bug") {
                    return Some("debugging".to_string());
                }
                if commit_lower.contains("feat") || commit_lower.contains("feature") {
                    return Some("feature development".to_string());
                }
                if commit_lower.contains("refactor") {
                    return Some("refactoring".to_string());
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_inference_creation() {
        let engine = ContextInferenceEngine::new();
        let _context = engine.infer_context();
    }
}
