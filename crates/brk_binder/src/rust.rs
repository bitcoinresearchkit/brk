use std::collections::HashSet;
use std::fmt::Write as FmtWrite;
use std::fs;
use std::io;
use std::path::Path;

use brk_types::{Index, TreeNode};

use crate::{ClientMetadata, Endpoint, IndexSetPattern, PatternField, StructuralPattern, get_node_fields, to_pascal_case, to_snake_case};

/// Generate Rust client from metadata and OpenAPI endpoints
pub fn generate_rust_client(
    metadata: &ClientMetadata,
    endpoints: &[Endpoint],
    output_dir: &Path,
) -> io::Result<()> {
    let mut output = String::new();

    // Header
    writeln!(output, "// Auto-generated BRK Rust client").unwrap();
    writeln!(output, "// Do not edit manually\n").unwrap();
    writeln!(output, "#![allow(non_camel_case_types)]").unwrap();
    writeln!(output, "#![allow(dead_code)]\n").unwrap();

    // Imports
    generate_imports(&mut output);

    // Generate base client
    generate_base_client(&mut output);

    // Generate MetricNode
    generate_metric_node(&mut output);

    // Generate index accessor structs (for each unique set of indexes)
    generate_index_accessors(&mut output, &metadata.index_set_patterns);

    // Generate pattern structs (reusable, appearing 2+ times)
    generate_pattern_structs(&mut output, &metadata.structural_patterns, metadata);

    // Generate tree - each node uses its pattern or is generated inline
    generate_tree(&mut output, &metadata.catalog, metadata);

    // Generate main client with API methods
    generate_main_client(&mut output, endpoints);

    fs::write(output_dir.join("client.rs"), output)?;

    Ok(())
}

fn generate_imports(output: &mut String) {
    writeln!(
        output,
        r#"use std::marker::PhantomData;
use serde::de::DeserializeOwned;
use brk_types::*;

"#
    )
    .unwrap();
}

fn generate_base_client(output: &mut String) {
    writeln!(
        output,
        r#"/// Error type for BRK client operations.
#[derive(Debug)]
pub struct BrkError {{
    pub message: String,
}}

impl std::fmt::Display for BrkError {{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
        write!(f, "{{}}", self.message)
    }}
}}

impl std::error::Error for BrkError {{}}

/// Result type for BRK client operations.
pub type Result<T> = std::result::Result<T, BrkError>;

/// Options for configuring the BRK client.
#[derive(Debug, Clone)]
pub struct BrkClientOptions {{
    pub base_url: String,
    pub timeout_ms: u64,
}}

impl Default for BrkClientOptions {{
    fn default() -> Self {{
        Self {{
            base_url: "http://localhost:3000".to_string(),
            timeout_ms: 30000,
        }}
    }}
}}

/// Base HTTP client for making requests.
#[derive(Debug, Clone)]
pub struct BrkClientBase {{
    base_url: String,
    client: reqwest::blocking::Client,
}}

impl BrkClientBase {{
    /// Create a new client with the given base URL.
    pub fn new(base_url: impl Into<String>) -> Result<Self> {{
        let base_url = base_url.into();
        let client = reqwest::blocking::Client::new();
        Ok(Self {{ base_url, client }})
    }}

    /// Create a new client with options.
    pub fn with_options(options: BrkClientOptions) -> Result<Self> {{
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_millis(options.timeout_ms))
            .build()
            .map_err(|e| BrkError {{ message: e.to_string() }})?;
        Ok(Self {{
            base_url: options.base_url,
            client,
        }})
    }}

    /// Make a GET request.
    pub fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {{
        let url = format!("{{}}{{}}", self.base_url, path);
        self.client
            .get(&url)
            .send()
            .map_err(|e| BrkError {{ message: e.to_string() }})?
            .json()
            .map_err(|e| BrkError {{ message: e.to_string() }})
    }}
}}

"#
    )
    .unwrap();
}

fn generate_metric_node(output: &mut String) {
    writeln!(
        output,
        r#"/// A metric node that can fetch data for different indexes.
pub struct MetricNode<'a, T> {{
    client: &'a BrkClientBase,
    path: String,
    _marker: PhantomData<T>,
}}

impl<'a, T: DeserializeOwned> MetricNode<'a, T> {{
    pub fn new(client: &'a BrkClientBase, path: String) -> Self {{
        Self {{
            client,
            path,
            _marker: PhantomData,
        }}
    }}

