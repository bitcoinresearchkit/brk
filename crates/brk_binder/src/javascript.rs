use std::{collections::HashSet, fmt::Write as FmtWrite, fs, io, path::Path};

use brk_types::{Index, TreeNode};
use serde_json::Value;

use crate::{
    ClientMetadata, Endpoint, FieldNamePosition, IndexSetPattern, PatternField, StructuralPattern,
    TypeSchemas, extract_inner_type, get_fields_with_child_info, get_first_leaf_name,
    get_node_fields, get_pattern_instance_base, to_camel_case, to_pascal_case,
};

/// Generate JavaScript + JSDoc client from metadata and OpenAPI endpoints.
///
/// `output_path` is the full path to the output file (e.g., "modules/brk-client/index.js").
pub fn generate_javascript_client(
    metadata: &ClientMetadata,
    endpoints: &[Endpoint],
    schemas: &TypeSchemas,
    output_path: &Path,
) -> io::Result<()> {
    let mut output = String::new();

    writeln!(output, "// Auto-generated BRK JavaScript client").unwrap();
    writeln!(output, "// Do not edit manually\n").unwrap();

    generate_type_definitions(&mut output, schemas);
    generate_base_client(&mut output);
    generate_index_accessors(&mut output, &metadata.index_set_patterns);
    generate_structural_patterns(&mut output, &metadata.structural_patterns, metadata);
    generate_tree_typedefs(&mut output, &metadata.catalog, metadata);
    generate_main_client(&mut output, &metadata.catalog, metadata, endpoints);

    fs::write(output_path, output)?;

    Ok(())
}

