use std::collections::BTreeMap;
use std::fmt::Write as FmtWrite;
use std::fs;
use std::io;
use std::path::Path;

use brk_types::{MetricLeaf, TreeNode};

use super::{to_camel_case, ClientMetadata, IndexPattern};

/// Generate TypeScript client from metadata
pub fn generate_typescript_client(metadata: &ClientMetadata, output_dir: &Path) -> io::Result<()> {
    let mut output = String::new();

    // Header
    writeln!(output, "// Auto-generated BRK TypeScript client").unwrap();
    writeln!(output, "// Do not edit manually\n").unwrap();

    // Generate pattern interfaces for index groupings
    generate_pattern_interfaces(&mut output, &metadata.patterns);

    // Generate value type aliases
    generate_value_types(&mut output, metadata);

    // Generate the base client class
    generate_base_client(&mut output);

    // Generate tree types from catalog
    generate_tree_types(&mut output, &metadata.catalog);

    // Generate the main client class with tree
    generate_main_client(&mut output, &metadata.catalog);

    fs::write(output_dir.join("client.ts"), output)?;

    Ok(())
}

/// Generate TypeScript interfaces for common index patterns
fn generate_pattern_interfaces(output: &mut String, patterns: &[IndexPattern]) {
    writeln!(output, "// Index pattern interfaces").unwrap();
    writeln!(output, "// Reusable patterns for metrics with the same index groupings\n").unwrap();

    for pattern in patterns {
        let pattern_name = pattern_to_name(pattern);
        writeln!(output, "export interface {}<T> {{", pattern_name).unwrap();

        for index in &pattern.indexes {
            let field_name = to_camel_case(&index.serialize_long());
            writeln!(output, "  {}: MetricNode<T>;", field_name).unwrap();
        }

        writeln!(output, "}}\n").unwrap();
    }
}

/// Generate TypeScript type aliases for value types
fn generate_value_types(output: &mut String, metadata: &ClientMetadata) {
    writeln!(output, "// Value type aliases").unwrap();
    writeln!(output, "// Maps Rust types to TypeScript types\n").unwrap();

    // Collect unique value types
    let mut value_types: Vec<&str> = metadata
        .metrics
        .values()
        .map(|m| m.value_type.as_str())
        .collect();
    value_types.sort();
    value_types.dedup();

    for vt in value_types {
        let ts_type = rust_type_to_ts(vt);
        writeln!(output, "export type {} = {};", vt, ts_type).unwrap();
    }
    writeln!(output).unwrap();
}

