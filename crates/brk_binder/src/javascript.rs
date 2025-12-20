use std::collections::HashSet;
use std::fmt::Write as FmtWrite;
use std::fs;
use std::io;
use std::path::Path;

use brk_types::{Index, TreeNode};
use serde_json::Value;

use crate::{
    ClientMetadata, Endpoint, FieldNamePosition, IndexSetPattern, PatternField, StructuralPattern,
    TypeSchemas, extract_inner_type, get_first_leaf_name, get_node_fields,
    get_pattern_instance_base, to_camel_case, to_pascal_case, unwrap_allof,
};

/// Generate JavaScript + JSDoc client from metadata and OpenAPI endpoints
pub fn generate_javascript_client(
    metadata: &ClientMetadata,
    endpoints: &[Endpoint],
    schemas: &TypeSchemas,
    output_dir: &Path,
) -> io::Result<()> {
    let mut output = String::new();

    // Header
    writeln!(output, "// Auto-generated BRK JavaScript client").unwrap();
    writeln!(output, "// Do not edit manually\n").unwrap();

    // Generate type definitions from OpenAPI schemas
    generate_type_definitions(&mut output, schemas);

    // Generate the base client class
    generate_base_client(&mut output);

    // Generate index accessor factory functions
    generate_index_accessors(&mut output, &metadata.index_set_patterns);

    // Generate structural pattern factory functions
    generate_structural_patterns(&mut output, &metadata.structural_patterns, metadata);

    // Generate tree JSDoc typedefs
    generate_tree_typedefs(&mut output, &metadata.catalog, metadata);

    // Generate the main client class with tree and API methods
    generate_main_client(&mut output, &metadata.catalog, metadata, endpoints);

    fs::write(output_dir.join("client.js"), output)?;

    Ok(())
}

/// Generate JSDoc type definitions from OpenAPI schemas
fn generate_type_definitions(output: &mut String, schemas: &TypeSchemas) {
    if schemas.is_empty() {
        return;
    }

    writeln!(output, "// Type definitions\n").unwrap();

    for (name, schema) in schemas {
        let js_type = schema_to_js_type(schema);

        if is_primitive_alias(schema) {
            // Simple type alias: @typedef {number} Height
            writeln!(output, "/** @typedef {{{}}} {} */", js_type, name).unwrap();
        } else if let Some(props) = schema.get("properties").and_then(|p| p.as_object()) {
            // Object type with properties
            writeln!(output, "/**").unwrap();
            writeln!(output, " * @typedef {{Object}} {}", name).unwrap();
            for (prop_name, prop_schema) in props {
                let prop_type = schema_to_js_type(prop_schema);
                let required = schema
                    .get("required")
                    .and_then(|r| r.as_array())
                    .map(|arr| arr.iter().any(|v| v.as_str() == Some(prop_name)))
                    .unwrap_or(false);
                let optional = if required { "" } else { "=" };
                writeln!(
                    output,
                    " * @property {{{}{}}} {}",
                    prop_type, optional, prop_name
                )
                .unwrap();
            }
            writeln!(output, " */").unwrap();
        } else {
            // Other schemas - just typedef
            writeln!(output, "/** @typedef {{{}}} {} */", js_type, name).unwrap();
        }
    }
    writeln!(output).unwrap();
}

/// Check if schema represents a primitive type alias (like Height = number)
fn is_primitive_alias(schema: &Value) -> bool {
    schema.get("properties").is_none()
        && schema.get("items").is_none()
        && schema.get("anyOf").is_none()
        && schema.get("oneOf").is_none()
        && schema.get("enum").is_none()
}

