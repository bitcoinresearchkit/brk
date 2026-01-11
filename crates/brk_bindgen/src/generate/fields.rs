//! Shared field generation logic.
//!
//! This module contains the core field generation logic that is shared
//! across all language backends. The `LanguageSyntax` trait is used to
//! abstract over language-specific formatting.

use std::fmt::Write;

use brk_types::MetricLeafWithSchema;

use crate::{ClientMetadata, LanguageSyntax, PatternField, StructuralPattern};

/// Create a path suffix from a name.
/// Adds `_` prefix only if the name doesn't already start with `_`.
fn path_suffix(name: &str) -> String {
    if name.starts_with('_') {
        name.to_string()
    } else {
        format!("_{}", name)
    }
}

/// Generate a parameterized field using the language syntax.
///
/// This is used for pattern instances where fields use an accumulated
/// metric name that's built up through the tree traversal.
pub fn generate_parameterized_field<S: LanguageSyntax>(
    output: &mut String,
    syntax: &S,
    field: &PatternField,
    pattern: &StructuralPattern,
    metadata: &ClientMetadata,
    indent: &str,
) {
    let field_name = syntax.field_name(&field.name);
    let type_ann = metadata.field_type_annotation(field, pattern.is_generic, None, syntax.generic_syntax());

    // Compute path expression from field position
    let path_expr = pattern
        .get_field_position(&field.name)
        .map(|pos| syntax.position_expr(pos, "acc"))
        .unwrap_or_else(|| syntax.path_expr("acc", &path_suffix(&field.name)));

    let value = if metadata.is_pattern_type(&field.rust_type) {
        syntax.constructor(&field.rust_type, &path_expr)
    } else if let Some(accessor) = metadata.find_index_set_pattern(&field.indexes) {
        syntax.constructor(&accessor.name, &path_expr)
    } else if field.is_branch() {
        // Non-pattern branch - instantiate the nested struct
        syntax.constructor(&field.rust_type, &path_expr)
    } else {
        panic!(
            "Field '{}' has no matching pattern or index accessor. All metrics must be indexed.",
            field.name
        )
    };

    writeln!(output, "{}", syntax.field_init(indent, &field_name, &type_ann, &value)).unwrap();
}

/// Generate a tree-path field using the language syntax.
///
/// This is the fallback for non-parameterizable patterns where fields
/// use a base path that's extended with the field name.
pub fn generate_tree_path_field<S: LanguageSyntax>(
    output: &mut String,
    syntax: &S,
    field: &PatternField,
    metadata: &ClientMetadata,
    indent: &str,
) {
    let field_name = syntax.field_name(&field.name);
    let type_ann = metadata.field_type_annotation(field, false, None, syntax.generic_syntax());
    let path_expr = syntax.path_expr("base_path", &path_suffix(&field.name));

    let value = if metadata.is_pattern_type(&field.rust_type) {
        syntax.constructor(&field.rust_type, &path_expr)
    } else if let Some(accessor) = metadata.find_index_set_pattern(&field.indexes) {
        syntax.constructor(&accessor.name, &path_expr)
    } else if field.is_branch() {
        // Non-pattern branch - instantiate the nested struct
        syntax.constructor(&field.rust_type, &path_expr)
    } else {
        panic!(
            "Field '{}' has no matching pattern or index accessor. All metrics must be indexed.",
            field.name
        )
    };

    writeln!(output, "{}", syntax.field_init(indent, &field_name, &type_ann, &value)).unwrap();
}

/// Generate a tree node field with a specific child node for pattern instance base detection.
///
/// This is used when generating tree nodes where we need to detect the pattern instance
/// base from descendant leaf names.
pub fn generate_tree_node_field<S: LanguageSyntax>(
    output: &mut String,
    syntax: &S,
    field: &PatternField,
    metadata: &ClientMetadata,
    indent: &str,
    child_name: &str,
    pattern_base: Option<&str>,
) {
    let field_name = syntax.field_name(&field.name);
    let type_ann = metadata.field_type_annotation(field, false, None, syntax.generic_syntax());

    let value = if metadata.is_pattern_type(&field.rust_type) {
        // Check if this pattern is parameterizable
        let pattern = metadata.find_pattern(&field.rust_type);
        let is_parameterizable = pattern.is_some_and(|p| p.is_parameterizable());

        if is_parameterizable {
            if let Some(base) = pattern_base {
                // Use the detected metric base
                let path = syntax.string_literal(base);
                syntax.constructor(&field.rust_type, &path)
            } else {
                // Fallback to tree path
                let path_expr = syntax.path_expr("base_path", &path_suffix(child_name));
                syntax.constructor(&field.rust_type, &path_expr)
            }
        } else {
            let path_expr = syntax.path_expr("base_path", &path_suffix(child_name));
            syntax.constructor(&field.rust_type, &path_expr)
        }
    } else if let Some(accessor) = metadata.find_index_set_pattern(&field.indexes) {
        // Leaf field - use actual metric name if provided
        if let Some(metric_name) = pattern_base {
            let path = syntax.string_literal(metric_name);
            syntax.constructor(&accessor.name, &path)
        } else {
            let path_expr = syntax.path_expr("base_path", &path_suffix(child_name));
            syntax.constructor(&accessor.name, &path_expr)
        }
    } else if field.is_branch() {
        // Non-pattern branch - instantiate the nested struct
        let path_expr = syntax.path_expr("base_path", &path_suffix(child_name));
        syntax.constructor(&field.rust_type, &path_expr)
    } else {
        // All metrics must be indexed
        panic!(
            "Field '{}' is a leaf with no index accessor. All metrics must be indexed.",
            field.name
        )
    };

    writeln!(output, "{}", syntax.field_init(indent, &field_name, &type_ann, &value)).unwrap();
}

/// Generate a leaf field using the actual metric name from the TreeNode::Leaf.
///
/// This is the shared implementation for all language backends. It uses
/// `leaf.name()` directly to get the correct metric name, avoiding any
/// path concatenation that could produce incorrect names.
///
/// # Arguments
/// * `output` - The string buffer to write to
/// * `syntax` - The language syntax implementation
/// * `client_expr` - The client expression (e.g., "client.clone()", "this", "client")
/// * `tree_field_name` - The field name from the tree structure
/// * `leaf` - The Leaf node containing the actual metric name and indexes
/// * `metadata` - Client metadata for looking up index patterns
/// * `indent` - Indentation string
pub fn generate_leaf_field<S: LanguageSyntax>(
    output: &mut String,
    syntax: &S,
    client_expr: &str,
    tree_field_name: &str,
    leaf: &MetricLeafWithSchema,
    metadata: &ClientMetadata,
    indent: &str,
) {
    let field_name = syntax.field_name(tree_field_name);
    let accessor = metadata
        .find_index_set_pattern(leaf.indexes())
        .unwrap_or_else(|| {
            panic!(
                "Metric '{}' has no matching index pattern. All metrics must be indexed.",
                leaf.name()
            )
        });

    let type_ann = metadata.field_type_annotation_from_leaf(leaf, syntax.generic_syntax());
    let metric_name = syntax.string_literal(leaf.name());
    let value = format!(
        "{}({}, {})",
        syntax.constructor_name(&accessor.name),
        client_expr,
        metric_name
    );

    writeln!(
        output,
        "{}",
        syntax.field_init(indent, &field_name, &type_ann, &value)
    )
    .unwrap();
}