/// Generate the base BrkClient class with HTTP functionality
fn generate_base_client(output: &mut String) {
    writeln!(
        output,
        r#"// Base HTTP client
export interface BrkClientOptions {{
  baseUrl: string;
  timeout?: number;
}}

export class BrkClientBase {{
  private baseUrl: string;
  private timeout: number;

  constructor(options: BrkClientOptions | string) {{
    if (typeof options === 'string') {{
      this.baseUrl = options.replace(/\/$/, '');
      this.timeout = 30000;
    }} else {{
      this.baseUrl = options.baseUrl.replace(/\/$/, '');
      this.timeout = options.timeout ?? 30000;
    }}
  }}

  async get<T>(path: string): Promise<T> {{
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

export class BrkError extends Error {{
  constructor(message: string, public statusCode?: number) {{
    super(message);
    this.name = 'BrkError';
  }}
}}

// Metric node with fetch capability
export class MetricNode<T> {{
  constructor(private client: BrkClientBase, private path: string) {{}}

  async fetch(): Promise<T> {{
    return this.client.get<T>(this.path);
  }}

  toString(): string {{
    return this.path;
  }}
}}

"#
    )
    .unwrap();
}

/// Generate TypeScript types for the catalog tree
fn generate_tree_types(output: &mut String, catalog: &TreeNode) {
    writeln!(output, "// Catalog tree types\n").unwrap();
    generate_node_type(output, "CatalogTree", catalog, "");
}

/// Recursively generate type for a tree node
fn generate_node_type(output: &mut String, name: &str, node: &TreeNode, path: &str) {
    match node {
        TreeNode::Leaf(leaf) => {
            // Leaf nodes are MetricNode<ValueType>
            // No separate interface needed, handled inline
        }
        TreeNode::Branch(children) => {
            writeln!(output, "export interface {} {{", name).unwrap();

            for (child_name, child_node) in children {
                let field_name = to_camel_case(child_name);
                let child_path = if path.is_empty() {
                    format!("/{}", child_name)
                } else {
                    format!("{}/{}", path, child_name)
                };

                match child_node {
                    TreeNode::Leaf(leaf) => {
                        let value_type = &leaf.value_type;
                        writeln!(output, "  {}: MetricNode<{}>;", field_name, value_type).unwrap();
                    }
                    TreeNode::Branch(_) => {
                        let child_type_name = format!("{}_{}", name, to_pascal_case(child_name));
                        writeln!(output, "  {}: {};", field_name, child_type_name).unwrap();
                    }
                }
            }

            writeln!(output, "}}\n").unwrap();

            // Generate child types
            for (child_name, child_node) in children {
                if let TreeNode::Branch(_) = child_node {
                    let child_type_name = format!("{}_{}", name, to_pascal_case(child_name));
                    let child_path = if path.is_empty() {
                        format!("/{}", child_name)
                    } else {
                        format!("{}/{}", path, child_name)
                    };
                    generate_node_type(output, &child_type_name, child_node, &child_path);
                }
            }
        }
    }
}

/// Generate the main client class with initialized tree
fn generate_main_client(output: &mut String, catalog: &TreeNode) {
    writeln!(output, "// Main client class with catalog tree").unwrap();
    writeln!(output, "export class BrkClient extends BrkClientBase {{").unwrap();
    writeln!(output, "  readonly tree: CatalogTree;\n").unwrap();
    writeln!(output, "  constructor(options: BrkClientOptions | string) {{").unwrap();
    writeln!(output, "    super(options);").unwrap();
    writeln!(output, "    this.tree = this._buildTree();").unwrap();
    writeln!(output, "  }}\n").unwrap();

    // Generate _buildTree method
    writeln!(output, "  private _buildTree(): CatalogTree {{").unwrap();
    writeln!(output, "    return {{").unwrap();
    generate_tree_initializer(output, catalog, "", 3);
    writeln!(output, "    }};").unwrap();
    writeln!(output, "  }}").unwrap();
    writeln!(output, "}}").unwrap();
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
                TreeNode::Leaf(leaf) => {
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

/// Convert pattern to a TypeScript interface name
fn pattern_to_name(pattern: &IndexPattern) -> String {
    let index_names: Vec<String> = pattern
        .indexes
        .iter()
        .map(|i| to_pascal_case(&i.serialize_long()))
        .collect();
    format!("Pattern_{}", index_names.join("_"))
}

/// Convert Rust type name to TypeScript type
fn rust_type_to_ts(rust_type: &str) -> &'static str {
    match rust_type {
        // Numeric types
        "f32" | "f64" | "StoredF32" | "StoredF64" => "number",
        "u8" | "u16" | "u32" | "u64" | "i8" | "i16" | "i32" | "i64" => "number",
        "usize" | "isize" => "number",

        // Boolean
        "bool" => "boolean",

        // String types
        "String" | "str" => "string",

        // Bitcoin types (typically numeric or string representations)
        "Sats" | "SatsPerVbyte" | "WU" | "VBytes" => "number",
        "Height" | "Timestamp" => "number",
        "Blockhash" | "Txid" => "string",

        // Arrays/Vecs become arrays
        _ if rust_type.starts_with("Vec<") => "unknown[]",

        // Default to unknown for unmapped types
        _ => "unknown",
    }
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
