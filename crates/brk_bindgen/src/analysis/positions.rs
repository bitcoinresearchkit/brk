//! Pattern mode detection and field part extraction.
//!
//! This module analyzes pattern instances to detect whether they use
//! suffix mode (fields append to acc) or prefix mode (fields prepend to acc),
//! and extracts the field parts (relatives or prefixes) for code generation.

use std::collections::BTreeMap;

use brk_types::TreeNode;

use super::{
    find_common_prefix, find_common_suffix, get_node_fields, get_shortest_leaf_name,
    normalize_prefix,
};
use crate::{PatternBaseResult, PatternField, PatternMode, StructuralPattern, build_child_path};

/// Result of analyzing a single pattern instance.
#[derive(Debug, Clone)]
struct InstanceAnalysis {
    /// The base to return to parent (used for nesting)
    base: String,
    /// For suffix mode: field -> relative name
    /// For prefix mode: field -> prefix
    field_parts: BTreeMap<String, String>,
    /// Whether this instance appears to be suffix mode
    is_suffix_mode: bool,
}

/// Analyze all pattern instances and determine their modes.
///
/// This is the main entry point for mode detection. It processes
/// the tree bottom-up, collecting analysis for each pattern instance,
/// then determines the consistent mode for each pattern.
///
/// Returns a map from tree paths to their computed PatternBaseResult.
/// This map is used during generation to check pattern compatibility.
pub fn analyze_pattern_modes(
    tree: &TreeNode,
    patterns: &mut [StructuralPattern],
    pattern_lookup: &BTreeMap<Vec<PatternField>, String>,
) -> BTreeMap<String, PatternBaseResult> {
    // Collect analyses from all instances, keyed by pattern name
    let mut all_analyses: BTreeMap<String, Vec<InstanceAnalysis>> = BTreeMap::new();
    // Also collect base results for each node, keyed by tree path
    let mut node_bases: BTreeMap<String, PatternBaseResult> = BTreeMap::new();

    // Bottom-up traversal
    collect_instance_analyses(tree, "", pattern_lookup, &mut all_analyses, &mut node_bases);

    // For each pattern, determine mode from collected instances
    for pattern in patterns.iter_mut() {
        if let Some(analyses) = all_analyses.get(&pattern.name) {
            pattern.mode = determine_pattern_mode(analyses, &pattern.fields);
        }
    }

    node_bases
}

/// Recursively collect instance analyses bottom-up.
/// Returns the "base" for this node (used by parent for its analysis).
///
/// Also stores the PatternBaseResult for each node in `node_bases`, keyed by path.
fn collect_instance_analyses(
    node: &TreeNode,
    path: &str,
    pattern_lookup: &BTreeMap<Vec<PatternField>, String>,
    all_analyses: &mut BTreeMap<String, Vec<InstanceAnalysis>>,
    node_bases: &mut BTreeMap<String, PatternBaseResult>,
) -> Option<String> {
    match node {
        TreeNode::Leaf(leaf) => {
            // Leaves return their metric name as the base
            Some(leaf.name().to_string())
        }
        TreeNode::Branch(children) => {
            // First, process all children recursively (bottom-up)
            let mut child_bases: BTreeMap<String, String> = BTreeMap::new();
            for (field_name, child_node) in children {
                let child_path = build_child_path(path, field_name);
                if let Some(base) = collect_instance_analyses(
                    child_node,
                    &child_path,
                    pattern_lookup,
                    all_analyses,
                    node_bases,
                ) {
                    child_bases.insert(field_name.clone(), base);
                }
            }

            if child_bases.is_empty() {
                return None;
            }

            // Analyze this instance
            let mut analysis = analyze_instance(&child_bases);

            // When some field_parts are empty (children returned the same base),
            // replace empty parts with discriminators derived from shortest leaf names.
            let has_empty = analysis.field_parts.values().any(|v| v.is_empty());
            let has_nonempty = analysis.field_parts.values().any(|v| !v.is_empty());
            if has_empty && has_nonempty {
                // Mixed case: some fields have parts, some don't.
                // Use shortest leaf to derive discriminators for empty fields.
                let prefix = format!("{}_", analysis.base);
                for (field_name, child_node) in children {
                    if let Some(part) = analysis.field_parts.get(field_name)
                        && part.is_empty()
                        && let Some(leaf) = get_shortest_leaf_name(child_node)
                        && let Some(suffix) = leaf.strip_prefix(&prefix)
                        && !suffix.is_empty()
                        // Only use if the suffix starts with the field key,
                        // avoiding internal sub-field names like "0sd" from a Price child
                        && suffix.starts_with(field_name.trim_start_matches('_'))
                    {
                        analysis
                            .field_parts
                            .insert(field_name.clone(), suffix.to_string());
                    }
                }
            } else if has_empty && analysis.field_parts.len() > 1 {
                // All-empty case: all children returned the same base.
                // Re-analyze using shortest leaf names which may differentiate.
                let mut leaf_bases: BTreeMap<String, String> = BTreeMap::new();
                for (field_name, child_node) in children {
                    if let Some(leaf) = get_shortest_leaf_name(child_node) {
                        leaf_bases.insert(field_name.clone(), leaf);
                    }
                }
                if leaf_bases.len() == child_bases.len() {
                    let leaf_analysis = analyze_instance(&leaf_bases);
                    if !leaf_analysis.field_parts.values().all(|v| v.is_empty()) {
                        analysis.field_parts = leaf_analysis.field_parts;
                    }
                }
            }

            // Store the base result for this node
            // Note: has_outlier is false because we use recursive base computation
            // which gives correct bases without needing outlier detection
            node_bases.insert(
                path.to_string(),
                PatternBaseResult {
                    base: analysis.base.clone(),
                    has_outlier: false,
                    is_suffix_mode: analysis.is_suffix_mode,
                    field_parts: analysis.field_parts.clone(),
                },
            );

            // Get the pattern name for this node (if any)
            let fields = get_node_fields(children, pattern_lookup);
            if let Some(pattern_name) = pattern_lookup.get(&fields) {
                all_analyses
                    .entry(pattern_name.clone())
                    .or_default()
                    .push(analysis.clone());
            }

            // Return the base for parent
            Some(analysis.base)
        }
    }
}

