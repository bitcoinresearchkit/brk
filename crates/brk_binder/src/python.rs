use std::collections::HashSet;
use std::fmt::Write as FmtWrite;
use std::fs;
use std::io;
use std::path::Path;

use brk_types::{Index, TreeNode};
use serde_json::Value;

use crate::{
    ClientMetadata, Endpoint, IndexSetPattern, PatternField, StructuralPattern, TypeSchemas,
    get_node_fields, to_pascal_case, to_snake_case,
};

/// Generate Python client from metadata and OpenAPI endpoints
pub fn generate_python_client(
    metadata: &ClientMetadata,
    endpoints: &[Endpoint],
    schemas: &TypeSchemas,
    output_dir: &Path,
) -> io::Result<()> {
    let mut output = String::new();

    // Header
    writeln!(output, "# Auto-generated BRK Python client").unwrap();
    writeln!(output, "# Do not edit manually\n").unwrap();
    writeln!(output, "from __future__ import annotations").unwrap();
    writeln!(
        output,
        "from typing import TypeVar, Generic, Any, Optional, List, TypedDict"
    )
    .unwrap();
    writeln!(output, "import httpx\n").unwrap();

    // Type variable for generic MetricNode
    writeln!(output, "T = TypeVar('T')\n").unwrap();

    // Generate type definitions from OpenAPI schemas
    generate_type_definitions(&mut output, schemas);

    // Generate base client class
    generate_base_client(&mut output);

    // Generate MetricNode class
    generate_metric_node(&mut output);

    // Generate index accessor classes
    generate_index_accessors(&mut output, &metadata.index_set_patterns);

    // Generate structural pattern classes
    generate_structural_patterns(&mut output, &metadata.structural_patterns, metadata);

    // Generate tree classes
    generate_tree_classes(&mut output, &metadata.catalog, metadata);

    // Generate main client with tree and API methods
    generate_main_client(&mut output, endpoints);

    fs::write(output_dir.join("client.py"), output)?;

    Ok(())
}

/// Generate Python type definitions from OpenAPI schemas
fn generate_type_definitions(output: &mut String, schemas: &TypeSchemas) {
    if schemas.is_empty() {
        return;
    }

    writeln!(output, "# Type definitions\n").unwrap();

    for (name, schema) in schemas {
        if let Some(props) = schema.get("properties").and_then(|p| p.as_object()) {
            // Object type -> TypedDict
            writeln!(output, "class {}(TypedDict):", name).unwrap();
            for (prop_name, prop_schema) in props {
                let prop_type = schema_to_python_type(prop_schema);
                writeln!(output, "    {}: {}", prop_name, prop_type).unwrap();
            }
            writeln!(output).unwrap();
        } else {
            // Primitive type alias
            let py_type = schema_to_python_type(schema);
            writeln!(output, "{} = {}", name, py_type).unwrap();
        }
    }
    writeln!(output).unwrap();
}

/// Convert JSON Schema to Python type
fn schema_to_python_type(schema: &Value) -> String {
    // Handle $ref
    if let Some(ref_path) = schema.get("$ref").and_then(|r| r.as_str()) {
        return ref_path.rsplit('/').next().unwrap_or("Any").to_string();
    }

    // Handle type field
    if let Some(ty) = schema.get("type").and_then(|t| t.as_str()) {
        return match ty {
            "integer" => "int".to_string(),
            "number" => "float".to_string(),
            "boolean" => "bool".to_string(),
            "string" => "str".to_string(),
            "null" => "None".to_string(),
            "array" => {
                let item_type = schema
                    .get("items")
                    .map(schema_to_python_type)
                    .unwrap_or_else(|| "Any".to_string());
                format!("List[{}]", item_type)
            }
            "object" => "dict".to_string(),
            _ => "Any".to_string(),
        };
    }

    // Handle anyOf/oneOf
    if let Some(variants) = schema
        .get("anyOf")
        .or_else(|| schema.get("oneOf"))
        .and_then(|v| v.as_array())
    {
        let types: Vec<String> = variants.iter().map(schema_to_python_type).collect();
        return types.join(" | ");
    }

    "Any".to_string()
}

