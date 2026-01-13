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
    ClientMetadata, GenericSyntax, IndexSetPattern, JavaScriptSyntax, StructuralPattern, VERSION,
    generate_parameterized_field, to_camel_case,
};

/// Generate the base BrkClient class with HTTP functionality.
pub fn generate_base_client(output: &mut String) {
    writeln!(
        output,
        r#"/**
 * @typedef {{Object}} BrkClientOptions
 * @property {{string}} baseUrl - Base URL for the API
 * @property {{number}} [timeout] - Request timeout in milliseconds
 * @property {{string|boolean}} [cache] - Enable browser cache with default name (true), custom name (string), or disable (false). No effect in Node.js. Default: true
 */

const _isBrowser = typeof window !== 'undefined' && 'caches' in window;
const _runIdle = (/** @type {{VoidFunction}} */ fn) => (globalThis.requestIdleCallback ?? setTimeout)(fn);
const _defaultCacheName = '__BRK_CLIENT__';

/**
 * @param {{string|boolean|undefined}} cache
 * @returns {{Promise<Cache | null>}}
 */
const _openCache = (cache) => {{
  if (!_isBrowser || cache === false) return Promise.resolve(null);
  const name = typeof cache === 'string' ? cache : _defaultCacheName;
  return caches.open(name).catch(() => null);
}};

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
 * @typedef {{Object}} MetricData
 * @property {{number}} total - Total number of data points
 * @property {{number}} start - Start index (inclusive)
 * @property {{number}} end - End index (exclusive)
 * @property {{T[]}} data - The metric data
 */
/** @typedef {{MetricData<any>}} AnyMetricData */

/**
 * Thenable interface for await support.
 * @template T
 * @typedef {{(onfulfilled?: (value: MetricData<T>) => MetricData<T>, onrejected?: (reason: Error) => never) => Promise<MetricData<T>>}} Thenable
 */

/**
 * Metric endpoint builder. Callable (returns itself) so both .by.dateindex and .by.dateindex() work.
 * @template T
 * @typedef {{Object}} MetricEndpointBuilder
 * @property {{(index: number) => SingleItemBuilder<T>}} get - Get single item at index
 * @property {{(start?: number, end?: number) => RangeBuilder<T>}} slice - Slice like Array.slice
 * @property {{(n: number) => RangeBuilder<T>}} first - Get first n items
 * @property {{(n: number) => RangeBuilder<T>}} last - Get last n items
 * @property {{(n: number) => SkippedBuilder<T>}} skip - Skip first n items, chain with take()
 * @property {{(onUpdate?: (value: MetricData<T>) => void) => Promise<MetricData<T>>}} fetch - Fetch all data
 * @property {{() => Promise<string>}} fetchCsv - Fetch all data as CSV
 * @property {{Thenable<T>}} then - Thenable (await endpoint)
 * @property {{string}} path - The endpoint path
 */
/** @typedef {{MetricEndpointBuilder<any>}} AnyMetricEndpointBuilder */

/**
 * @template T
 * @typedef {{Object}} SingleItemBuilder
 * @property {{(onUpdate?: (value: MetricData<T>) => void) => Promise<MetricData<T>>}} fetch - Fetch the item
 * @property {{() => Promise<string>}} fetchCsv - Fetch as CSV
 * @property {{Thenable<T>}} then - Thenable
 */

/**
 * @template T
 * @typedef {{Object}} SkippedBuilder
 * @property {{(n: number) => RangeBuilder<T>}} take - Take n items after skipped position
 * @property {{(onUpdate?: (value: MetricData<T>) => void) => Promise<MetricData<T>>}} fetch - Fetch from skipped position to end
 * @property {{() => Promise<string>}} fetchCsv - Fetch as CSV
 * @property {{Thenable<T>}} then - Thenable
 */

/**
 * @template T
 * @typedef {{Object}} RangeBuilder
 * @property {{(onUpdate?: (value: MetricData<T>) => void) => Promise<MetricData<T>>}} fetch - Fetch the range
 * @property {{() => Promise<string>}} fetchCsv - Fetch as CSV
 * @property {{Thenable<T>}} then - Thenable
 */

/**
 * @template T
 * @typedef {{Object}} MetricPattern
 * @property {{string}} name - The metric name
 * @property {{Readonly<Partial<Record<Index, MetricEndpointBuilder<T>>>>}} by - Index endpoints as lazy getters. Access via .by.dateindex or .by['dateindex']
 * @property {{() => Index[]}} indexes - Get the list of available indexes
 * @property {{(index: Index) => MetricEndpointBuilder<T>|undefined}} get - Get an endpoint for a specific index
 */

/** @typedef {{MetricPattern<any>}} AnyMetricPattern */

/**
 * Create a metric endpoint builder with typestate pattern.
 * @template T
 * @param {{BrkClientBase}} client
 * @param {{string}} name - The metric vec name
 * @param {{Index}} index - The index name
 * @returns {{MetricEndpointBuilder<T>}}
 */
function _endpoint(client, name, index) {{
  const p = `/api/metric/${{name}}/${{index}}`;

  /**
   * @param {{number}} [start]
   * @param {{number}} [end]
   * @param {{string}} [format]
   * @returns {{string}}
   */
  const buildPath = (start, end, format) => {{
    const params = new URLSearchParams();
    if (start !== undefined) params.set('start', String(start));
    if (end !== undefined) params.set('end', String(end));
    if (format) params.set('format', format);
    const query = params.toString();
    return query ? `${{p}}?${{query}}` : p;
  }};

  /**
   * @param {{number}} [start]
   * @param {{number}} [end]
   * @returns {{RangeBuilder<T>}}
   */
  const rangeBuilder = (start, end) => ({{
    fetch(onUpdate) {{ return client.getJson(buildPath(start, end), onUpdate); }},
    fetchCsv() {{ return client.getText(buildPath(start, end, 'csv')); }},
    then(resolve, reject) {{ return this.fetch().then(resolve, reject); }},
  }});

  /**
   * @param {{number}} index
   * @returns {{SingleItemBuilder<T>}}
   */
  const singleItemBuilder = (index) => ({{
    fetch(onUpdate) {{ return client.getJson(buildPath(index, index + 1), onUpdate); }},
    fetchCsv() {{ return client.getText(buildPath(index, index + 1, 'csv')); }},
    then(resolve, reject) {{ return this.fetch().then(resolve, reject); }},
  }});

  /**
   * @param {{number}} start
   * @returns {{SkippedBuilder<T>}}
   */
  const skippedBuilder = (start) => ({{
    take(n) {{ return rangeBuilder(start, start + n); }},
    fetch(onUpdate) {{ return client.getJson(buildPath(start, undefined), onUpdate); }},
    fetchCsv() {{ return client.getText(buildPath(start, undefined, 'csv')); }},
    then(resolve, reject) {{ return this.fetch().then(resolve, reject); }},
  }});

  /** @type {{MetricEndpointBuilder<T>}} */
  const endpoint = {{
    get(index) {{ return singleItemBuilder(index); }},
    slice(start, end) {{ return rangeBuilder(start, end); }},
    first(n) {{ return rangeBuilder(undefined, n); }},
    last(n) {{ return n === 0 ? rangeBuilder(undefined, 0) : rangeBuilder(-n, undefined); }},
    skip(n) {{ return skippedBuilder(n); }},
    fetch(onUpdate) {{ return client.getJson(buildPath(), onUpdate); }},
    fetchCsv() {{ return client.getText(buildPath(undefined, undefined, 'csv')); }},
    then(resolve, reject) {{ return this.fetch().then(resolve, reject); }},
    get path() {{ return p; }},
  }};

  return endpoint;
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
    /** @type {{Promise<Cache | null>}} */
    this._cachePromise = _openCache(isString ? undefined : options.cache);
  }}

  /**
   * @param {{string}} path
   * @returns {{Promise<Response>}}
   */
  async get(path) {{
    const base = this.baseUrl.endsWith('/') ? this.baseUrl.slice(0, -1) : this.baseUrl;
    const url = `${{base}}${{path}}`;
    const res = await fetch(url, {{ signal: AbortSignal.timeout(this.timeout) }});
    if (!res.ok) throw new BrkError(`HTTP ${{res.status}}: ${{url}}`, res.status);
    return res;
  }}

  /**
   * Make a GET request with stale-while-revalidate caching
   * @template T
   * @param {{string}} path
   * @param {{(value: T) => void}} [onUpdate] - Called when data is available
   * @returns {{Promise<T>}}
   */
  async getJson(path, onUpdate) {{
    const base = this.baseUrl.endsWith('/') ? this.baseUrl.slice(0, -1) : this.baseUrl;
    const url = `${{base}}${{path}}`;
    const cache = await this._cachePromise;
    const cachedRes = await cache?.match(url);
    const cachedJson = cachedRes ? await cachedRes.json() : null;

    if (cachedJson) onUpdate?.(cachedJson);
    if (globalThis.navigator?.onLine === false) {{
      if (cachedJson) return cachedJson;
      throw new BrkError('Offline and no cached data available');
    }}

    try {{
      const res = await this.get(path);
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

  /**
   * Make a GET request and return raw text (for CSV responses)
   * @param {{string}} path
   * @returns {{Promise<string>}}
   */
  async getText(path) {{
    const res = await this.get(path);
    return res.text();
  }}
}}

/**
 * Build metric name with suffix.
 * @param {{string}} acc - Accumulated prefix
 * @param {{string}} s - Metric suffix
 * @returns {{string}}
 */
const _m = (acc, s) => s ? (acc ? `${{acc}}_${{s}}` : s) : acc;

/**
 * Build metric name with prefix.
 * @param {{string}} prefix - Prefix to prepend
 * @param {{string}} acc - Accumulated name
 * @returns {{string}}
 */
const _p = (prefix, acc) => acc ? `${{prefix}}_${{acc}}` : prefix;

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
        write_static_const(
            output,
            name,
            &serde_json::to_string_pretty(&camel_value).unwrap(),
        );
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
        .map(|(i, line)| {
            if i == 0 {
                line.to_string()
            } else {
                format!("  {}", line)
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn write_static_const(output: &mut String, name: &str, json: &str) {
    writeln!(
        output,
        "  {} = /** @type {{const}} */ ({});\n",
        name,
        indent_json_const(json)
    )
    .unwrap();
}

/// Generate index accessor factory functions.
pub fn generate_index_accessors(output: &mut String, patterns: &[IndexSetPattern]) {
    if patterns.is_empty() {
        return;
    }

    writeln!(output, "// Index accessor factory functions\n").unwrap();

    for pattern in patterns {
        // Use 'readonly' to indicate these are getters (lazy evaluation)
        let by_fields: Vec<String> = pattern
            .indexes
            .iter()
            .map(|idx| {
                format!(
                    "readonly {}: MetricEndpointBuilder<T>",
                    idx.serialize_long()
                )
            })
            .collect();
        let by_type = format!("{{ {} }}", by_fields.join(", "));

        writeln!(output, "/**").unwrap();
        writeln!(
            output,
            " * Metric pattern with index endpoints as lazy getters."
        )
        .unwrap();
        writeln!(
            output,
            " * Access via property (.by.dateindex) or bracket notation (.by['dateindex'])."
        )
        .unwrap();
        writeln!(output, " * @template T").unwrap();
        writeln!(
            output,
            " * @typedef {{{{ name: string, by: {}, indexes: () => Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }}}} {}",
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
            let comma = if i < pattern.indexes.len() - 1 {
                ","
            } else {
                ""
            };
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
        // Generate typedef
        writeln!(output, "/**").unwrap();
        if pattern.is_generic {
            writeln!(output, " * @template T").unwrap();
        }
        writeln!(output, " * @typedef {{Object}} {}", pattern.name).unwrap();
        for field in &pattern.fields {
            let js_type = metadata.field_type_annotation(
                field,
                pattern.is_generic,
                None,
                GenericSyntax::JAVASCRIPT,
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

        // Generate factory function for ALL patterns
        writeln!(output, "/**").unwrap();
        writeln!(output, " * Create a {} pattern node", pattern.name).unwrap();
        if pattern.is_generic {
            writeln!(output, " * @template T").unwrap();
        }
        writeln!(output, " * @param {{BrkClientBase}} client").unwrap();
        writeln!(output, " * @param {{string}} acc - Accumulated metric name").unwrap();
        let return_type = if pattern.is_generic {
            format!("{}<T>", pattern.name)
        } else {
            pattern.name.clone()
        };
        writeln!(output, " * @returns {{{}}}", return_type).unwrap();
        writeln!(output, " */").unwrap();

        writeln!(output, "function create{}(client, acc) {{", pattern.name).unwrap();
        writeln!(output, "  return {{").unwrap();

        let syntax = JavaScriptSyntax;
        for field in &pattern.fields {
            generate_parameterized_field(output, &syntax, field, pattern, metadata, "    ");
        }

        writeln!(output, "  }};").unwrap();
        writeln!(output, "}}\n").unwrap();
    }
}
