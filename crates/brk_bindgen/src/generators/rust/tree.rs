//! Rust tree structure generation.

use std::collections::HashSet;
use std::fmt::Write;

use brk_types::TreeNode;

use crate::{
    ClientMetadata, LanguageSyntax, PatternField, RustSyntax, child_type_name,
    generate_leaf_field, generate_tree_node_field, get_node_fields, get_pattern_instance_base,
    prepare_tree_node, to_snake_case,
};

use super::client::field_type_with_generic;

/// Generate tree structs.
pub fn generate_tree(output: &mut String, catalog: &TreeNode, metadata: &ClientMetadata) {
    writeln!(output, "// Catalog tree\n").unwrap();

    let pattern_lookup = metadata.pattern_lookup();
    let mut generated = HashSet::new();
    generate_tree_node(
        output,
        "CatalogTree",
        catalog,
        &pattern_lookup,
        metadata,
        &mut generated,
    );
}

fn generate_tree_node(
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

    writeln!(output, "/// Catalog tree node.").unwrap();
    writeln!(output, "pub struct {} {{", name).unwrap();

    for ((field, child_fields), (child_name, _)) in
        ctx.fields_with_child_info.iter().zip(ctx.children.iter())
    {
        let field_name = to_snake_case(&field.name);
        let type_annotation = metadata.resolve_tree_field_type(
            child_fields.as_deref(),
            name,
            child_name,
            |generic| field_type_with_generic(field, metadata, false, generic),
        );
        writeln!(output, "    pub {}: {},", field_name, type_annotation).unwrap();
    }

    writeln!(output, "}}\n").unwrap();

    writeln!(output, "impl {} {{", name).unwrap();
    writeln!(
        output,
        "    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {{"
    )
    .unwrap();
    writeln!(output, "        Self {{").unwrap();

    let syntax = RustSyntax;
    for ((field_info, child_fields), (child_name, child_node)) in
        ctx.fields_with_child_info.iter().zip(ctx.children.iter())
    {
        let field_name = to_snake_case(&field_info.name);

        // Check if this is a pattern type and if it's parameterizable
        let is_parameterizable = child_fields
            .as_ref()
            .is_some_and(|cf| metadata.is_parameterizable_fields(cf));

        if metadata.is_pattern_type(&field_info.rust_type) && is_parameterizable {
            // Parameterizable pattern: use pattern constructor with metric base
            let pattern_base = get_pattern_instance_base(child_node);
            generate_tree_node_field(
                output,
                &syntax,
                field_info,
                metadata,
                "            ",
                child_name,
                Some(&pattern_base),
            );
        } else if child_fields.is_some() {
            // Non-parameterizable pattern or regular branch: use inline struct
            let child_struct = child_type_name(name, child_name);
            let path_expr = syntax.path_expr("base_path", &format!("_{}", child_name));
            writeln!(
                output,
                "            {}: {}::new(client.clone(), {}),",
                field_name, child_struct, path_expr
            )
            .unwrap();
        } else if let TreeNode::Leaf(leaf) = child_node {
            // Leaf field - use shared helper
            generate_leaf_field(
                output,
                &syntax,
                "client.clone()",
                child_name,
                leaf,
                metadata,
                "            ",
            );
        } else {
            panic!(
                "Field '{}' is a leaf with no TreeNode::Leaf. This shouldn't happen.",
                field_info.name
            );
        }
    }

    writeln!(output, "        }}").unwrap();
    writeln!(output, "    }}").unwrap();
    writeln!(output, "}}\n").unwrap();

    for (child_name, child_node) in ctx.children {
        if let TreeNode::Branch(grandchildren) = child_node {
            let child_fields = get_node_fields(grandchildren, pattern_lookup);
            // Generate child struct if no pattern match OR pattern is not parameterizable
            if !metadata.is_parameterizable_fields(&child_fields) {
                let child_struct = child_type_name(name, child_name);
                generate_tree_node(
                    output,
                    &child_struct,
                    child_node,
                    pattern_lookup,
                    metadata,
                    generated,
                );
            }
        }
    }
}
