/// Advanced clustering algorithms for activity analysis
/// Implements Jaccard similarity and semantic grouping strategies
use std::collections::{HashMap, HashSet};
use crate::analyzer::activity_clusterer::Activity;

/// Trait for clustering strategies
pub trait ClusteringStrategy {
    fn cluster(&self, activities: Vec<Activity>) -> Vec<ActivityCluster>;
    fn name(&self) -> &'static str;
}

/// Clustered group of related activities
#[derive(Debug, Clone)]
pub struct ActivityCluster {
    pub id: String,
    pub title: String,
    pub activities: Vec<Activity>,
    pub similarity_score: f64, // 0.0-1.0
    pub cluster_type: ClusterType,
}

/// Type of clustering applied
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClusterType {
    /// Time-window based (default, fast)
    TimeWindow,
    /// Jaccard similarity based (more accurate)
    Semantic,
    /// Hierarchical clustering
    Hierarchical,
}

impl ActivityCluster {
    /// Calculate average focus score for cluster
    pub fn average_focus(&self) -> f64 {
        if self.activities.is_empty() {
            return 0.0;
        }
        let total: f64 = self
            .activities
            .iter()
            .map(|a| {
                let time_diff = a.end_time.saturating_sub(a.start_time);
                // More continuous activity = higher focus
                if time_diff > 300 {
                    0.8
                } else if time_diff > 60 {
                    0.6
                } else {
                    0.3
                }
            })
            .sum();
        total / self.activities.len() as f64
    }

    /// Get primary technologies used
    pub fn technologies(&self) -> Vec<String> {
        let mut tech_map: HashMap<String, usize> = HashMap::new();

        for activity in &self.activities {
            for file in &activity.files {
                if let Some(ext) = file.split('.').last() {
                    let tech = match ext {
                        "js" | "jsx" | "ts" | "tsx" => "JavaScript",
                        "py" => "Python",
                        "rs" => "Rust",
                        "go" => "Go",
                        "java" => "Java",
                        "cpp" | "cc" | "c" => "C++",
                        "rb" => "Ruby",
                        "php" => "PHP",
                        _ => ext,
                    };
                    *tech_map.entry(tech.to_string()).or_insert(0) += 1;
                }
            }
        }

        let mut techs: Vec<_> = tech_map.into_iter().collect();
        techs.sort_by(|a, b| b.1.cmp(&a.1));
        techs.into_iter().take(3).map(|(tech, _)| tech).collect()
    }

    /// Get total duration in minutes
    pub fn duration_minutes(&self) -> u64 {
        self.activities
            .iter()
            .map(|a| a.end_time.saturating_sub(a.start_time) / 60)
            .sum()
    }
}

/// Jaccard similarity based clustering
pub struct JaccardClusterer {
    /// Similarity threshold (0.0-1.0)
    pub threshold: f64,
}

impl JaccardClusterer {
    pub fn new(threshold: f64) -> Self {
        JaccardClusterer {
            threshold: threshold.clamp(0.0, 1.0),
        }
    }

    /// Calculate Jaccard similarity between two activities
    fn jaccard_similarity(a1: &Activity, a2: &Activity) -> f64 {
        let set1: HashSet<_> = a1.files.iter().collect();
        let set2: HashSet<_> = a2.files.iter().collect();

        if set1.is_empty() && set2.is_empty() {
            return 1.0;
        }

        let intersection = set1.intersection(&set2).count();
        let union = set1.union(&set2).count();

        if union == 0 {
            0.0
        } else {
            intersection as f64 / union as f64
        }
    }

    /// Build semantic clusters based on file similarity
    fn build_clusters_from_similarity(&self, activities: Vec<Activity>) -> Vec<Vec<Activity>> {
        let mut clusters: Vec<Vec<Activity>> = Vec::new();

        for activity in activities {
            let mut added = false;

            // Try to add to existing cluster
            for cluster in &mut clusters {
                for existing in cluster.iter() {
                    if Self::jaccard_similarity(&activity, existing) > self.threshold {
                        cluster.push(activity.clone());
                        added = true;
                        break;
                    }
                }
                if added {
                    break;
                }
            }

            // Create new cluster if not similar to any existing
            if !added {
                clusters.push(vec![activity]);
            }
        }

        clusters
    }
}

impl ClusteringStrategy for JaccardClusterer {
    fn cluster(&self, activities: Vec<Activity>) -> Vec<ActivityCluster> {
        let clusters = self.build_clusters_from_similarity(activities);

        clusters
            .into_iter()
            .enumerate()
            .map(|(idx, activities)| {
                // Calculate average similarity within cluster
                let mut similarities = vec![];
                for (i, a1) in activities.iter().enumerate() {
                    for a2 in activities.iter().skip(i + 1) {
                        similarities.push(Self::jaccard_similarity(a1, a2));
                    }
                }

                let avg_similarity = if similarities.is_empty() {
                    1.0
                } else {
                    similarities.iter().sum::<f64>() / similarities.len() as f64
                };

                let primary_files: HashSet<_> = activities
                    .iter()
                    .flat_map(|a| a.files.iter())
                    .collect();
                let title = if primary_files.is_empty() {
                    format!("Activity Group {}", idx)
                } else {
                    format!("Work on {}", primary_files.iter().next().unwrap())
                };

                ActivityCluster {
                    id: format!("semantic-{}", idx),
                    title,
                    activities,
                    similarity_score: avg_similarity,
                    cluster_type: ClusterType::Semantic,
                }
            })
            .collect()
    }

