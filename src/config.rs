/// Configuration management for Worklie
use anyhow::{Result, anyhow};
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use std::fs;

/// Worklie configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorklieConfig {
    /// Cache management
    pub cache: CacheConfig,
    /// Analysis parameters
    pub analysis: AnalysisConfig,
    /// Output preferences
    pub output: OutputConfig,
}

/// Cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// Days to retain cache data (default: 90)
    pub retention_days: u32,
    /// Enable cache persistence (default: true)
    pub enabled: bool,
    /// Cache location (default: ~/.worklie/cache.db)
    #[serde(skip)]
    pub location: PathBuf,
}

/// Analysis configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisConfig {
    /// Time window for activity clustering in minutes (default: 30)
    pub cluster_time_window_minutes: u64,
    /// Focus score threshold for "deep work" (0.0-1.0, default: 0.7)
    pub focus_threshold: f64,
    /// Session gap threshold in minutes (default: 30)
    pub session_gap_minutes: u64,
    /// Maximum number of commits to analyze (0 = unlimited, default: 0)
    pub max_commits_per_command: usize,
}

/// Output configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    /// Enable colored output (default: true)
    pub colors: bool,
    /// Show progress indicators (default: true)
    pub progress: bool,
    /// Maximum output width in characters (0 = auto-detect, default: 0)
    pub max_width: usize,
    /// Show timestamps in logs (default: false)
    pub show_timestamps: bool,
}

impl Default for WorklieConfig {
    fn default() -> Self {
        let config_home = Self::config_dir();
        let cache_location = config_home.join("cache.db");

        WorklieConfig {
            cache: CacheConfig {
                retention_days: 90,
                enabled: true,
                location: cache_location,
            },
            analysis: AnalysisConfig {
                cluster_time_window_minutes: 30,
                focus_threshold: 0.7,
                session_gap_minutes: 30,
                max_commits_per_command: 0,
            },
            output: OutputConfig {
                colors: true,
                progress: true,
                max_width: 0,
                show_timestamps: false,
            },
        }
    }
}

impl WorklieConfig {
    /// Get or create configuration directory
    pub fn config_dir() -> PathBuf {
        let home = dirs::home_dir().expect("Could not determine home directory");
        home.join(".worklie")
    }

    /// Get configuration file path
    pub fn config_file() -> PathBuf {
        Self::config_dir().join("config.toml")
    }

    /// Load configuration from file, or use defaults
    pub fn load() -> Result<Self> {
        let config_file = Self::config_file();

        if config_file.exists() {
            let content = fs::read_to_string(&config_file)?;
            let mut config: WorklieConfig = toml::from_str(&content)?;
            // Ensure cache location is set
            if config.cache.location.as_os_str().is_empty() {
                config.cache.location = Self::config_dir().join("cache.db");
            }
            Ok(config)
        } else {
            // Create default configuration
            let config = Self::default();
            let _ = config.save(); // Try to save, but don't fail if unable
            Ok(config)
        }
    }

    /// Save configuration to file
    pub fn save(&self) -> Result<()> {
        let config_dir = Self::config_dir();
        fs::create_dir_all(&config_dir)?;

        let config_file = Self::config_file();
        let content = toml::to_string_pretty(self)?;
        fs::write(&config_file, content)?;

        Ok(())
    }

