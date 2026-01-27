//! Rust tree structure generation.

use std::collections::BTreeSet;
use std::fmt::Write;

use brk_types::TreeNode;

use crate::{
    ClientMetadata, GenericSyntax, LanguageSyntax, PatternField, RustSyntax, build_child_path,
    generate_leaf_field, generate_tree_node_field, prepare_tree_node, to_snake_case,
};

/// Generate tree structs.
pub fn generate_tree(output: &mut String, catalog: &TreeNode, metadata: &ClientMetadata) {
    writeln!(output, "// Metrics tree\n").unwrap();

    let pattern_lookup = metadata.pattern_lookup();
    let mut generated = BTreeSet::new();
    generate_tree_node(
        output,
        "MetricsTree",
        "",
        catalog,
        &pattern_lookup,
        metadata,
        &mut generated,
    );
}

fn generate_tree_node(
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

    // Generate struct definition
    writeln!(output, "/// Metrics tree node.").unwrap();
    writeln!(output, "pub struct {} {{", name).unwrap();

    for child in &ctx.children {
        let field_name = to_snake_case(child.name);
        let type_annotation = if child.should_inline {
            child.inline_type_name.clone()
        } else {
            metadata.resolve_tree_field_type(
                &child.field,
                child.child_fields.as_deref(),
                name,
                child.name,
                GenericSyntax::RUST,
            )
        };
        writeln!(output, "    pub {}: {},", field_name, type_annotation).unwrap();
    }

    writeln!(output, "}}\n").unwrap();

    // Generate impl block
    writeln!(output, "impl {} {{", name).unwrap();
    writeln!(
        output,
        "    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {{"
    )
    .unwrap();
    writeln!(output, "        Self {{").unwrap();

    let syntax = RustSyntax;
    for child in &ctx.children {
        let field_name = to_snake_case(child.name);

        if child.is_leaf {
            if let TreeNode::Leaf(leaf) = child.node {
                generate_leaf_field(
                    output,
                    &syntax,
                    "client.clone()",
                    child.name,
                    leaf,
                    metadata,
                    "            ",
                );
            }
        } else if child.should_inline {
            // Inline struct type - only for nodes that don't match any pattern
            let path_expr = syntax.path_expr("base_path", &format!("_{}", child.name));
            writeln!(
                output,
                "            {}: {}::new(client.clone(), {}),",
                field_name, child.inline_type_name, path_expr
            )
            .unwrap();
        } else {
            // Pattern type - use ::new() constructor
            // All patterns have ::new(), parameterizable ones use detected mode,
            // non-parameterizable ones use field name fallback
            generate_tree_node_field(
                output,
                &syntax,
                &child.field,
                metadata,
                "            ",
                child.name,
                Some(&child.base_result.base),
            );
        }
    }

    writeln!(output, "        }}").unwrap();
    writeln!(output, "    }}").unwrap();
    writeln!(output, "}}\n").unwrap();

    // Generate child structs
    for child in &ctx.children {
        if child.should_inline {
            let child_path = build_child_path(path, child.name);
            generate_tree_node(
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
}
