use std::{collections::HashSet, fmt::Write as FmtWrite, fs, io, path::Path};

use brk_grouper::{
    AGE_RANGE_NAMES, AMOUNT_RANGE_NAMES, EPOCH_NAMES, GE_AMOUNT_NAMES, LT_AMOUNT_NAMES,
    MAX_AGE_NAMES, MIN_AGE_NAMES, SPENDABLE_TYPE_NAMES, TERM_NAMES, YEAR_NAMES,
};
use brk_types::{pools, Index, TreeNode};
use serde::Serialize;
use serde_json::Value;

use crate::{
    ClientMetadata, Endpoint, FieldNamePosition, IndexSetPattern, PatternField, StructuralPattern,
    TypeSchemas, VERSION, extract_inner_type, get_fields_with_child_info, get_node_fields,
    get_pattern_instance_base, to_pascal_case, to_snake_case,
};

/// Generate Python client from metadata and OpenAPI endpoints.
///
/// `output_path` is the full path to the output file (e.g., "packages/brk_client/__init__.py").
pub fn generate_python_client(
    metadata: &ClientMetadata,
    endpoints: &[Endpoint],
    schemas: &TypeSchemas,
    output_path: &Path,
) -> io::Result<()> {
    let mut output = String::new();

    writeln!(output, "# Auto-generated BRK Python client").unwrap();
    writeln!(output, "# Do not edit manually\n").unwrap();
    writeln!(output, "from __future__ import annotations").unwrap();
    writeln!(
        output,
        "from typing import TypeVar, Generic, Any, Optional, List, Literal, TypedDict, Final, Union"
    )
    .unwrap();
    writeln!(output, "import httpx\n").unwrap();
    writeln!(output, "T = TypeVar('T')\n").unwrap();

    generate_constants(&mut output);
    generate_type_definitions(&mut output, schemas);
    generate_base_client(&mut output);
    generate_metric_node(&mut output);
    generate_index_accessors(&mut output, &metadata.index_set_patterns);
    generate_structural_patterns(&mut output, &metadata.structural_patterns, metadata);
    generate_tree_classes(&mut output, &metadata.catalog, metadata);
    generate_main_client(&mut output, endpoints);

    fs::write(output_path, output)?;

    Ok(())
}

fn generate_constants(output: &mut String) {
    writeln!(output, "# Constants\n").unwrap();

    // VERSION
    writeln!(output, "VERSION: Final[str] = \"v{VERSION}\"\n").unwrap();

    // INDEXES
    let indexes = Index::all();
    writeln!(output, "INDEXES: Final[tuple[str, ...]] = (").unwrap();
    for index in &indexes {
        writeln!(output, "    \"{}\",", index.serialize_long()).unwrap();
    }
    writeln!(output, ")\n").unwrap();

    // POOL_ID_TO_POOL_NAME
    let pools = pools();
    let mut sorted_pools: Vec<_> = pools.iter().collect();
    sorted_pools.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    writeln!(output, "POOL_ID_TO_POOL_NAME: Final[dict[str, str]] = {{").unwrap();
    for pool in &sorted_pools {
        writeln!(output, "    \"{}\": \"{}\",", pool.slug(), pool.name).unwrap();
    }
    writeln!(output, "}}\n").unwrap();

    // Cohort names
    generate_cohort_names(output);
}

fn generate_cohort_names(output: &mut String) {
    fn export_const<T: Serialize>(output: &mut String, name: &str, value: &T) {
        let json = serde_json::to_string_pretty(value).unwrap();
        writeln!(output, "{}: Final = {}\n", name, json).unwrap();
    }

    writeln!(output, "# Cohort names\n").unwrap();

    export_const(output, "TERM_NAMES", &TERM_NAMES);
    export_const(output, "EPOCH_NAMES", &EPOCH_NAMES);
    export_const(output, "YEAR_NAMES", &YEAR_NAMES);
    export_const(output, "SPENDABLE_TYPE_NAMES", &SPENDABLE_TYPE_NAMES);
    export_const(output, "AGE_RANGE_NAMES", &AGE_RANGE_NAMES);
    export_const(output, "MAX_AGE_NAMES", &MAX_AGE_NAMES);
    export_const(output, "MIN_AGE_NAMES", &MIN_AGE_NAMES);
    export_const(output, "AMOUNT_RANGE_NAMES", &AMOUNT_RANGE_NAMES);
    export_const(output, "GE_AMOUNT_NAMES", &GE_AMOUNT_NAMES);
    export_const(output, "LT_AMOUNT_NAMES", &LT_AMOUNT_NAMES);
}