    fn name(&self) -> &'static str {
        "Jaccard Similarity"
    }
}

/// Hierarchical clustering strategy
pub struct HierarchicalClusterer;

impl HierarchicalClusterer {
    /// Perform hierarchical clustering with distance threshold
    pub fn cluster_hierarchical(
        activities: Vec<Activity>,
        distance_threshold: f64,
    ) -> Vec<Vec<Activity>> {
        if activities.is_empty() {
            return Vec::new();
        }

        let mut clusters: Vec<Vec<Activity>> = activities.into_iter().map(|a| vec![a]).collect();

        // Merge clusters while distance is below threshold
        loop {
            let mut best_merge = None;
            let mut best_distance = distance_threshold;

            // Find closest pair of clusters
            for i in 0..clusters.len() {
                for j in (i + 1)..clusters.len() {
                    if let Some(dist) = Self::cluster_distance(&clusters[i], &clusters[j]) {
                        if dist < best_distance {
                            best_distance = dist;
                            best_merge = Some((i, j));
                        }
                    }
                }
            }

            if let Some((i, j)) = best_merge {
                // Merge clusters (remove j first since j > i)
                let mut cluster_j = clusters.remove(j);
                clusters[i].append(&mut cluster_j);
            } else {
                break;
            }
        }

        clusters
    }

    /// Calculate distance between two clusters (average linkage)
    fn cluster_distance(c1: &[Activity], c2: &[Activity]) -> Option<f64> {
        if c1.is_empty() || c2.is_empty() {
            return None;
        }

        let mut distances = vec![];
        for a1 in c1 {
            for a2 in c2 {
                let time_dist = ((a1.end_time as i64 - a2.start_time as i64).abs() as f64).ln();
                distances.push(time_dist);
            }
        }

        if distances.is_empty() {
            None
        } else {
            Some(distances.iter().sum::<f64>() / distances.len() as f64)
        }
    }
}

impl ClusteringStrategy for HierarchicalClusterer {
    fn cluster(&self, activities: Vec<Activity>) -> Vec<ActivityCluster> {
        let clusters = Self::cluster_hierarchical(activities, 5.0);

        clusters
            .into_iter()
            .enumerate()
            .map(|(idx, activities)| ActivityCluster {
                id: format!("hierarchical-{}", idx),
                title: format!("Work Session {}", idx + 1),
                activities,
                similarity_score: 0.75,
                cluster_type: ClusterType::Hierarchical,
            })
            .collect()
    }

    fn name(&self) -> &'static str {
        "Hierarchical"
    }
}

/// Clustering algorithm manager
pub struct AdvancedClusterer;

impl AdvancedClusterer {
    /// Get available strategies
    pub fn available_strategies() -> Vec<&'static str> {
        vec!["time-window", "semantic", "hierarchical"]
    }

    /// Create appropriate clusterer based on strategy name
    pub fn create_strategy(strategy: &str) -> Box<dyn ClusteringStrategy> {
        match strategy.to_lowercase().as_str() {
            "semantic" | "jaccard" => Box::new(JaccardClusterer::new(0.3)),
            "hierarchical" => Box::new(HierarchicalClusterer),
            _ => Box::new(JaccardClusterer::new(0.5)), // Default similarity-based
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jaccard_similarity() {
        let mut a1 = Activity {
            id: "a1".to_string(),
            title: "Test 1".to_string(),
            description: String::new(),
            start_time: 0,
            end_time: 100,
            commands: vec![],
            commits: vec![],
            files: vec!["file1.rs".to_string(), "file2.rs".to_string()],
            directory: None,
        };

        let mut a2 = Activity {
            id: "a2".to_string(),
            title: "Test 2".to_string(),
            description: String::new(),
            start_time: 100,
            end_time: 200,
            commands: vec![],
            commits: vec![],
            files: vec!["file1.rs".to_string(), "file3.rs".to_string()],
            directory: None,
        };

        let similarity = JaccardClusterer::jaccard_similarity(&a1, &a2);
        // Intersection: {file1.rs} = 1, Union: {file1, file2, file3} = 3
        // Similarity = 1/3 ≈ 0.333
        assert!(similarity > 0.3 && similarity < 0.4);
    }

    #[test]
    fn test_cluster_technologies() {
        let cluster = ActivityCluster {
            id: "test".to_string(),
            title: "Test Cluster".to_string(),
            activities: vec![],
            similarity_score: 0.8,
            cluster_type: ClusterType::Semantic,
        };

        let techs = cluster.technologies();
        assert_eq!(techs.len(), 0); // No activities, no technologies
    }

    #[test]
    fn test_available_strategies() {
        let strategies = AdvancedClusterer::available_strategies();
        assert!(strategies.contains(&"semantic"));
        assert!(strategies.contains(&"hierarchical"));
    }
}
