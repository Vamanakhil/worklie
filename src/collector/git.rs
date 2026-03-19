use std::process::Command;
use std::io;

/// Collects git repository information
pub struct GitCollector;

impl GitCollector {
    /// Create a new GitCollector
    pub fn new() -> Self {
        Self
    }

    /// Check if current directory is a git repository
    pub fn is_git_repo(&self) -> bool {
        Command::new("git")
            .arg("rev-parse")
            .arg("--is-inside-work-tree")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    /// Get the current git repository root path
    pub fn repo_root(&self) -> Option<String> {
        Command::new("git")
            .arg("rev-parse")
            .arg("--show-toplevel")
            .output()
            .ok()
            .and_then(|output| {
                if output.status.success() {
                    String::from_utf8(output.stdout).ok()
                } else {
                    None
                }
            })
            .map(|s| s.trim().to_string())
    }

    /// Get current git branch
    pub fn current_branch(&self) -> Option<String> {
        Command::new("git")
            .arg("rev-parse")
            .arg("--abbrev-ref")
            .arg("HEAD")
            .output()
            .ok()
            .and_then(|output| {
                if output.status.success() {
                    String::from_utf8(output.stdout).ok()
                } else {
                    None
                }
            })
            .map(|s| s.trim().to_string())
    }

    /// Get recent git commits (limited by count)
    /// Format: hash:timestamp:author:subject
    pub fn recent_commits(&self, limit: usize) -> io::Result<Vec<String>> {
        let output = Command::new("git")
            .arg("log")
            .arg(format!("-{}", limit))
            .arg("--pretty=format:%h:%at:%an:%s")
            .output()?;

        if !output.status.success() {
            return Ok(Vec::new());
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut commits = Vec::new();

        for line in stdout.lines() {
            let line = line.trim();
            if !line.is_empty() {
                commits.push(line.to_string());
            }
        }

        Ok(commits)
    }

    /// Get git status (porcelain format for easy parsing)
    /// Reserved for Phase 3b: File change tracking
    #[allow(dead_code)]
    pub fn status(&self) -> io::Result<Vec<String>> {
        let output = Command::new("git")
            .arg("status")
            .arg("--porcelain")
            .output()?;

        if !output.status.success() {
            return Ok(Vec::new());
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut files = Vec::new();

        for line in stdout.lines() {
            let line = line.trim();
            if !line.is_empty() {
                // Extract filename (skip the status prefix)
                let parts: Vec<&str> = line.splitn(2, ' ').collect();
                if parts.len() == 2 {
                    files.push(parts[1].to_string());
                }
            }
        }

        Ok(files)
    }

    /// Get git diff stat (files changed)
    /// Reserved for Phase 3b: File change tracking
    #[allow(dead_code)]
    pub fn diff_stat(&self) -> io::Result<Vec<String>> {
        let output = Command::new("git")
            .arg("diff")
            .arg("--name-only")
            .output()?;

        if !output.status.success() {
            return Ok(Vec::new());
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut files = Vec::new();

        for line in stdout.lines() {
            let line = line.trim();
            if !line.is_empty() {
                files.push(line.to_string());
            }
        }

        Ok(files)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_git_repo() {
        let collector = GitCollector::new();
        // This will depend on whether we're in a git repo during test
        // Just ensuring it doesn't panic
        let _result = collector.is_git_repo();
    }
}