    /// Fetch all data points for this metric.
    pub fn get(&self) -> Result<Vec<T>> {{
        self.client.get(&self.path)
    }}

    /// Fetch data points within a date range.
    pub fn get_range(&self, from: &str, to: &str) -> Result<Vec<T>> {{
        let path = format!("{{}}?from={{}}&to={{}}", self.path, from, to);
        self.client.get(&path)
    }}
}}

"#
    )
    .unwrap();
}

/// Generate index accessor structs for each unique set of indexes
fn generate_index_accessors(output: &mut String, patterns: &[IndexSetPattern]) {
    if patterns.is_empty() {
        return;
    }

    writeln!(output, "// Index accessor structs\n").unwrap();

    for pattern in patterns {
        writeln!(output, "/// Index accessor for metrics with {} indexes.", pattern.indexes.len()).unwrap();
        writeln!(output, "pub struct {}<'a, T> {{", pattern.name).unwrap();

        for index in &pattern.indexes {
            let field_name = index_to_field_name(index);
            writeln!(output, "    pub {}: MetricNode<'a, T>,", field_name).unwrap();
        }

        writeln!(output, "    _marker: PhantomData<T>,").unwrap();
        writeln!(output, "}}\n").unwrap();

        // Generate impl block with constructor
        writeln!(output, "impl<'a, T: DeserializeOwned> {}<'a, T> {{", pattern.name).unwrap();
        writeln!(output, "    pub fn new(client: &'a BrkClientBase, base_path: &str) -> Self {{").unwrap();
        writeln!(output, "        Self {{").unwrap();

        for index in &pattern.indexes {
            let field_name = index_to_field_name(index);
            let path_segment = index.serialize_long();
            writeln!(
                output,
                "            {}: MetricNode::new(client, format!(\"{{base_path}}/{}\")),",
                field_name, path_segment
            ).unwrap();
        }

        writeln!(output, "            _marker: PhantomData,").unwrap();
        writeln!(output, "        }}").unwrap();
        writeln!(output, "    }}").unwrap();
        writeln!(output, "}}\n").unwrap();
    }
}

/// Convert an Index to a snake_case field name (e.g., DateIndex -> by_date_index)
fn index_to_field_name(index: &Index) -> String {
    format!("by_{}", to_snake_case(index.serialize_long()))
}

/// Generate pattern structs (those appearing 2+ times)
fn generate_pattern_structs(output: &mut String, patterns: &[StructuralPattern], metadata: &ClientMetadata) {
    if patterns.is_empty() {
        return;
    }

    writeln!(output, "// Reusable pattern structs\n").unwrap();

    for pattern in patterns {
        writeln!(output, "/// Pattern struct for repeated tree structure.").unwrap();
        writeln!(output, "pub struct {}<'a> {{", pattern.name).unwrap();

        for field in &pattern.fields {
            let field_name = to_snake_case(&field.name);
            let type_annotation = field_to_type_annotation(field, metadata);
            writeln!(output, "    pub {}: {},", field_name, type_annotation).unwrap();
        }

        writeln!(output, "}}\n").unwrap();

        // Generate impl block with constructor
        writeln!(output, "impl<'a> {}<'a> {{", pattern.name).unwrap();
        writeln!(output, "    pub fn new(client: &'a BrkClientBase, base_path: &str) -> Self {{").unwrap();
        writeln!(output, "        Self {{").unwrap();

        for field in &pattern.fields {
            let field_name = to_snake_case(&field.name);
            if metadata.is_pattern_type(&field.rust_type) {
                writeln!(
                    output,
                    "            {}: {}::new(client, &format!(\"{{base_path}}/{}\"))," ,
                    field_name, field.rust_type, field.name
                ).unwrap();
            } else if field_uses_accessor(field, metadata) {
                let accessor = metadata.find_index_set_pattern(&field.indexes).unwrap();
                writeln!(
                    output,
                    "            {}: {}::new(client, &format!(\"{{base_path}}/{}\"))," ,
                    field_name, accessor.name, field.name
                ).unwrap();
            } else {
                writeln!(
                    output,
                    "            {}: MetricNode::new(client, format!(\"{{base_path}}/{}\"))," ,
                    field_name, field.name
                ).unwrap();
            }
        }

        writeln!(output, "        }}").unwrap();
        writeln!(output, "    }}").unwrap();
        writeln!(output, "}}\n").unwrap();
    }
}