/// Convert JSON Schema to JavaScript/JSDoc type
fn schema_to_js_type(schema: &Value) -> String {
    // Unwrap single-element allOf (schemars uses this for composition)
    let schema = unwrap_allof(schema);

    // Handle $ref
    if let Some(ref_path) = schema.get("$ref").and_then(|r| r.as_str()) {
        return ref_path.rsplit('/').next().unwrap_or("*").to_string();
    }

    // Handle enum (array of string values)
    if let Some(enum_values) = schema.get("enum").and_then(|e| e.as_array()) {
        let literals: Vec<String> = enum_values
            .iter()
            .filter_map(|v| v.as_str())
            .map(|s| format!("\"{}\"", s))
            .collect();
        if !literals.is_empty() {
            return format!("({})", literals.join("|"));
        }
    }

    // Handle type field
    if let Some(ty) = schema.get("type").and_then(|t| t.as_str()) {
        return match ty {
            "integer" | "number" => "number".to_string(),
            "boolean" => "boolean".to_string(),
            "string" => "string".to_string(),
            "null" => "null".to_string(),
            "array" => {
                let item_type = schema
                    .get("items")
                    .map(schema_to_js_type)
                    .unwrap_or_else(|| "*".to_string());
                format!("{}[]", item_type)
            }
            "object" => "Object".to_string(),
            _ => "*".to_string(),
        };
    }

    // Handle anyOf/oneOf
    if let Some(variants) = schema
        .get("anyOf")
        .or_else(|| schema.get("oneOf"))
        .and_then(|v| v.as_array())
    {
        let types: Vec<String> = variants.iter().map(schema_to_js_type).collect();
        return format!("({})", types.join("|"));
    }

    "*".to_string()
}

/// Generate the base BrkClient class with HTTP functionality
fn generate_base_client(output: &mut String) {
    writeln!(
        output,
        r#"/**
 * @typedef {{Object}} BrkClientOptions
 * @property {{string}} baseUrl - Base URL for the API
 * @property {{number}} [timeout] - Request timeout in milliseconds
 */

const _isBrowser = typeof window !== 'undefined' && 'caches' in window;
const _runIdle = (fn) => (globalThis.requestIdleCallback ?? setTimeout)(fn);

/** @type {{Promise<Cache | null>}} */
const _cachePromise = _isBrowser
  ? caches.open('__BRK_CLIENT__').catch(() => null)
  : Promise.resolve(null);

/**
 * Custom error class for BRK client errors
 */
class BrkError extends Error {{
  /**
   * @param {{string}} message
   * @param {{number}} [status]
   */
  constructor(message, status) {{
    super(message);
    this.name = 'BrkError';
    this.status = status;
  }}
}}

/**
 * A metric node that can fetch data for different indexes.
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
   * Fetch all data points for this metric.
   * @param {{(value: T[]) => void}} [onUpdate] - Called when data is available (may be called twice: cache then fresh)
   * @returns {{Promise<T[] | null>}}
   */
  get(onUpdate) {{
    return this._client.get(this._path, onUpdate);
  }}

  /**
   * Fetch data points within a range.
   * @param {{string | number}} from
   * @param {{string | number}} to
   * @param {{(value: T[]) => void}} [onUpdate] - Called when data is available (may be called twice: cache then fresh)
   * @returns {{Promise<T[] | null>}}
   */
  getRange(from, to, onUpdate) {{
    return this._client.get(`${{this._path}}?from=${{from}}&to=${{to}}`, onUpdate);
  }}
}}

/**
 * Base HTTP client for making requests with caching support
 */
class BrkClientBase {{
  /**
   * @param {{BrkClientOptions|string}} options
   */
  constructor(options) {{
    const isString = typeof options === 'string';
    this.baseUrl = isString ? options : options.baseUrl;
    this.timeout = isString ? 5000 : (options.timeout ?? 5000);
  }}

