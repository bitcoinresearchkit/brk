//! Shared tree generation helpers.

use std::collections::{HashMap, HashSet};

use brk_types::TreeNode;

use crate::{ClientMetadata, PatternField, get_fields_with_child_info};

/// Context for generating a tree node, returned by `prepare_tree_node`.
pub struct TreeNodeContext<'a> {
    /// The children of the branch node.
    pub children: &'a std::collections::BTreeMap<String, TreeNode>,
    /// Fields with optional child field info for generic pattern lookup.
    pub fields_with_child_info: Vec<(PatternField, Option<Vec<PatternField>>)>,
    /// Just the fields (for pattern lookup).
    pub fields: Vec<PatternField>,
}

/// Prepare a tree node for generation.
/// Returns None if the node should be skipped (not a branch, already generated,
/// or matches a parameterizable pattern).
pub fn prepare_tree_node<'a>(
    node: &'a TreeNode,
    name: &str,
    pattern_lookup: &HashMap<Vec<PatternField>, String>,
    metadata: &ClientMetadata,
    generated: &mut HashSet<String>,
) -> Option<TreeNodeContext<'a>> {
    let TreeNode::Branch(children) = node else {
        return None;
    };

    let fields_with_child_info = get_fields_with_child_info(children, name, pattern_lookup);
    let fields: Vec<PatternField> = fields_with_child_info
        .iter()
        .map(|(f, _)| f.clone())
        .collect();

    // Skip if this matches a parameterizable pattern
    if let Some(pattern_name) = pattern_lookup.get(&fields)
        && pattern_name != name
        && metadata.is_parameterizable(pattern_name)
    {
        return None;
    }

    // Skip if already generated
    if generated.contains(name) {
        return None;
    }
    generated.insert(name.to_string());

    Some(TreeNodeContext {
        children,
        fields_with_child_info,
        fields,
    })
}