fn generate_type_definitions(output: &mut String, schemas: &TypeSchemas) {
    if schemas.is_empty() {
        return;
    }

    writeln!(output, "// Type definitions\n").unwrap();

    for (name, schema) in schemas {
        let js_type = schema_to_js_type_ctx(schema, Some(name));

        if is_primitive_alias(schema) {
            writeln!(output, "/** @typedef {{{}}} {} */", js_type, name).unwrap();
        } else if let Some(props) = schema.get("properties").and_then(|p| p.as_object()) {
            writeln!(output, "/**").unwrap();
            writeln!(output, " * @typedef {{Object}} {}", name).unwrap();
            for (prop_name, prop_schema) in props {
                let prop_type = schema_to_js_type_ctx(prop_schema, Some(name));
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
            writeln!(output, "/** @typedef {{{}}} {} */", js_type, name).unwrap();
        }
    }
    writeln!(output).unwrap();
}

fn is_primitive_alias(schema: &Value) -> bool {
    schema.get("properties").is_none()
        && schema.get("items").is_none()
        && schema.get("anyOf").is_none()
        && schema.get("oneOf").is_none()
        && schema.get("enum").is_none()
}

fn json_type_to_js(ty: &str, schema: &Value, current_type: Option<&str>) -> String {
    match ty {
        "integer" | "number" => "number".to_string(),
        "boolean" => "boolean".to_string(),
        "string" => "string".to_string(),
        "null" => "null".to_string(),
        "array" => {
            let item_type = schema
                .get("items")
                .map(|s| schema_to_js_type_ctx(s, current_type))
                .unwrap_or_else(|| "*".to_string());
            format!("{}[]", item_type)
        }
        "object" => {
            if let Some(add_props) = schema.get("additionalProperties") {
                let value_type = schema_to_js_type_ctx(add_props, current_type);
                return format!("{{ [key: string]: {} }}", value_type);
            }
            "Object".to_string()
        }
        _ => "*".to_string(),
    }
}

fn schema_to_js_type_ctx(schema: &Value, current_type: Option<&str>) -> String {
    if let Some(all_of) = schema.get("allOf").and_then(|v| v.as_array()) {
        for item in all_of {
            let resolved = schema_to_js_type_ctx(item, current_type);
            if resolved != "*" {
                return resolved;
            }
        }
    }

    if let Some(ref_path) = schema.get("$ref").and_then(|r| r.as_str()) {
        return ref_path.rsplit('/').next().unwrap_or("*").to_string();
    }

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

    if let Some(ty) = schema.get("type") {
        if let Some(type_array) = ty.as_array() {
            let types: Vec<String> = type_array
                .iter()
                .filter_map(|t| t.as_str())
                .filter(|t| *t != "null")
                .map(|t| json_type_to_js(t, schema, current_type))
                .collect();
            let has_null = type_array.iter().any(|t| t.as_str() == Some("null"));

            if types.len() == 1 {
                let base_type = &types[0];
                return if has_null {
                    format!("?{}", base_type)
                } else {
                    base_type.clone()
                };
            } else if !types.is_empty() {
                let union = format!("({})", types.join("|"));
                return if has_null {
                    format!("?{}", union)
                } else {
                    union
                };
            }
        }

        if let Some(ty_str) = ty.as_str() {
            return json_type_to_js(ty_str, schema, current_type);
        }
    }

    if let Some(variants) = schema
        .get("anyOf")
        .or_else(|| schema.get("oneOf"))
        .and_then(|v| v.as_array())
    {
        let types: Vec<String> = variants
            .iter()
            .map(|v| schema_to_js_type_ctx(v, current_type))
            .collect();
        let filtered: Vec<_> = types.iter().filter(|t| *t != "*").collect();
        if !filtered.is_empty() {
            return format!(
                "({})",
                filtered
                    .iter()
                    .map(|s| s.as_str())
                    .collect::<Vec<_>>()
                    .join("|")
            );
        }
        return format!("({})", types.join("|"));
    }

    if let Some(format) = schema.get("format").and_then(|f| f.as_str()) {
        return match format {
            "int32" | "int64" => "number".to_string(),
            "float" | "double" => "number".to_string(),
            "date" | "date-time" => "string".to_string(),
            _ => "*".to_string(),
        };
    }

    "*".to_string()
}

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

fn generate_index_accessors(output: &mut String, patterns: &[IndexSetPattern]) {
    if patterns.is_empty() {
        return;
    }

    writeln!(output, "// Index accessor factory functions\n").unwrap();

    for pattern in patterns {
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

fn index_to_camel_case(index: &Index) -> String {
    format!("by{}", to_pascal_case(index.serialize_long()))
}

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
        let is_parameterizable = pattern.is_parameterizable();

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
        if pattern.is_generic {
            writeln!(output, " * @template T").unwrap();
        }
        writeln!(output, " * @param {{BrkClientBase}} client").unwrap();
        if is_parameterizable {
            writeln!(output, " * @param {{string}} acc - Accumulated metric name").unwrap();
        } else {
            writeln!(output, " * @param {{string}} basePath").unwrap();
        }
        let return_type = if pattern.is_generic {
            format!("{}<T>", pattern.name)
        } else {
            pattern.name.clone()
        };
        writeln!(output, " * @returns {{{}}}", return_type).unwrap();
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

fn generate_parameterized_field(
    output: &mut String,
    field: &PatternField,
    pattern: &StructuralPattern,
    metadata: &ClientMetadata,
    comma: &str,
) {
    let field_name_js = to_camel_case(&field.name);

    if metadata.is_pattern_type(&field.rust_type) {
        let child_acc = if let Some(pos) = pattern.get_field_position(&field.name) {
            match pos {
                FieldNamePosition::Append(suffix) => format!("`${{acc}}{}`", suffix),
                FieldNamePosition::Prepend(prefix) => format!("`{}{}`", prefix, "${acc}"),
                FieldNamePosition::Identity => "acc".to_string(),
                FieldNamePosition::SetBase(base) => format!("'{}'", base),
            }
        } else {
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

    let metric_expr = if let Some(pos) = pattern.get_field_position(&field.name) {
        match pos {
            FieldNamePosition::Append(suffix) => format!("`/${{acc}}{suffix}`"),
            FieldNamePosition::Prepend(prefix) => format!("`/{prefix}${{acc}}`"),
            FieldNamePosition::Identity => "`/${acc}`".to_string(),
            FieldNamePosition::SetBase(base) => format!("'/{base}'"),
        }
    } else {
        format!("`/${{acc}}_{}`", field.name)
    };

    if metadata.field_uses_accessor(field) {
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
    } else if metadata.field_uses_accessor(field) {
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

fn field_to_js_type_generic(
    field: &PatternField,
    metadata: &ClientMetadata,
    is_generic: bool,
) -> String {
    field_to_js_type_with_generic_value(field, metadata, is_generic, None)
}

fn field_to_js_type_with_generic_value(
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
                .unwrap_or(if is_generic { "T" } else { "unknown" });
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
        let js_type =
            field_to_js_type_with_generic_value(field, metadata, false, generic_value_type);
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

    writeln!(
        output,
        "export {{ BrkClient, BrkClientBase, BrkError, MetricNode }};"
    )
    .unwrap();
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
                        let pattern = metadata
                            .structural_patterns
                            .iter()
                            .find(|p| &p.name == pattern_name);
                        let is_parameterizable =
                            pattern.map(|p| p.is_parameterizable()).unwrap_or(false);

                        let arg = if is_parameterizable {
                            get_pattern_instance_base(child_node, child_name)
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
    if let Some(leaf_name) = get_first_leaf_name(node)
        && let Some(pos) = leaf_name.find(field_name)
    {
        if pos == 0 {
            return field_name.to_string();
        } else if leaf_name.chars().nth(pos - 1) == Some('_') {
            if parent_acc.is_empty() {
                return field_name.to_string();
            }
            return format!("{}_{}", parent_acc, field_name);
        }
    }

    if parent_acc.is_empty() {
        field_name.to_string()
    } else {
        format!("{}_{}", parent_acc, field_name)
    }
}

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
        if let Some(desc) = &endpoint.description
            && endpoint.summary.as_ref() != Some(desc)
        {
            writeln!(output, "   * @description {}", desc).unwrap();
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
    to_camel_case(&endpoint.operation_name())
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