/// Generate the base BrkClient class with HTTP functionality
fn generate_base_client(output: &mut String) {
    writeln!(
        output,
        r#"class BrkError(Exception):
    """Custom error class for BRK client errors."""

    def __init__(self, message: str, status: Optional[int] = None):
        super().__init__(message)
        self.status = status


class BrkClientBase:
    """Base HTTP client for making requests."""

    def __init__(self, base_url: str, timeout: float = 30.0):
        self.base_url = base_url
        self.timeout = timeout
        self._client = httpx.Client(timeout=timeout)

    def get(self, path: str) -> Any:
        """Make a GET request."""
        try:
            response = self._client.get(f"{{self.base_url}}{{path}}")
            response.raise_for_status()
            return response.json()
        except httpx.HTTPStatusError as e:
            raise BrkError(f"HTTP error: {{e.response.status_code}}", e.response.status_code)
        except httpx.RequestError as e:
            raise BrkError(str(e))

    def close(self):
        """Close the HTTP client."""
        self._client.close()

    def __enter__(self):
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        self.close()

"#
    )
    .unwrap();
}

/// Generate the MetricNode class
fn generate_metric_node(output: &mut String) {
    writeln!(
        output,
        r#"class MetricNode(Generic[T]):
    """A metric node that can fetch data for different indexes."""

    def __init__(self, client: BrkClientBase, path: str):
        self._client = client
        self._path = path

    def get(self) -> List[T]:
        """Fetch all data points for this metric."""
        return self._client.get(self._path)

    def get_range(self, from_date: str, to_date: str) -> List[T]:
        """Fetch data points within a date range."""
        return self._client.get(f"{{self._path}}?from={{from_date}}&to={{to_date}}")

"#
    )
    .unwrap();
}

/// Generate index accessor classes
fn generate_index_accessors(output: &mut String, patterns: &[IndexSetPattern]) {
    if patterns.is_empty() {
        return;
    }

    writeln!(output, "# Index accessor classes\n").unwrap();

    for pattern in patterns {
        writeln!(output, "class {}(Generic[T]):", pattern.name).unwrap();
        writeln!(
            output,
            "    \"\"\"Index accessor for metrics with {} indexes.\"\"\"",
            pattern.indexes.len()
        )
        .unwrap();
        writeln!(output, "    ").unwrap();
        writeln!(
            output,
            "    def __init__(self, client: BrkClientBase, base_path: str):"
        )
        .unwrap();

        for index in &pattern.indexes {
            let field_name = index_to_snake_case(index);
            let path_segment = index.serialize_long();
            writeln!(
                output,
                "        self.{}: MetricNode[T] = MetricNode(client, f'{{base_path}}/{}')",
                field_name, path_segment
            )
            .unwrap();
        }

        writeln!(output).unwrap();
    }
}

/// Convert an Index to a snake_case field name (e.g., DateIndex -> by_date_index)
fn index_to_snake_case(index: &Index) -> String {
    format!("by_{}", to_snake_case(index.serialize_long()))
}

/// Generate structural pattern classes
fn generate_structural_patterns(
    output: &mut String,
    patterns: &[StructuralPattern],
    metadata: &ClientMetadata,
) {
    if patterns.is_empty() {
        return;
    }

    writeln!(output, "# Reusable structural pattern classes\n").unwrap();

    for pattern in patterns {
        writeln!(output, "class {}:", pattern.name).unwrap();
        writeln!(
            output,
            "    \"\"\"Pattern struct for repeated tree structure.\"\"\""
        )
        .unwrap();
        writeln!(output, "    ").unwrap();
        writeln!(
            output,
            "    def __init__(self, client: BrkClientBase, base_path: str):"
        )
        .unwrap();

        for field in &pattern.fields {
            let py_type = field_to_python_type(field, metadata);
            if metadata.is_pattern_type(&field.rust_type) {
                writeln!(
                    output,
                    "        self.{}: {} = {}(client, f'{{base_path}}/{}')",
                    to_snake_case(&field.name),
                    py_type,
                    field.rust_type,
                    field.name
                )
                .unwrap();
            } else if field_uses_accessor(field, metadata) {
                let accessor = metadata.find_index_set_pattern(&field.indexes).unwrap();
                writeln!(
                    output,
                    "        self.{}: {} = {}(client, f'{{base_path}}/{}')",
                    to_snake_case(&field.name),
                    py_type,
                    accessor.name,
                    field.name
                )
                .unwrap();
            } else {
                writeln!(
                    output,
                    "        self.{}: {} = MetricNode(client, f'{{base_path}}/{}')",
                    to_snake_case(&field.name),
                    py_type,
                    field.name
                )
                .unwrap();
            }
        }

        writeln!(output).unwrap();
    }
}

