//! Python tree structure generation.

use std::collections::HashSet;
use std::fmt::Write;

use brk_types::TreeNode;

use crate::{
    ClientMetadata, GenericSyntax, PatternField, PythonSyntax, child_type_name, generate_leaf_field,
    get_node_fields, get_pattern_instance_base, prepare_tree_node, to_snake_case,
};

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
    let Some(ctx) = prepare_tree_node(node, name, pattern_lookup, metadata, generated) else {
        return;
    };

    writeln!(output, "class {}:", name).unwrap();
    writeln!(output, "    \"\"\"Catalog tree node.\"\"\"").unwrap();
    writeln!(output, "    ").unwrap();
    writeln!(
        output,
        "    def __init__(self, client: BrkClientBase, base_path: str = ''):"
    )
    .unwrap();

    let syntax = PythonSyntax;
    for ((field, child_fields_opt), (child_name, child_node)) in
        ctx.fields_with_child_info.iter().zip(ctx.children.iter())
    {
        let py_type = metadata.resolve_tree_field_type(
            field,
            child_fields_opt.as_deref(),
            name,
            child_name,
            GenericSyntax::PYTHON,
        );
        let field_name_py = to_snake_case(&field.name);

        if metadata.is_pattern_type(&field.rust_type) && metadata.is_parameterizable(&field.rust_type)
        {
            // Parameterizable pattern: use pattern class with metric base
            let metric_base = get_pattern_instance_base(child_node);
            writeln!(
                output,
                "        self.{}: {} = {}(client, '{}')",
                field_name_py, py_type, field.rust_type, metric_base
            )
            .unwrap();
        } else if let TreeNode::Leaf(leaf) = child_node {
            // Leaf node: use shared helper
            generate_leaf_field(output, &syntax, "client", child_name, leaf, metadata, "        ");
        } else if field.is_branch() {
            // Non-parameterizable pattern or regular branch: generate inline class
            let inline_class = child_type_name(name, &field.name);
            writeln!(
                output,
                "        self.{}: {} = {}(client)",
                field_name_py, inline_class, inline_class
            )
            .unwrap();
        } else {
            panic!(
                "Field '{}' has no matching index pattern. All metrics must be indexed.",
                field.name
            );
        }
    }

    writeln!(output).unwrap();

    // Generate child classes
    for (child_name, child_node) in ctx.children {
        if let TreeNode::Branch(grandchildren) = child_node {
            let child_fields = get_node_fields(grandchildren, pattern_lookup);

            // Generate inline class if no pattern match OR pattern is not parameterizable
            if !metadata.is_parameterizable_fields(&child_fields) {
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
