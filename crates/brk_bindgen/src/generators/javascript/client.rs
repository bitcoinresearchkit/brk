//! JavaScript base client and pattern factory generation.

use std::fmt::Write;

use brk_cohort::{
    AGE_RANGE_NAMES, AMOUNT_RANGE_NAMES, EPOCH_NAMES, GE_AMOUNT_NAMES, LT_AMOUNT_NAMES,
    MAX_AGE_NAMES, MIN_AGE_NAMES, SPENDABLE_TYPE_NAMES, TERM_NAMES, YEAR_NAMES,
};
use brk_types::{Index, PoolSlug, pools};
use serde::Serialize;
use serde_json::Value;

use crate::{
    ClientMetadata, GenericSyntax, IndexSetPattern, JavaScriptSyntax, PatternField,
    StructuralPattern, VERSION, generate_parameterized_field, generate_tree_path_field,
    to_camel_case,
};

/// Generate the base BrkClient class with HTTP functionality.
pub fn generate_base_client(output: &mut String) {
    writeln!(
        output,
        r#"/**
 * @typedef {{Object}} BrkClientOptions
 * @property {{string}} baseUrl - Base URL for the API
 * @property {{number}} [timeout] - Request timeout in milliseconds
 */

const _isBrowser = typeof window !== 'undefined' && 'caches' in window;
const _runIdle = (/** @type {{VoidFunction}} */ fn) => (globalThis.requestIdleCallback ?? setTimeout)(fn);

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
 * @template T
 * @typedef {{Object}} Endpoint
 * @property {{(onUpdate?: (value: T[]) => void) => Promise<T[]>}} get - Fetch all data points
 * @property {{(from?: number, to?: number, onUpdate?: (value: T[]) => void) => Promise<T[]>}} range - Fetch data in range
 * @property {{string}} path - The endpoint path
 */

/**
 * @template T
 * @typedef {{Object}} MetricPattern
 * @property {{string}} name - The metric name
 * @property {{Partial<Record<Index, Endpoint<T>>>}} by - Index endpoints (lazy getters)
 * @property {{() => Index[]}} indexes - Get the list of available indexes
 * @property {{(index: Index) => Endpoint<T>|undefined}} get - Get an endpoint for a specific index
 */

/**
 * Create an endpoint for a metric index.
 * @template T
 * @param {{BrkClientBase}} client
 * @param {{string}} name - The metric vec name
 * @param {{Index}} index - The index name
 * @returns {{Endpoint<T>}}
 */
function _endpoint(client, name, index) {{
  const p = `/api/metric/${{name}}/${{index}}`;
  return {{
    get: (onUpdate) => client.get(p, onUpdate),
    range: (from, to, onUpdate) => {{
      const params = new URLSearchParams();
      if (from !== undefined) params.set('from', String(from));
      if (to !== undefined) params.set('to', String(to));
      const query = params.toString();
      return client.get(query ? `${{p}}?${{query}}` : p, onUpdate);
    }},
    get path() {{ return p; }},
  }};
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
   * @returns {{Promise<T>}}
   */
  async get(path, onUpdate) {{
    const base = this.baseUrl.endsWith('/') ? this.baseUrl.slice(0, -1) : this.baseUrl;
    const url = `${{base}}${{path}}`;
    const cache = await _cachePromise;
    const cachedRes = await cache?.match(url);
    const cachedJson = cachedRes ? await cachedRes.json() : null;

    if (cachedJson) onUpdate?.(cachedJson);
    if (!globalThis.navigator?.onLine) {{
      if (cachedJson) return cachedJson;
      throw new BrkError('Offline and no cached data available');
    }}

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

/**
 * Build metric name with optional prefix.
 * @param {{string}} acc - Accumulated prefix
 * @param {{string}} s - Metric suffix
 * @returns {{string}}
 */
const _m = (acc, s) => acc ? `${{acc}}_${{s}}` : s;

"#
    )
    .unwrap();
}

/// Generate static constants for the BrkClient class.
pub fn generate_static_constants(output: &mut String) {
    fn instance_const<T: Serialize>(output: &mut String, name: &str, value: &T) {
        write_static_const(output, name, &serde_json::to_string_pretty(value).unwrap());
    }

    fn instance_const_raw(output: &mut String, name: &str, value: &str) {
        writeln!(output, "  {} = {};\n", name, value).unwrap();
    }

    instance_const_raw(output, "VERSION", &format!("\"v{}\"", VERSION));

    let indexes = Index::all();
    let indexes_json: Vec<&'static str> = indexes.iter().map(|i| i.serialize_long()).collect();
    instance_const(output, "INDEXES", &indexes_json);

    let pools = pools();
    let mut sorted_pools: Vec<_> = pools.iter().collect();
    sorted_pools.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    let pool_map: std::collections::BTreeMap<PoolSlug, &'static str> =
        sorted_pools.iter().map(|p| (p.slug(), p.name)).collect();
    instance_const(output, "POOL_ID_TO_POOL_NAME", &pool_map);

    fn instance_const_camel<T: Serialize>(output: &mut String, name: &str, value: &T) {
        let json_value: Value = serde_json::to_value(value).unwrap();
        let camel_value = camel_case_top_level_keys(json_value);
        write_static_const(output, name, &serde_json::to_string_pretty(&camel_value).unwrap());
    }

    instance_const_camel(output, "TERM_NAMES", &TERM_NAMES);
    instance_const_camel(output, "EPOCH_NAMES", &EPOCH_NAMES);
    instance_const_camel(output, "YEAR_NAMES", &YEAR_NAMES);
    instance_const_camel(output, "SPENDABLE_TYPE_NAMES", &SPENDABLE_TYPE_NAMES);
    instance_const_camel(output, "AGE_RANGE_NAMES", &AGE_RANGE_NAMES);
    instance_const_camel(output, "MAX_AGE_NAMES", &MAX_AGE_NAMES);
    instance_const_camel(output, "MIN_AGE_NAMES", &MIN_AGE_NAMES);
    instance_const_camel(output, "AMOUNT_RANGE_NAMES", &AMOUNT_RANGE_NAMES);
    instance_const_camel(output, "GE_AMOUNT_NAMES", &GE_AMOUNT_NAMES);
    instance_const_camel(output, "LT_AMOUNT_NAMES", &LT_AMOUNT_NAMES);
}

fn camel_case_top_level_keys(value: Value) -> Value {
    match value {
        Value::Object(map) => {
            let new_map: serde_json::Map<String, Value> = map
                .into_iter()
                .map(|(k, v)| (to_camel_case(&k), v))
                .collect();
            Value::Object(new_map)
        }
        other => other,
    }
}

fn indent_json_const(json: &str) -> String {
    json.lines()
        .enumerate()
        .map(|(i, line)| if i == 0 { line.to_string() } else { format!("  {}", line) })
        .collect::<Vec<_>>()
        .join("\n")
}

fn write_static_const(output: &mut String, name: &str, json: &str) {
    writeln!(output, "  {} = /** @type {{const}} */ ({});\n", name, indent_json_const(json)).unwrap();
}

/// Generate index accessor factory functions.
pub fn generate_index_accessors(output: &mut String, patterns: &[IndexSetPattern]) {
    if patterns.is_empty() {
        return;
    }

    writeln!(output, "// Index accessor factory functions\n").unwrap();

    for pattern in patterns {
        let by_fields: Vec<String> = pattern
            .indexes
            .iter()
            .map(|idx| format!("{}: Endpoint<T>", idx.serialize_long()))
            .collect();
        let by_type = format!("{{ {} }}", by_fields.join(", "));

        writeln!(output, "/**").unwrap();
        writeln!(output, " * @template T").unwrap();
        writeln!(
            output,
            " * @typedef {{{{ name: string, by: {}, indexes: () => Index[], get: (index: Index) => Endpoint<T>|undefined }}}} {}",
            by_type, pattern.name
        )
        .unwrap();
        writeln!(output, " */\n").unwrap();

        writeln!(output, "/**").unwrap();
        writeln!(output, " * Create a {} accessor", pattern.name).unwrap();
        writeln!(output, " * @template T").unwrap();
        writeln!(output, " * @param {{BrkClientBase}} client").unwrap();
        writeln!(output, " * @param {{string}} name - The metric vec name").unwrap();
        writeln!(output, " * @returns {{{}<T>}}", pattern.name).unwrap();
        writeln!(output, " */").unwrap();
        writeln!(output, "function create{}(client, name) {{", pattern.name).unwrap();
        writeln!(output, "  return {{").unwrap();
        writeln!(output, "    name,").unwrap();
        writeln!(output, "    by: {{").unwrap();

        for (i, index) in pattern.indexes.iter().enumerate() {
            let index_name = index.serialize_long();
            let comma = if i < pattern.indexes.len() - 1 { "," } else { "" };
            writeln!(
                output,
                "      get {}() {{ return _endpoint(client, name, '{}'); }}{}",
                index_name, index_name, comma
            )
            .unwrap();
        }

        writeln!(output, "    }},").unwrap();
        writeln!(output, "    indexes() {{").unwrap();

        write!(output, "      return [").unwrap();
        for (i, index) in pattern.indexes.iter().enumerate() {
            if i > 0 {
                write!(output, ", ").unwrap();
            }
            write!(output, "'{}'", index.serialize_long()).unwrap();
        }
        writeln!(output, "];").unwrap();

        writeln!(output, "    }},").unwrap();
        writeln!(output, "    get(index) {{").unwrap();
        writeln!(output, "      if (this.indexes().includes(index)) {{").unwrap();
        writeln!(output, "        return _endpoint(client, name, index);").unwrap();
        writeln!(output, "      }}").unwrap();
        writeln!(output, "    }}").unwrap();
        writeln!(output, "  }};").unwrap();
        writeln!(output, "}}\n").unwrap();
    }
}

/// Generate structural pattern factory functions.
pub fn generate_structural_patterns(
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
            let js_type = field_type_annotation(field, metadata, pattern.is_generic);
            writeln!(
                output,
                " * @property {{{}}} {}",
                js_type,
                to_camel_case(&field.name)
            )
            .unwrap();
        }
        writeln!(output, " */\n").unwrap();

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

        let param_name = if is_parameterizable { "acc" } else { "basePath" };
        writeln!(output, "function create{}(client, {}) {{", pattern.name, param_name).unwrap();
        writeln!(output, "  return {{").unwrap();

        let syntax = JavaScriptSyntax;
        for field in &pattern.fields {
            if is_parameterizable {
                generate_parameterized_field(output, &syntax, field, pattern, metadata, "    ");
            } else {
                generate_tree_path_field(output, &syntax, field, metadata, "    ");
            }
        }

        writeln!(output, "  }};").unwrap();
        writeln!(output, "}}\n").unwrap();
    }
}

fn field_type_annotation(field: &PatternField, metadata: &ClientMetadata, is_generic: bool) -> String {
    metadata.field_type_annotation(field, is_generic, None, GenericSyntax::JAVASCRIPT)
}

/// Get field type with specific generic value type.
pub fn field_type_with_generic(
    field: &PatternField,
    metadata: &ClientMetadata,
    is_generic: bool,
    generic_value_type: Option<&str>,
) -> String {
    metadata.field_type_annotation(field, is_generic, generic_value_type, GenericSyntax::JAVASCRIPT)
}