fn generate_type_definitions(output: &mut String, schemas: &TypeSchemas) {
    if schemas.is_empty() {
        return;
    }

    writeln!(output, "# Type definitions\n").unwrap();

    let sorted_names = topological_sort_schemas(schemas);

    for name in sorted_names {
        let Some(schema) = schemas.get(&name) else {
            continue;
        };
        if let Some(props) = schema.get("properties").and_then(|p| p.as_object()) {
            writeln!(output, "class {}(TypedDict):", name).unwrap();
            for (prop_name, prop_schema) in props {
                let prop_type = schema_to_python_type_ctx(prop_schema, Some(&name));
                let safe_name = escape_python_keyword(prop_name);
                writeln!(output, "    {}: {}", safe_name, prop_type).unwrap();
            }
            writeln!(output).unwrap();
        // } else if is_enum_schema(schema) {
        //     let py_type = schema_to_python_type_ctx(schema, Some(&name));
        //     writeln!(output, "{} = {}", name, py_type).unwrap();
        } else {
            let py_type = schema_to_python_type_ctx(schema, Some(&name));
            writeln!(output, "{} = {}", name, py_type).unwrap();
        }
    }
    writeln!(output).unwrap();
}

/// Topologically sort schema names so dependencies come before dependents (avoids forward references).
/// Types that reference other types (via $ref) must be defined after their dependencies.
fn topological_sort_schemas(schemas: &TypeSchemas) -> Vec<String> {
    use std::collections::{HashMap, HashSet};

    // Build dependency graph
    let mut deps: HashMap<String, HashSet<String>> = HashMap::new();
    for (name, schema) in schemas {
        let mut type_deps = HashSet::new();
        collect_schema_refs(schema, &mut type_deps);
        // Only keep deps that are in our schemas
        type_deps.retain(|d| schemas.contains_key(d));
        deps.insert(name.clone(), type_deps);
    }

    // Kahn's algorithm for topological sort
    let mut in_degree: HashMap<String, usize> = HashMap::new();
    for name in schemas.keys() {
        in_degree.insert(name.clone(), 0);
    }
    for type_deps in deps.values() {
        for dep in type_deps {
            *in_degree.entry(dep.clone()).or_insert(0) += 1;
        }
    }

    // Start with types that have no dependents (are not referenced by others)
    let mut queue: Vec<String> = in_degree
        .iter()
        .filter(|(_, count)| **count == 0)
        .map(|(name, _)| name.clone())
        .collect();
    queue.sort(); // Deterministic order

    let mut result = Vec::new();
    while let Some(name) = queue.pop() {
        result.push(name.clone());
        if let Some(type_deps) = deps.get(&name) {
            for dep in type_deps {
                if let Some(count) = in_degree.get_mut(dep) {
                    *count = count.saturating_sub(1);
                    if *count == 0 {
                        queue.push(dep.clone());
                        queue.sort(); // Keep sorted for determinism
                    }
                }
            }
        }
    }

    // Reverse so dependencies come first
    result.reverse();

    // Add any types that weren't processed (e.g., due to circular refs or other edge cases)
    let result_set: HashSet<_> = result.iter().cloned().collect();
    let mut missing: Vec<_> = schemas
        .keys()
        .filter(|k| !result_set.contains(*k))
        .cloned()
        .collect();
    missing.sort();
    result.extend(missing);

    result
}

/// Collect all type references ($ref) from a schema
fn collect_schema_refs(schema: &Value, refs: &mut std::collections::HashSet<String>) {
    match schema {
        Value::Object(map) => {
            if let Some(ref_path) = map.get("$ref").and_then(|r| r.as_str())
                && let Some(type_name) = ref_path.rsplit('/').next()
            {
                refs.insert(type_name.to_string());
            }
            for value in map.values() {
                collect_schema_refs(value, refs);
            }
        }
        Value::Array(arr) => {
            for item in arr {
                collect_schema_refs(item, refs);
            }
        }
        _ => {}
    }
}

