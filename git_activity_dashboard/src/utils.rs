//! Shared utility functions
//!
//! Common formatting, sorting, and helper functions used across modules.

use std::collections::HashMap;

/// Format a number with thousand separators
pub fn format_number(n: u32) -> String {
    if n >= 1_000_000 {
        format!("{:.1}M", n as f64 / 1_000_000.0)
    } else if n >= 1_000 {
        format!("{:.1}K", n as f64 / 1_000.0)
    } else {
        n.to_string()
    }
}

/// Format a large number with full thousand separators (e.g., 1,234,567)
pub fn format_number_full(n: u32) -> String {
    let s = n.to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    result.chars().rev().collect()
}

/// Get human-readable label for a contribution type key
pub fn contribution_type_label(ctype: &str) -> &'static str {
    match ctype {
        "productioncode" => "Production Code",
        "tests" => "Tests",
        "documentation" => "Documentation",
        "specsconfig" => "Specs & Config",
        "infrastructure" => "Infrastructure",
        "styling" => "Styling",
        "buildartifacts" => "Build Artifacts",
        "assets" => "Assets",
        "generated" => "Generated",
        "data" => "Data",
        "other" => "Other",
        _ => "Other",
    }
}

/// Sort a HashMap by value in descending order
pub fn sort_by_value<K: Clone>(map: &HashMap<K, u32>) -> Vec<(K, u32)> {
    let mut sorted: Vec<_> = map.iter().map(|(k, v)| (k.clone(), *v)).collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));
    sorted
}

/// Sort a HashMap by value and return references (no cloning)
pub fn sort_by_value_ref<K>(map: &HashMap<K, u32>) -> Vec<(&K, &u32)> {
    let mut sorted: Vec<_> = map.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1));
    sorted
}

/// Calculate percentage breakdown from a HashMap
pub fn calculate_percentages<K: Clone + std::hash::Hash + Eq>(
    map: &HashMap<K, u32>,
) -> HashMap<K, f64> {
    let total: u64 = map.values().map(|v| *v as u64).sum();
    if total == 0 {
        return HashMap::new();
    }

    map.iter()
        .map(|(k, v)| {
            let pct = (*v as f64 / total as f64) * 100.0;
            (k.clone(), (pct * 10.0).round() / 10.0) // Round to 1 decimal
        })
        .collect()
}

/// Truncate a string to a maximum length, adding ellipsis if needed
pub fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else if max_len <= 3 {
        s.chars().take(max_len).collect()
    } else {
        format!("{}...", &s[..max_len - 3])
    }
}

/// Generate a simple text-based bar chart
pub fn bar_chart(value: f64, max_width: usize) -> String {
    let width = ((value / 100.0) * max_width as f64).round() as usize;
    "\u{2588}".repeat(width.min(max_width))
}

/// Extract file extension from a path
pub fn get_file_extension(filepath: &str) -> String {
    std::path::Path::new(filepath)
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| format!(".{}", e.to_lowercase()))
        .unwrap_or_else(|| "(no ext)".to_string())
}

/// Check if a path matches any of the given patterns (case-insensitive)
pub fn matches_any_pattern(path: &str, patterns: &[&str]) -> bool {
    let path_lower = path.to_lowercase();
    patterns.iter().any(|p| path_lower.contains(&p.to_lowercase()))
}

/// Check if a path has any of the given extensions (case-insensitive)
pub fn has_extension(path: &str, extensions: &[&str]) -> bool {
    let path_lower = path.to_lowercase();
    extensions.iter().any(|e| path_lower.ends_with(&e.to_lowercase()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_number() {
        assert_eq!(format_number(500), "500");
        assert_eq!(format_number(1500), "1.5K");
        assert_eq!(format_number(1500000), "1.5M");
    }

    #[test]
    fn test_format_number_full() {
        assert_eq!(format_number_full(1234567), "1,234,567");
        assert_eq!(format_number_full(123), "123");
    }

    #[test]
    fn test_contribution_type_label() {
        assert_eq!(contribution_type_label("productioncode"), "Production Code");
        assert_eq!(contribution_type_label("tests"), "Tests");
        assert_eq!(contribution_type_label("unknown"), "Other");
    }

    #[test]
    fn test_sort_by_value() {
        let map: HashMap<String, u32> = [
            ("a".to_string(), 10),
            ("b".to_string(), 30),
            ("c".to_string(), 20),
        ]
        .into_iter()
        .collect();

        let sorted = sort_by_value(&map);
        assert_eq!(sorted[0].0, "b");
        assert_eq!(sorted[1].0, "c");
        assert_eq!(sorted[2].0, "a");
    }

    #[test]
    fn test_calculate_percentages() {
        let map: HashMap<String, u32> = [
            ("a".to_string(), 50),
            ("b".to_string(), 50),
        ]
        .into_iter()
        .collect();

        let pcts = calculate_percentages(&map);
        assert_eq!(pcts.get("a"), Some(&50.0));
        assert_eq!(pcts.get("b"), Some(&50.0));
    }

    #[test]
    fn test_truncate() {
        assert_eq!(truncate("hello", 10), "hello");
        assert_eq!(truncate("hello world", 8), "hello...");
    }

    #[test]
    fn test_bar_chart() {
        assert_eq!(bar_chart(50.0, 10), "\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}");
        assert_eq!(bar_chart(100.0, 10), "\u{2588}".repeat(10));
    }

    #[test]
    fn test_get_file_extension() {
        assert_eq!(get_file_extension("file.rs"), ".rs");
        assert_eq!(get_file_extension("Makefile"), "(no ext)");
    }

    #[test]
    fn test_matches_any_pattern() {
        assert!(matches_any_pattern("src/tests/foo.rs", &["test", "spec"]));
        assert!(!matches_any_pattern("src/main.rs", &["test", "spec"]));
    }

    #[test]
    fn test_has_extension() {
        assert!(has_extension("file.RS", &[".rs", ".py"]));
        assert!(!has_extension("file.js", &[".rs", ".py"]));
    }
}
