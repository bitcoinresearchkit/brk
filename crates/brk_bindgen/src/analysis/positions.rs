//! Pattern mode detection and field part extraction.
//!
//! This module analyzes pattern instances to detect whether they use
//! suffix mode (fields append to acc) or prefix mode (fields prepend to acc),
//! and extracts the field parts (relatives or prefixes) for code generation.

use std::collections::HashMap;

use brk_types::TreeNode;

use super::{find_common_prefix, find_common_suffix, get_node_fields};
use crate::{PatternField, PatternMode, StructuralPattern};

/// Result of analyzing a single pattern instance.
#[derive(Debug, Clone)]
struct InstanceAnalysis {
    /// The base to return to parent (used for nesting)
    base: String,
    /// For suffix mode: field -> relative name
    /// For prefix mode: field -> prefix
    field_parts: HashMap<String, String>,
    /// Whether this instance appears to be suffix mode
    is_suffix_mode: bool,
}

/// Analyze all pattern instances and determine their modes.
///
/// This is the main entry point for mode detection. It processes
/// the tree bottom-up, collecting analysis for each pattern instance,
/// then determines the consistent mode for each pattern.
pub fn analyze_pattern_modes(
    tree: &TreeNode,
    patterns: &mut [StructuralPattern],
    pattern_lookup: &HashMap<Vec<PatternField>, String>,
) {
    // Collect analyses from all instances, keyed by pattern name
    let mut all_analyses: HashMap<String, Vec<InstanceAnalysis>> = HashMap::new();

    // Bottom-up traversal
    collect_instance_analyses(tree, pattern_lookup, &mut all_analyses);

    // For each pattern, determine mode from collected instances
    for pattern in patterns.iter_mut() {
        if let Some(analyses) = all_analyses.get(&pattern.name) {
            pattern.mode = determine_pattern_mode(analyses, &pattern.fields);
        }
    }
}

