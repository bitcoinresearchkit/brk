use std::fmt::Write as FmtWrite;
use std::fs;
use std::io;
use std::path::Path;

use brk_types::TreeNode;

use super::{schema_to_jsdoc, to_camel_case, ClientMetadata, IndexPattern};

/// Generate JavaScript + JSDoc client from metadata
pub fn generate_javascript_client(metadata: &ClientMetadata, output_dir: &Path) -> io::Result<()> {
    let mut output = String::new();

    // Header
    writeln!(output, "// Auto-generated BRK JavaScript client").unwrap();
    writeln!(output, "// Do not edit manually\n").unwrap();

    // Generate pattern JSDoc typedefs for index groupings
    generate_pattern_typedefs(&mut output, &metadata.patterns);

    // Generate the base client class
    generate_base_client(&mut output);

    // Generate tree JSDoc typedefs from catalog
    generate_tree_typedefs(&mut output, &metadata.catalog);

    // Generate the main client class with tree
    generate_main_client(&mut output, &metadata.catalog);

    fs::write(output_dir.join("client.js"), output)?;

    Ok(())
}

/// Generate JSDoc typedefs for common index patterns
fn generate_pattern_typedefs(output: &mut String, patterns: &[IndexPattern]) {
    writeln!(output, "// Index pattern typedefs").unwrap();
    writeln!(output, "// Reusable patterns for metrics with the same index groupings\n").unwrap();

    for pattern in patterns {
        let pattern_name = pattern_to_name(pattern);
        writeln!(output, "/**").unwrap();
        writeln!(output, " * @template T").unwrap();
        writeln!(output, " * @typedef {{Object}} {}", pattern_name).unwrap();

        for index in &pattern.indexes {
            let field_name = to_camel_case(&index.serialize_long());
            writeln!(output, " * @property {{MetricNode<T>}} {}", field_name).unwrap();
        }

        writeln!(output, " */\n").unwrap();
    }
}

/// Generate the base BrkClient class with HTTP functionality
fn generate_base_client(output: &mut String) {
    writeln!(
        output,
        r#"/**
 * @typedef {{Object}} BrkClientOptions
 * @property {{string}} baseUrl - The base URL for the API
 * @property {{number}} [timeout] - Request timeout in ms (default: 30000)
 */

/**
 * Base HTTP client
 */
class BrkClientBase {{
  /**
   * @param {{BrkClientOptions|string}} options
   */
  constructor(options) {{
    if (typeof options === 'string') {{
      this.baseUrl = options.replace(/\/$/, '');
      this.timeout = 30000;
    }} else {{
      this.baseUrl = options.baseUrl.replace(/\/$/, '');
      this.timeout = options.timeout ?? 30000;
    }}
  }}

  /**
   * @template T
   * @param {{string}} path
   * @returns {{Promise<T>}}
   */
  async get(path) {{
    const controller = new AbortController();
    const timeoutId = setTimeout(() => controller.abort(), this.timeout);

    try {{
      const response = await fetch(`${{this.baseUrl}}${{path}}`, {{
        signal: controller.signal,
        headers: {{ 'Accept': 'application/json' }},
      }});

      if (!response.ok) {{
        throw new BrkError(`HTTP ${{response.status}}: ${{response.statusText}}`, response.status);
      }}

      return await response.json();
    }} finally {{
      clearTimeout(timeoutId);
    }}
  }}
}}

/**
 * Error class for BRK API errors
 */
class BrkError extends Error {{
  /**
   * @param {{string}} message
   * @param {{number}} [statusCode]
   */
  constructor(message, statusCode) {{
    super(message);
    this.name = 'BrkError';
    this.statusCode = statusCode;
  }}
}}

/**
 * Metric node with fetch capability
 * @template T
 */
class MetricNode {{
  /**
   * @param {{BrkClientBase}} client
   * @param {{string}} path
   */
  constructor(client, path) {{
    this._client = client;
    this._path = path;
  }}

  /**
   * Fetch the metric value
   * @returns {{Promise<T>}}
   */
  async fetch() {{
    return this._client.get(this._path);
  }}

  toString() {{
    return this._path;
  }}
}}

"#
    )
    .unwrap();
}

/// Generate JSDoc typedefs for the catalog tree
fn generate_tree_typedefs(output: &mut String, catalog: &TreeNode) {
    writeln!(output, "// Catalog tree typedefs\n").unwrap();
    generate_node_typedef(output, "CatalogTree", catalog, "");
}