    /// Validate configuration values with detailed error messages and cross-field checks
    pub fn validate(&self) -> Result<()> {
        // Focus threshold validation
        if self.analysis.focus_threshold < 0.0 || self.analysis.focus_threshold > 1.0 {
            return Err(anyhow!(
                "focus_threshold must be between 0.0 and 1.0, got {}. \
                 This parameter controls the minimum score for deep work detection.",
                self.analysis.focus_threshold
            ));
        }

        // Cluster time window validation
        if self.analysis.cluster_time_window_minutes == 0 {
            return Err(anyhow!(
                "cluster_time_window_minutes must be greater than 0. \
                 This parameter controls how activities are grouped in time."
            ));
        }

        if self.analysis.cluster_time_window_minutes > 1440 {
            return Err(anyhow!(
                "cluster_time_window_minutes should not exceed 1440 (24 hours), got {}",
                self.analysis.cluster_time_window_minutes
            ));
        }

        // Session gap validation
        if self.analysis.session_gap_minutes == 0 {
            return Err(anyhow!(
                "session_gap_minutes must be greater than 0. \
                 This parameter controls session boundaries."
            ));
        }

        if self.analysis.session_gap_minutes > 1440 {
            return Err(anyhow!(
                "session_gap_minutes should not exceed 1440 (24 hours), got {}",
                self.analysis.session_gap_minutes
            ));
        }

        // Cross-field validation
        if self.analysis.session_gap_minutes < self.analysis.cluster_time_window_minutes {
            return Err(anyhow!(
                "session_gap_minutes ({}) should be >= cluster_time_window_minutes ({}). \
                 Session gaps should typically be larger than clustering windows.",
                self.analysis.session_gap_minutes,
                self.analysis.cluster_time_window_minutes
            ));
        }

        // Cache validation
        if self.cache.retention_days == 0 {
            return Err(anyhow!(
                "retention_days must be greater than 0. \
                 Cache retention must be at least 1 day."
            ));
        }

        if self.cache.retention_days > 36500 {
            return Err(anyhow!(
                "retention_days should not exceed 36500 (100 years), got {}",
                self.cache.retention_days
            ));
        }

        // Output max_width validation
        if self.output.max_width > 0 && self.output.max_width < 40 {
            return Err(anyhow!(
                "max_width should be either 0 (auto) or at least 40, got {}",
                self.output.max_width
            ));
        }

        Ok(())
    }

    /// Reset to defaults
    pub fn reset_to_defaults() -> Result<()> {
        let config = Self::default();
        config.save()?;
        println!("Configuration reset to defaults at: {}", Self::config_file().display());
        Ok(())
    }

    /// Print current configuration
    pub fn print(&self) {
        println!("Worklie Configuration");
        println!("═══════════════════════════════════════");
        println!("\n[cache]");
        println!("  retention_days = {}", self.cache.retention_days);
        println!("  enabled = {}", self.cache.enabled);

        println!("\n[analysis]");
        println!("  cluster_time_window_minutes = {}", self.analysis.cluster_time_window_minutes);
        println!("  focus_threshold = {}", self.analysis.focus_threshold);
        println!("  session_gap_minutes = {}", self.analysis.session_gap_minutes);
        println!("  max_commits_per_command = {}", self.analysis.max_commits_per_command);

        println!("\n[output]");
        println!("  colors = {}", self.output.colors);
        println!("  progress = {}", self.output.progress);
        println!("  max_width = {}", self.output.max_width);
        println!("  show_timestamps = {}", self.output.show_timestamps);

        println!("\nConfiguration file: {}", Self::config_file().display());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = WorklieConfig::default();
        assert_eq!(config.cache.retention_days, 90);
        assert_eq!(config.analysis.focus_threshold, 0.7);
        assert_eq!(config.analysis.cluster_time_window_minutes, 30);
    }

    #[test]
    fn test_config_validation() {
        let mut config = WorklieConfig::default();
        assert!(config.validate().is_ok());

        config.analysis.focus_threshold = 1.5;
        assert!(config.validate().is_err());

        config.analysis.focus_threshold = 0.7;
        config.analysis.cluster_time_window_minutes = 0;
        assert!(config.validate().is_err());

        // Test cross-field validation
        config.analysis.cluster_time_window_minutes = 60;
        config.analysis.session_gap_minutes = 30;
        assert!(config.validate().is_err()); // session_gap < cluster_time_window

        config.analysis.session_gap_minutes = 60;
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_session_gap_validation() {
        let mut config = WorklieConfig::default();
        config.analysis.session_gap_minutes = 0;
        assert!(config.validate().is_err());

        config.analysis.session_gap_minutes = 1500; // > 1440
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_retention_days_validation() {
        let mut config = WorklieConfig::default();
        config.cache.retention_days = 0;
        assert!(config.validate().is_err());

        config.cache.retention_days = 36501; // > 36500
        assert!(config.validate().is_err());

        config.cache.retention_days = 90;
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_max_width_validation() {
        let mut config = WorklieConfig::default();

        // max_width = 0 (auto) is valid
        config.output.max_width = 0;
        assert!(config.validate().is_ok());

        // max_width = 30 (< 40) is invalid
        config.output.max_width = 30;
        assert!(config.validate().is_err());

        // max_width = 100 is valid
        config.output.max_width = 100;
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_paths() {
        let config_dir = WorklieConfig::config_dir();
        let config_file = WorklieConfig::config_file();
        assert!(config_file.ends_with("config.toml"));
        assert!(config_dir.to_string_lossy().contains(".worklie"));
    }
}