/// Convert a single JSON type string to Python type
fn json_type_to_python(ty: &str, schema: &Value, current_type: Option<&str>) -> String {
    match ty {
        "integer" => "int".to_string(),
        "number" => "float".to_string(),
        "boolean" => "bool".to_string(),
        "string" => "str".to_string(),
        "null" => "None".to_string(),
        "array" => {
            let item_type = schema
                .get("items")
                .map(|s| schema_to_python_type_ctx(s, current_type))
                .unwrap_or_else(|| "Any".to_string());
            format!("List[{}]", item_type)
        }
        "object" => {
            if let Some(add_props) = schema.get("additionalProperties") {
                let value_type = schema_to_python_type_ctx(add_props, current_type);
                return format!("dict[str, {}]", value_type);
            }
            "dict".to_string()
        }
        _ => "Any".to_string(),
    }
}

/// Convert JSON Schema to Python type with context for detecting self-references
fn schema_to_python_type_ctx(schema: &Value, current_type: Option<&str>) -> String {
    if let Some(all_of) = schema.get("allOf").and_then(|v| v.as_array()) {
        for item in all_of {
            let resolved = schema_to_python_type_ctx(item, current_type);
            if resolved != "Any" {
                return resolved;
            }
        }
    }

    // Handle $ref
    if let Some(ref_path) = schema.get("$ref").and_then(|r| r.as_str()) {
        let type_name = ref_path.rsplit('/').next().unwrap_or("Any");
        // Quote self-references to handle recursive types
        if current_type == Some(type_name) {
            return format!("\"{}\"", type_name);
        }
        return type_name.to_string();
    }

    // Handle enum (array of string values)
    if let Some(enum_values) = schema.get("enum").and_then(|e| e.as_array()) {
        let literals: Vec<String> = enum_values
            .iter()
            .filter_map(|v| v.as_str())
            .map(|s| format!("\"{}\"", s))
            .collect();
        if !literals.is_empty() {
            return format!("Literal[{}]", literals.join(", "));
        }
    }

    if let Some(ty) = schema.get("type") {
        if let Some(type_array) = ty.as_array() {
            let types: Vec<String> = type_array
                .iter()
                .filter_map(|t| t.as_str())
                .filter(|t| *t != "null") // Filter out null for cleaner Optional handling
                .map(|t| json_type_to_python(t, schema, current_type))
                .collect();
            let has_null = type_array.iter().any(|t| t.as_str() == Some("null"));

            if types.len() == 1 {
                let base_type = &types[0];
                return if has_null {
                    format!("Optional[{}]", base_type)
                } else {
                    base_type.clone()
                };
            } else if !types.is_empty() {
                let union = format!("Union[{}]", types.join(", "));
                return if has_null {
                    format!("Optional[{}]", union)
                } else {
                    union
                };
            }
        }

        if let Some(ty_str) = ty.as_str() {
            return json_type_to_python(ty_str, schema, current_type);
        }
    }

    if let Some(variants) = schema
        .get("anyOf")
        .or_else(|| schema.get("oneOf"))
        .and_then(|v| v.as_array())
    {
        let types: Vec<String> = variants
            .iter()
            .map(|v| schema_to_python_type_ctx(v, current_type))
            .collect();
        let filtered: Vec<_> = types.iter().filter(|t| *t != "Any").collect();
        if !filtered.is_empty() {
            return format!(
                "Union[{}]",
                filtered
                    .iter()
                    .map(|s| s.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
        return format!("Union[{}]", types.join(", "));
    }

    // Check for format hint without type (common in OpenAPI)
    if let Some(format) = schema.get("format").and_then(|f| f.as_str()) {
        return match format {
            "int32" | "int64" => "int".to_string(),
            "float" | "double" => "float".to_string(),
            "date" | "date-time" => "str".to_string(),
            _ => "Any".to_string(),
        };
    }

    "Any".to_string()
}

/// Make a name safe for Python: escape keywords and prefix digit-starting names
fn escape_python_keyword(name: &str) -> String {
    const PYTHON_KEYWORDS: &[&str] = &[
        "False", "None", "True", "and", "as", "assert", "async", "await", "break", "class",
        "continue", "def", "del", "elif", "else", "except", "finally", "for", "from", "global",
        "if", "import", "in", "is", "lambda", "nonlocal", "not", "or", "pass", "raise", "return",
        "try", "while", "with", "yield",
    ];
    // Names starting with digit need underscore prefix
    let name = if name
        .chars()
        .next()
        .map(|c| c.is_ascii_digit())
        .unwrap_or(false)
    {
        format!("_{}", name)
    } else {
        name.to_string()
    };
    // Reserved keywords get underscore suffix
    if PYTHON_KEYWORDS.contains(&name.as_str()) {
        format!("{}_", name)
    } else {
        name
    }
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

    def get_range(self, from_val: Optional[str] = None, to_val: Optional[str] = None) -> List[T]:
        """Fetch data points within a range."""
        params = []
        if from_val is not None:
            params.append(f"from={{from_val}}")
        if to_val is not None:
            params.append(f"to={{to_val}}")
        query = "&".join(params)
        return self._client.get(f"{{self._path}}?{{query}}" if query else self._path)

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
        let is_parameterizable = pattern.is_parameterizable();

        // For generic patterns, inherit from Generic[T]
        if pattern.is_generic {
            writeln!(output, "class {}(Generic[T]):", pattern.name).unwrap();
        } else {
            writeln!(output, "class {}:", pattern.name).unwrap();
        }
        writeln!(
            output,
            "    \"\"\"Pattern struct for repeated tree structure.\"\"\""
        )
        .unwrap();
        writeln!(output, "    ").unwrap();

        if is_parameterizable {
            writeln!(
                output,
                "    def __init__(self, client: BrkClientBase, acc: str):"
            )
            .unwrap();
            writeln!(
                output,
                "        \"\"\"Create pattern node with accumulated metric name.\"\"\""
            )
            .unwrap();
        } else {
            writeln!(
                output,
                "    def __init__(self, client: BrkClientBase, base_path: str):"
            )
            .unwrap();
        }

        for field in &pattern.fields {
            if is_parameterizable {
                generate_parameterized_python_field(output, field, pattern, metadata);
            } else {
                generate_tree_path_python_field(output, field, metadata);
            }
        }

        writeln!(output).unwrap();
    }
}

/// Generate a field using parameterized (prepend/append) metric name construction
fn generate_parameterized_python_field(
    output: &mut String,
    field: &PatternField,
    pattern: &StructuralPattern,
    metadata: &ClientMetadata,
) {
    let field_name = to_snake_case(&field.name);
    let py_type = field_to_python_type_generic(field, metadata, pattern.is_generic);

    // For branch fields, pass the accumulated name to nested pattern
    if metadata.is_pattern_type(&field.rust_type) {
        let child_acc = if let Some(pos) = pattern.get_field_position(&field.name) {
            match pos {
                FieldNamePosition::Append(suffix) => format!("f'{{acc}}{}'", suffix),
                FieldNamePosition::Prepend(prefix) => format!("f'{}{{acc}}'", prefix),
                FieldNamePosition::Identity => "acc".to_string(),
                FieldNamePosition::SetBase(base) => format!("'{}'", base),
            }
        } else {
            format!("f'{{acc}}_{}'", field.name)
        };

        writeln!(
            output,
            "        self.{}: {} = {}(client, {})",
            field_name, py_type, field.rust_type, child_acc
        )
        .unwrap();
        return;
    }

    // For leaf fields, construct the metric path based on position
    let metric_expr = if let Some(pos) = pattern.get_field_position(&field.name) {
        match pos {
            FieldNamePosition::Append(suffix) => format!("f'/{{acc}}{}'", suffix),
            FieldNamePosition::Prepend(prefix) => format!("f'/{}{{acc}}'", prefix),
            FieldNamePosition::Identity => "f'/{acc}'".to_string(),
            FieldNamePosition::SetBase(base) => format!("'/{}'", base),
        }
    } else {
        format!("f'/{{acc}}_{}'", field.name)
    };

    if metadata.field_uses_accessor(field) {
        let accessor = metadata.find_index_set_pattern(&field.indexes).unwrap();
        writeln!(
            output,
            "        self.{}: {} = {}(client, {})",
            field_name, py_type, accessor.name, metric_expr
        )
        .unwrap();
    } else {
        writeln!(
            output,
            "        self.{}: {} = MetricNode(client, {})",
            field_name, py_type, metric_expr
        )
        .unwrap();
    }
}

/// Generate a field using tree path construction (fallback for non-parameterizable patterns)
fn generate_tree_path_python_field(
    output: &mut String,
    field: &PatternField,
    metadata: &ClientMetadata,
) {
    let field_name = to_snake_case(&field.name);
    let py_type = field_to_python_type(field, metadata);

    if metadata.is_pattern_type(&field.rust_type) {
        writeln!(
            output,
            "        self.{}: {} = {}(client, f'{{base_path}}/{}')",
            field_name, py_type, field.rust_type, field.name
        )
        .unwrap();
    } else if metadata.field_uses_accessor(field) {
        let accessor = metadata.find_index_set_pattern(&field.indexes).unwrap();
        writeln!(
            output,
            "        self.{}: {} = {}(client, f'{{base_path}}/{}')",
            field_name, py_type, accessor.name, field.name
        )
        .unwrap();
    } else {
        writeln!(
            output,
            "        self.{}: {} = MetricNode(client, f'{{base_path}}/{}')",
            field_name, py_type, field.name
        )
        .unwrap();
    }
}

/// Convert pattern field to Python type annotation
fn field_to_python_type(field: &PatternField, metadata: &ClientMetadata) -> String {
    field_to_python_type_generic(field, metadata, false)
}

/// Convert pattern field to Python type annotation, with optional generic support
fn field_to_python_type_generic(
    field: &PatternField,
    metadata: &ClientMetadata,
    is_generic: bool,
) -> String {
    field_to_python_type_with_generic_value(field, metadata, is_generic, None)
}

/// Convert pattern field to Python type annotation.
/// - `is_generic`: If true and field.rust_type is "T", use T in the output
/// - `generic_value_type`: For branch fields that reference a generic pattern, this is the concrete type to substitute
fn field_to_python_type_with_generic_value(
    field: &PatternField,
    metadata: &ClientMetadata,
    is_generic: bool,
    generic_value_type: Option<&str>,
) -> String {
    // For generic patterns, use T instead of concrete value type
    // Also extract inner type from wrappers like Close<Dollars> -> Dollars
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
                .unwrap_or(if is_generic { "T" } else { "Any" });
            return format!("{}[{}]", field.rust_type, type_param);
        }
        field.rust_type.clone()
    } else if field.is_branch() {
        // Non-pattern branch struct
        field.rust_type.clone()
    } else if let Some(accessor) = metadata.find_index_set_pattern(&field.indexes) {
        // Leaf with accessor - use value_type as the generic
        format!("{}[{}]", accessor.name, value_type)
    } else {
        // Leaf - use value_type as the generic
        format!("MetricNode[{}]", value_type)
    }
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
    let TreeNode::Branch(children) = node else {
        return;
    };

    let fields_with_child_info = get_fields_with_child_info(children, name, pattern_lookup);
    let fields: Vec<PatternField> = fields_with_child_info
        .iter()
        .map(|(f, _)| f.clone())
        .collect();

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

    for ((field, child_fields_opt), (child_name, child_node)) in
        fields_with_child_info.iter().zip(children.iter())
    {
        // Look up type parameter for generic patterns
        let generic_value_type = child_fields_opt
            .as_ref()
            .and_then(|cf| metadata.get_type_param(cf))
            .map(String::as_str);
        let py_type =
            field_to_python_type_with_generic_value(field, metadata, false, generic_value_type);
        let field_name_py = to_snake_case(&field.name);

        if metadata.is_pattern_type(&field.rust_type) {
            let pattern = metadata.find_pattern(&field.rust_type);
            let is_parameterizable = pattern.is_some_and(|p| p.is_parameterizable());

            if is_parameterizable {
                let metric_base = get_pattern_instance_base(child_node, child_name);
                writeln!(
                    output,
                    "        self.{}: {} = {}(client, '{}')",
                    field_name_py, py_type, field.rust_type, metric_base
                )
                .unwrap();
            } else {
                writeln!(
                    output,
                    "        self.{}: {} = {}(client, f'{{base_path}}/{}')",
                    field_name_py, py_type, field.rust_type, field.name
                )
                .unwrap();
            }
        } else if metadata.field_uses_accessor(field) {
            let accessor = metadata.find_index_set_pattern(&field.indexes).unwrap();
            writeln!(
                output,
                "        self.{}: {} = {}(client, f'{{base_path}}/{}')",
                field_name_py, py_type, accessor.name, field.name
            )
            .unwrap();
        } else if field.is_branch() {
            // Non-pattern branch - instantiate the nested class
            writeln!(
                output,
                "        self.{}: {} = {}(client, f'{{base_path}}/{}')",
                field_name_py, py_type, field.rust_type, field.name
            )
            .unwrap();
        } else {
            // Leaf - use MetricNode with base_path
            writeln!(
                output,
                "        self.{}: {} = MetricNode(client, f'{{base_path}}/{}')",
                field_name_py, py_type, field.name
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
        if !endpoint.should_generate() {
            continue;
        }

        let method_name = endpoint_to_method_name(endpoint);
        let return_type = endpoint
            .response_type
            .as_deref()
            .map(js_type_to_python)
            .unwrap_or_else(|| "Any".to_string());

        // Build method signature
        let params = build_method_params(endpoint);
        writeln!(
            output,
            "    def {}(self{}) -> {}:",
            method_name, params, return_type
        )
        .unwrap();

        // Docstring
        match (&endpoint.summary, &endpoint.description) {
            (Some(summary), Some(desc)) if summary != desc => {
                writeln!(output, "        \"\"\"{}.", summary.trim_end_matches('.')).unwrap();
                writeln!(output).unwrap();
                writeln!(output, "        {}\"\"\"", desc).unwrap();
            }
            (Some(summary), _) => {
                writeln!(output, "        \"\"\"{}\"\"\"", summary).unwrap();
            }
            (None, Some(desc)) => {
                writeln!(output, "        \"\"\"{}\"\"\"", desc).unwrap();
            }
            (None, None) => {}
        }

        // Build path
        let path = build_path_template(&endpoint.path, &endpoint.path_params);

        if endpoint.query_params.is_empty() {
            if endpoint.path_params.is_empty() {
                writeln!(output, "        return self.get('{}')", path).unwrap();
            } else {
                writeln!(output, "        return self.get(f'{}')", path).unwrap();
            }
        } else {
            writeln!(output, "        params = []").unwrap();
            for param in &endpoint.query_params {
                // Use safe name for Python variable, original name for API query parameter
                let safe_name = escape_python_keyword(&param.name);
                if param.required {
                    writeln!(
                        output,
                        "        params.append(f'{}={{{}}}')",
                        param.name, safe_name
                    )
                    .unwrap();
                } else {
                    writeln!(
                        output,
                        "        if {} is not None: params.append(f'{}={{{}}}')",
                        safe_name, param.name, safe_name
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
    to_snake_case(&endpoint.operation_name())
}

/// Convert JS-style type to Python type (e.g., "Txid[]" -> "List[Txid]", "number" -> "int")
fn js_type_to_python(js_type: &str) -> String {
    if let Some(inner) = js_type.strip_suffix("[]") {
        format!("List[{}]", js_type_to_python(inner))
    } else {
        match js_type {
            "number" => "int".to_string(),
            "boolean" => "bool".to_string(),
            "string" => "str".to_string(),
            "null" => "None".to_string(),
            "Object" | "object" => "dict".to_string(),
            "*" => "Any".to_string(),
            _ => js_type.to_string(),
        }
    }
}

fn build_method_params(endpoint: &Endpoint) -> String {
    let mut params = Vec::new();
    for param in &endpoint.path_params {
        let safe_name = escape_python_keyword(&param.name);
        params.push(format!(", {}: str", safe_name));
    }
    for param in &endpoint.query_params {
        let safe_name = escape_python_keyword(&param.name);
        if param.required {
            params.push(format!(", {}: str", safe_name));
        } else {
            params.push(format!(", {}: Optional[str] = None", safe_name));
        }
    }
    params.join("")
}

fn build_path_template(path: &str, path_params: &[super::Parameter]) -> String {
    let mut result = path.to_string();
    for param in path_params {
        let placeholder = format!("{{{}}}", param.name);
        // Use escaped name for Python variable interpolation in f-string
        let safe_name = escape_python_keyword(&param.name);
        let interpolation = format!("{{{}}}", safe_name);
        result = result.replace(&placeholder, &interpolation);
    }
    result
}