/// Try to detect a template pattern when instances have different field_parts.
///
/// Supports two cases:
/// 1. **Embedded discriminator**: a substring varies per instance within field_parts.
///    E.g., `ratio_pct99_bps` vs `ratio_pct1_bps` → template `ratio_{disc}_bps`
/// 2. **Suffix discriminator**: a common suffix is appended to all field_parts.
///    E.g., `ratio_sd` vs `ratio_sd_4y` → template `ratio_sd{disc}`
fn try_detect_template(
    majority: &[&InstanceAnalysis],
    fields: &[PatternField],
) -> Option<PatternMode> {
    if majority.len() < 2 {
        return None;
    }

    // Strategy 1: Find an embedded discriminator (shortest non-empty field_part
    // that differs between instances and appears as substring in other parts)
    if let Some(mode) = try_embedded_disc(majority, fields) {
        return Some(mode);
    }

    // Strategy 2: Find a common suffix difference across ALL field_parts
    try_suffix_disc(majority, fields)
}

/// Strategy 1: embedded discriminator (e.g., pct99 inside ratio_pct99_bps)
fn try_embedded_disc(
    majority: &[&InstanceAnalysis],
    fields: &[PatternField],
) -> Option<PatternMode> {
    let first = &majority[0];
    let second = &majority[1];

    // Find the discriminator: shortest non-empty field_part that differs
    let disc_field = fields
        .iter()
        .filter_map(|f| first.field_parts.get(&f.name).map(|v| (&f.name, v)))
        .filter(|(_, v)| !v.is_empty())
        .min_by_key(|(_, v)| v.len())?;

    let disc_first = disc_field.1;
    let disc_second = second.field_parts.get(disc_field.0)?;

    if disc_first == disc_second || disc_first.is_empty() || disc_second.is_empty() {
        return None;
    }

    // Build templates by replacing the discriminator with {disc}
    let mut templates = BTreeMap::new();
    for field in fields {
        let part = first.field_parts.get(&field.name)?;
        let template = part.replacen(disc_first, "{disc}", 1);
        templates.insert(field.name.clone(), template);
    }

    // Verify ALL instances match
    for analysis in majority {
        let inst_disc = analysis.field_parts.get(disc_field.0)?;
        for field in fields {
            let part = analysis.field_parts.get(&field.name)?;
            let expected = templates.get(&field.name)?.replace("{disc}", inst_disc);
            if part != &expected {
                return None;
            }
        }
    }

    Some(PatternMode::Templated { templates })
}

