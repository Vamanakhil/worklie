/// Output formatting utilities for improved CLI readability

/// Terminal colors for output (basic ANSI support)
pub struct Colors;

impl Colors {
    pub const GREEN: &'static str = "\x1b[32m";
    pub const YELLOW: &'static str = "\x1b[33m";
    pub const CYAN: &'static str = "\x1b[36m";
    pub const RESET: &'static str = "\x1b[0m";
    pub const BOLD: &'static str = "\x1b[1m";
    pub const DIM: &'static str = "\x1b[2m";
    pub const BRIGHT_GREEN: &'static str = "\x1b[92m";
    pub const BRIGHT_YELLOW: &'static str = "\x1b[93m";
    pub const BRIGHT_BLUE: &'static str = "\x1b[94m";
}

/// Section formatter for consistent output structure
pub struct Section {
    title: String,
    content: Vec<String>,
}

impl Section {
    pub fn new(title: &str) -> Self {
        Section {
            title: title.to_string(),
            content: Vec::new(),
        }
    }

    pub fn add_line(&mut self, line: &str) {
        self.content.push(line.to_string());
    }

    pub fn add_key_value(&mut self, key: &str, value: &str) {
        self.content.push(format!("  {} {}", key, value));
    }

    pub fn add_formatted(&mut self, key: &str, value: &str, color: &str) {
        self.content.push(format!(
            "  {} {}{}{}",
            key, color, value, Colors::RESET
        ));
    }

    pub fn render(&self) -> String {
        let mut output = String::new();
        output.push_str(&format!(
            "{}{}{}\n",
            Colors::BOLD, self.title, Colors::RESET
        ));
        output.push_str(&"=".repeat(self.title.len()));
        output.push('\n');

        for line in &self.content {
            output.push_str(line);
            output.push('\n');
        }

        output
    }
}

/// Format a percentage with color coding
pub fn format_percentage(value: f64) -> String {
    let color = if value >= 0.7 {
        Colors::GREEN
    } else if value >= 0.4 {
        Colors::YELLOW
    } else {
        Colors::BRIGHT_YELLOW
    };

    format!("{}{:.1}%{}", color, value * 100.0, Colors::RESET)
}

/// Format a duration in human-readable format
pub fn format_duration_minutes(minutes: u64) -> String {
    let hours = minutes / 60;
    let mins = minutes % 60;

    if hours > 0 {
        format!("{}h {}m", hours, mins)
    } else {
        format!("{}m", mins)
    }
}

/// Format a metric with optional unit
pub struct MetricFormatter;

impl MetricFormatter {
    pub fn format_count(value: usize, unit: &str) -> String {
        if value == 1 {
            format!("{} {}", value, unit)
        } else {
            format!("{} {}s", value, unit)
        }
    }

    pub fn format_score(score: f64) -> String {
        let color = if score >= 0.7 {
            Colors::GREEN
        } else if score >= 0.4 {
            Colors::YELLOW
        } else {
            Colors::BRIGHT_YELLOW
        };
        format!("{}{:.2}{}", color, score, Colors::RESET)
    }

    pub fn format_trend(direction: &str, symbol: &str) -> String {
        match direction {
            "up" => format!("{}{} Up{}", Colors::GREEN, symbol, Colors::RESET),
            "down" => format!("{}{} Down{}", Colors::BRIGHT_YELLOW, symbol, Colors::RESET),
            _ => format!("{}{} Stable{}", Colors::CYAN, symbol, Colors::RESET),
        }
    }
}

/// Box drawing for enhanced UI
pub struct BoxStyle;

impl BoxStyle {
    pub fn separator() -> String {
        "─".repeat(50)
    }

    pub fn header_separator() -> String {
        "═".repeat(50)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_section_rendering() {
        let mut section = Section::new("Test Section");
        section.add_line("Line 1");
        section.add_key_value("Key:", "Value");
        let rendered = section.render();
        assert!(rendered.contains("Test Section"));
        assert!(rendered.contains("Line 1"));
        assert!(rendered.contains("Key:"));
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration_minutes(65), "1h 5m");
        assert_eq!(format_duration_minutes(45), "45m");
    }

    #[test]
    fn test_metric_formatter() {
        assert_eq!(MetricFormatter::format_count(1, "file"), "1 file");
        assert_eq!(MetricFormatter::format_count(5, "file"), "5 files");
    }
}