/// Convert a PatternField to the full type annotation
fn field_to_type_annotation(field: &PatternField, metadata: &ClientMetadata) -> String {
    if metadata.is_pattern_type(&field.rust_type) {
        format!("{}<'a>", field.rust_type)
    } else if let Some(accessor) = metadata.find_index_set_pattern(&field.indexes) {
        // Leaf with a reusable accessor pattern
        format!("{}<'a, {}>", accessor.name, field.rust_type)
    } else {
        // Leaf with unique index set - use MetricNode directly
        format!("MetricNode<'a, {}>", field.rust_type)
    }
}

/// Check if a field should use an index accessor
fn field_uses_accessor(field: &PatternField, metadata: &ClientMetadata) -> bool {
    metadata.find_index_set_pattern(&field.indexes).is_some()
}

/// Generate the catalog tree structure
fn generate_tree(
    output: &mut String,
    catalog: &TreeNode,
    metadata: &ClientMetadata,
) {
    writeln!(output, "// Catalog tree\n").unwrap();

    let pattern_lookup = metadata.pattern_lookup();
    let mut generated = HashSet::new();
    generate_tree_node(output, "CatalogTree", catalog, &pattern_lookup, metadata, &mut generated);
}

/// Recursively generate tree nodes
fn generate_tree_node(
    output: &mut String,
    name: &str,
    node: &TreeNode,
    pattern_lookup: &std::collections::HashMap<Vec<PatternField>, String>,
    metadata: &ClientMetadata,
    generated: &mut HashSet<String>,
) {
    if let TreeNode::Branch(children) = node {
        // Build the signature for this node
        let mut fields: Vec<PatternField> = children
            .iter()
            .map(|(child_name, child_node)| {
                let (rust_type, json_type, indexes) = match child_node {
                    TreeNode::Leaf(leaf) => (
                        leaf.value_type().to_string(),
                        leaf.schema.get("type").and_then(|v| v.as_str()).unwrap_or("object").to_string(),
                        leaf.indexes().clone(),
                    ),
                    TreeNode::Branch(grandchildren) => {
                        // Get pattern name for this child
                        let child_fields = get_node_fields(grandchildren, pattern_lookup);
                        let pattern_name = pattern_lookup
                            .get(&child_fields)
                            .cloned()
                            .unwrap_or_else(|| format!("{}_{}", name, to_pascal_case(child_name)));
                        (pattern_name.clone(), pattern_name, std::collections::BTreeSet::new())
                    }
                };
                PatternField {
                    name: child_name.clone(),
                    rust_type,
                    json_type,
                    indexes,
                }
            })
            .collect();
        fields.sort_by(|a, b| a.name.cmp(&b.name));

        // Check if this matches a reusable pattern
        if let Some(pattern_name) = pattern_lookup.get(&fields) {
            // This node matches a pattern that will be generated separately
            // Don't generate it here, it's already in pattern_structs
            if pattern_name != name {
                return;
            }
        }

        // Generate this struct if not already generated
        if generated.contains(name) {
            return;
        }
        generated.insert(name.to_string());

        writeln!(output, "/// Catalog tree node.").unwrap();
        writeln!(output, "pub struct {}<'a> {{", name).unwrap();

        for field in &fields {
            let field_name = to_snake_case(&field.name);
            let type_annotation = field_to_type_annotation(field, metadata);
            writeln!(output, "    pub {}: {},", field_name, type_annotation).unwrap();
        }

        writeln!(output, "}}\n").unwrap();

        // Generate impl block
        writeln!(output, "impl<'a> {}<'a> {{", name).unwrap();
        writeln!(output, "    pub fn new(client: &'a BrkClientBase, base_path: &str) -> Self {{").unwrap();
        writeln!(output, "        Self {{").unwrap();

        for field in &fields {
            let field_name = to_snake_case(&field.name);
            if metadata.is_pattern_type(&field.rust_type) {
                writeln!(
                    output,
                    "            {}: {}::new(client, &format!(\"{{base_path}}/{}\"))," ,
                    field_name, field.rust_type, field.name
                ).unwrap();
            } else if field_uses_accessor(field, metadata) {
                let accessor = metadata.find_index_set_pattern(&field.indexes).unwrap();
                writeln!(
                    output,
                    "            {}: {}::new(client, &format!(\"{{base_path}}/{}\"))," ,
                    field_name, accessor.name, field.name
                ).unwrap();
            } else {
                writeln!(
                    output,
                    "            {}: MetricNode::new(client, format!(\"{{base_path}}/{}\"))," ,
                    field_name, field.name
                ).unwrap();
            }
        }

        writeln!(output, "        }}").unwrap();
        writeln!(output, "    }}").unwrap();
        writeln!(output, "}}\n").unwrap();

        // Recursively generate child nodes that aren't patterns
        for (child_name, child_node) in children {
            if let TreeNode::Branch(grandchildren) = child_node {
                let child_fields = get_node_fields(grandchildren, pattern_lookup);
                if !pattern_lookup.contains_key(&child_fields) {
                    let child_struct_name = format!("{}_{}", name, to_pascal_case(child_name));
                    generate_tree_node(output, &child_struct_name, child_node, pattern_lookup, metadata, generated);
                }
            }
        }
    }
}