/// Convert pattern field to Python type annotation
fn field_to_python_type(field: &PatternField, metadata: &ClientMetadata) -> String {
    if metadata.is_pattern_type(&field.rust_type) {
        // Pattern type - use pattern name directly
        field.rust_type.clone()
    } else if let Some(accessor) = metadata.find_index_set_pattern(&field.indexes) {
        // Leaf with accessor - use rust_type as the generic (e.g., DateIndexAccessor[Height])
        format!("{}[{}]", accessor.name, field.rust_type)
    } else {
        // Leaf - use rust_type as the generic (e.g., MetricNode[Height])
        format!("MetricNode[{}]", field.rust_type)
    }
}

/// Check if a field should use an index accessor
fn field_uses_accessor(field: &PatternField, metadata: &ClientMetadata) -> bool {
    metadata.find_index_set_pattern(&field.indexes).is_some()
}

/// Generate tree classes
fn generate_tree_classes(output: &mut String, catalog: &TreeNode, metadata: &ClientMetadata) {
    writeln!(output, "# Catalog tree classes\n").unwrap();

    let pattern_lookup = metadata.pattern_lookup();
    let mut generated = HashSet::new();
    generate_tree_class(
        output,
        "CatalogTree",
        catalog,
        &pattern_lookup,
        metadata,
        &mut generated,
    );
}

/// Recursively generate tree classes
fn generate_tree_class(
    output: &mut String,
    name: &str,
    node: &TreeNode,
    pattern_lookup: &std::collections::HashMap<Vec<PatternField>, String>,
    metadata: &ClientMetadata,
    generated: &mut HashSet<String>,
) {
    if let TreeNode::Branch(children) = node {
        // Build signature
        let fields = get_node_fields(children, pattern_lookup);

        // Skip if this matches a pattern (already generated)
        if pattern_lookup.contains_key(&fields)
            && pattern_lookup.get(&fields) != Some(&name.to_string())
        {
            return;
        }

        if generated.contains(name) {
            return;
        }
        generated.insert(name.to_string());

        writeln!(output, "class {}:", name).unwrap();
        writeln!(output, "    \"\"\"Catalog tree node.\"\"\"").unwrap();
        writeln!(output, "    ").unwrap();
        writeln!(
            output,
            "    def __init__(self, client: BrkClientBase, base_path: str = ''):"
        )
        .unwrap();

        for field in &fields {
            let py_type = field_to_python_type(field, metadata);
            if metadata.is_pattern_type(&field.rust_type) {
                writeln!(
                    output,
                    "        self.{}: {} = {}(client, f'{{base_path}}/{}')",
                    to_snake_case(&field.name),
                    py_type,
                    field.rust_type,
                    field.name
                )
                .unwrap();
            } else if field_uses_accessor(field, metadata) {
                let accessor = metadata.find_index_set_pattern(&field.indexes).unwrap();
                writeln!(
                    output,
                    "        self.{}: {} = {}(client, f'{{base_path}}/{}')",
                    to_snake_case(&field.name),
                    py_type,
                    accessor.name,
                    field.name
                )
                .unwrap();
            } else {
                writeln!(
                    output,
                    "        self.{}: {} = MetricNode(client, f'{{base_path}}/{}')",
                    to_snake_case(&field.name),
                    py_type,
                    field.name
                )
                .unwrap();
            }
        }

        writeln!(output).unwrap();

        // Generate child classes
        for (child_name, child_node) in children {
            if let TreeNode::Branch(grandchildren) = child_node {
                let child_fields = get_node_fields(grandchildren, pattern_lookup);
                if !pattern_lookup.contains_key(&child_fields) {
                    let child_class_name = format!("{}_{}", name, to_pascal_case(child_name));
                    generate_tree_class(
                        output,
                        &child_class_name,
                        child_node,
                        pattern_lookup,
                        metadata,
                        generated,
                    );
                }
            }
        }
    }
}

