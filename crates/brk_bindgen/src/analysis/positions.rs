//! Field position detection for pattern instances.
//!
//! This module bridges the name analysis with pattern field positions,
//! processing patterns bottom-up to determine how each field modifies
//! the accumulated metric name.

use std::collections::HashMap;

use brk_types::TreeNode;

use super::{analyze_pattern_level, get_node_fields};
use crate::{FieldNamePosition, PatternField, StructuralPattern};

/// Analyze field positions for all patterns using bottom-up tree traversal.
///
/// This is the main entry point for field position detection. It processes
/// the tree bottom-up, analyzing each pattern instance and aggregating
/// the positions across all instances.
pub fn analyze_all_field_positions(
    tree: &TreeNode,
    patterns: &mut [StructuralPattern],
    pattern_lookup: &HashMap<Vec<PatternField>, String>,
) {
    let mut all_positions: HashMap<String, HashMap<String, Vec<FieldNamePosition>>> =
        HashMap::new();

    // Collect positions from all instances bottom-up
    collect_positions_bottom_up(tree, pattern_lookup, &mut all_positions);

    // Merge positions into patterns
    for pattern in patterns.iter_mut() {
        if let Some(field_positions) = all_positions.get(&pattern.name) {
            pattern.field_positions = merge_field_positions(field_positions);
        }
    }
}

/// Recursively collect field positions bottom-up.
/// Returns the effective base for this node (used by parent level).
fn collect_positions_bottom_up(
    node: &TreeNode,
    pattern_lookup: &HashMap<Vec<PatternField>, String>,
    all_positions: &mut HashMap<String, HashMap<String, Vec<FieldNamePosition>>>,
) -> Option<String> {
    match node {
        TreeNode::Leaf(leaf) => {
            // Leaves return their vec name as the effective base
            Some(leaf.name().to_string())
        }
        TreeNode::Branch(children) => {
            // First, process all children recursively (bottom-up)
            let mut child_bases: HashMap<String, String> = HashMap::new();
            for (field_name, child_node) in children {
                if let Some(base) = collect_positions_bottom_up(child_node, pattern_lookup, all_positions) {
                    child_bases.insert(field_name.clone(), base);
                }
            }

            // Build child names for this level's analysis
            let child_names: Vec<(String, String)> = children
                .keys()
                .filter_map(|field_name| {
                    child_bases
                        .get(field_name)
                        .map(|base| (field_name.clone(), base.clone()))
                })
                .collect();

            if child_names.is_empty() {
                return None;
            }

            // Analyze this level
            let analysis = analyze_pattern_level(&child_names);

            // Get the pattern name for this node (if any)
            let fields = get_node_fields(children, pattern_lookup);
            if let Some(pattern_name) = pattern_lookup.get(&fields) {
                // Record field positions for this pattern instance
                for (field_name, position) in &analysis.field_positions {
                    all_positions
                        .entry(pattern_name.clone())
                        .or_default()
                        .entry(field_name.clone())
                        .or_default()
                        .push(position.clone());
                }
            }

            // Return our base for the parent level
            Some(analysis.base)
        }
    }
}

/// Check if a list of positions contains incompatible values.
///
/// Positions are incompatible if there are multiple different non-Identity positions,
/// meaning different pattern instances use different naming conventions.
fn has_incompatible_positions(positions: &[FieldNamePosition]) -> bool {
    let non_identity: Vec<_> = positions
        .iter()
        .filter(|p| !matches!(p, FieldNamePosition::Identity))
        .collect();

    if non_identity.len() <= 1 {
        return false;
    }

    // Check if all non-identity positions are the same
    let first = &non_identity[0];
    non_identity.iter().skip(1).any(|p| p != first)
}

/// Merge multiple observed positions for each field into a single position.
///
/// Returns an empty map if any field has incompatible positions across instances,
/// which will cause `is_parameterizable()` to return false for the pattern.
fn merge_field_positions(
    field_positions: &HashMap<String, Vec<FieldNamePosition>>,
) -> HashMap<String, FieldNamePosition> {
    // First check for incompatible positions
    for positions in field_positions.values() {
        if has_incompatible_positions(positions) {
            // Incompatible positions found - pattern cannot be parameterized
            return HashMap::new();
        }
    }

    // All positions are compatible, proceed with merge
    field_positions
        .iter()
        .filter_map(|(field_name, positions)| {
            if positions.is_empty() {
                return None;
            }

            // Prefer Append/Prepend over Identity, as Identity at root-level
            // is handled by empty acc and conditional position expressions
            let preferred = positions
                .iter()
                .find(|p| !matches!(p, FieldNamePosition::Identity))
                .cloned()
                .unwrap_or_else(|| positions[0].clone());

            Some((field_name.clone(), preferred))
        })
        .collect()
}