  /**
   * Make a GET request with stale-while-revalidate caching
   * @template T
   * @param {{string}} path
   * @param {{(value: T) => void}} [onUpdate] - Called when data is available
   * @returns {{Promise<T | null>}}
   */
  async get(path, onUpdate) {{
    const url = `${{this.baseUrl}}${{path}}`;
    const cache = await _cachePromise;
    const cachedRes = await cache?.match(url);
    const cachedJson = cachedRes ? await cachedRes.json() : null;

    if (cachedJson) onUpdate?.(cachedJson);
    if (!globalThis.navigator?.onLine) return cachedJson;

    try {{
      const res = await fetch(url, {{ signal: AbortSignal.timeout(this.timeout) }});
      if (!res.ok) throw new BrkError(`HTTP ${{res.status}}`, res.status);
      if (cachedRes?.headers.get('ETag') === res.headers.get('ETag')) return cachedJson;

      const cloned = res.clone();
      const json = await res.json();
      onUpdate?.(json);
      if (cache) _runIdle(() => cache.put(url, cloned));
      return json;
    }} catch (e) {{
      if (cachedJson) return cachedJson;
      throw e;
    }}
  }}
}}

"#
    )
    .unwrap();
}

/// Generate index accessor factory functions
fn generate_index_accessors(output: &mut String, patterns: &[IndexSetPattern]) {
    if patterns.is_empty() {
        return;
    }

    writeln!(output, "// Index accessor factory functions\n").unwrap();

    for pattern in patterns {
        // Generate JSDoc typedef for the accessor
        writeln!(output, "/**").unwrap();
        writeln!(output, " * @template T").unwrap();
        writeln!(output, " * @typedef {{Object}} {}", pattern.name).unwrap();
        for index in &pattern.indexes {
            let field_name = index_to_camel_case(index);
            writeln!(output, " * @property {{MetricNode<T>}} {}", field_name).unwrap();
        }
        writeln!(output, " */\n").unwrap();

        // Generate factory function
        writeln!(output, "/**").unwrap();
        writeln!(output, " * Create a {} accessor", pattern.name).unwrap();
        writeln!(output, " * @template T").unwrap();
        writeln!(output, " * @param {{BrkClientBase}} client").unwrap();
        writeln!(output, " * @param {{string}} basePath").unwrap();
        writeln!(output, " * @returns {{{}<T>}}", pattern.name).unwrap();
        writeln!(output, " */").unwrap();
        writeln!(
            output,
            "function create{}(client, basePath) {{",
            pattern.name
        )
        .unwrap();
        writeln!(output, "  return {{").unwrap();

        for (i, index) in pattern.indexes.iter().enumerate() {
            let field_name = index_to_camel_case(index);
            let path_segment = index.serialize_long();
            let comma = if i < pattern.indexes.len() - 1 {
                ","
            } else {
                ""
            };
            writeln!(
                output,
                "    {}: new MetricNode(client, `${{basePath}}/{}`){}",
                field_name, path_segment, comma
            )
            .unwrap();
        }

        writeln!(output, "  }};").unwrap();
        writeln!(output, "}}\n").unwrap();
    }
}

/// Convert an Index to a camelCase field name (e.g., DateIndex -> byDateIndex)
fn index_to_camel_case(index: &Index) -> String {
    format!("by{}", to_pascal_case(index.serialize_long()))
}

/// Generate structural pattern factory functions
fn generate_structural_patterns(
    output: &mut String,
    patterns: &[StructuralPattern],
    metadata: &ClientMetadata,
) {
    if patterns.is_empty() {
        return;
    }

    writeln!(output, "// Reusable structural pattern factories\n").unwrap();

    for pattern in patterns {
        // Check if this pattern is parameterizable (has field positions detected)
        let is_parameterizable = pattern.is_parameterizable();

        // Generate JSDoc typedef
        writeln!(output, "/**").unwrap();
        if pattern.is_generic {
            writeln!(output, " * @template T").unwrap();
        }
        writeln!(output, " * @typedef {{Object}} {}", pattern.name).unwrap();
        for field in &pattern.fields {
            let js_type = field_to_js_type_generic(field, metadata, pattern.is_generic);
            writeln!(
                output,
                " * @property {{{}}} {}",
                js_type,
                to_camel_case(&field.name)
            )
            .unwrap();
        }
        writeln!(output, " */\n").unwrap();

        // Generate factory function
        writeln!(output, "/**").unwrap();
        writeln!(output, " * Create a {} pattern node", pattern.name).unwrap();
        writeln!(output, " * @param {{BrkClientBase}} client").unwrap();
        if is_parameterizable {
            writeln!(output, " * @param {{string}} acc - Accumulated metric name").unwrap();
        } else {
            writeln!(output, " * @param {{string}} basePath").unwrap();
        }
        writeln!(output, " * @returns {{{}}}", pattern.name).unwrap();
        writeln!(output, " */").unwrap();

        let param_name = if is_parameterizable {
            "acc"
        } else {
            "basePath"
        };
        writeln!(
            output,
            "function create{}(client, {}) {{",
            pattern.name, param_name
        )
        .unwrap();
        writeln!(output, "  return {{").unwrap();

        for (i, field) in pattern.fields.iter().enumerate() {
            let comma = if i < pattern.fields.len() - 1 {
                ","
            } else {
                ""
            };

            if is_parameterizable {
                generate_parameterized_field(output, field, pattern, metadata, comma);
            } else {
                generate_tree_path_field(output, field, metadata, comma);
            }
        }

        writeln!(output, "  }};").unwrap();
        writeln!(output, "}}\n").unwrap();
    }
}

