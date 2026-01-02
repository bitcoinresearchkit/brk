//! Vec name deconstruction and reconstruction logic.
//!
//! This module analyzes vec names bottom-up to detect common denominators
//! (prefixes or suffixes) and field positions for pattern instances.

use std::collections::HashMap;

use crate::FieldNamePosition;

/// Common denominator found across children's effective names.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommonDenominator {
    /// Children share this prefix. Fields append their unique suffix.
    /// Example: children are ["addrs_0sats", "addrs_1sats"], common = "addrs_"
    Prefix(String),
    /// Children share this suffix. Fields prepend their unique prefix.
    /// Example: children are ["cumulative_supply", "net_supply"], common = "_supply"
    Suffix(String),
    /// No common part found. Fields use Identity (field = base).
    None,
}

/// Result of analyzing a pattern level.
#[derive(Debug, Clone)]
pub struct PatternAnalysis {
    /// The common prefix/suffix found across all children.
    pub common: CommonDenominator,
    /// What's left after stripping the common part (passed to parent).
    pub base: String,
    /// How each field modifies the accumulated name.
    pub field_positions: HashMap<String, FieldNamePosition>,
}

/// Analyze a pattern level using child effective names.
///
/// This is the core algorithm that detects common prefix/suffix and
/// determines field positions for each child.
///
/// # Arguments
/// * `child_names` - Vec of (field_name, effective_name) pairs
///   where effective_name is either:
///   - For leaves: the leaf's vec name
///   - For branches: the base returned by analyzing that branch
pub fn analyze_pattern_level(child_names: &[(String, String)]) -> PatternAnalysis {
    if child_names.is_empty() {
        return PatternAnalysis {
            common: CommonDenominator::None,
            base: String::new(),
            field_positions: HashMap::new(),
        };
    }

    if child_names.len() == 1 {
        let (field_name, effective) = &child_names[0];
        let mut positions = HashMap::new();

        // Try suffix match: effective ends with "_fieldname"
        let suffix_pattern = format!("_{}", field_name);
        if let Some(base) = effective.strip_suffix(&suffix_pattern) {
            positions.insert(
                field_name.clone(),
                FieldNamePosition::Append(suffix_pattern),
            );
            return PatternAnalysis {
                common: CommonDenominator::None,
                base: base.to_string(),
                field_positions: positions,
            };
        }

        // Try prefix match: effective starts with "fieldname_"
        let prefix_pattern = format!("{}_", field_name);
        if let Some(base) = effective.strip_prefix(&prefix_pattern) {
            positions.insert(
                field_name.clone(),
                FieldNamePosition::Prepend(prefix_pattern),
            );
            return PatternAnalysis {
                common: CommonDenominator::None,
                base: base.to_string(),
                field_positions: positions,
            };
        }

        // Field equals effective OR field doesn't appear → Identity
        // Root-level instances where field == effective are handled by
        // passing empty `acc` and conditional position expressions
        positions.insert(field_name.clone(), FieldNamePosition::Identity);
        return PatternAnalysis {
            common: CommonDenominator::None,
            base: effective.clone(),
            field_positions: positions,
        };
    }

    let effective_names: Vec<&str> = child_names.iter().map(|(_, n)| n.as_str()).collect();

    // Try to find common prefix first
    if let Some(prefix) = find_common_prefix(&effective_names)
        && !prefix.is_empty()
    {
        let base = prefix.trim_end_matches('_').to_string();
        let mut positions = HashMap::new();
        for (field_name, effective) in child_names {
            // If effective equals the base (prefix without underscore), use Identity
            if effective == &base {
                positions.insert(field_name.clone(), FieldNamePosition::Identity);
            } else if let Some(suffix) = effective.strip_prefix(&prefix) {
                // Normal case: effective has the full prefix
                let suffix_with_underscore = if suffix.starts_with('_') {
                    suffix.to_string()
                } else {
                    format!("_{}", suffix)
                };
                positions.insert(
                    field_name.clone(),
                    FieldNamePosition::Append(suffix_with_underscore),
                );
            } else {
                // Fallback: use Identity if strip_prefix fails unexpectedly
                positions.insert(field_name.clone(), FieldNamePosition::Identity);
            }
        }
        return PatternAnalysis {
            common: CommonDenominator::Prefix(prefix),
            base,
            field_positions: positions,
        };
    }

    // Try to find common suffix
    if let Some(suffix) = find_common_suffix(&effective_names)
        && !suffix.is_empty()
    {
        let mut positions = HashMap::new();
        for (field_name, effective) in child_names {
            let prefix = effective
                .strip_suffix(&suffix)
                .unwrap_or(effective)
                .to_string();
            let prefix_with_underscore = if prefix.ends_with('_') {
                prefix
            } else {
                format!("{}_", prefix)
            };
            positions.insert(
                field_name.clone(),
                FieldNamePosition::Prepend(prefix_with_underscore),
            );
        }
        let base = suffix.trim_start_matches('_').to_string();
        return PatternAnalysis {
            common: CommonDenominator::Suffix(suffix),
            base,
            field_positions: positions,
        };
    }

    // No common part - use Identity for all fields
    let mut positions = HashMap::new();
    for (field_name, _) in child_names {
        positions.insert(field_name.clone(), FieldNamePosition::Identity);
    }

    // Use the first name as base (they're all independent)
    let base = child_names
        .first()
        .map(|(_, n)| n.clone())
        .unwrap_or_default();

    PatternAnalysis {
        common: CommonDenominator::None,
        base,
        field_positions: positions,
    }
}

