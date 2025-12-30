use std::{collections::HashSet, fmt::Write as FmtWrite, fs, io, path::Path};

use brk_types::{Index, TreeNode};

use crate::{
    ClientMetadata, Endpoint, FieldNamePosition, IndexSetPattern, PatternField, StructuralPattern,
    extract_inner_type, get_fields_with_child_info, get_node_fields, get_pattern_instance_base,
    to_pascal_case, to_snake_case,
};

/// Generate Rust client from metadata and OpenAPI endpoints.
///
/// `output_path` is the full path to the output file (e.g., "crates/brk_client/src/lib.rs").
pub fn generate_rust_client(
    metadata: &ClientMetadata,
    endpoints: &[Endpoint],
    output_path: &Path,
) -> io::Result<()> {
    let mut output = String::new();

    writeln!(output, "// Auto-generated BRK Rust client").unwrap();
    writeln!(output, "// Do not edit manually\n").unwrap();
    writeln!(output, "#![allow(non_camel_case_types)]").unwrap();
    writeln!(output, "#![allow(dead_code)]").unwrap();
    writeln!(output, "#![allow(unused_variables)]").unwrap();
    writeln!(output, "#![allow(clippy::useless_format)]").unwrap();
    writeln!(output, "#![allow(clippy::unnecessary_to_owned)]\n").unwrap();

    generate_imports(&mut output);
    generate_base_client(&mut output);
    generate_metric_node(&mut output);
    generate_index_accessors(&mut output, &metadata.index_set_patterns);
    generate_pattern_structs(&mut output, &metadata.structural_patterns, metadata);
    generate_tree(&mut output, &metadata.catalog, metadata);
    generate_main_client(&mut output, endpoints);

    fs::write(output_path, output)?;

    Ok(())
}

fn generate_imports(output: &mut String) {
    writeln!(
        output,
        r#"use std::sync::Arc;
use serde::de::DeserializeOwned;
pub use brk_cohort::*;
pub use brk_types::*;

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
    pub timeout_secs: u64,
}}

impl Default for BrkClientOptions {{
    fn default() -> Self {{
        Self {{
            base_url: "http://localhost:3000".to_string(),
            timeout_secs: 30,
        }}
    }}
}}

/// Base HTTP client for making requests.
#[derive(Debug, Clone)]
pub struct BrkClientBase {{
    base_url: String,
    timeout_secs: u64,
}}

impl BrkClientBase {{
    /// Create a new client with the given base URL.
    pub fn new(base_url: impl Into<String>) -> Self {{
        Self {{
            base_url: base_url.into(),
            timeout_secs: 30,
        }}
    }}

    /// Create a new client with options.
    pub fn with_options(options: BrkClientOptions) -> Self {{
        Self {{
            base_url: options.base_url,
            timeout_secs: options.timeout_secs,
        }}
    }}