/// Recursively generate typedef for a tree node
fn generate_node_typedef(output: &mut String, name: &str, node: &TreeNode, path: &str) {
    match node {
        TreeNode::Leaf(_leaf) => {
            // Leaf nodes are MetricNode<ValueType>
            // No separate typedef needed, handled inline
        }
        TreeNode::Branch(children) => {
            writeln!(output, "/**").unwrap();
            writeln!(output, " * @typedef {{Object}} {}", name).unwrap();

            for (child_name, child_node) in children {
                let field_name = to_camel_case(child_name);

                match child_node {
                    TreeNode::Leaf(leaf) => {
                        let js_type = schema_to_jsdoc(&leaf.schema);
                        writeln!(
                            output,
                            " * @property {{MetricNode<{}>}} {}",
                            js_type, field_name
                        )
                        .unwrap();
                    }
                    TreeNode::Branch(_) => {
                        let child_type_name = format!("{}_{}", name, to_pascal_case(child_name));
                        writeln!(output, " * @property {{{}}} {}", child_type_name, field_name)
                            .unwrap();
                    }
                }
            }

            writeln!(output, " */\n").unwrap();

            // Generate child typedefs
            for (child_name, child_node) in children {
                if let TreeNode::Branch(_) = child_node {
                    let child_type_name = format!("{}_{}", name, to_pascal_case(child_name));
                    let child_path = if path.is_empty() {
                        format!("/{}", child_name)
                    } else {
                        format!("{}/{}", path, child_name)
                    };
                    generate_node_typedef(output, &child_type_name, child_node, &child_path);
                }
            }
        }
    }
}

/// Generate the main client class with initialized tree
fn generate_main_client(output: &mut String, catalog: &TreeNode) {
    writeln!(output, "/**").unwrap();
    writeln!(output, " * Main BRK client with catalog tree").unwrap();
    writeln!(output, " * @extends BrkClientBase").unwrap();
    writeln!(output, " */").unwrap();
    writeln!(output, "class BrkClient extends BrkClientBase {{").unwrap();
    writeln!(output, "  /**").unwrap();
    writeln!(output, "   * @param {{BrkClientOptions|string}} options").unwrap();
    writeln!(output, "   */").unwrap();
    writeln!(output, "  constructor(options) {{").unwrap();
    writeln!(output, "    super(options);").unwrap();
    writeln!(output, "    /** @type {{CatalogTree}} */").unwrap();
    writeln!(output, "    this.tree = this._buildTree();").unwrap();
    writeln!(output, "  }}\n").unwrap();

    // Generate _buildTree method
    writeln!(output, "  /**").unwrap();
    writeln!(output, "   * @private").unwrap();
    writeln!(output, "   * @returns {{CatalogTree}}").unwrap();
    writeln!(output, "   */").unwrap();
    writeln!(output, "  _buildTree() {{").unwrap();
    writeln!(output, "    return {{").unwrap();
    generate_tree_initializer(output, catalog, "", 3);
    writeln!(output, "    }};").unwrap();
    writeln!(output, "  }}").unwrap();
    writeln!(output, "}}\n").unwrap();

    // Export for ES modules
    writeln!(output, "export {{ BrkClient, BrkClientBase, BrkError, MetricNode }};").unwrap();
}

/// Generate the tree initializer code
fn generate_tree_initializer(output: &mut String, node: &TreeNode, path: &str, indent: usize) {
    let indent_str = "  ".repeat(indent);

    if let TreeNode::Branch(children) = node {
        for (i, (child_name, child_node)) in children.iter().enumerate() {
            let field_name = to_camel_case(child_name);
            let child_path = if path.is_empty() {
                format!("/{}", child_name)
            } else {
                format!("{}/{}", path, child_name)
            };

            let comma = if i < children.len() - 1 { "," } else { "" };

            match child_node {
                TreeNode::Leaf(_) => {
                    writeln!(
                        output,
                        "{}{}: new MetricNode(this, '{}'){}",
                        indent_str, field_name, child_path, comma
                    )
                    .unwrap();
                }
                TreeNode::Branch(_) => {
                    writeln!(output, "{}{}: {{", indent_str, field_name).unwrap();
                    generate_tree_initializer(output, child_node, &child_path, indent + 1);
                    writeln!(output, "{}}}{}", indent_str, comma).unwrap();
                }
            }
        }
    }
}

/// Convert pattern to a JSDoc typedef name
fn pattern_to_name(pattern: &IndexPattern) -> String {
    let index_names: Vec<String> = pattern
        .indexes
        .iter()
        .map(|i| to_pascal_case(&i.serialize_long()))
        .collect();
    format!("Pattern_{}", index_names.join("_"))
}

/// Convert string to PascalCase
fn to_pascal_case(s: &str) -> String {
    s.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect()
}