/// Find the longest common prefix among all strings.
/// The prefix must end at an underscore boundary for semantic coherence.
fn find_common_prefix(names: &[&str]) -> Option<String> {
    if names.is_empty() {
        return None;
    }

    let first = names[0];
    if first.is_empty() {
        return None;
    }

    // Find character-by-character common prefix
    let mut prefix_len = 0;
    for (i, ch) in first.chars().enumerate() {
        if names.iter().all(|n| n.chars().nth(i) == Some(ch)) {
            prefix_len = i + 1;
        } else {
            break;
        }
    }

    if prefix_len == 0 {
        return None;
    }

    let raw_prefix = &first[..prefix_len];

    // If raw_prefix exactly matches one of the names, it's a complete metric name.
    // In this case, return it with trailing underscore to preserve the full name.
    if names.contains(&raw_prefix) {
        return Some(format!("{}_", raw_prefix));
    }

    // Find the last underscore position to get a clean boundary
    // Prefer ending at an underscore for semantic coherence
    if let Some(last_underscore) = raw_prefix.rfind('_')
        && last_underscore > 0
    {
        let clean_prefix = &first[..=last_underscore];
        // Verify this still works for all names
        if names.iter().all(|n| n.starts_with(clean_prefix)) {
            return Some(clean_prefix.to_string());
        }
    }

    // If no underscore boundary works, the full prefix must end at an underscore
    if raw_prefix.ends_with('_') {
        return Some(raw_prefix.to_string());
    }

    None
}

