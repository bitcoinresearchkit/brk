//! JavaScript tree structure generation.

use std::collections::HashSet;
use std::fmt::Write;

use brk_types::TreeNode;

use crate::{
    ClientMetadata, Endpoint, GenericSyntax, JavaScriptSyntax, PatternField,
    generate_leaf_field, get_first_leaf_name, get_node_fields, get_pattern_instance_base,
    infer_accumulated_name, prepare_tree_node, to_camel_case,
};

use super::api::generate_api_methods;
use super::client::generate_static_constants;

/// Generate JSDoc typedefs for the metrics tree.
pub fn generate_tree_typedefs(output: &mut String, catalog: &TreeNode, metadata: &ClientMetadata) {
    writeln!(output, "// Catalog tree typedefs\n").unwrap();

    let pattern_lookup = metadata.pattern_lookup();
    let mut generated = HashSet::new();
    generate_tree_typedef(
        output,
        "MetricsTree",
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
    let Some(ctx) = prepare_tree_node(node, name, pattern_lookup, metadata, generated) else {
        return;
    };

    writeln!(output, "/**").unwrap();
    writeln!(output, " * @typedef {{Object}} {}", name).unwrap();

    for child in &ctx.children {
        let js_type = if child.should_inline {
            child.inline_type_name.clone()
        } else {
            metadata.resolve_tree_field_type(
                &child.field,
                child.child_fields.as_deref(),
                name,
                child.name,
                GenericSyntax::JAVASCRIPT,
            )
        };

        writeln!(
            output,
            " * @property {{{}}} {}",
            js_type,
            to_camel_case(&child.field.name)
        )
        .unwrap();
    }

    writeln!(output, " */\n").unwrap();

    // Generate child typedefs
    for child in &ctx.children {
        if child.should_inline {
            generate_tree_typedef(
                output,
                &child.inline_type_name,
                child.node,
                pattern_lookup,
                metadata,
                generated,
            );
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
    writeln!(
        output,
        " * Main BRK client with metrics tree and API methods"
    )
    .unwrap();
    writeln!(output, " * @extends BrkClientBase").unwrap();
    writeln!(output, " */").unwrap();
    writeln!(output, "class BrkClient extends BrkClientBase {{").unwrap();

    generate_static_constants(output);

    writeln!(output, "  /**").unwrap();
    writeln!(output, "   * @param {{BrkClientOptions|string}} options").unwrap();
    writeln!(output, "   */").unwrap();
    writeln!(output, "  constructor(options) {{").unwrap();
    writeln!(output, "    super(options);").unwrap();
    writeln!(output, "    /** @type {{MetricsTree}} */").unwrap();
    writeln!(output, "    this.metrics = this._buildTree('');").unwrap();
    writeln!(output, "  }}\n").unwrap();

    writeln!(output, "  /**").unwrap();
    writeln!(output, "   * @private").unwrap();
    writeln!(output, "   * @param {{string}} basePath").unwrap();
    writeln!(output, "   * @returns {{MetricsTree}}").unwrap();
    writeln!(output, "   */").unwrap();
    writeln!(output, "  _buildTree(basePath) {{").unwrap();
    writeln!(output, "    return {{").unwrap();
    generate_tree_initializer(output, catalog, "", 3, &pattern_lookup, metadata);
    writeln!(output, "    }};").unwrap();
    writeln!(output, "  }}\n").unwrap();

    writeln!(output, "  /**").unwrap();
    writeln!(output, "   * Create a dynamic metric endpoint builder for any metric/index combination.").unwrap();
    writeln!(output, "   *").unwrap();
    writeln!(output, "   * Use this for programmatic access when the metric name is determined at runtime.").unwrap();
    writeln!(output, "   * For type-safe access, use the `metrics` tree instead.").unwrap();
    writeln!(output, "   *").unwrap();
    writeln!(output, "   * @param {{string}} metric - The metric name").unwrap();
    writeln!(output, "   * @param {{Index}} index - The index name").unwrap();
    writeln!(output, "   * @returns {{MetricEndpointBuilder<unknown>}}").unwrap();
    writeln!(output, "   */").unwrap();
    writeln!(output, "  metric(metric, index) {{").unwrap();
    writeln!(output, "    return _endpoint(this, metric, index);").unwrap();
    writeln!(output, "  }}\n").unwrap();

    generate_api_methods(output, endpoints);

    writeln!(output, "}}\n").unwrap();

    writeln!(output, "export {{ BrkClient, BrkError }};").unwrap();
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

    let syntax = JavaScriptSyntax;
    if let TreeNode::Branch(children) = node {
        for (child_name, child_node) in children.iter() {
            match child_node {
                TreeNode::Leaf(leaf) => {
                    // Use shared helper for leaf fields
                    generate_leaf_field(
                        output,
                        &syntax,
                        "this",
                        child_name,
                        leaf,
                        metadata,
                        &indent_str,
                    );
                }
                TreeNode::Branch(grandchildren) => {
                    let field_name = to_camel_case(child_name);
                    let child_fields = get_node_fields(grandchildren, pattern_lookup);
                    // Use pattern factory if ANY pattern matches (not just parameterizable)
                    let pattern_name = pattern_lookup.get(&child_fields);

                    let base_result = get_pattern_instance_base(child_node);

                    // Use pattern factory only if no outlier was detected
                    if let Some(pattern_name) = pattern_name.filter(|_| !base_result.has_outlier) {
                        writeln!(
                            output,
                            "{}{}: create{}(this, '{}'),",
                            indent_str, field_name, pattern_name, base_result.base
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
                        writeln!(output, "{}}},", indent_str).unwrap();
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
