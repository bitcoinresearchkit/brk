//! Python tree structure generation.

use std::collections::BTreeSet;
use std::fmt::Write;

use brk_types::TreeNode;

use crate::{
    ClientMetadata, GenericSyntax, PatternField, PythonSyntax, build_child_path,
    generate_leaf_field, prepare_tree_node, to_snake_case,
};

/// Generate tree classes
pub fn generate_tree_classes(output: &mut String, catalog: &TreeNode, metadata: &ClientMetadata) {
    writeln!(output, "# Metrics tree classes\n").unwrap();

    let pattern_lookup = metadata.pattern_lookup();
    let mut generated = BTreeSet::new();
    generate_tree_class(
        output,
        "MetricsTree",
        "",
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
    path: &str,
    node: &TreeNode,
    pattern_lookup: &std::collections::BTreeMap<Vec<PatternField>, String>,
    metadata: &ClientMetadata,
    generated: &mut BTreeSet<String>,
) {
    let Some(ctx) = prepare_tree_node(node, name, path, pattern_lookup, metadata, generated) else {
        return;
    };

    // Generate child classes FIRST (post-order traversal)
    // This ensures children are defined before parent references them
    for child in &ctx.children {
        if child.should_inline {
            let child_path = build_child_path(path, child.name);
            generate_tree_class(
                output,
                &child.inline_type_name,
                &child_path,
                child.node,
                pattern_lookup,
                metadata,
                generated,
            );
        }
    }

    // THEN generate the current class (after all children are defined)
    writeln!(output, "class {}:", name).unwrap();
    writeln!(output, "    \"\"\"Metrics tree node.\"\"\"").unwrap();
    writeln!(output, "    ").unwrap();
    writeln!(
        output,
        "    def __init__(self, client: BrkClientBase, base_path: str = ''):"
    )
    .unwrap();

    let syntax = PythonSyntax;
    for child in &ctx.children {
        let field_name_py = to_snake_case(child.name);

        if child.is_leaf {
            if let TreeNode::Leaf(leaf) = child.node {
                generate_leaf_field(
                    output, &syntax, "client", child.name, leaf, metadata, "        ",
                );
            }
        } else if child.should_inline {
            // Inline class
            writeln!(
                output,
                "        self.{}: {} = {}(client)",
                field_name_py, child.inline_type_name, child.inline_type_name
            )
            .unwrap();
        } else {
            // Use pattern class with metric base
            let py_type = metadata.resolve_tree_field_type(
                &child.field,
                child.child_fields.as_deref(),
                name,
                child.name,
                GenericSyntax::PYTHON,
            );
            writeln!(
                output,
                "        self.{}: {} = {}(client, '{}')",
                field_name_py, py_type, child.field.rust_type, child.base_result.base
            )
            .unwrap();
        }
    }

    writeln!(output).unwrap();
}