/// Find the longest common suffix among all strings.
/// The suffix must start at an underscore boundary for semantic coherence.
fn find_common_suffix(names: &[&str]) -> Option<String> {
    if names.is_empty() {
        return None;
    }

    let first = names[0];
    if first.is_empty() {
        return None;
    }

    // Find character-by-character common suffix (from the end)
    let first_chars: Vec<char> = first.chars().collect();
    let mut suffix_len = 0;

    for i in 0..first_chars.len() {
        let idx_from_end = first_chars.len() - 1 - i;
        let ch = first_chars[idx_from_end];

        let all_match = names.iter().all(|n| {
            let n_chars: Vec<char> = n.chars().collect();
            if i >= n_chars.len() {
                return false;
            }
            n_chars[n_chars.len() - 1 - i] == ch
        });

        if all_match {
            suffix_len = i + 1;
        } else {
            break;
        }
    }

    if suffix_len == 0 {
        return None;
    }

    let raw_suffix = &first[first.len() - suffix_len..];

    // Find the first underscore position to get a clean boundary
    if let Some(first_underscore) = raw_suffix.find('_')
        && first_underscore < raw_suffix.len() - 1
    {
        let clean_suffix = &raw_suffix[first_underscore..];
        // Verify this still works for all names
        if names.iter().all(|n| n.ends_with(clean_suffix)) {
            return Some(clean_suffix.to_string());
        }
    }

    // If no underscore boundary works, the full suffix must start with underscore
    if raw_suffix.starts_with('_') {
        return Some(raw_suffix.to_string());
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_common_prefix() {
        let names = vec!["addrs_0sats", "addrs_1sats", "addrs_2sats"];
        assert_eq!(find_common_prefix(&names), Some("addrs_".to_string()));
    }

    #[test]
    fn test_common_suffix() {
        let names = vec!["cumulative_supply", "net_supply", "total_supply"];
        assert_eq!(find_common_suffix(&names), Some("_supply".to_string()));
    }

    #[test]
    fn test_no_common() {
        let names = vec!["foo", "bar", "baz"];
        assert_eq!(find_common_prefix(&names), None);
        assert_eq!(find_common_suffix(&names), None);
    }

    #[test]
    fn test_analyze_pattern_level_prefix() {
        let children = vec![
            ("_0sats".to_string(), "addrs_0sats".to_string()),
            ("_1sats".to_string(), "addrs_1sats".to_string()),
        ];
        let analysis = analyze_pattern_level(&children);

        assert!(matches!(analysis.common, CommonDenominator::Prefix(_)));
        assert_eq!(analysis.base, "addrs");
        assert!(matches!(
            analysis.field_positions.get("_0sats"),
            Some(FieldNamePosition::Append(_))
        ));
    }

    #[test]
    fn test_analyze_pattern_level_suffix() {
        let children = vec![
            ("cumulative".to_string(), "cumulative_supply".to_string()),
            ("net".to_string(), "net_supply".to_string()),
        ];
        let analysis = analyze_pattern_level(&children);

        assert!(matches!(analysis.common, CommonDenominator::Suffix(_)));
        assert_eq!(analysis.base, "supply");
        assert!(matches!(
            analysis.field_positions.get("cumulative"),
            Some(FieldNamePosition::Prepend(_))
        ));
    }

    #[test]
    fn test_single_child_suffix() {
        // Field "count" appears as suffix "_count" in "activity_count"
        let children = vec![("count".to_string(), "activity_count".to_string())];
        let analysis = analyze_pattern_level(&children);

        assert!(matches!(analysis.common, CommonDenominator::None));
        assert_eq!(analysis.base, "activity");
        assert_eq!(
            analysis.field_positions.get("count"),
            Some(&FieldNamePosition::Append("_count".to_string()))
        );
    }

    #[test]
    fn test_single_child_prefix() {
        // Field "cumulative" appears as prefix "cumulative_" in "cumulative_supply"
        let children = vec![("cumulative".to_string(), "cumulative_supply".to_string())];
        let analysis = analyze_pattern_level(&children);

        assert!(matches!(analysis.common, CommonDenominator::None));
        assert_eq!(analysis.base, "supply");
        assert_eq!(
            analysis.field_positions.get("cumulative"),
            Some(&FieldNamePosition::Prepend("cumulative_".to_string()))
        );
    }

    #[test]
    fn test_single_child_identity_equal() {
        // Field "supply" equals effective "supply" → Identity
        // (root-level handling is done via empty acc and conditional expressions)
        let children = vec![("supply".to_string(), "supply".to_string())];
        let analysis = analyze_pattern_level(&children);

        assert!(matches!(analysis.common, CommonDenominator::None));
        assert_eq!(analysis.base, "supply");
        assert_eq!(
            analysis.field_positions.get("supply"),
            Some(&FieldNamePosition::Identity)
        );
    }

    #[test]
    fn test_single_child_identity_structural() {
        // Field "x" doesn't appear in "a_b" - it's structural grouping
        let children = vec![("x".to_string(), "a_b".to_string())];
        let analysis = analyze_pattern_level(&children);

        assert!(matches!(analysis.common, CommonDenominator::None));
        assert_eq!(analysis.base, "a_b"); // passes through unchanged
        assert_eq!(
            analysis.field_positions.get("x"),
            Some(&FieldNamePosition::Identity)
        );
    }

    #[test]
    fn test_common_prefix_exact_match() {
        // When one name exactly matches the common prefix, preserve the full name
        // This fixes the realized_loss vs realized_count bug
        let names = vec!["realized_loss", "realized_loss_cumulative"];
        assert_eq!(
            find_common_prefix(&names),
            Some("realized_loss_".to_string())
        );
    }

    #[test]
    fn test_common_prefix_exact_match_multiple() {
        // Multiple children with same base name
        let names = vec!["realized_loss", "realized_loss", "realized_loss_cumulative"];
        assert_eq!(
            find_common_prefix(&names),
            Some("realized_loss_".to_string())
        );
    }

    #[test]
    fn test_analyze_pattern_level_full_base() {
        // When names are like [realized_loss, realized_loss_cumulative],
        // base should be "realized_loss" not "realized"
        let children = vec![
            ("sum".to_string(), "realized_loss".to_string()),
            (
                "cumulative".to_string(),
                "realized_loss_cumulative".to_string(),
            ),
        ];
        let analysis = analyze_pattern_level(&children);

        assert!(matches!(analysis.common, CommonDenominator::Prefix(_)));
        assert_eq!(analysis.base, "realized_loss");
        // sum effective equals base, so position is Identity
        assert_eq!(
            analysis.field_positions.get("sum"),
            Some(&FieldNamePosition::Identity)
        );
        // cumulative has suffix "_cumulative" after the base
        assert_eq!(
            analysis.field_positions.get("cumulative"),
            Some(&FieldNamePosition::Append("_cumulative".to_string()))
        );
    }
}
