//! JavaScript tree structure generation.

use std::collections::HashSet;
use std::fmt::Write;

use brk_types::TreeNode;

use crate::{
    ClientMetadata, Endpoint, PatternField, child_type_name, get_fields_with_child_info,
    get_first_leaf_name, get_node_fields, get_pattern_instance_base, infer_accumulated_name,
    to_camel_case,
};

use super::api::generate_api_methods;
use super::client::{field_type_with_generic, generate_static_constants};

/// Generate JSDoc typedefs for the catalog tree.
pub fn generate_tree_typedefs(output: &mut String, catalog: &TreeNode, metadata: &ClientMetadata) {
    writeln!(output, "// Catalog tree typedefs\n").unwrap();

    let pattern_lookup = metadata.pattern_lookup();
    let mut generated = HashSet::new();
    generate_tree_typedef(
        output,
        "CatalogTree",
        catalog,
        &pattern_lookup,
        metadata,
        &mut generated,
    );
}

fn generate_tree_typedef(
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

    if pattern_lookup.contains_key(&fields)
        && pattern_lookup.get(&fields) != Some(&name.to_string())
    {
        return;
    }

    if generated.contains(name) {
        return;
    }
    generated.insert(name.to_string());

    writeln!(output, "/**").unwrap();
    writeln!(output, " * @typedef {{Object}} {}", name).unwrap();

    for (field, child_fields) in &fields_with_child_info {
        let generic_value_type = child_fields
            .as_ref()
            .and_then(|cf| metadata.get_type_param(cf))
            .map(String::as_str);
        let js_type = field_type_with_generic(field, metadata, false, generic_value_type);
        writeln!(
            output,
            " * @property {{{}}} {}",
            js_type,
            to_camel_case(&field.name)
        )
        .unwrap();
    }

    writeln!(output, " */\n").unwrap();

    for (child_name, child_node) in children {
        if let TreeNode::Branch(grandchildren) = child_node {
            let child_fields = get_node_fields(grandchildren, pattern_lookup);
            if !pattern_lookup.contains_key(&child_fields) {
                let child_type = child_type_name(name, child_name);
                generate_tree_typedef(
                    output,
                    &child_type,
                    child_node,
                    pattern_lookup,
                    metadata,
                    generated,
                );
            }
        }
    }
}

/// Generate the main BrkClient class.
pub fn generate_main_client(
    output: &mut String,
    catalog: &TreeNode,
    metadata: &ClientMetadata,
    endpoints: &[Endpoint],
) {
    let pattern_lookup = metadata.pattern_lookup();

    writeln!(output, "/**").unwrap();
    writeln!(output, " * Main BRK client with catalog tree and API methods").unwrap();
    writeln!(output, " * @extends BrkClientBase").unwrap();
    writeln!(output, " */").unwrap();
    writeln!(output, "class BrkClient extends BrkClientBase {{").unwrap();

    generate_static_constants(output);

    writeln!(output, "  /**").unwrap();
    writeln!(output, "   * @param {{BrkClientOptions|string}} options").unwrap();
    writeln!(output, "   */").unwrap();
    writeln!(output, "  constructor(options) {{").unwrap();
    writeln!(output, "    super(options);").unwrap();
    writeln!(output, "    /** @type {{CatalogTree}} */").unwrap();
    writeln!(output, "    this.tree = this._buildTree('');").unwrap();
    writeln!(output, "  }}\n").unwrap();

    writeln!(output, "  /**").unwrap();
    writeln!(output, "   * @private").unwrap();
    writeln!(output, "   * @param {{string}} basePath").unwrap();
    writeln!(output, "   * @returns {{CatalogTree}}").unwrap();
    writeln!(output, "   */").unwrap();
    writeln!(output, "  _buildTree(basePath) {{").unwrap();
    writeln!(output, "    return {{").unwrap();
    generate_tree_initializer(output, catalog, "", 3, &pattern_lookup, metadata);
    writeln!(output, "    }};").unwrap();
    writeln!(output, "  }}\n").unwrap();

    generate_api_methods(output, endpoints);

    writeln!(output, "}}\n").unwrap();

    writeln!(output, "export {{ BrkClient, BrkClientBase, BrkError }};").unwrap();
}

fn generate_tree_initializer(
    output: &mut String,
    node: &TreeNode,
    accumulated_name: &str,
    indent: usize,
    pattern_lookup: &std::collections::HashMap<Vec<PatternField>, String>,
    metadata: &ClientMetadata,
) {
    let indent_str = "  ".repeat(indent);

    if let TreeNode::Branch(children) = node {
        for (i, (child_name, child_node)) in children.iter().enumerate() {
            let field_name = to_camel_case(child_name);
            let comma = if i < children.len() - 1 { "," } else { "" };

            match child_node {
                TreeNode::Leaf(leaf) => {
                    let accessor = metadata
                        .find_index_set_pattern(leaf.indexes())
                        .unwrap_or_else(|| {
                            panic!(
                                "Metric '{}' has no matching index pattern. All metrics must be indexed.",
                                leaf.name()
                            )
                        });
                    writeln!(
                        output,
                        "{}{}: create{}(this, '{}'){}",
                        indent_str, field_name, accessor.name, leaf.name(), comma
                    )
                    .unwrap();
                }
                TreeNode::Branch(grandchildren) => {
                    let child_fields = get_node_fields(grandchildren, pattern_lookup);
                    if let Some(pattern_name) = pattern_lookup.get(&child_fields) {
                        let pattern = metadata
                            .structural_patterns
                            .iter()
                            .find(|p| &p.name == pattern_name);
                        let is_parameterizable =
                            pattern.map(|p| p.is_parameterizable()).unwrap_or(false);

                        let arg = if is_parameterizable {
                            get_pattern_instance_base(child_node)
                        } else if accumulated_name.is_empty() {
                            format!("/{}", child_name)
                        } else {
                            format!("{}/{}", accumulated_name, child_name)
                        };

                        writeln!(
                            output,
                            "{}{}: create{}(this, '{}'){}",
                            indent_str, field_name, pattern_name, arg, comma
                        )
                        .unwrap();
                    } else {
                        let child_acc =
                            infer_child_accumulated_name(child_node, accumulated_name, child_name);
                        writeln!(output, "{}{}: {{", indent_str, field_name).unwrap();
                        generate_tree_initializer(
                            output,
                            child_node,
                            &child_acc,
                            indent + 1,
                            pattern_lookup,
                            metadata,
                        );
                        writeln!(output, "{}}}{}", indent_str, comma).unwrap();
                    }
                }
            }
        }
    }
}

fn infer_child_accumulated_name(node: &TreeNode, parent_acc: &str, field_name: &str) -> String {
    let leaf_name = get_first_leaf_name(node).unwrap_or_default();
    infer_accumulated_name(parent_acc, field_name, &leaf_name)
}