/// Generate a field using parameterized (prepend/append) metric name construction
fn generate_parameterized_field(
    output: &mut String,
    field: &PatternField,
    pattern: &StructuralPattern,
    metadata: &ClientMetadata,
    comma: &str,
) {
    let field_name_js = to_camel_case(&field.name);

    // For branch fields, pass the accumulated name to nested pattern
    if metadata.is_pattern_type(&field.rust_type) {
        // Get the field position to determine how to transform the accumulated name
        let child_acc = if let Some(pos) = pattern.get_field_position(&field.name) {
            match pos {
                FieldNamePosition::Append(suffix) => format!("`${{acc}}{}`", suffix),
                FieldNamePosition::Prepend(prefix) => format!("`{}{}`", prefix, "${acc}"),
                FieldNamePosition::Identity => "acc".to_string(),
                FieldNamePosition::SetBase(base) => format!("'{}'", base),
            }
        } else {
            // Fallback: append field name
            format!("`${{acc}}_{}`", field.name)
        };

        writeln!(
            output,
            "    {}: create{}(client, {}){}",
            field_name_js, field.rust_type, child_acc, comma
        )
        .unwrap();
        return;
    }

    // For leaf fields, construct the metric path based on position
    let metric_expr = if let Some(pos) = pattern.get_field_position(&field.name) {
        match pos {
            FieldNamePosition::Append(suffix) => format!("`/${{acc}}{suffix}`"),
            FieldNamePosition::Prepend(prefix) => format!("`/{prefix}${{acc}}`"),
            FieldNamePosition::Identity => "`/${acc}`".to_string(),
            FieldNamePosition::SetBase(base) => format!("'/{base}'"),
        }
    } else {
        // Fallback: use field name appended
        format!("`/${{acc}}_{}`", field.name)
    };

    if field_uses_accessor(field, metadata) {
        let accessor = metadata.find_index_set_pattern(&field.indexes).unwrap();
        writeln!(
            output,
            "    {}: create{}(client, {}){}",
            field_name_js, accessor.name, metric_expr, comma
        )
        .unwrap();
    } else {
        writeln!(
            output,
            "    {}: new MetricNode(client, {}){}",
            field_name_js, metric_expr, comma
        )
        .unwrap();
    }
}

/// Generate a field using tree path construction (fallback for non-parameterizable patterns)
fn generate_tree_path_field(
    output: &mut String,
    field: &PatternField,
    metadata: &ClientMetadata,
    comma: &str,
) {
    let field_name_js = to_camel_case(&field.name);

    if metadata.is_pattern_type(&field.rust_type) {
        writeln!(
            output,
            "    {}: create{}(client, `${{basePath}}/{}`){}",
            field_name_js, field.rust_type, field.name, comma
        )
        .unwrap();
    } else if field_uses_accessor(field, metadata) {
        let accessor = metadata.find_index_set_pattern(&field.indexes).unwrap();
        writeln!(
            output,
            "    {}: create{}(client, `${{basePath}}/{}`){}",
            field_name_js, accessor.name, field.name, comma
        )
        .unwrap();
    } else {
        writeln!(
            output,
            "    {}: new MetricNode(client, `${{basePath}}/{}`){}",
            field_name_js, field.name, comma
        )
        .unwrap();
    }
}