/// Generate the main client class
fn generate_main_client(output: &mut String, endpoints: &[Endpoint]) {
    writeln!(output, "class BrkClient(BrkClientBase):").unwrap();
    writeln!(
        output,
        "    \"\"\"Main BRK client with catalog tree and API methods.\"\"\""
    )
    .unwrap();
    writeln!(output, "    ").unwrap();
    writeln!(
        output,
        "    def __init__(self, base_url: str = 'http://localhost:3000', timeout: float = 30.0):"
    )
    .unwrap();
    writeln!(output, "        super().__init__(base_url, timeout)").unwrap();
    writeln!(output, "        self.tree = CatalogTree(self)").unwrap();
    writeln!(output).unwrap();

    // Generate API methods
    generate_api_methods(output, endpoints);
}

/// Generate API methods from OpenAPI endpoints
fn generate_api_methods(output: &mut String, endpoints: &[Endpoint]) {
    for endpoint in endpoints {
        if endpoint.method != "GET" {
            continue;
        }

        let method_name = endpoint_to_method_name(endpoint);
        let return_type = endpoint.response_type.as_deref().unwrap_or("Any");

        // Build method signature
        let params = build_method_params(endpoint);
        writeln!(
            output,
            "    def {}(self{}) -> {}:",
            method_name, params, return_type
        )
        .unwrap();

        // Docstring
        if let Some(summary) = &endpoint.summary {
            writeln!(output, "        \"\"\"{}\"\"\"", summary).unwrap();
        }

        // Build path
        let path = build_path_template(&endpoint.path, &endpoint.path_params);

        if endpoint.query_params.is_empty() {
            writeln!(output, "        return self.get(f'{}')", path).unwrap();
        } else {
            writeln!(output, "        params = []").unwrap();
            for param in &endpoint.query_params {
                if param.required {
                    writeln!(
                        output,
                        "        params.append(f'{}={{{}}}')",
                        param.name, param.name
                    )
                    .unwrap();
                } else {
                    writeln!(
                        output,
                        "        if {} is not None: params.append(f'{}={{{}}}')",
                        param.name, param.name, param.name
                    )
                    .unwrap();
                }
            }
            writeln!(output, "        query = '&'.join(params)").unwrap();
            writeln!(
                output,
                "        return self.get(f'{}{{\"?\" + query if query else \"\"}}')",
                path
            )
            .unwrap();
        }

        writeln!(output).unwrap();
    }
}

fn endpoint_to_method_name(endpoint: &Endpoint) -> String {
    if let Some(op_id) = &endpoint.operation_id {
        return to_snake_case(op_id);
    }
    let parts: Vec<&str> = endpoint
        .path
        .split('/')
        .filter(|s| !s.is_empty() && !s.starts_with('{'))
        .collect();
    format!("get_{}", parts.join("_"))
}

fn build_method_params(endpoint: &Endpoint) -> String {
    let mut params = Vec::new();
    for param in &endpoint.path_params {
        params.push(format!(", {}: str", param.name));
    }
    for param in &endpoint.query_params {
        if param.required {
            params.push(format!(", {}: str", param.name));
        } else {
            params.push(format!(", {}: Optional[str] = None", param.name));
        }
    }
    params.join("")
}

fn build_path_template(path: &str, path_params: &[super::Parameter]) -> String {
    let mut result = path.to_string();
    for param in path_params {
        let placeholder = format!("{{{}}}", param.name);
        let interpolation = format!("{{{{{}}}}}", param.name);
        result = result.replace(&placeholder, &interpolation);
    }
    result
}
