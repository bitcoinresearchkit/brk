use std::collections::HashSet;
use std::fmt::Write as FmtWrite;
use std::fs;
use std::io;
use std::path::Path;

use brk_types::{Index, TreeNode};

use super::{ClientMetadata, Endpoint, IndexSetPattern, PatternField, StructuralPattern, get_node_fields, to_camel_case, to_pascal_case};

/// Generate JavaScript + JSDoc client from metadata and OpenAPI endpoints
pub fn generate_javascript_client(
    metadata: &ClientMetadata,
    endpoints: &[Endpoint],
    output_dir: &Path,
) -> io::Result<()> {
    let mut output = String::new();

    // Header
    writeln!(output, "// Auto-generated BRK JavaScript client").unwrap();
    writeln!(output, "// Do not edit manually\n").unwrap();

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
        writeln!(output, "function create{}(client, basePath) {{", pattern.name).unwrap();
        writeln!(output, "  return {{").unwrap();

        for (i, index) in pattern.indexes.iter().enumerate() {
            let field_name = index_to_camel_case(index);
            let path_segment = index.serialize_long();
            let comma = if i < pattern.indexes.len() - 1 { "," } else { "" };
            writeln!(
                output,
                "    {}: new MetricNode(client, `${{basePath}}/{}`){}",
                field_name, path_segment, comma
            ).unwrap();
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
fn generate_structural_patterns(output: &mut String, patterns: &[StructuralPattern], metadata: &ClientMetadata) {
    if patterns.is_empty() {
        return;
    }

    writeln!(output, "// Reusable structural pattern factories\n").unwrap();

    for pattern in patterns {
        // Generate JSDoc typedef
        writeln!(output, "/**").unwrap();
        writeln!(output, " * @typedef {{Object}} {}", pattern.name).unwrap();
        for field in &pattern.fields {
            let js_type = field_to_js_type(field, metadata);
            writeln!(output, " * @property {{{}}} {}", js_type, to_camel_case(&field.name)).unwrap();
        }
        writeln!(output, " */\n").unwrap();

        // Generate factory function
        writeln!(output, "/**").unwrap();
        writeln!(output, " * Create a {} pattern node", pattern.name).unwrap();
        writeln!(output, " * @param {{BrkClientBase}} client").unwrap();
        writeln!(output, " * @param {{string}} basePath").unwrap();
        writeln!(output, " * @returns {{{}}}", pattern.name).unwrap();
        writeln!(output, " */").unwrap();
        writeln!(output, "function create{}(client, basePath) {{", pattern.name).unwrap();
        writeln!(output, "  return {{").unwrap();

        for (i, field) in pattern.fields.iter().enumerate() {
            let comma = if i < pattern.fields.len() - 1 { "," } else { "" };
            if metadata.is_pattern_type(&field.rust_type) {
                writeln!(
                    output,
                    "    {}: create{}(client, `${{basePath}}/{}`){}",
                    to_camel_case(&field.name), field.rust_type, field.name, comma
                ).unwrap();
            } else if field_uses_accessor(field, metadata) {
                let accessor = metadata.find_index_set_pattern(&field.indexes).unwrap();
                writeln!(
                    output,
                    "    {}: create{}(client, `${{basePath}}/{}`){}",
                    to_camel_case(&field.name), accessor.name, field.name, comma
                ).unwrap();
            } else {
                writeln!(
                    output,
                    "    {}: new MetricNode(client, `${{basePath}}/{}`){}",
                    to_camel_case(&field.name), field.name, comma
                ).unwrap();
            }
        }

        writeln!(output, "  }};").unwrap();
        writeln!(output, "}}\n").unwrap();
    }
}

/// Convert pattern field to JavaScript/JSDoc type
fn field_to_js_type(field: &PatternField, metadata: &ClientMetadata) -> String {
    if metadata.is_pattern_type(&field.rust_type) {
        // Pattern type - use pattern name directly
        field.rust_type.clone()
    } else if let Some(accessor) = metadata.find_index_set_pattern(&field.indexes) {
        // Leaf with a reusable accessor pattern
        let js_type = json_type_to_js(&field.json_type);
        format!("{}<{}>", accessor.name, js_type)
    } else {
        // Leaf with unique index set - use MetricNode directly
        let js_type = json_type_to_js(&field.json_type);
        format!("MetricNode<{}>", js_type)
    }
}

/// Check if a field should use an index accessor
fn field_uses_accessor(field: &PatternField, metadata: &ClientMetadata) -> bool {
    metadata.find_index_set_pattern(&field.indexes).is_some()
}

/// Convert JSON Schema type to JSDoc type
fn json_type_to_js(json_type: &str) -> &str {
    match json_type {
        "integer" | "number" => "number",
        "boolean" => "boolean",
        "string" => "string",
        "array" => "Array",
        "object" => "Object",
        _ => "*",
    }
}

/// Generate tree typedefs
fn generate_tree_typedefs(
    output: &mut String,
    catalog: &TreeNode,
    metadata: &ClientMetadata,
) {
    writeln!(output, "// Catalog tree typedefs\n").unwrap();

    let pattern_lookup = metadata.pattern_lookup();
    let mut generated = HashSet::new();
    generate_tree_typedef(output, "CatalogTree", catalog, &pattern_lookup, metadata, &mut generated);
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
        // Build signature
        let fields = get_node_fields(children, pattern_lookup);

        // Skip if this matches a pattern (already generated)
        if pattern_lookup.contains_key(&fields) && pattern_lookup.get(&fields) != Some(&name.to_string()) {
            return;
        }

        if generated.contains(name) {
            return;
        }
        generated.insert(name.to_string());

        writeln!(output, "/**").unwrap();
        writeln!(output, " * @typedef {{Object}} {}", name).unwrap();

        for field in &fields {
            let js_type = field_to_js_type(field, metadata);
            writeln!(output, " * @property {{{}}} {}", js_type, to_camel_case(&field.name)).unwrap();
        }

        writeln!(output, " */\n").unwrap();

        // Generate child typedefs
        for (child_name, child_node) in children {
            if let TreeNode::Branch(grandchildren) = child_node {
                let child_fields = get_node_fields(grandchildren, pattern_lookup);
                if !pattern_lookup.contains_key(&child_fields) {
                    let child_type_name = format!("{}_{}", name, to_pascal_case(child_name));
                    generate_tree_typedef(output, &child_type_name, child_node, pattern_lookup, metadata, generated);
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
    writeln!(output, " * Main BRK client with catalog tree and API methods").unwrap();
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
    writeln!(output, "export {{ BrkClient, BrkClientBase, BrkError, MetricNode }};").unwrap();
}

/// Generate tree initializer
fn generate_tree_initializer(
    output: &mut String,
    node: &TreeNode,
    path: &str,
    indent: usize,
    pattern_lookup: &std::collections::HashMap<Vec<PatternField>, String>,
    metadata: &ClientMetadata,
) {
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
                    if let Some(accessor) = metadata.find_index_set_pattern(leaf.indexes()) {
                        writeln!(
                            output,
                            "{}{}: create{}(this, '{}'){}",
                            indent_str, field_name, accessor.name, child_path, comma
                        ).unwrap();
                    } else {
                        writeln!(
                            output,
                            "{}{}: new MetricNode(this, '{}'){}",
                            indent_str, field_name, child_path, comma
                        ).unwrap();
                    }
                }
                TreeNode::Branch(grandchildren) => {
                    let child_fields = get_node_fields(grandchildren, pattern_lookup);
                    if let Some(pattern_name) = pattern_lookup.get(&child_fields) {
                        writeln!(
                            output,
                            "{}{}: create{}(this, '{}'){}",
                            indent_str, field_name, pattern_name, child_path, comma
                        ).unwrap();
                    } else {
                        writeln!(output, "{}{}: {{", indent_str, field_name).unwrap();
                        generate_tree_initializer(output, child_node, &child_path, indent + 1, pattern_lookup, metadata);
                        writeln!(output, "{}}}{}", indent_str, comma).unwrap();
                    }
                }
            }
        }
    }
}

/// Generate API methods
fn generate_api_methods(output: &mut String, endpoints: &[Endpoint]) {
    for endpoint in endpoints {
        if endpoint.method != "GET" {
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
            writeln!(output, "   * @param {{{}}} {} {}", param.param_type, param.name, desc).unwrap();
        }
        for param in &endpoint.query_params {
            let optional = if param.required { "" } else { "=" };
            let desc = param.description.as_deref().unwrap_or("");
            writeln!(output, "   * @param {{{}{}}} [{}] {}", param.param_type, optional, param.name, desc).unwrap();
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
                    writeln!(output, "    params.set('{}', String({}));", param.name, param.name).unwrap();
                } else {
                    writeln!(output, "    if ({} !== undefined) params.set('{}', String({}));", param.name, param.name, param.name).unwrap();
                }
            }
            writeln!(output, "    const query = params.toString();").unwrap();
            writeln!(output, "    return this.get(`{}${{query ? '?' + query : ''}}`);", path).unwrap();
        }

        writeln!(output, "  }}\n").unwrap();
    }
}

fn endpoint_to_method_name(endpoint: &Endpoint) -> String {
    if let Some(op_id) = &endpoint.operation_id {
        return to_camel_case(op_id);
    }
    let parts: Vec<&str> = endpoint.path.split('/').filter(|s| !s.is_empty() && !s.starts_with('{')).collect();
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