    /// Make a GET request.
    pub fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {{
        let url = format!("{{}}{{}}", self.base_url, path);
        let response = minreq::get(&url)
            .with_timeout(self.timeout_secs)
            .send()
            .map_err(|e| BrkError {{ message: e.to_string() }})?;

        if response.status_code >= 400 {{
            return Err(BrkError {{
                message: format!("HTTP {{}}", response.status_code),
            }});
        }}

        response
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
pub struct MetricNode<T> {{
    client: Arc<BrkClientBase>,
    path: String,
    _marker: std::marker::PhantomData<T>,
}}

impl<T: DeserializeOwned> MetricNode<T> {{
    pub fn new(client: Arc<BrkClientBase>, path: String) -> Self {{
        Self {{
            client,
            path,
            _marker: std::marker::PhantomData,
        }}
    }}

    /// Fetch all data points for this metric.
    pub fn get(&self) -> Result<Vec<T>> {{
        self.client.get(&self.path)
    }}

    /// Fetch data points within a range.
    pub fn get_range(&self, from: Option<&str>, to: Option<&str>) -> Result<Vec<T>> {{
        let mut params = Vec::new();
        if let Some(f) = from {{ params.push(format!("from={{}}", f)); }}
        if let Some(t) = to {{ params.push(format!("to={{}}", t)); }}
        let path = if params.is_empty() {{
            self.path.clone()
        }} else {{
            format!("{{}}?{{}}", self.path, params.join("&"))
        }};
        self.client.get(&path)
    }}
}}

"#
    )
    .unwrap();
}

fn generate_index_accessors(output: &mut String, patterns: &[IndexSetPattern]) {
    if patterns.is_empty() {
        return;
    }

    writeln!(output, "// Index accessor structs\n").unwrap();

    for pattern in patterns {
        writeln!(
            output,
            "/// Index accessor for metrics with {} indexes.",
            pattern.indexes.len()
        )
        .unwrap();
        writeln!(output, "pub struct {}<T> {{", pattern.name).unwrap();

        for index in &pattern.indexes {
            let field_name = index_to_field_name(index);
            writeln!(output, "    pub {}: MetricNode<T>,", field_name).unwrap();
        }

        writeln!(output, "}}\n").unwrap();

        // Generate impl block with constructor
        writeln!(output, "impl<T: DeserializeOwned> {}<T> {{", pattern.name).unwrap();
        writeln!(
            output,
            "    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {{"
        )
        .unwrap();
        writeln!(output, "        Self {{").unwrap();

        for index in &pattern.indexes {
            let field_name = index_to_field_name(index);
            let path_segment = index.serialize_long();
            writeln!(
                output,
                "            {}: MetricNode::new(client.clone(), format!(\"{{base_path}}/{}\")),",
                field_name, path_segment
            )
            .unwrap();
        }

        writeln!(output, "        }}").unwrap();
        writeln!(output, "    }}").unwrap();
        writeln!(output, "}}\n").unwrap();
    }
}

fn index_to_field_name(index: &Index) -> String {
    format!("by_{}", to_snake_case(index.serialize_long()))
}

fn generate_pattern_structs(
    output: &mut String,
    patterns: &[StructuralPattern],
    metadata: &ClientMetadata,
) {
    if patterns.is_empty() {
        return;
    }

    writeln!(output, "// Reusable pattern structs\n").unwrap();

    for pattern in patterns {
        let is_parameterizable = pattern.is_parameterizable();
        let generic_params = if pattern.is_generic { "<T>" } else { "" };

        writeln!(output, "/// Pattern struct for repeated tree structure.").unwrap();
        writeln!(output, "pub struct {}{} {{", pattern.name, generic_params).unwrap();

        for field in &pattern.fields {
            let field_name = to_snake_case(&field.name);
            let type_annotation =
                field_to_type_annotation_generic(field, metadata, pattern.is_generic);
            writeln!(output, "    pub {}: {},", field_name, type_annotation).unwrap();
        }

        writeln!(output, "}}\n").unwrap();

        // Generate impl block with constructor
        let impl_generic = if pattern.is_generic {
            "<T: DeserializeOwned>"
        } else {
            ""
        };
        writeln!(
            output,
            "impl{} {}{} {{",
            impl_generic, pattern.name, generic_params
        )
        .unwrap();

        if is_parameterizable {
            writeln!(
                output,
                "    /// Create a new pattern node with accumulated metric name."
            )
            .unwrap();
            writeln!(
                output,
                "    pub fn new(client: Arc<BrkClientBase>, acc: &str) -> Self {{"
            )
            .unwrap();
        } else {
            writeln!(
                output,
                "    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {{"
            )
            .unwrap();
        }
        writeln!(output, "        Self {{").unwrap();

        for field in &pattern.fields {
            if is_parameterizable {
                generate_parameterized_rust_field(output, field, pattern, metadata);
            } else {
                generate_tree_path_rust_field(output, field, metadata);
            }
        }

        writeln!(output, "        }}").unwrap();
        writeln!(output, "    }}").unwrap();
        writeln!(output, "}}\n").unwrap();
    }
}

fn generate_parameterized_rust_field(
    output: &mut String,
    field: &PatternField,
    pattern: &StructuralPattern,
    metadata: &ClientMetadata,
) {
    let field_name = to_snake_case(&field.name);

    if metadata.is_pattern_type(&field.rust_type) {
        let child_acc = if let Some(pos) = pattern.get_field_position(&field.name) {
            match pos {
                FieldNamePosition::Append(suffix) => format!("&format!(\"{{acc}}{}\")", suffix),
                FieldNamePosition::Prepend(prefix) => format!("&format!(\"{}{{acc}}\")", prefix),
                FieldNamePosition::Identity => "acc".to_string(),
                FieldNamePosition::SetBase(base) => format!("\"{}\"", base),
            }
        } else {
            format!("&format!(\"{{acc}}_{}\")", field.name)
        };

        writeln!(
            output,
            "            {}: {}::new(client.clone(), {}),",
            field_name, field.rust_type, child_acc
        )
        .unwrap();
        return;
    }

    let metric_expr = if let Some(pos) = pattern.get_field_position(&field.name) {
        match pos {
            FieldNamePosition::Append(suffix) => format!("format!(\"{{acc}}{}\")", suffix),
            FieldNamePosition::Prepend(prefix) => format!("format!(\"{}{{acc}}\")", prefix),
            FieldNamePosition::Identity => "acc.to_string()".to_string(),
            FieldNamePosition::SetBase(base) => format!("\"{}\".to_string()", base),
        }
    } else {
        format!("format!(\"{{acc}}_{}\")", field.name)
    };

    if metadata.field_uses_accessor(field) {
        let accessor = metadata.find_index_set_pattern(&field.indexes).unwrap();
        writeln!(
            output,
            "            {}: {}::new(client.clone(), &{}),",
            field_name, accessor.name, metric_expr
        )
        .unwrap();
    } else {
        writeln!(
            output,
            "            {}: MetricNode::new(client.clone(), {}),",
            field_name, metric_expr
        )
        .unwrap();
    }
}

fn generate_tree_path_rust_field(
    output: &mut String,
    field: &PatternField,
    metadata: &ClientMetadata,
) {
    let field_name = to_snake_case(&field.name);

    if metadata.is_pattern_type(&field.rust_type) {
        writeln!(
            output,
            "            {}: {}::new(client.clone(), &format!(\"{{base_path}}_{}\")),",
            field_name, field.rust_type, field.name
        )
        .unwrap();
    } else if metadata.field_uses_accessor(field) {
        let accessor = metadata.find_index_set_pattern(&field.indexes).unwrap();
        writeln!(
            output,
            "            {}: {}::new(client.clone(), &format!(\"{{base_path}}_{}\")),",
            field_name, accessor.name, field.name
        )
        .unwrap();
    } else {
        writeln!(
            output,
            "            {}: MetricNode::new(client.clone(), format!(\"{{base_path}}_{}\")),",
            field_name, field.name
        )
        .unwrap();
    }
}

fn field_to_type_annotation_generic(
    field: &PatternField,
    metadata: &ClientMetadata,
    is_generic: bool,
) -> String {
    field_to_type_annotation_with_generic(field, metadata, is_generic, None)
}

fn field_to_type_annotation_with_generic(
    field: &PatternField,
    metadata: &ClientMetadata,
    is_generic: bool,
    generic_value_type: Option<&str>,
) -> String {
    let value_type = if is_generic && field.rust_type == "T" {
        "T".to_string()
    } else {
        extract_inner_type(&field.rust_type)
    };

    if metadata.is_pattern_type(&field.rust_type) {
        if metadata.is_pattern_generic(&field.rust_type) {
            // Use type_param from field, then generic_value_type, then T if parent is generic
            let type_param = field
                .type_param
                .as_deref()
                .or(generic_value_type)
                .unwrap_or(if is_generic { "T" } else { "_" });
            return format!("{}<{}>", field.rust_type, type_param);
        }
        field.rust_type.clone()
    } else if field.is_branch() {
        // Non-pattern branch struct
        field.rust_type.clone()
    } else if let Some(accessor) = metadata.find_index_set_pattern(&field.indexes) {
        format!("{}<{}>", accessor.name, value_type)
    } else {
        format!("MetricNode<{}>", value_type)
    }
}

fn generate_tree(output: &mut String, catalog: &TreeNode, metadata: &ClientMetadata) {
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
        let type_annotation =
            field_to_type_annotation_with_generic(field, metadata, false, generic_value_type);
        writeln!(output, "    pub {}: {},", field_name, type_annotation).unwrap();
    }

