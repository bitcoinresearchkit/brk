//! JavaScript base client and pattern factory generation.

use std::fmt::Write;

use crate::{
    ClientConstants, ClientMetadata, CohortConstants, GenericSyntax, IndexSetPattern,
    JavaScriptSyntax, StructuralPattern, camel_case_keys, format_json,
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

// Date conversion constants and helpers
const _GENESIS = new Date(2009, 0, 3);  // dateindex 0, weekindex 0
const _DAY_ONE = new Date(2009, 0, 9);  // dateindex 1 (6 day gap after genesis)
const _MS_PER_DAY = 24 * 60 * 60 * 1000;
const _MS_PER_WEEK = 7 * _MS_PER_DAY;
const _DATE_INDEXES = new Set(['dateindex', 'weekindex', 'monthindex', 'yearindex', 'quarterindex', 'semesterindex', 'decadeindex']);

/** @param {{number}} months @returns {{globalThis.Date}} */
const _addMonths = (months) => new Date(2009, months, 1);

/**
 * Convert an index value to a Date for date-based indexes.
 * @param {{Index}} index - The index type
 * @param {{number}} i - The index value
 * @returns {{globalThis.Date}}
 */
function indexToDate(index, i) {{
  switch (index) {{
    case 'dateindex': return i === 0 ? _GENESIS : new Date(_DAY_ONE.getTime() + (i - 1) * _MS_PER_DAY);
    case 'weekindex': return new Date(_GENESIS.getTime() + i * _MS_PER_WEEK);
    case 'monthindex': return _addMonths(i);
    case 'yearindex': return new Date(2009 + i, 0, 1);
    case 'quarterindex': return _addMonths(i * 3);
    case 'semesterindex': return _addMonths(i * 6);
    case 'decadeindex': return new Date(2009 + i * 10, 0, 1);
    default: throw new Error(`${{index}} is not a date-based index`);
  }}
}}

/**
 * Check if an index type is date-based.
 * @param {{Index}} index
 * @returns {{boolean}}
 */
function isDateIndex(index) {{
  return _DATE_INDEXES.has(index);
}}

/**
 * Wrap raw metric data with helper methods.
 * @template T
 * @param {{MetricData<T>}} raw - Raw JSON response
 * @returns {{MetricData<T>}}
 */
function _wrapMetricData(raw) {{
  const {{ index, start, end, data }} = raw;
  return /** @type {{MetricData<T>}} */ ({{
    ...raw,
    dates() {{
      /** @type {{globalThis.Date[]}} */
      const result = [];
      for (let i = start; i < end; i++) result.push(indexToDate(index, i));
      return result;
    }},
    indexes() {{
      /** @type {{number[]}} */
      const result = [];
      for (let i = start; i < end; i++) result.push(i);
      return result;
    }},
    toDateMap() {{
      /** @type {{Map<globalThis.Date, T>}} */
      const map = new Map();
      for (let i = 0; i < data.length; i++) map.set(indexToDate(index, start + i), data[i]);
      return map;
    }},
    toIndexMap() {{
      /** @type {{Map<number, T>}} */
      const map = new Map();
      for (let i = 0; i < data.length; i++) map.set(start + i, data[i]);
      return map;
    }},
    dateEntries() {{
      /** @type {{Array<[globalThis.Date, T]>}} */
      const result = [];
      for (let i = 0; i < data.length; i++) result.push([indexToDate(index, start + i), data[i]]);
      return result;
    }},
    indexEntries() {{
      /** @type {{Array<[number, T]>}} */
      const result = [];
      for (let i = 0; i < data.length; i++) result.push([start + i, data[i]]);
      return result;
    }},
    *iter() {{
      for (let i = 0; i < data.length; i++) yield [start + i, data[i]];
    }},
    *iterDates() {{
      for (let i = 0; i < data.length; i++) yield [indexToDate(index, start + i), data[i]];
    }},
    [Symbol.iterator]() {{
      return this.iter();
    }},
  }});
}}

/**
 * @template T
 * @typedef {{Object}} MetricData
 * @property {{number}} version - Version of the metric data
 * @property {{Index}} index - The index type used for this query
 * @property {{number}} total - Total number of data points
 * @property {{number}} start - Start index (inclusive)
 * @property {{number}} end - End index (exclusive)
 * @property {{string}} stamp - ISO 8601 timestamp of when the response was generated
 * @property {{T[]}} data - The metric data
 * @property {{() => globalThis.Date[]}} dates - Convert index range to dates (date-based indexes only)
 * @property {{() => number[]}} indexes - Get index range as array
 * @property {{() => Map<globalThis.Date, T>}} toDateMap - Return data as Map keyed by date (date-based only)
 * @property {{() => Map<number, T>}} toIndexMap - Return data as Map keyed by index
 * @property {{() => Array<[globalThis.Date, T]>}} dateEntries - Return data as [date, value] pairs (date-based only)
 * @property {{() => Array<[number, T]>}} indexEntries - Return data as [index, value] pairs
 * @property {{() => IterableIterator<[number, T]>}} iter - Iterate over [index, value] pairs
 * @property {{() => IterableIterator<[globalThis.Date, T]>}} iterDates - Iterate over [date, value] pairs (date-based only)
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
 * @property {{() => readonly Index[]}} indexes - Get the list of available indexes
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
    fetch(onUpdate) {{ return client._fetchMetricData(buildPath(start, end), onUpdate); }},
    fetchCsv() {{ return client.getText(buildPath(start, end, 'csv')); }},
    then(resolve, reject) {{ return this.fetch().then(resolve, reject); }},
  }});

  /**
   * @param {{number}} idx
   * @returns {{SingleItemBuilder<T>}}
   */
  const singleItemBuilder = (idx) => ({{
    fetch(onUpdate) {{ return client._fetchMetricData(buildPath(idx, idx + 1), onUpdate); }},
    fetchCsv() {{ return client.getText(buildPath(idx, idx + 1, 'csv')); }},
    then(resolve, reject) {{ return this.fetch().then(resolve, reject); }},
  }});

  /**
   * @param {{number}} start
   * @returns {{SkippedBuilder<T>}}
   */
  const skippedBuilder = (start) => ({{
    take(n) {{ return rangeBuilder(start, start + n); }},
    fetch(onUpdate) {{ return client._fetchMetricData(buildPath(start, undefined), onUpdate); }},
    fetchCsv() {{ return client.getText(buildPath(start, undefined, 'csv')); }},
    then(resolve, reject) {{ return this.fetch().then(resolve, reject); }},
  }});

  /** @type {{MetricEndpointBuilder<T>}} */
  const endpoint = {{
    get(idx) {{ return singleItemBuilder(idx); }},
    slice(start, end) {{ return rangeBuilder(start, end); }},
    first(n) {{ return rangeBuilder(undefined, n); }},
    last(n) {{ return n === 0 ? rangeBuilder(undefined, 0) : rangeBuilder(-n, undefined); }},
    skip(n) {{ return skippedBuilder(n); }},
    fetch(onUpdate) {{ return client._fetchMetricData(buildPath(), onUpdate); }},
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

  /**
   * Fetch metric data and wrap with helper methods (internal)
   * @template T
   * @param {{string}} path
   * @param {{(value: MetricData<T>) => void}} [onUpdate]
   * @returns {{Promise<MetricData<T>>}}
   */
  async _fetchMetricData(path, onUpdate) {{
    const wrappedOnUpdate = onUpdate ? (/** @type {{MetricData<T>}} */ raw) => onUpdate(_wrapMetricData(raw)) : undefined;
    const raw = await this.getJson(path, wrappedOnUpdate);
    return _wrapMetricData(raw);
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
    let constants = ClientConstants::collect();

    // VERSION, INDEXES, POOL_ID_TO_POOL_NAME
    writeln!(output, "  VERSION = \"{}\";\n", constants.version).unwrap();
    write_static_const(output, "INDEXES", &format_json(&constants.indexes));
    write_static_const(output, "POOL_ID_TO_POOL_NAME", &format_json(&constants.pool_map));

    // Cohort constants with camelCase keys
    for (name, value) in CohortConstants::all() {
        write_static_const(output, name, &format_json(&camel_case_keys(value)));
    }

    // Helper methods
    writeln!(
        output,
        r#"  /**
   * Convert an index value to a Date for date-based indexes.
   * @param {{Index}} index - The index type
   * @param {{number}} i - The index value
   * @returns {{globalThis.Date}}
   */
  indexToDate(index, i) {{
    return indexToDate(index, i);
  }}

  /**
   * Check if an index type is date-based.
   * @param {{Index}} index
   * @returns {{boolean}}
   */
  isDateIndex(index) {{
    return isDateIndex(index);
  }}
"#
    )
    .unwrap();
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

    writeln!(output, "// Index group constants and factory\n").unwrap();

    // Generate index array constants (e.g., _i1 = ["dateindex", "height"])
    for (i, pattern) in patterns.iter().enumerate() {
        write!(output, "const _i{} = /** @type {{const}} */ ([", i + 1).unwrap();
        for (j, index) in pattern.indexes.iter().enumerate() {
            if j > 0 {
                write!(output, ", ").unwrap();
            }
            write!(output, "\"{}\"", index.serialize_long()).unwrap();
        }
        writeln!(output, "]);").unwrap();
    }
    writeln!(output).unwrap();

    // Generate ONE generic metric pattern factory
    writeln!(
        output,
        r#"/**
 * Generic metric pattern factory.
 * @template T
 * @param {{BrkClientBase}} client
 * @param {{string}} name - The metric vec name
 * @param {{readonly Index[]}} indexes - The supported indexes
 */
function _mp(client, name, indexes) {{
  const by = /** @type {{any}} */ ({{}});
  for (const idx of indexes) {{
    Object.defineProperty(by, idx, {{
      get() {{ return _endpoint(client, name, idx); }},
      enumerable: true,
      configurable: true
    }});
  }}
  return {{
    name,
    by,
    indexes() {{ return indexes; }},
    /** @param {{Index}} index */
    get(index) {{ return indexes.includes(index) ? _endpoint(client, name, index) : undefined; }}
  }};
}}
"#
    )
    .unwrap();

    // Generate typedefs and thin wrapper functions
    for (i, pattern) in patterns.iter().enumerate() {
        // Generate typedef for type safety
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

        writeln!(
            output,
            "/** @template T @typedef {{{{ name: string, by: {}, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }}}} {} */",
            by_type, pattern.name
        )
        .unwrap();

        // Generate thin wrapper that calls the generic factory
        writeln!(
            output,
            "/** @template T @param {{BrkClientBase}} client @param {{string}} name @returns {{{}<T>}} */",
            pattern.name
        )
        .unwrap();
        writeln!(
            output,
            "function create{}(client, name) {{ return _mp(client, name, _i{}); }}",
            pattern.name,
            i + 1
        )
        .unwrap();
    }
    writeln!(output).unwrap();
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