/// Strategy 2: suffix discriminator (e.g., all field_parts differ by `_4y` suffix)
fn try_suffix_disc(
    majority: &[&InstanceAnalysis],
    fields: &[PatternField],
) -> Option<PatternMode> {
    let first = &majority[0];

    // For each other instance, check if ALL field_parts differ from the first
    // by the same suffix. Use the first field to detect the suffix.
    let ref_field = &fields[0].name;
    let ref_first = first.field_parts.get(ref_field)?;

    // Build templates from the first instance
    // Non-empty parts get {disc} appended; empty parts (identity) stay empty
    let mut templates = BTreeMap::new();
    for field in fields {
        let part = first.field_parts.get(&field.name)?;
        if part.is_empty() {
            templates.insert(field.name.clone(), String::new());
        } else {
            templates.insert(field.name.clone(), format!("{part}{{disc}}"));
        }
    }

    // Verify ALL other instances: non-empty parts differ by the same suffix
    for analysis in &majority[1..] {
        let ref_other = analysis.field_parts.get(ref_field)?;
        let suffix = ref_other.strip_prefix(ref_first)?;

        for field in fields {
            let first_part = first.field_parts.get(&field.name)?;
            let other_part = analysis.field_parts.get(&field.name)?;

            if first_part.is_empty() {
                // Identity field — must stay empty in all instances
                if !other_part.is_empty() {
                    return None;
                }
            } else {
                let expected = format!("{first_part}{suffix}");
                if other_part != &expected {
                    return None;
                }
            }
        }
    }

    Some(PatternMode::Templated { templates })
}

/// Analyze a single pattern instance from its child bases.
fn analyze_instance(child_bases: &BTreeMap<String, String>) -> InstanceAnalysis {
    let bases: Vec<&str> = child_bases.values().map(|s| s.as_str()).collect();

    // Try suffix mode first: look for common prefix among children
    if let Some(common_prefix) = find_common_prefix(&bases) {
        let base = common_prefix.trim_end_matches('_').to_string();
        let mut field_parts = BTreeMap::new();

        for (field_name, child_base) in child_bases {
            // Relative = child_base with common prefix stripped
            // If child_base equals base, relative is empty (identity field)
            let relative = if child_base == &base {
                String::new()
            } else {
                child_base
                    .strip_prefix(&common_prefix)
                    .unwrap_or(child_base)
                    .to_string()
            };
            field_parts.insert(field_name.clone(), relative);
        }

        // If all field_parts are empty (all children returned the same base),
        // use the field keys as suffix discriminators. This handles patterns like
        // period windows (all/_4y/_2y/_1y) where children differ by a suffix
        // that corresponds to the tree key.
        if field_parts.len() > 1 && field_parts.values().all(|v| v.is_empty()) {
            // Can't differentiate — this pattern is non-parameterizable
            return InstanceAnalysis {
                base,
                field_parts,
                is_suffix_mode: true,
            };
        }

        return InstanceAnalysis {
            base,
            field_parts,
            is_suffix_mode: true,
        };
    }

    // Try prefix mode: look for common suffix among children
    if let Some(common_suffix) = find_common_suffix(&bases) {
        let base = common_suffix.trim_start_matches('_').to_string();
        let mut field_parts = BTreeMap::new();

        for (field_name, child_base) in child_bases {
            // Prefix = child_base with common suffix stripped, normalized to end with _
            let prefix = child_base
                .strip_suffix(&common_suffix)
                .map(normalize_prefix)
                .unwrap_or_default();
            field_parts.insert(field_name.clone(), prefix);
        }

        return InstanceAnalysis {
            base,
            field_parts,
            is_suffix_mode: false,
        };
    }

    // No common prefix or suffix - use empty base so _m(base, relative) returns just the relative.
    // This handles cases like utxo_cohorts.all.activity where children have completely
    // different bases (coinblocks_destroyed, coindays_destroyed, etc.)
    let field_parts = child_bases
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();

    InstanceAnalysis {
        base: String::new(),
        field_parts,
        is_suffix_mode: true,
    }
}

