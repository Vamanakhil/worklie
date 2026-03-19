use anyhow::{anyhow, Result};
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use crate::analyzer::activity_clusterer::Activity;
use crate::parser::history_parser::ParsedCommit;

/// Cached daily activity snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailySnapshot {
    pub timestamp: u64,           // Unix timestamp of when recorded
    pub date: String,             // YYYY-MM-DD format
    pub repo_path: String,
    #[serde(skip)]
    pub activities: Vec<Activity>,
    #[serde(skip)]
    pub parsed_commits: Vec<ParsedCommit>,
}

/// Persistent cache manager using SQLite
pub struct CacheManager {
    db_path: PathBuf,
    conn: Connection,
}

impl CacheManager {
    /// Create or connect to cache database
    pub fn new() -> Result<Self> {
        let cache_dir = Self::get_cache_dir()?;

        // Create cache directory if it doesn't exist
        if !cache_dir.exists() {
            std::fs::create_dir_all(&cache_dir)
                .map_err(|e| anyhow!("Failed to create cache directory: {}", e))?;
        }

        let db_path = cache_dir.join("worklie.db");
        let conn = Connection::open(&db_path)
            .map_err(|e| anyhow!("Failed to open database: {}", e))?;

        let manager = CacheManager { db_path, conn };
        manager.init_schema()?;

        Ok(manager)
    }

    /// Initialize database schema
    fn init_schema(&self) -> Result<()> {
        self.conn
            .execute_batch(
                "CREATE TABLE IF NOT EXISTS snapshots (
                    id INTEGER PRIMARY KEY,
                    repo_path TEXT NOT NULL,
                    date TEXT NOT NULL,
                    timestamp INTEGER NOT NULL,
                    activities_json TEXT NOT NULL,
                    commits_json TEXT NOT NULL,
                    created_at INTEGER NOT NULL,
                    UNIQUE(repo_path, date)
                );

                CREATE INDEX IF NOT EXISTS idx_repo_date
                    ON snapshots(repo_path, date);

                CREATE INDEX IF NOT EXISTS idx_repo_timestamp
                    ON snapshots(repo_path, timestamp);",
            )
            .map_err(|e| anyhow!("Failed to initialize database schema: {}", e))?;

