//! JavaScript tree structure generation.

use std::collections::HashSet;
use std::fmt::Write;

use brk_types::TreeNode;

use crate::{
    ClientMetadata, Endpoint, GenericSyntax, JavaScriptSyntax, PatternField, build_child_path,
    generate_leaf_field, prepare_tree_node, to_camel_case,
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
        "",
        catalog,
        &pattern_lookup,
        metadata,
        &mut generated,
    );
}

fn generate_tree_typedef(
    output: &mut String,
    name: &str,
    path: &str,
    node: &TreeNode,
    pattern_lookup: &std::collections::HashMap<Vec<PatternField>, String>,
    metadata: &ClientMetadata,
    generated: &mut HashSet<String>,
) {
    let Some(ctx) = prepare_tree_node(node, name, path, pattern_lookup, metadata, generated) else {
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
            let child_path = build_child_path(path, child.name);
            generate_tree_typedef(
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
    let mut generated = HashSet::new();
    generate_tree_initializer(
        output,
        catalog,
        "MetricsTree",
        "",
        3,
        &pattern_lookup,
        metadata,
        &mut generated,
    );
    writeln!(output, "    }};").unwrap();
    writeln!(output, "  }}\n").unwrap();

    writeln!(output, "  /**").unwrap();
    writeln!(
        output,
        "   * Create a dynamic metric endpoint builder for any metric/index combination."
    )
    .unwrap();
    writeln!(output, "   *").unwrap();
    writeln!(
        output,
        "   * Use this for programmatic access when the metric name is determined at runtime."
    )
    .unwrap();
    writeln!(
        output,
        "   * For type-safe access, use the `metrics` tree instead."
    )
    .unwrap();
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

#[allow(clippy::too_many_arguments)]
fn generate_tree_initializer(
    output: &mut String,
    node: &TreeNode,
    name: &str,
    path: &str,
    indent: usize,
    pattern_lookup: &std::collections::HashMap<Vec<PatternField>, String>,
    metadata: &ClientMetadata,
    generated: &mut HashSet<String>,
) {
    let indent_str = "  ".repeat(indent);

    let Some(ctx) = prepare_tree_node(node, name, path, pattern_lookup, metadata, generated) else {
        return;
    };

    let syntax = JavaScriptSyntax;
    for child in &ctx.children {
        let field_name = to_camel_case(child.name);

        if child.is_leaf {
            if let TreeNode::Leaf(leaf) = child.node {
                generate_leaf_field(
                    output,
                    &syntax,
                    "this",
                    child.name,
                    leaf,
                    metadata,
                    &indent_str,
                );
            }
        } else if child.should_inline {
            // Inline object
            let child_path = build_child_path(path, child.name);
            writeln!(output, "{}{}: {{", indent_str, field_name).unwrap();
            generate_tree_initializer(
                output,
                child.node,
                &child.inline_type_name,
                &child_path,
                indent + 1,
                pattern_lookup,
                metadata,
                generated,
            );
            writeln!(output, "{}}},", indent_str).unwrap();
        } else {
            // Use pattern factory
            writeln!(
                output,
                "{}{}: create{}(this, '{}'),",
                indent_str, field_name, child.field.rust_type, child.base_result.base
            )
            .unwrap();
        }
    }
}
