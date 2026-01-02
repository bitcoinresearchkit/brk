//! Rust tree structure generation.

use std::collections::HashSet;
use std::fmt::Write;

use brk_types::TreeNode;

use crate::{
    ClientMetadata, PatternField, RustSyntax, child_type_name, generate_tree_node_field,
    get_fields_with_child_info, get_node_fields, get_pattern_instance_base, to_snake_case,
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
    let TreeNode::Branch(children) = node else {
        return;
    };

    let fields_with_child_info = get_fields_with_child_info(children, name, pattern_lookup);
    let fields: Vec<PatternField> = fields_with_child_info
        .iter()
        .map(|(f, _)| f.clone())
        .collect();

    if let Some(pattern_name) = pattern_lookup.get(&fields)
        && pattern_name != name
    {
        return;
    }

    if generated.contains(name) {
        return;
    }
    generated.insert(name.to_string());

    writeln!(output, "/// Catalog tree node.").unwrap();
    writeln!(output, "pub struct {} {{", name).unwrap();

    for (field, child_fields) in &fields_with_child_info {
        let field_name = to_snake_case(&field.name);
        // Look up type parameter for generic patterns
        let generic_value_type = child_fields
            .as_ref()
            .and_then(|cf| metadata.get_type_param(cf))
            .map(String::as_str);
        let type_annotation = field_type_with_generic(field, metadata, false, generic_value_type);
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
    for (field, (child_name, child_node)) in fields.iter().zip(children.iter()) {
        // Detect pattern base for parameterizable patterns
        let pattern_base = if metadata.is_pattern_type(&field.rust_type) {
            let pattern = metadata.find_pattern(&field.rust_type);
            if pattern.is_some_and(|p| p.is_parameterizable()) {
                Some(get_pattern_instance_base(child_node))
            } else {
                None
            }
        } else {
            None
        };
        generate_tree_node_field(output, &syntax, field, metadata, "            ", child_name, pattern_base.as_deref());
    }

    writeln!(output, "        }}").unwrap();
    writeln!(output, "    }}").unwrap();
    writeln!(output, "}}\n").unwrap();

    for (child_name, child_node) in children {
        if let TreeNode::Branch(grandchildren) = child_node {
            let child_fields = get_node_fields(grandchildren, pattern_lookup);
            if !pattern_lookup.contains_key(&child_fields) {
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
