//! Shared field generation logic.
//!
//! This module contains the core field generation logic that is shared
//! across all language backends. The `LanguageSyntax` trait is used to
//! abstract over language-specific formatting.

use std::fmt::Write;

use brk_types::MetricLeafWithSchema;

use crate::{ClientMetadata, LanguageSyntax, PatternBaseResult, PatternField, StructuralPattern};

/// Create a path suffix from a name.
/// Adds `_` prefix only if the name doesn't already start with `_`.
fn path_suffix(name: &str) -> String {
    if name.starts_with('_') {
        name.to_string()
    } else {
        format!("_{}", name)
    }
}

/// Compute path expression from pattern mode and field part.
fn compute_path_expr<S: LanguageSyntax>(
    syntax: &S,
    pattern: &StructuralPattern,
    field: &PatternField,
    base_var: &str,
) -> String {
    match pattern.get_field_part(&field.name) {
        Some(part) => {
            if pattern.is_templated() {
                // Templated: replace {disc} with disc variable at runtime
                syntax.template_expr(base_var, part)
            } else if pattern.is_suffix_mode() {
                syntax.suffix_expr(base_var, part)
            } else {
                syntax.prefix_expr(part, base_var)
            }
        }
        None => syntax.path_expr(base_var, &path_suffix(&field.name)),
    }
}

/// Compute field value from path expression.
fn compute_field_value<S: LanguageSyntax>(
    syntax: &S,
    field: &PatternField,
    metadata: &ClientMetadata,
    path_expr: &str,
) -> String {
    if metadata.is_pattern_type(&field.rust_type) {
        syntax.constructor(&field.rust_type, path_expr)
    } else if let Some(accessor) = metadata.find_index_set_pattern(&field.indexes) {
        syntax.constructor(&accessor.name, path_expr)
    } else if field.is_branch() {
        syntax.constructor(&field.rust_type, path_expr)
    } else {
        panic!(
            "Field '{}' has no matching pattern or index accessor. All metrics must be indexed.",
            field.name
        )
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
    let type_ann =
        metadata.field_type_annotation(field, pattern.is_generic, None, syntax.generic_syntax());
    let path_expr = compute_path_expr(syntax, pattern, field, "acc");

    // When calling a templated child pattern, pass acc and disc separately
    let value = if let Some(child_pattern) = metadata.find_pattern(&field.rust_type)
        && child_pattern.is_templated()
    {
        let disc_template = pattern
            .get_field_part(&field.name)
            .unwrap_or(&field.name);
        let disc_arg = syntax.disc_arg_expr(disc_template);
        let acc_arg = syntax.suffix_expr("acc", ""); // identity — returns acc or acc.clone()
        syntax.constructor(&field.rust_type, &format!("{acc_arg}, {disc_arg}"))
    } else {
        compute_field_value(syntax, field, metadata, &path_expr)
    };

    writeln!(
        output,
        "{}",
        syntax.field_init(indent, &field_name, &type_ann, &value)
    )
    .unwrap();
}

/// Generate a tree node field using pre-computed base results.
///
/// Handles pattern fields (both templated and non-templated), leaf fields,
/// and non-pattern branch fields. For templated patterns, extracts the
/// discriminator from the base result's field_parts.
pub fn generate_tree_node_field<S: LanguageSyntax>(
    output: &mut String,
    syntax: &S,
    field: &PatternField,
    metadata: &ClientMetadata,
    indent: &str,
    child_name: &str,
    client_expr: &str,
    base_result: Option<&PatternBaseResult>,
) {
    let field_name = syntax.field_name(&field.name);
    let type_ann = metadata.field_type_annotation(field, false, None, syntax.generic_syntax());

    let value = if metadata.is_pattern_type(&field.rust_type) {
        let pattern = metadata.find_pattern(&field.rust_type);
        let use_base = pattern.is_some_and(|p| p.is_parameterizable()) && base_result.is_some();

        if use_base {
            let br = base_result.unwrap();
            let base_arg = syntax.string_literal(&br.base);
            if let Some(pat) = pattern
                && pat.is_templated()
            {
                let disc = pat
                    .extract_disc_from_instance(&br.field_parts)
                    .unwrap_or_default();
                let disc_arg = syntax.string_literal(&disc);
                format!(
                    "{}({}, {}, {})",
                    syntax.constructor_name(&field.rust_type),
                    client_expr,
                    base_arg,
                    disc_arg
                )
            } else {
                format!(
                    "{}({}, {})",
                    syntax.constructor_name(&field.rust_type),
                    client_expr,
                    base_arg
                )
            }
        } else {
            let path_arg = syntax.path_expr("base_path", &path_suffix(child_name));
            format!(
                "{}({}, {})",
                syntax.constructor_name(&field.rust_type),
                client_expr,
                path_arg
            )
        }
    } else if let Some(accessor) = metadata.find_index_set_pattern(&field.indexes) {
        let path_arg = base_result
            .map(|br| syntax.string_literal(&br.base))
            .unwrap_or_else(|| syntax.path_expr("base_path", &path_suffix(child_name)));
        format!(
            "{}({}, {})",
            syntax.constructor_name(&accessor.name),
            client_expr,
            path_arg
        )
    } else if field.is_branch() {
        let path_expr = syntax.path_expr("base_path", &path_suffix(child_name));
        format!(
            "{}({}, {})",
            syntax.constructor_name(&field.rust_type),
            client_expr,
            path_expr
        )
    } else {
        panic!(
            "Field '{}' is a leaf with no index accessor. All metrics must be indexed.",
            field.name
        )
    };

    writeln!(
        output,
        "{}",
        syntax.field_init(indent, &field_name, &type_ann, &value)
    )
    .unwrap();
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