    writeln!(output, "}}\n").unwrap();

    writeln!(output, "impl {} {{", name).unwrap();
    writeln!(
        output,
        "    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {{"
    )
    .unwrap();
    writeln!(output, "        Self {{").unwrap();

    for (field, (child_name, child_node)) in fields.iter().zip(children.iter()) {
        let field_name = to_snake_case(&field.name);
        if metadata.is_pattern_type(&field.rust_type) {
            let pattern = metadata.find_pattern(&field.rust_type);
            let is_parameterizable = pattern.is_some_and(|p| p.is_parameterizable());

            if is_parameterizable {
                let metric_base = get_pattern_instance_base(child_node, child_name);
                writeln!(
                    output,
                    "            {}: {}::new(client.clone(), \"{}\"),",
                    field_name, field.rust_type, metric_base
                )
                .unwrap();
            } else {
                writeln!(
                    output,
                    "            {}: {}::new(client.clone(), &format!(\"{{base_path}}_{}\")),",
                    field_name, field.rust_type, field.name
                )
                .unwrap();
            }
        } else if metadata.field_uses_accessor(field) {
            let accessor = metadata.find_index_set_pattern(&field.indexes).unwrap();
            writeln!(
                output,
                "            {}: {}::new(client.clone(), &format!(\"{{base_path}}_{}\")),",
                field_name, accessor.name, field.name
            )
            .unwrap();
        } else if field.is_branch() {
            // Non-pattern branch - instantiate the nested struct
            writeln!(
                output,
                "            {}: {}::new(client.clone(), &format!(\"{{base_path}}_{}\")),",
                field_name, field.rust_type, field.name
            )
            .unwrap();
        } else {
            // Leaf - use MetricNode with base_path
            writeln!(
                output,
                "            {}: MetricNode::new(client.clone(), format!(\"{{base_path}}_{}\")),",
                field_name, field.name
            )
            .unwrap();
        }
    }

