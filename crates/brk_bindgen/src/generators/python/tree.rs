//! Python tree structure generation.

use std::collections::HashSet;
use std::fmt::Write;

use brk_types::TreeNode;

use crate::{
    ClientMetadata, PatternField, child_type_name, get_fields_with_child_info, get_node_fields,
    get_pattern_instance_base, to_snake_case,
};

use super::client::field_type_with_generic;

/// Generate tree classes
pub fn generate_tree_classes(output: &mut String, catalog: &TreeNode, metadata: &ClientMetadata) {
    writeln!(output, "# Catalog tree classes\n").unwrap();

    let pattern_lookup = metadata.pattern_lookup();
    let mut generated = HashSet::new();
    generate_tree_class(
        output,
        "CatalogTree",
        catalog,
        &pattern_lookup,
        metadata,
        &mut generated,
    );
}

/// Recursively generate tree classes
fn generate_tree_class(
    output: &mut String,
    name: &str,
    node: &TreeNode,
    pattern_lookup: &std::collections::HashMap<Vec<PatternField>, String>,
    metadata: &ClientMetadata,
    generated: &mut HashSet<String>,
) {
    let TreeNode::Branch(children) = node else {
        return;
    };

    let fields_with_child_info = get_fields_with_child_info(children, name, pattern_lookup);
    let fields: Vec<PatternField> = fields_with_child_info
        .iter()
        .map(|(f, _)| f.clone())
        .collect();

    // Skip if this matches a pattern (already generated)
    if pattern_lookup.contains_key(&fields)
        && pattern_lookup.get(&fields) != Some(&name.to_string())
    {
        return;
    }

    if generated.contains(name) {
        return;
    }
    generated.insert(name.to_string());

    writeln!(output, "class {}:", name).unwrap();
    writeln!(output, "    \"\"\"Catalog tree node.\"\"\"").unwrap();
    writeln!(output, "    ").unwrap();
    writeln!(
        output,
        "    def __init__(self, client: BrkClientBase, base_path: str = ''):"
    )
    .unwrap();

    for ((field, child_fields_opt), (_child_name, child_node)) in
        fields_with_child_info.iter().zip(children.iter())
    {
        // Look up type parameter for generic patterns
        let generic_value_type = child_fields_opt
            .as_ref()
            .and_then(|cf| metadata.get_type_param(cf))
            .map(String::as_str);
        let py_type = field_type_with_generic(field, metadata, false, generic_value_type);
        let field_name_py = to_snake_case(&field.name);

        if metadata.is_pattern_type(&field.rust_type) {
            let pattern = metadata.find_pattern(&field.rust_type);
            let is_parameterizable = pattern.is_some_and(|p| p.is_parameterizable());

            if is_parameterizable {
                let metric_base = get_pattern_instance_base(child_node);
                writeln!(
                    output,
                    "        self.{}: {} = {}(client, '{}')",
                    field_name_py, py_type, field.rust_type, metric_base
                )
                .unwrap();
            } else {
                writeln!(
                    output,
                    "        self.{}: {} = {}(client, f'{{base_path}}_{}')",
                    field_name_py, py_type, field.rust_type, field.name
                )
                .unwrap();
            }
        } else if metadata.field_uses_accessor(field) {
            let accessor = metadata.find_index_set_pattern(&field.indexes).unwrap();
            writeln!(
                output,
                "        self.{}: {} = {}(client, f'{{base_path}}_{}')",
                field_name_py, py_type, accessor.name, field.name
            )
            .unwrap();
        } else if field.is_branch() {
            // Non-pattern branch - instantiate the nested class
            writeln!(
                output,
                "        self.{}: {} = {}(client, f'{{base_path}}_{}')",
                field_name_py, py_type, field.rust_type, field.name
            )
            .unwrap();
        } else {
            // All metrics must be indexed - this should not be reached
            panic!(
                "Field '{}' has no matching index pattern. All metrics must be indexed.",
                field.name
            );
        }
    }

    writeln!(output).unwrap();

    // Generate child classes
    for (child_name, child_node) in children {
        if let TreeNode::Branch(grandchildren) = child_node {
            let child_fields = get_node_fields(grandchildren, pattern_lookup);
            if !pattern_lookup.contains_key(&child_fields) {
                let child_class = child_type_name(name, child_name);
                generate_tree_class(
                    output,
                    &child_class,
                    child_node,
                    pattern_lookup,
                    metadata,
                    generated,
                );
            }
        }
    }
}
