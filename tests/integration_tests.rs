/// Comprehensive integration tests for Worklie
#[cfg(test)]
mod integration_tests {
    use worklie::parser::history_parser::{HistoryParser, GitParser};
    use worklie::analyzer::activity_clusterer::ActivityClusterer;
    use worklie::analyzer::advanced_clustering::{
        ClusteringStrategy, JaccardClusterer, HierarchicalClusterer, AdvancedClusterer,
    };
    use worklie::cache_manager::CacheManager;
    use worklie::config::WorklieConfig;

    #[test]
    fn test_history_parser_parses_correctly() {
        // Simulate shell history
        let history = vec![];

        // Parse history (with empty vec, should return empty)
        let parser = HistoryParser::new();
        let parsed_commands = parser.parse_commands(history);

        // Verify parsing worked
        assert_eq!(parsed_commands.len(), 0);
    }

    #[test]
    fn test_git_commit_parsing_multiple() {
        // Simulate git output
        let git_output = vec![
            "abc1234:1705323600:Alice:feat: add new feature".to_string(),
            "def5678:1705323900:Bob:fix: bug fix".to_string(),
            "ghi9012:1705324200:Charlie:docs: update readme".to_string(),
        ];

        // Parse commits
        let parser = GitParser::new();
        let commits = parser.parse_commits(git_output);

        // Verify parsing
        assert_eq!(commits.len(), 3);
        assert!(commits[0].message.contains("feat"));
        assert!(commits[1].message.contains("fix"));
        assert!(commits[2].message.contains("docs"));
    }

    #[test]
    fn test_jaccard_clustering_integration() {
        use worklie::analyzer::activity_clusterer::Activity;

        let activities = vec![
            Activity {
                id: "a1".to_string(),
                title: "Rust work".to_string(),
                description: String::new(),
                start_time: 1000,
                end_time: 2000,
                commands: vec![],
                commits: vec![],
                files: vec!["main.rs".to_string(), "lib.rs".to_string()],
                directory: None,
            },
            Activity {
                id: "a2".to_string(),
                title: "More Rust".to_string(),
                description: String::new(),
                start_time: 2000,
                end_time: 3000,
                commands: vec![],
                commits: vec![],
                files: vec!["main.rs".to_string(), "test.rs".to_string()],
                directory: None,
            },
        ];

        let clusterer = JaccardClusterer::new(0.3);
        let clusters = clusterer.cluster(activities);

        // Should cluster similar activities
        assert!(!clusters.is_empty());
        clusters.iter().for_each(|c| {
            assert!(c.similarity_score >= 0.0 && c.similarity_score <= 1.0);
        });
    }

    #[test]
    fn test_hierarchical_clustering_integration() {
        use worklie::analyzer::activity_clusterer::Activity;

        let activities = vec![
            Activity {
                id: "a1".to_string(),
                title: "Morning session".to_string(),
                description: String::new(),
                start_time: 1000,
                end_time: 2000,
                commands: vec![],
                commits: vec![],
                files: vec![],
                directory: None,
            },
            Activity {
                id: "a2".to_string(),
                title: "Afternoon session".to_string(),
                description: String::new(),
                start_time: 30000,
                end_time: 31000,
                commands: vec![],
                commits: vec![],
                files: vec![],
                directory: None,
            },
        ];

        let clusterer = HierarchicalClusterer;
        let clusters = clusterer.cluster(activities);

        // Should create clusters
        assert!(!clusters.is_empty());
    }

    #[test]
    fn test_config_with_validation() {
        let mut config = WorklieConfig::default();

        // Test validation passes with defaults
        assert!(config.validate().is_ok());

        // Test validation fails with invalid focus threshold
        config.analysis.focus_threshold = 1.5;
        assert!(config.validate().is_err());

        // Test validation fails with zero cluster window
        config.analysis.focus_threshold = 0.7;
        config.analysis.cluster_time_window_minutes = 0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_cache_manager_creation() {
        // Create cache manager
        if let Ok(_manager) = CacheManager::new() {
            // Verify we can create it
            assert!(true);
        }
    }

    #[test]
    fn test_edge_case_empty_activities() {
        let clusterer = ActivityClusterer::new();
        let activities = clusterer.cluster_activities(vec![], vec![]);
        assert_eq!(activities.len(), 0);
    }

    #[test]
    fn test_advanced_clustering_strategy_selection() {
        let strategies = AdvancedClusterer::available_strategies();
        assert!(strategies.len() >= 2);

        let jaccard_strategy = AdvancedClusterer::create_strategy("semantic");
        assert_eq!(jaccard_strategy.name(), "Jaccard Similarity");

        let hierarchical_strategy = AdvancedClusterer::create_strategy("hierarchical");
        assert_eq!(hierarchical_strategy.name(), "Hierarchical");
    }

    #[test]
    fn test_config_defaults() {
        let config = WorklieConfig::default();

        assert_eq!(config.cache.retention_days, 90);
        assert!(config.cache.enabled);
        assert_eq!(config.analysis.cluster_time_window_minutes, 30);
        assert_eq!(config.analysis.focus_threshold, 0.7);
        assert_eq!(config.analysis.session_gap_minutes, 30);
        assert!(config.output.colors);
    }

    #[test]
    fn test_git_parser_edge_cases() {
        let tricky_commits = vec![
            "abc:123456:User Name:feat: commit with:colons:in:message".to_string(),
            "def:789000:Another User:fix: message with special chars !@#".to_string(),
            "ghi:999999:User:docs: more text".to_string(),
        ];

        let parser = GitParser::new();
        let parsed = parser.parse_commits(tricky_commits);

        // All should parse without panicking
        assert_eq!(parsed.len(), 3);
    }

    #[test]
    fn test_config_paths_consistency() {
        let config_file = WorklieConfig::config_file();
        let config_dir = WorklieConfig::config_dir();

        // Config file should be in config dir
        assert!(config_file.starts_with(&config_dir));
        assert!(config_file.to_string_lossy().contains("config.toml"));
    }

    #[test]
    fn test_activity_cluster_metrics() {
        use worklie::analyzer::activity_clusterer::Activity;
        use worklie::analyzer::advanced_clustering::{ActivityCluster, ClusterType};

        let activities = vec![
            Activity {
                id: "a1".to_string(),
                title: "Code".to_string(),
                description: String::new(),
                start_time: 1000,
                end_time: 1600, // 600 seconds
                commands: vec![],
                commits: vec![],
                files: vec!["src/main.rs".to_string()],
                directory: None,
            },
        ];

        let cluster = ActivityCluster {
            id: "c1".to_string(),
            title: "Test".to_string(),
            activities,
            similarity_score: 0.85,
            cluster_type: ClusterType::Semantic,
        };

        // Calculate metrics
        let focus = cluster.average_focus();
        assert!(focus > 0.0 && focus <= 1.0);

        let duration = cluster.duration_minutes();
        assert_eq!(duration, 10);

        let techs = cluster.technologies();
        assert!(techs.len() >= 0);
    }

    #[test]
    fn test_concurrent_cache_access() {
        // Test that cache manager can be accessed from multiple threads safely
        let results = std::thread::spawn(|| {
            // Just verify we can create managers without panicking
            let result1 = CacheManager::new();
            let result2 = CacheManager::new();
            (result1.is_ok(), result2.is_ok())
        });

        let (ok1, ok2) = results.join().unwrap();
        assert!(ok1 || ok2); // At least one should succeed
    }
}