/// Convert pattern field to JavaScript/JSDoc type, with optional generic support
fn field_to_js_type_generic(
    field: &PatternField,
    metadata: &ClientMetadata,
    is_generic: bool,
) -> String {
    field_to_js_type_with_generic_value(field, metadata, is_generic, None)
}

/// Convert pattern field to JavaScript/JSDoc type.
/// - `is_generic`: If true and field.rust_type is "T", use T in the output
/// - `generic_value_type`: For branch fields that reference a generic pattern, this is the concrete type to substitute
fn field_to_js_type_with_generic_value(
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
        // Check if this pattern is generic and we have a value type
        if metadata.is_pattern_generic(&field.rust_type)
            && let Some(vt) = generic_value_type
        {
            return format!("{}<{}>", field.rust_type, vt);
        }
        field.rust_type.clone()
    } else if let Some(accessor) = metadata.find_index_set_pattern(&field.indexes) {
        // Leaf with accessor - use value_type as the generic
        format!("{}<{}>", accessor.name, value_type)
    } else {
        // Leaf - use value_type as the generic
        format!("MetricNode<{}>", value_type)
    }
}

/// Check if a field should use an index accessor
fn field_uses_accessor(field: &PatternField, metadata: &ClientMetadata) -> bool {
    metadata.find_index_set_pattern(&field.indexes).is_some()
}

