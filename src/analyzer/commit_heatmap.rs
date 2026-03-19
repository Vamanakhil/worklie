/// Commit activity heatmap for time-based productivity visualization
use std::collections::HashMap;
use crate::parser::history_parser::ParsedCommit;
use crate::output_formatter::Colors;
use chrono::Timelike;

/// Represents activity intensity at a specific time
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TimeSlot {
    pub day_of_week: u32, // 0=Monday, 6=Sunday
    pub hour: u32,        // 0-23
}

/// Heatmap data for visualization
#[derive(Debug, Clone)]
pub struct CommitHeatmap {
    /// Map of time slot to activity count
    pub slots: HashMap<TimeSlot, usize>,
    /// Max activity count (for color scaling)
    pub max_activity: usize,
}

impl CommitHeatmap {
    /// Generate heatmap from commits
    pub fn from_commits(commits: &[ParsedCommit]) -> Self {
        let mut slots: HashMap<TimeSlot, usize> = HashMap::new();
        let mut max_activity = 0;

        for commit in commits {
            // Try to get day of week and hour from timestamp
            if let Some((dow, hour)) = Self::extract_time_info(commit.timestamp) {
                let slot = TimeSlot {
                    day_of_week: dow,
                    hour,
                };
                let count = slots.entry(slot).or_insert(0);
                *count += 1;
                if *count > max_activity {
                    max_activity = *count;
                }
            }
        }

        CommitHeatmap {
            slots,
            max_activity: max_activity.max(1), // Ensure at least 1 to prevent division by zero
        }
    }

    /// Extract day of week and hour from timestamp
    fn extract_time_info(timestamp: u64) -> Option<(u32, u32)> {
        use chrono::{DateTime, Local, Datelike, Weekday};
        use std::time::UNIX_EPOCH;

        let duration = std::time::Duration::from_secs(timestamp);
        if let Some(time) = UNIX_EPOCH.checked_add(duration) {
            let datetime = DateTime::<Local>::from(time);
            let weekday = datetime.weekday();
            let dow = match weekday {
                Weekday::Mon => 0,
                Weekday::Tue => 1,
                Weekday::Wed => 2,
                Weekday::Thu => 3,
                Weekday::Fri => 4,
                Weekday::Sat => 5,
                Weekday::Sun => 6,
            };
            let hour = datetime.hour();
            return Some((dow, hour));
        }
        None
    }

    /// Get color intensity for a count (0-3: no activity, low, medium, high)
    fn get_intensity_level(count: usize, max: usize) -> usize {
        if count == 0 {
            0
        } else if count <= max / 3 {
            1
        } else if count <= (2 * max) / 3 {
            2
        } else {
            3
        }
    }

    /// Render heatmap as ASCII art with colors
    pub fn render(&self) -> String {
        let mut output = String::new();
        output.push_str(&format!(
            "{}Commit Activity Heatmap{}\n",
            Colors::BOLD, Colors::RESET
        ));
        output.push_str(&"═".repeat(70));
        output.push('\n');

        let days = vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

        // Hours header
        output.push_str("     ");
        for hour in 0..24 {
            if hour % 3 == 0 {
                output.push_str(&format!("{:>2} ", hour));
            } else {
                output.push_str("   ");
            }
        }
        output.push('\n');

        // Heatmap rows
        for (day_idx, day_name) in days.iter().enumerate() {
            output.push_str(&format!("{:>3} ", day_name));

            for hour in 0..24 {
                let slot = TimeSlot {
                    day_of_week: day_idx as u32,
                    hour: hour as u32,
                };

                let count = self.slots.get(&slot).copied().unwrap_or(0);
                let intensity = Self::get_intensity_level(count, self.max_activity);
                let block = self.render_block(intensity);
                output.push_str(&block);
            }

            output.push('\n');
        }

        // Legend
        output.push_str("     ");
        output.push_str(&format!(
            "{}■ No activity  {}■ Low  {}■ Medium  {}■ High{}",
            Colors::DIM, Colors::BRIGHT_BLUE, Colors::BRIGHT_YELLOW, Colors::GREEN, Colors::RESET
        ));
        output.push('\n');

        output
    }

    /// Render a single heatmap block with color
    fn render_block(&self, intensity: usize) -> String {
        let color = match intensity {
            0 => Colors::DIM,
            1 => Colors::BRIGHT_BLUE,
            2 => Colors::BRIGHT_YELLOW,
            3 => Colors::GREEN,
            _ => Colors::BRIGHT_GREEN,
        };
        format!("{}█{} ", color, Colors::RESET)
    }

    /// Get peak hours (most productive times)
    pub fn peak_hours(&self) -> Vec<(u32, usize)> {
        let mut hourly_totals: HashMap<u32, usize> = HashMap::new();

        for (slot, count) in &self.slots {
            *hourly_totals.entry(slot.hour).or_insert(0) += count;
        }

        let mut peaks: Vec<(u32, usize)> = hourly_totals.into_iter().collect();
        peaks.sort_by(|a, b| b.1.cmp(&a.1));
        peaks.truncate(5);
        peaks
    }

    /// Get most active days
    pub fn peak_days(&self) -> Vec<(String, usize)> {
        let days = vec![
            "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday",
        ];
        let mut daily_totals: HashMap<u32, usize> = HashMap::new();

        for (slot, count) in &self.slots {
            *daily_totals.entry(slot.day_of_week).or_insert(0) += count;
        }

        let mut peaks: Vec<(String, usize)> = daily_totals
            .into_iter()
            .map(|(dow, count)| (days[dow as usize].to_string(), count))
            .collect();

        peaks.sort_by(|a, b| b.1.cmp(&a.1));
        peaks
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heatmap_from_empty_commits() {
        let heatmap = CommitHeatmap::from_commits(&[]);
        assert_eq!(heatmap.slots.len(), 0);
        assert_eq!(heatmap.max_activity, 1);
    }

    #[test]
    fn test_intensity_levels() {
        assert_eq!(CommitHeatmap::get_intensity_level(0, 10), 0);
        assert_eq!(CommitHeatmap::get_intensity_level(2, 10), 1);
        assert_eq!(CommitHeatmap::get_intensity_level(5, 10), 2);
        assert_eq!(CommitHeatmap::get_intensity_level(10, 10), 3);
    }

    #[test]
    fn test_heatmap_render_no_panic() {
        let heatmap = CommitHeatmap::from_commits(&[]);
        let rendered = heatmap.render();
        assert!(rendered.contains("Commit Activity Heatmap"));
        assert!(rendered.contains("Mon"));
        assert!(rendered.contains("Sun"));
    }
}