    writeln!(output, "        }}").unwrap();
    writeln!(output, "    }}").unwrap();
    writeln!(output, "}}\n").unwrap();

    for (child_name, child_node) in children {
        if let TreeNode::Branch(grandchildren) = child_node {
            let child_fields = get_node_fields(grandchildren, pattern_lookup);
            if !pattern_lookup.contains_key(&child_fields) {
                let child_struct_name = format!("{}_{}", name, to_pascal_case(child_name));
                generate_tree_node(
                    output,
                    &child_struct_name,
                    child_node,
                    pattern_lookup,
                    metadata,
                    generated,
                );
            }
        }
    }
}

fn generate_main_client(output: &mut String, endpoints: &[Endpoint]) {
    writeln!(
        output,
        r#"/// Main BRK client with catalog tree and API methods.
pub struct BrkClient {{
    base: Arc<BrkClientBase>,
    tree: CatalogTree,
}}

impl BrkClient {{
    /// Client version.
    pub const VERSION: &'static str = "v{VERSION}";

    /// Create a new client with the given base URL.
    pub fn new(base_url: impl Into<String>) -> Self {{
        let base = Arc::new(BrkClientBase::new(base_url));
        let tree = CatalogTree::new(base.clone(), "");
        Self {{ base, tree }}
    }}

    /// Create a new client with options.
    pub fn with_options(options: BrkClientOptions) -> Self {{
        let base = Arc::new(BrkClientBase::with_options(options));
        let tree = CatalogTree::new(base.clone(), "");
        Self {{ base, tree }}
    }}

    /// Get the catalog tree for navigating metrics.
    pub fn tree(&self) -> &CatalogTree {{
        &self.tree
    }}
"#,
        VERSION = crate::VERSION
    )
    .unwrap();

    generate_api_methods(output, endpoints);

    writeln!(output, "}}").unwrap();
}

fn generate_api_methods(output: &mut String, endpoints: &[Endpoint]) {
    for endpoint in endpoints {
        if !endpoint.should_generate() {
            continue;
        }

        let method_name = endpoint_to_method_name(endpoint);
        let return_type = endpoint
            .response_type
            .as_deref()
            .map(js_type_to_rust)
            .unwrap_or_else(|| "serde_json::Value".to_string());

        writeln!(
            output,
            "    /// {}",
            endpoint.summary.as_deref().unwrap_or(&method_name)
        )
        .unwrap();
        if let Some(desc) = &endpoint.description
            && endpoint.summary.as_ref() != Some(desc)
        {
            writeln!(output, "    ///").unwrap();
            writeln!(output, "    /// {}", desc).unwrap();
        }

        let params = build_method_params(endpoint);
        writeln!(
            output,
            "    pub fn {}(&self{}) -> Result<{}> {{",
            method_name, params, return_type
        )
        .unwrap();

        let path = build_path_template(&endpoint.path, &endpoint.path_params);

        if endpoint.query_params.is_empty() {
            writeln!(output, "        self.base.get(&format!(\"{}\"))", path).unwrap();
        } else {
            writeln!(output, "        let mut query = Vec::new();").unwrap();
            for param in &endpoint.query_params {
                if param.required {
                    writeln!(
                        output,
                        "        query.push(format!(\"{}={{}}\", {}));",
                        param.name, param.name
                    )
                    .unwrap();
                } else {
                    writeln!(
                        output,
                        "        if let Some(v) = {} {{ query.push(format!(\"{}={{}}\", v)); }}",
                        param.name, param.name
                    )
                    .unwrap();
                }
            }
            writeln!(output, "        let query_str = if query.is_empty() {{ String::new() }} else {{ format!(\"?{{}}\", query.join(\"&\")) }};").unwrap();
            writeln!(
                output,
                "        self.base.get(&format!(\"{}{{}}\", query_str))",
                path
            )
            .unwrap();
        }

        writeln!(output, "    }}\n").unwrap();
    }
}

fn endpoint_to_method_name(endpoint: &Endpoint) -> String {
    to_snake_case(&endpoint.operation_name())
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

fn js_type_to_rust(js_type: &str) -> String {
    if let Some(inner) = js_type.strip_suffix("[]") {
        format!("Vec<{}>", js_type_to_rust(inner))
    } else {
        match js_type {
            "string" => "String".to_string(),
            "number" => "f64".to_string(),
            "boolean" => "bool".to_string(),
            "*" => "serde_json::Value".to_string(),
            other => other.to_string(),
        }
    }
}
