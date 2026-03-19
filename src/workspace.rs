use anyhow::Result;
use std::path::{Path, PathBuf};
use std::fs;
use serde::{Serialize, Deserialize};

/// Configuration for a single repository in a workspace
/// Reserved for Phase 3b: Advanced workspace configuration system
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoConfig {
    pub path: String,
    pub name: String,
    pub tags: Vec<String>, // e.g., ["backend", "critical"]
}

/// Workspace configuration (optional for advanced use)
/// Reserved for Phase 3b: Advanced workspace configuration system
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceConfig {
    pub name: String,
    pub repositories: Vec<RepoConfig>,
    pub excluded: Vec<String>, // Glob patterns to skip
}

/// Discovers and manages Git repositories in a workspace
pub struct WorkspaceManager;

impl WorkspaceManager {
    /// Find all Git repositories in a directory (recursive, max depth 3)
    pub fn find_repositories(root: &Path) -> Result<Vec<PathBuf>> {
        let mut repos = Vec::new();
        Self::find_repos_recursive(root, 0, &mut repos)?;
        Ok(repos)
    }

    /// Recursively find git repositories (max 3 levels deep)
    fn find_repos_recursive(
        path: &Path,
        depth: usize,
        repos: &mut Vec<PathBuf>,
    ) -> Result<()> {
        // Don't go too deep
        if depth > 3 {
            return Ok(());
        }

        // Check if this is a git repo
        if Self::is_git_repo(path) {
            repos.push(path.to_path_buf());
            return Ok(()); // Don't recurse into a git repo
        }

        // Skip hidden directories and common exclusions
        if let Some(name) = path.file_name() {
            let name_str = name.to_string_lossy();
            if name_str.starts_with('.')
                || matches!(name_str.as_ref(), "target" | "node_modules" | ".cargo")
            {
                return Ok(());
            }
        }

        // Recurse into subdirectories
        if path.is_dir() {
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() && path.file_name().map(|n| !n.to_string_lossy().starts_with('.')).unwrap_or(false) {
                    Self::find_repos_recursive(&path, depth + 1, repos)?;
                }
            }
        }

        Ok(())
    }

    /// Check if a directory is a git repository
    fn is_git_repo(path: &Path) -> bool {
        std::process::Command::new("git")
            .arg("-C")
            .arg(path)
            .arg("rev-parse")
            .arg("--is-inside-work-tree")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    /// Get the name of a repository from its path
    /// Reserved for Phase 3b: Advanced repo naming and organization
    #[allow(dead_code)]
    pub fn repo_name(path: &Path) -> String {
        path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown")
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repo_name_extraction() {
        let path = PathBuf::from("/home/user/projects/my-repo");
        assert_eq!(WorkspaceManager::repo_name(&path), "my-repo");
    }

    #[test]
    fn test_workspace_manager_creation() {
        let _manager = WorkspaceManager;
    }
}