/// Determine the consistent mode for a pattern from all its instances.
/// Picks the majority mode (suffix vs prefix), then requires all instances
/// in that mode to agree on field_parts. Minority-mode instances get inlined.
fn determine_pattern_mode(
    analyses: &[InstanceAnalysis],
    fields: &[PatternField],
) -> Option<PatternMode> {
    analyses.first()?;

    // Pick the majority mode
    let suffix_count = analyses.iter().filter(|a| a.is_suffix_mode).count();
    let is_suffix = suffix_count * 2 >= analyses.len();

    // All instances of the majority mode must agree on field_parts
    let majority: Vec<_> = analyses
        .iter()
        .filter(|a| a.is_suffix_mode == is_suffix)
        .collect();
    let first_majority = majority.first()?;

    // Verify all required fields have parts
    for field in fields {
        if !first_majority.field_parts.contains_key(&field.name) {
            return None;
        }
    }

    if majority
        .iter()
        .all(|a| a.field_parts == first_majority.field_parts)
    {
        let field_parts = first_majority.field_parts.clone();

        return if is_suffix {
            Some(PatternMode::Suffix {
                relatives: field_parts,
            })
        } else {
            Some(PatternMode::Prefix {
                prefixes: field_parts,
            })
        };
    }

    // Instances disagree on field_parts. Try to detect a template pattern:
    // if each field's value varies by exactly one substring that's different
    // per instance, we can use a Templated mode with {disc} placeholder.
    if is_suffix {
        try_detect_template(&majority, fields)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze_instance_suffix_mode() {
        let mut child_bases = BTreeMap::new();
        child_bases.insert("max".to_string(), "lth_cost_basis_max".to_string());
        child_bases.insert("min".to_string(), "lth_cost_basis_min".to_string());
        child_bases.insert("percentiles".to_string(), "lth_cost_basis".to_string());

        let analysis = analyze_instance(&child_bases);

        assert!(analysis.is_suffix_mode);
        assert_eq!(analysis.base, "lth_cost_basis");
        assert_eq!(analysis.field_parts.get("max"), Some(&"max".to_string()));
        assert_eq!(analysis.field_parts.get("min"), Some(&"min".to_string()));
        assert_eq!(
            analysis.field_parts.get("percentiles"),
            Some(&"".to_string())
        );
    }

    #[test]
    fn test_analyze_instance_prefix_mode() {
        // Period-prefixed metrics like "1y_lump_sum_stack", "1m_lump_sum_stack"
        // share a common suffix "_lump_sum_stack" with different period prefixes
        let mut child_bases = BTreeMap::new();
        child_bases.insert("_1y".to_string(), "1y_lump_sum_stack".to_string());
        child_bases.insert("_1m".to_string(), "1m_lump_sum_stack".to_string());
        child_bases.insert("_1w".to_string(), "1w_lump_sum_stack".to_string());

        let analysis = analyze_instance(&child_bases);

        assert!(!analysis.is_suffix_mode);
        assert_eq!(analysis.base, "lump_sum_stack");
        assert_eq!(analysis.field_parts.get("_1y"), Some(&"1y_".to_string()));
        assert_eq!(analysis.field_parts.get("_1m"), Some(&"1m_".to_string()));
        assert_eq!(analysis.field_parts.get("_1w"), Some(&"1w_".to_string()));
    }

    #[test]
    fn test_analyze_instance_root_suffix() {
        // At root level with suffix naming convention
        let mut child_bases = BTreeMap::new();
        child_bases.insert("max".to_string(), "cost_basis_max".to_string());
        child_bases.insert("min".to_string(), "cost_basis_min".to_string());
        child_bases.insert("percentiles".to_string(), "cost_basis".to_string());

        let analysis = analyze_instance(&child_bases);

        // With suffix naming, common prefix is "cost_basis_" (since cost_basis is one of the names)
        assert!(analysis.is_suffix_mode);
        assert_eq!(analysis.base, "cost_basis");
        assert_eq!(analysis.field_parts.get("max"), Some(&"max".to_string()));
        assert_eq!(analysis.field_parts.get("min"), Some(&"min".to_string()));
        assert_eq!(
            analysis.field_parts.get("percentiles"),
            Some(&"".to_string())
        );
    }

    #[test]
    fn test_determine_pattern_mode_majority_voting() {
        // Test that majority voting works when instances have mixed modes.
        // This simulates CostBasisPattern2: most instances use suffix mode,
        // but root-level uses prefix mode (max_cost_basis, min_cost_basis, cost_basis).
        use std::collections::BTreeSet;

        let fields = vec![
            PatternField {
                name: "max".to_string(),
                rust_type: "TestType".to_string(),
                json_type: "number".to_string(),
                indexes: BTreeSet::new(),
                type_param: None,
            },
            PatternField {
                name: "min".to_string(),
                rust_type: "TestType".to_string(),
                json_type: "number".to_string(),
                indexes: BTreeSet::new(),
                type_param: None,
            },
            PatternField {
                name: "percentiles".to_string(),
                rust_type: "TestType".to_string(),
                json_type: "number".to_string(),
                indexes: BTreeSet::new(),
                type_param: None,
            },
        ];

        // 3 suffix mode instances (majority)
        let suffix1 = InstanceAnalysis {
            base: "lth_cost_basis".to_string(),
            field_parts: [
                ("max".to_string(), "max".to_string()),
                ("min".to_string(), "min".to_string()),
                ("percentiles".to_string(), "".to_string()),
            ]
            .into_iter()
            .collect(),
            is_suffix_mode: true,
        };
        let suffix2 = InstanceAnalysis {
            base: "sth_cost_basis".to_string(),
            field_parts: [
                ("max".to_string(), "max".to_string()),
                ("min".to_string(), "min".to_string()),
                ("percentiles".to_string(), "".to_string()),
            ]
            .into_iter()
            .collect(),
            is_suffix_mode: true,
        };
        let suffix3 = InstanceAnalysis {
            base: "utxo_cost_basis".to_string(),
            field_parts: [
                ("max".to_string(), "max".to_string()),
                ("min".to_string(), "min".to_string()),
                ("percentiles".to_string(), "".to_string()),
            ]
            .into_iter()
            .collect(),
            is_suffix_mode: true,
        };

        // 1 prefix mode instance (minority - root level)
        let prefix1 = InstanceAnalysis {
            base: "cost_basis".to_string(),
            field_parts: [
                ("max".to_string(), "max_".to_string()),
                ("min".to_string(), "min_".to_string()),
                ("percentiles".to_string(), "".to_string()),
            ]
            .into_iter()
            .collect(),
            is_suffix_mode: false,
        };

        let analyses = vec![suffix1, suffix2, suffix3, prefix1];

        let mode = determine_pattern_mode(&analyses, &fields);

        // Should pick suffix mode (majority) with the common field_parts
        assert!(mode.is_some());
        match mode.unwrap() {
            PatternMode::Suffix { relatives } => {
                assert_eq!(relatives.get("max"), Some(&"max".to_string()));
                assert_eq!(relatives.get("min"), Some(&"min".to_string()));
                assert_eq!(relatives.get("percentiles"), Some(&"".to_string()));
            }
            PatternMode::Prefix { .. } => panic!("Expected suffix mode, got prefix mode"),
            PatternMode::Templated { .. } => panic!("Expected suffix mode, got templated mode"),
        }
    }

    #[test]
    fn test_determine_pattern_mode_all_same() {
        // Test when all instances agree on mode and field_parts
        use std::collections::BTreeSet;

        let fields = vec![
            PatternField {
                name: "max".to_string(),
                rust_type: "TestType".to_string(),
                json_type: "number".to_string(),
                indexes: BTreeSet::new(),
                type_param: None,
            },
            PatternField {
                name: "min".to_string(),
                rust_type: "TestType".to_string(),
                json_type: "number".to_string(),
                indexes: BTreeSet::new(),
                type_param: None,
            },
        ];

        let instance1 = InstanceAnalysis {
            base: "metric_a".to_string(),
            field_parts: [
                ("max".to_string(), "max".to_string()),
                ("min".to_string(), "min".to_string()),
            ]
            .into_iter()
            .collect(),
            is_suffix_mode: true,
        };
        let instance2 = InstanceAnalysis {
            base: "metric_b".to_string(),
            field_parts: [
                ("max".to_string(), "max".to_string()),
                ("min".to_string(), "min".to_string()),
            ]
            .into_iter()
            .collect(),
            is_suffix_mode: true,
        };

        let analyses = vec![instance1, instance2];
        let mode = determine_pattern_mode(&analyses, &fields);

        assert!(mode.is_some());
        match mode.unwrap() {
            PatternMode::Suffix { relatives } => {
                assert_eq!(relatives.get("max"), Some(&"max".to_string()));
                assert_eq!(relatives.get("min"), Some(&"min".to_string()));
            }
            PatternMode::Prefix { .. } => panic!("Expected suffix mode"),
            PatternMode::Templated { .. } => panic!("Expected suffix mode, got templated"),
        }
    }
}
