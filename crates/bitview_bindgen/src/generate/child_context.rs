use bitview_catalog::TreeNode;

use crate::{PatternBaseResult, PatternField};

/// Pre-computed context for a single child node.
pub struct ChildContext<'a> {
    /// The child's field name in the tree.
    pub name: &'a str,
    /// The child node.
    pub node: &'a TreeNode,
    /// The field info for this child (with type_param set for generic patterns).
    pub field: PatternField,
    /// Pattern analysis result.
    pub base_result: PatternBaseResult,
    /// Whether this is a leaf node.
    pub is_leaf: bool,
    /// Whether to use an inline type instead of a pattern type (only meaningful for branches).
    pub should_inline: bool,
    /// The type name to use for inline branches.
    pub inline_type_name: String,
}