/// Generate tree typedefs
fn generate_tree_typedefs(output: &mut String, catalog: &TreeNode, metadata: &ClientMetadata) {
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

/// Recursively generate tree typedefs
fn generate_tree_typedef(
    output: &mut String,
    name: &str,
    node: &TreeNode,
    pattern_lookup: &std::collections::HashMap<Vec<PatternField>, String>,
    metadata: &ClientMetadata,
    generated: &mut HashSet<String>,
) {
    if let TreeNode::Branch(children) = node {
        // Build signature with child field info for generic pattern lookup
        let fields_with_child_info: Vec<(PatternField, Option<Vec<PatternField>>)> = children
            .iter()
            .map(|(child_name, child_node)| {
                let (rust_type, json_type, indexes, child_fields) = match child_node {
                    TreeNode::Leaf(leaf) => (
                        leaf.value_type().to_string(),
                        leaf.schema
                            .get("type")
                            .and_then(|v| v.as_str())
                            .unwrap_or("object")
                            .to_string(),
                        leaf.indexes().clone(),
                        None,
                    ),
                    TreeNode::Branch(grandchildren) => {
                        let child_fields = get_node_fields(grandchildren, pattern_lookup);
                        let pattern_name = pattern_lookup
                            .get(&child_fields)
                            .cloned()
                            .unwrap_or_else(|| format!("{}_{}", name, to_pascal_case(child_name)));
                        (
                            pattern_name.clone(),
                            pattern_name,
                            std::collections::BTreeSet::new(),
                            Some(child_fields),
                        )
                    }
                };
                (
                    PatternField {
                        name: child_name.clone(),
                        rust_type,
                        json_type,
                        indexes,
                    },
                    child_fields,
                )
            })
            .collect();

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

        writeln!(output, "/**").unwrap();
        writeln!(output, " * @typedef {{Object}} {}", name).unwrap();

        for (field, child_fields) in &fields_with_child_info {
            // For generic patterns, extract the value type from child fields
            let generic_value_type = child_fields
                .as_ref()
                .and_then(|cf| metadata.get_generic_value_type(&field.rust_type, cf));
            let js_type = field_to_js_type_with_generic_value(
                field,
                metadata,
                false,
                generic_value_type.as_deref(),
            );
            writeln!(
                output,
                " * @property {{{}}} {}",
                js_type,
                to_camel_case(&field.name)
            )
            .unwrap();
        }

        writeln!(output, " */\n").unwrap();

        // Generate child typedefs
        for (child_name, child_node) in children {
            if let TreeNode::Branch(grandchildren) = child_node {
                let child_fields = get_node_fields(grandchildren, pattern_lookup);
                if !pattern_lookup.contains_key(&child_fields) {
                    let child_type_name = format!("{}_{}", name, to_pascal_case(child_name));
                    generate_tree_typedef(
                        output,
                        &child_type_name,
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

/// Generate main client
fn generate_main_client(
    output: &mut String,
    catalog: &TreeNode,
    metadata: &ClientMetadata,
    endpoints: &[Endpoint],
) {
    let pattern_lookup = metadata.pattern_lookup();

    writeln!(output, "/**").unwrap();
    writeln!(
        output,
        " * Main BRK client with catalog tree and API methods"
    )
    .unwrap();
    writeln!(output, " * @extends BrkClientBase").unwrap();
    writeln!(output, " */").unwrap();
    writeln!(output, "class BrkClient extends BrkClientBase {{").unwrap();
    writeln!(output, "  /**").unwrap();
    writeln!(output, "   * @param {{BrkClientOptions|string}} options").unwrap();
    writeln!(output, "   */").unwrap();
    writeln!(output, "  constructor(options) {{").unwrap();
    writeln!(output, "    super(options);").unwrap();
    writeln!(output, "    /** @type {{CatalogTree}} */").unwrap();
    writeln!(output, "    this.tree = this._buildTree('');").unwrap();
    writeln!(output, "  }}\n").unwrap();

    // Generate _buildTree method
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

    // Generate API methods
    generate_api_methods(output, endpoints);

    writeln!(output, "}}\n").unwrap();

    // Export
    writeln!(
        output,
        "export {{ BrkClient, BrkClientBase, BrkError, MetricNode }};"
    )
    .unwrap();
}

/// Generate tree initializer
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
                    // Use leaf.name() (vec.name()) for API path, not tree path
                    let metric_path = format!("/{}", leaf.name());
                    if let Some(accessor) = metadata.find_index_set_pattern(leaf.indexes()) {
                        writeln!(
                            output,
                            "{}{}: create{}(this, '{}'){}",
                            indent_str, field_name, accessor.name, metric_path, comma
                        )
                        .unwrap();
                    } else {
                        writeln!(
                            output,
                            "{}{}: new MetricNode(this, '{}'){}",
                            indent_str, field_name, metric_path, comma
                        )
                        .unwrap();
                    }
                }
                TreeNode::Branch(grandchildren) => {
                    let child_fields = get_node_fields(grandchildren, pattern_lookup);
                    if let Some(pattern_name) = pattern_lookup.get(&child_fields) {
                        // For parameterized patterns, derive accumulated metric name from first leaf
                        let pattern = metadata
                            .structural_patterns
                            .iter()
                            .find(|p| &p.name == pattern_name);
                        let is_parameterizable =
                            pattern.map(|p| p.is_parameterizable()).unwrap_or(false);

                        let arg = if is_parameterizable {
                            // Get the metric base from the first leaf descendant
                            get_pattern_instance_base(child_node, child_name)
                        } else {
                            // Fallback to tree path for non-parameterizable patterns
                            if accumulated_name.is_empty() {
                                format!("/{}", child_name)
                            } else {
                                format!("{}/{}", accumulated_name, child_name)
                            }
                        };

                        writeln!(
                            output,
                            "{}{}: create{}(this, '{}'){}",
                            indent_str, field_name, pattern_name, arg, comma
                        )
                        .unwrap();
                    } else {
                        // Not a pattern - recurse with accumulated name
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

/// Infer the accumulated metric name for a child node
fn infer_child_accumulated_name(node: &TreeNode, parent_acc: &str, field_name: &str) -> String {
    // Try to infer from first leaf descendant
    if let Some(leaf_name) = get_first_leaf_name(node) {
        // Look for field_name in the leaf metric name
        if let Some(pos) = leaf_name.find(field_name) {
            // The field_name appears in the metric - use it as base
            if pos == 0 {
                // At start - this is the base
                return field_name.to_string();
            } else if leaf_name.chars().nth(pos - 1) == Some('_') {
                // After underscore - likely an append pattern
                if parent_acc.is_empty() {
                    return field_name.to_string();
                }
                return format!("{}_{}", parent_acc, field_name);
            }
        }
    }

    // Fallback: append field name
    if parent_acc.is_empty() {
        field_name.to_string()
    } else {
        format!("{}_{}", parent_acc, field_name)
    }
}

/// Generate API methods
fn generate_api_methods(output: &mut String, endpoints: &[Endpoint]) {
    for endpoint in endpoints {
        if !endpoint.should_generate() {
            continue;
        }

        let method_name = endpoint_to_method_name(endpoint);
        let return_type = endpoint.response_type.as_deref().unwrap_or("*");

        writeln!(output, "  /**").unwrap();
        if let Some(summary) = &endpoint.summary {
            writeln!(output, "   * {}", summary).unwrap();
        }

        for param in &endpoint.path_params {
            let desc = param.description.as_deref().unwrap_or("");
            writeln!(
                output,
                "   * @param {{{}}} {} {}",
                param.param_type, param.name, desc
            )
            .unwrap();
        }
        for param in &endpoint.query_params {
            let optional = if param.required { "" } else { "=" };
            let desc = param.description.as_deref().unwrap_or("");
            writeln!(
                output,
                "   * @param {{{}{}}} [{}] {}",
                param.param_type, optional, param.name, desc
            )
            .unwrap();
        }

        writeln!(output, "   * @returns {{Promise<{}>}}", return_type).unwrap();
        writeln!(output, "   */").unwrap();

        let params = build_method_params(endpoint);
        writeln!(output, "  async {}({}) {{", method_name, params).unwrap();

        let path = build_path_template(&endpoint.path, &endpoint.path_params);

        if endpoint.query_params.is_empty() {
            writeln!(output, "    return this.get(`{}`);", path).unwrap();
        } else {
            writeln!(output, "    const params = new URLSearchParams();").unwrap();
            for param in &endpoint.query_params {
                if param.required {
                    writeln!(
                        output,
                        "    params.set('{}', String({}));",
                        param.name, param.name
                    )
                    .unwrap();
                } else {
                    writeln!(
                        output,
                        "    if ({} !== undefined) params.set('{}', String({}));",
                        param.name, param.name, param.name
                    )
                    .unwrap();
                }
            }
            writeln!(output, "    const query = params.toString();").unwrap();
            writeln!(
                output,
                "    return this.get(`{}${{query ? '?' + query : ''}}`);",
                path
            )
            .unwrap();
        }

        writeln!(output, "  }}\n").unwrap();
    }
}

fn endpoint_to_method_name(endpoint: &Endpoint) -> String {
    if let Some(op_id) = &endpoint.operation_id {
        return to_camel_case(op_id);
    }
    let parts: Vec<&str> = endpoint
        .path
        .split('/')
        .filter(|s| !s.is_empty() && !s.starts_with('{'))
        .collect();
    format!("get{}", to_pascal_case(&parts.join("_")))
}

fn build_method_params(endpoint: &Endpoint) -> String {
    let mut params = Vec::new();
    for param in &endpoint.path_params {
        params.push(param.name.clone());
    }
    for param in &endpoint.query_params {
        params.push(param.name.clone());
    }
    params.join(", ")
}

fn build_path_template(path: &str, path_params: &[super::Parameter]) -> String {
    let mut result = path.to_string();
    for param in path_params {
        let placeholder = format!("{{{}}}", param.name);
        let interpolation = format!("${{{}}}", param.name);
        result = result.replace(&placeholder, &interpolation);
    }
    result
}