/// Recursively collect instance analyses bottom-up.
/// Returns the "base" for this node (used by parent for its analysis).
fn collect_instance_analyses(
    node: &TreeNode,
    pattern_lookup: &HashMap<Vec<PatternField>, String>,
    all_analyses: &mut HashMap<String, Vec<InstanceAnalysis>>,
) -> Option<String> {
    match node {
        TreeNode::Leaf(leaf) => {
            // Leaves return their metric name as the base
            Some(leaf.name().to_string())
        }
        TreeNode::Branch(children) => {
            // First, process all children recursively (bottom-up)
            let mut child_bases: HashMap<String, String> = HashMap::new();
            for (field_name, child_node) in children {
                if let Some(base) =
                    collect_instance_analyses(child_node, pattern_lookup, all_analyses)
                {
                    child_bases.insert(field_name.clone(), base);
                }
            }

            if child_bases.is_empty() {
                return None;
            }

            // Analyze this instance
            let analysis = analyze_instance(&child_bases);

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

/// Analyze a single pattern instance from its child bases.
fn analyze_instance(child_bases: &HashMap<String, String>) -> InstanceAnalysis {
    let bases: Vec<&str> = child_bases.values().map(|s| s.as_str()).collect();

    // Try suffix mode first: look for common prefix among children
    if let Some(common_prefix) = find_common_prefix(&bases) {
        let base = common_prefix.trim_end_matches('_').to_string();
        let mut field_parts = HashMap::new();

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

        return InstanceAnalysis {
            base,
            field_parts,
            is_suffix_mode: true,
        };
    }

    // Try prefix mode: look for common suffix among children
    if let Some(common_suffix) = find_common_suffix(&bases) {
        let base = common_suffix.trim_start_matches('_').to_string();
        let mut field_parts = HashMap::new();

        for (field_name, child_base) in child_bases {
            // Prefix = child_base with common suffix stripped
            let prefix = child_base
                .strip_suffix(&common_suffix)
                .map(|s| {
                    // Ensure prefix ends with underscore if non-empty
                    if s.is_empty() {
                        String::new()
                    } else if s.ends_with('_') {
                        s.to_string()
                    } else {
                        format!("{}_", s)
                    }
                })
                .unwrap_or_default();
            field_parts.insert(field_name.clone(), prefix);
        }

        return InstanceAnalysis {
            base,
            field_parts,
            is_suffix_mode: false,
        };
    }

    // No common prefix or suffix - use first child's base and treat as suffix mode
    // with full metric names as relatives
    let base = child_bases.values().next().cloned().unwrap_or_default();
    let field_parts = child_bases
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();

    InstanceAnalysis {
        base,
        field_parts,
        is_suffix_mode: true,
    }
}

/// Determine the consistent mode for a pattern from all its instances.
/// Uses majority voting: if most instances agree on mode and field_parts,
/// use those. Minority instances will be inlined at usage sites.
fn determine_pattern_mode(
    analyses: &[InstanceAnalysis],
    fields: &[PatternField],
) -> Option<PatternMode> {
    if analyses.is_empty() {
        return None;
    }

    // Group instances by (mode, field_parts) signature
    let suffix_instances: Vec<_> = analyses.iter().filter(|a| a.is_suffix_mode).collect();
    let prefix_instances: Vec<_> = analyses.iter().filter(|a| !a.is_suffix_mode).collect();

    // Pick the majority mode group
    let (majority_instances, is_suffix) = if suffix_instances.len() >= prefix_instances.len() {
        (suffix_instances, true)
    } else {
        (prefix_instances, false)
    };

    if majority_instances.is_empty() {
        return None;
    }

    // Find the most common field_parts within the majority group
    // Convert to sorted Vec for comparison since HashMap isn't hashable
    let mut parts_counts: HashMap<Vec<(String, String)>, usize> = HashMap::new();
    for analysis in &majority_instances {
        let mut sorted: Vec<_> = analysis.field_parts.iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        sorted.sort();
        *parts_counts.entry(sorted).or_insert(0) += 1;
    }

    let (best_parts_vec, _count) = parts_counts.into_iter().max_by_key(|(_, count)| *count)?;
    let best_parts: HashMap<String, String> = best_parts_vec.into_iter().collect();

    // Verify all required fields have parts
    for field in fields {
        if !best_parts.contains_key(&field.name) {
            return None;
        }
    }

    let field_parts = best_parts;

    if is_suffix {
        Some(PatternMode::Suffix {
            relatives: field_parts,
        })
    } else {
        Some(PatternMode::Prefix {
            prefixes: field_parts,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze_instance_suffix_mode() {
        let mut child_bases = HashMap::new();
        child_bases.insert("max".to_string(), "lth_cost_basis_max".to_string());
        child_bases.insert("min".to_string(), "lth_cost_basis_min".to_string());
        child_bases.insert("percentiles".to_string(), "lth_cost_basis".to_string());

        let analysis = analyze_instance(&child_bases);

        assert!(analysis.is_suffix_mode);
        assert_eq!(analysis.base, "lth_cost_basis");
        assert_eq!(analysis.field_parts.get("max"), Some(&"max".to_string()));
        assert_eq!(analysis.field_parts.get("min"), Some(&"min".to_string()));
        assert_eq!(analysis.field_parts.get("percentiles"), Some(&"".to_string()));
    }

    #[test]
    fn test_analyze_instance_prefix_mode() {
        // Period-prefixed metrics like "1y_lump_sum_stack", "1m_lump_sum_stack"
        // share a common suffix "_lump_sum_stack" with different period prefixes
        let mut child_bases = HashMap::new();
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
        let mut child_bases = HashMap::new();
        child_bases.insert("max".to_string(), "cost_basis_max".to_string());
        child_bases.insert("min".to_string(), "cost_basis_min".to_string());
        child_bases.insert("percentiles".to_string(), "cost_basis".to_string());

        let analysis = analyze_instance(&child_bases);

        // With suffix naming, common prefix is "cost_basis_" (since cost_basis is one of the names)
        assert!(analysis.is_suffix_mode);
        assert_eq!(analysis.base, "cost_basis");
        assert_eq!(analysis.field_parts.get("max"), Some(&"max".to_string()));
        assert_eq!(analysis.field_parts.get("min"), Some(&"min".to_string()));
        assert_eq!(analysis.field_parts.get("percentiles"), Some(&"".to_string()));
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
            PatternMode::Prefix { .. } => {
                panic!("Expected suffix mode, got prefix mode");
            }
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
            PatternMode::Prefix { .. } => {
                panic!("Expected suffix mode");
            }
        }
    }
}