/// Generate the main client struct
fn generate_main_client(output: &mut String, endpoints: &[Endpoint]) {
    writeln!(
        output,
        r#"/// Main BRK client with catalog tree and API methods.
pub struct BrkClient {{
    base: BrkClientBase,
}}

impl BrkClient {{
    /// Create a new client with the given base URL.
    pub fn new(base_url: impl Into<String>) -> Result<Self> {{
        Ok(Self {{
            base: BrkClientBase::new(base_url)?,
        }})
    }}

    /// Create a new client with options.
    pub fn with_options(options: BrkClientOptions) -> Result<Self> {{
        Ok(Self {{
            base: BrkClientBase::with_options(options)?,
        }})
    }}

    /// Get the catalog tree for navigating metrics.
    pub fn tree(&self) -> CatalogTree<'_> {{
        CatalogTree::new(&self.base, "")
    }}
"#
    )
    .unwrap();

    // Generate API methods
    generate_api_methods(output, endpoints);

    writeln!(output, "}}").unwrap();
}

/// Generate API methods from OpenAPI endpoints
fn generate_api_methods(output: &mut String, endpoints: &[Endpoint]) {
    for endpoint in endpoints {
        if endpoint.method != "GET" {
            continue;
        }

        let method_name = endpoint_to_method_name(endpoint);
        let return_type = endpoint.response_type.as_deref().unwrap_or("serde_json::Value");

        // Build doc comment
        writeln!(output, "    /// {}", endpoint.summary.as_deref().unwrap_or(&method_name)).unwrap();

        // Build method signature
        let params = build_method_params(endpoint);
        writeln!(output, "    pub fn {}(&self{}) -> Result<{}> {{", method_name, params, return_type).unwrap();

        // Build path
        let path = build_path_template(&endpoint.path, &endpoint.path_params);

        if endpoint.query_params.is_empty() {
            writeln!(output, "        self.base.get(&format!(\"{}\"))", path).unwrap();
        } else {
            writeln!(output, "        let mut query = Vec::new();").unwrap();
            for param in &endpoint.query_params {
                if param.required {
                    writeln!(output, "        query.push(format!(\"{}={{}}\", {}));", param.name, param.name).unwrap();
                } else {
                    writeln!(output, "        if let Some(v) = {} {{ query.push(format!(\"{}={{}}\", v)); }}", param.name, param.name).unwrap();
                }
            }
            writeln!(output, "        let query_str = if query.is_empty() {{ String::new() }} else {{ format!(\"?{{}}\", query.join(\"&\")) }};").unwrap();
            writeln!(output, "        self.base.get(&format!(\"{}{{}}\", query_str))", path).unwrap();
        }

        writeln!(output, "    }}\n").unwrap();
    }
}

fn endpoint_to_method_name(endpoint: &Endpoint) -> String {
    if let Some(op_id) = &endpoint.operation_id {
        return to_snake_case(op_id);
    }
    let parts: Vec<&str> = endpoint.path.split('/').filter(|s| !s.is_empty() && !s.starts_with('{')).collect();
    format!("get_{}", parts.join("_"))
}

fn build_method_params(endpoint: &Endpoint) -> String {
    let mut params = Vec::new();
    for param in &endpoint.path_params {
        params.push(format!(", {}: &str", param.name));
    }
    for param in &endpoint.query_params {
        if param.required {
            params.push(format!(", {}: &str", param.name));
        } else {
            params.push(format!(", {}: Option<&str>", param.name));
        }
    }
    params.join("")
}

fn build_path_template(path: &str, path_params: &[super::Parameter]) -> String {
    let mut result = path.to_string();
    for param in path_params {
        let placeholder = format!("{{{}}}", param.name);
        let interpolation = format!("{{{}}}", param.name);
        result = result.replace(&placeholder, &interpolation);
    }
    result
}