        Ok(())
    }

    /// Save a daily snapshot
    pub fn save_day_snapshot(&mut self, snapshot: DailySnapshot) -> Result<()> {
        let activities_json = serde_json::to_string(&snapshot.activities)
            .map_err(|e| anyhow!("Failed to serialize activities: {}", e))?;

        let commits_json = serde_json::to_string(&snapshot.parsed_commits)
            .map_err(|e| anyhow!("Failed to serialize commits: {}", e))?;

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        self.conn
            .execute(
                "INSERT OR REPLACE INTO snapshots
                (repo_path, date, timestamp, activities_json, commits_json, created_at)
                VALUES (?, ?, ?, ?, ?, ?)",
                params![
                    &snapshot.repo_path,
                    &snapshot.date,
                    snapshot.timestamp,
                    activities_json,
                    commits_json,
                    now
                ],
            )
            .map_err(|e| anyhow!("Failed to save snapshot: {}", e))?;

        Ok(())
    }

    /// Load repository history for specified number of days
    pub fn load_repository_history(&self, repo_path: &str, days: u32) -> Result<Vec<DailySnapshot>> {
        // Calculate timestamp for cutoff date
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let cutoff_timestamp = now - (days as u64 * 86400);

        let mut stmt = self.conn
            .prepare(
                "SELECT date, timestamp, repo_path, activities_json, commits_json
                 FROM snapshots
                 WHERE repo_path = ? AND timestamp >= ?
                 ORDER BY timestamp DESC"
            )
            .map_err(|e| anyhow!("Failed to prepare query: {}", e))?;

        let snapshots = stmt
            .query_map(params![repo_path, cutoff_timestamp], |row| {
                let activities_json: String = row.get(3)?;
                let commits_json: String = row.get(4)?;

                let activities = serde_json::from_str(&activities_json)
                    .unwrap_or_default();
                let parsed_commits = serde_json::from_str(&commits_json)
                    .unwrap_or_default();

                Ok(DailySnapshot {
                    date: row.get(0)?,
                    timestamp: row.get(1)?,
                    repo_path: row.get(2)?,
                    activities,
                    parsed_commits,
                })
            })
            .map_err(|e| anyhow!("Failed to query snapshots: {}", e))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| anyhow!("Failed to collect snapshots: {}", e))?;

        Ok(snapshots)
    }

    /// Load latest snapshot for a repository
    pub fn load_latest_snapshot(&self, repo_path: &str) -> Result<Option<DailySnapshot>> {
        let mut stmt = self.conn
            .prepare(
                "SELECT date, timestamp, repo_path, activities_json, commits_json
                 FROM snapshots
                 WHERE repo_path = ?
                 ORDER BY timestamp DESC
                 LIMIT 1"
            )
            .map_err(|e| anyhow!("Failed to prepare query: {}", e))?;

        let snapshot = stmt
            .query_row(params![repo_path], |row| {
                let activities_json: String = row.get(3)?;
                let commits_json: String = row.get(4)?;

                let activities = serde_json::from_str(&activities_json)
                    .unwrap_or_default();
                let parsed_commits = serde_json::from_str(&commits_json)
                    .unwrap_or_default();

                Ok(DailySnapshot {
                    date: row.get(0)?,
                    timestamp: row.get(1)?,
                    repo_path: row.get(2)?,
                    activities,
                    parsed_commits,
                })
            })
            .optional()
            .map_err(|e| anyhow!("Failed to query latest snapshot: {}", e))?;

        Ok(snapshot)
    }

    /// Get last update timestamp for a repository
    pub fn get_last_update_timestamp(&self, repo_path: &str) -> Result<Option<u64>> {
        let mut stmt = self.conn
            .prepare(
                "SELECT MAX(timestamp) FROM snapshots WHERE repo_path = ?"
            )
            .map_err(|e| anyhow!("Failed to prepare query: {}", e))?;

        let timestamp: Option<u64> = stmt
            .query_row(params![repo_path], |row| row.get(0))
            .optional()
            .map_err(|e| anyhow!("Failed to query timestamp: {}", e))?
            .flatten();

        Ok(timestamp)
    }

    /// Clean up old snapshots (keep last N days)
    pub fn cleanup_old_snapshots(&mut self, days_to_keep: u32) -> Result<()> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let cutoff_timestamp = now - (days_to_keep as u64 * 86400);

        self.conn
            .execute(
                "DELETE FROM snapshots WHERE timestamp < ?",
                params![cutoff_timestamp],
            )
            .map_err(|e| anyhow!("Failed to cleanup snapshots: {}", e))?;

        Ok(())
    }

    /// Get list of all repositories in cache
    pub fn get_repository_list(&self) -> Result<Vec<String>> {
        let mut stmt = self.conn
            .prepare("SELECT DISTINCT repo_path FROM snapshots ORDER BY repo_path")
            .map_err(|e| anyhow!("Failed to prepare query: {}", e))?;

        let repos = stmt
            .query_map([], |row| row.get(0))
            .map_err(|e| anyhow!("Failed to query repositories: {}", e))?
            .collect::<Result<Vec<String>, _>>()
            .map_err(|e| anyhow!("Failed to collect repositories: {}", e))?;

        Ok(repos)
    }

    /// Get cache directory path
    fn get_cache_dir() -> Result<PathBuf> {
        let home = dirs::home_dir()
            .ok_or_else(|| anyhow!("Could not determine home directory"))?;

        Ok(home.join(".worklie").join("cache"))
    }

    /// Clear entire cache (for testing/reset)
    #[allow(dead_code)]
    pub fn clear_all(&mut self) -> Result<()> {
        self.conn
            .execute("DELETE FROM snapshots", [])
            .map_err(|e| anyhow!("Failed to clear cache: {}", e))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_manager_creation() {
        let manager = CacheManager::new();
        assert!(manager.is_ok());
    }

    #[test]
    fn test_save_and_load_snapshot() {
        let mut manager = CacheManager::new().unwrap();

        let snapshot = DailySnapshot {
            timestamp: 1234567890,
            date: "2026-03-18".to_string(),
            repo_path: "/test/repo".to_string(),
            activities: vec![],
            parsed_commits: vec![],
        };

        let result = manager.save_day_snapshot(snapshot.clone());
        assert!(result.is_ok());

        let loaded = manager.load_latest_snapshot("/test/repo").unwrap();
        assert!(loaded.is_some());
        assert_eq!(loaded.unwrap().date, "2026-03-18");

        // Cleanup
        manager.clear_all().unwrap();
    }

    #[test]
    fn test_cleanup_old_snapshots() {
        let mut manager = CacheManager::new().unwrap();

        // Add old snapshot
        let mut old_snapshot = DailySnapshot {
            timestamp: 100,  // Very old
            date: "2020-01-01".to_string(),
            repo_path: "/test/repo".to_string(),
            activities: vec![],
            parsed_commits: vec![],
        };
        manager.save_day_snapshot(old_snapshot).unwrap();

        // Cleanup snapshots older than 1 day
        let result = manager.cleanup_old_snapshots(1);
        assert!(result.is_ok());

        // Cleanup
        manager.clear_all().unwrap();
    }
}
