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
const _GENESIS = new Date(2009, 0, 3);  // day1 0, week1 0
const _DAY_ONE = new Date(2009, 0, 9);  // day1 1 (6 day gap after genesis)
const _MS_PER_DAY = 86400000;
const _MS_PER_WEEK = 7 * _MS_PER_DAY;
const _EPOCH_MS = 1230768000000;
const _DATE_INDEXES = new Set([
  'minute1', 'minute5', 'minute10', 'minute30',
  'hour1', 'hour4', 'hour12',
  'day1', 'day3', 'week1',
  'month1', 'month3', 'month6',
  'year1', 'year10',
]);

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
    case 'minute1': return new Date(_EPOCH_MS + i * 60000);
    case 'minute5': return new Date(_EPOCH_MS + i * 300000);
    case 'minute10': return new Date(_EPOCH_MS + i * 600000);
    case 'minute30': return new Date(_EPOCH_MS + i * 1800000);
    case 'hour1': return new Date(_EPOCH_MS + i * 3600000);
    case 'hour4': return new Date(_EPOCH_MS + i * 14400000);
    case 'hour12': return new Date(_EPOCH_MS + i * 43200000);
    case 'day1': return i === 0 ? _GENESIS : new Date(_DAY_ONE.getTime() + (i - 1) * _MS_PER_DAY);
    case 'day3': return new Date(_EPOCH_MS + i * 259200000);
    case 'week1': return new Date(_GENESIS.getTime() + i * _MS_PER_WEEK);
    case 'month1': return _addMonths(i);
    case 'month3': return _addMonths(i * 3);
    case 'month6': return _addMonths(i * 6);
    case 'year1': return new Date(2009 + i, 0, 1);
    case 'year10': return new Date(2009 + i * 10, 0, 1);
    default: throw new Error(`${{index}} is not a date-based index`);
  }}
}}

/**
 * Convert a Date to an index value for date-based indexes.
 * Returns the floor index (latest index whose date is <= the given date).
 * @param {{Index}} index - The index type
 * @param {{globalThis.Date}} d - The date to convert
 * @returns {{number}}
 */
function dateToIndex(index, d) {{
  const ms = d.getTime();
  switch (index) {{
    case 'minute1': return Math.floor((ms - _EPOCH_MS) / 60000);
    case 'minute5': return Math.floor((ms - _EPOCH_MS) / 300000);
    case 'minute10': return Math.floor((ms - _EPOCH_MS) / 600000);
    case 'minute30': return Math.floor((ms - _EPOCH_MS) / 1800000);
    case 'hour1': return Math.floor((ms - _EPOCH_MS) / 3600000);
    case 'hour4': return Math.floor((ms - _EPOCH_MS) / 14400000);
    case 'hour12': return Math.floor((ms - _EPOCH_MS) / 43200000);
    case 'day1': {{
      if (ms < _DAY_ONE.getTime()) return 0;
      return 1 + Math.floor((ms - _DAY_ONE.getTime()) / _MS_PER_DAY);
    }}
    case 'day3': return Math.floor((ms - _EPOCH_MS) / 259200000);
    case 'week1': return Math.floor((ms - _GENESIS.getTime()) / _MS_PER_WEEK);
    case 'month1': return (d.getFullYear() - 2009) * 12 + d.getMonth();
    case 'month3': return (d.getFullYear() - 2009) * 4 + Math.floor(d.getMonth() / 3);
    case 'month6': return (d.getFullYear() - 2009) * 2 + Math.floor(d.getMonth() / 6);
    case 'year1': return d.getFullYear() - 2009;
    case 'year10': return Math.floor((d.getFullYear() - 2009) / 10);
    default: throw new Error(`${{index}} is not a date-based index`);
  }}
}}

/**
 * Wrap raw metric data with helper methods.
 * @template T
 * @param {{MetricData<T>}} raw - Raw JSON response
 * @returns {{DateMetricData<T>}}
 */
function _wrapMetricData(raw) {{
  const {{ index, start, end, data }} = raw;
  const _dateBased = _DATE_INDEXES.has(index);
  return /** @type {{DateMetricData<T>}} */ ({{
    ...raw,
    isDateBased: _dateBased,
    indexes() {{
      /** @type {{number[]}} */
      const result = [];
      for (let i = start; i < end; i++) result.push(i);
      return result;
    }},
    keys() {{
      return this.indexes();
    }},
    entries() {{
      /** @type {{Array<[number, T]>}} */
      const result = [];
      for (let i = 0; i < data.length; i++) result.push([start + i, data[i]]);
      return result;
    }},
    toMap() {{
      /** @type {{Map<number, T>}} */
      const map = new Map();
      for (let i = 0; i < data.length; i++) map.set(start + i, data[i]);
      return map;
    }},
    *[Symbol.iterator]() {{
      for (let i = 0; i < data.length; i++) yield /** @type {{[number, T]}} */ ([start + i, data[i]]);
    }},
    // DateMetricData methods (only meaningful for date-based indexes)
    dates() {{
      /** @type {{globalThis.Date[]}} */
      const result = [];
      for (let i = start; i < end; i++) result.push(indexToDate(index, i));
      return result;
    }},
    dateEntries() {{
      /** @type {{Array<[globalThis.Date, T]>}} */
      const result = [];
      for (let i = 0; i < data.length; i++) result.push([indexToDate(index, start + i), data[i]]);
      return result;
    }},
    toDateMap() {{
      /** @type {{Map<globalThis.Date, T>}} */
      const map = new Map();
      for (let i = 0; i < data.length; i++) map.set(indexToDate(index, start + i), data[i]);
      return map;
    }},
  }});
}}

/**
 * @template T
 * @typedef {{Object}} MetricDataBase
 * @property {{number}} version - Version of the metric data
 * @property {{Index}} index - The index type used for this query
 * @property {{number}} total - Total number of data points
 * @property {{number}} start - Start index (inclusive)
 * @property {{number}} end - End index (exclusive)
 * @property {{string}} stamp - ISO 8601 timestamp of when the response was generated
 * @property {{T[]}} data - The metric data
 * @property {{boolean}} isDateBased - Whether this metric uses a date-based index
 * @property {{() => number[]}} indexes - Get index numbers
 * @property {{() => number[]}} keys - Get keys as index numbers (alias for indexes)
 * @property {{() => Array<[number, T]>}} entries - Get [index, value] pairs
 * @property {{() => Map<number, T>}} toMap - Convert to Map<index, value>
 */

/** @template T @typedef {{MetricDataBase<T> & Iterable<[number, T]>}} MetricData */

/**
 * @template T
 * @typedef {{Object}} DateMetricDataExtras
 * @property {{() => globalThis.Date[]}} dates - Get dates for each data point
 * @property {{() => Array<[globalThis.Date, T]>}} dateEntries - Get [date, value] pairs
 * @property {{() => Map<globalThis.Date, T>}} toDateMap - Convert to Map<date, value>
 */

/** @template T @typedef {{MetricData<T> & DateMetricDataExtras<T>}} DateMetricData */
/** @typedef {{MetricData<any>}} AnyMetricData */

/** @template T @typedef {{(onfulfilled?: (value: MetricData<T>) => any, onrejected?: (reason: Error) => never) => Promise<MetricData<T>>}} Thenable */
/** @template T @typedef {{(onfulfilled?: (value: DateMetricData<T>) => any, onrejected?: (reason: Error) => never) => Promise<DateMetricData<T>>}} DateThenable */

/**
 * @template T
 * @typedef {{Object}} MetricEndpointBuilder
 * @property {{(index: number) => SingleItemBuilder<T>}} get - Get single item at index
 * @property {{(start?: number, end?: number) => RangeBuilder<T>}} slice - Slice by index
 * @property {{(n: number) => RangeBuilder<T>}} first - Get first n items
 * @property {{(n: number) => RangeBuilder<T>}} last - Get last n items
 * @property {{(n: number) => SkippedBuilder<T>}} skip - Skip first n items, chain with take()
 * @property {{(onUpdate?: (value: MetricData<T>) => void) => Promise<MetricData<T>>}} fetch - Fetch all data
 * @property {{() => Promise<string>}} fetchCsv - Fetch all data as CSV
 * @property {{Thenable<T>}} then - Thenable (await endpoint)
 * @property {{string}} path - The endpoint path
 */

/**
 * @template T
 * @typedef {{Object}} DateMetricEndpointBuilder
 * @property {{(index: number | globalThis.Date) => DateSingleItemBuilder<T>}} get - Get single item at index or Date
 * @property {{(start?: number | globalThis.Date, end?: number | globalThis.Date) => DateRangeBuilder<T>}} slice - Slice by index or Date
 * @property {{(n: number) => DateRangeBuilder<T>}} first - Get first n items
 * @property {{(n: number) => DateRangeBuilder<T>}} last - Get last n items
 * @property {{(n: number) => DateSkippedBuilder<T>}} skip - Skip first n items, chain with take()
 * @property {{(onUpdate?: (value: DateMetricData<T>) => void) => Promise<DateMetricData<T>>}} fetch - Fetch all data
 * @property {{() => Promise<string>}} fetchCsv - Fetch all data as CSV
 * @property {{DateThenable<T>}} then - Thenable (await endpoint)
 * @property {{string}} path - The endpoint path
 */

/** @typedef {{MetricEndpointBuilder<any>}} AnyMetricEndpointBuilder */

/** @template T @typedef {{Object}} SingleItemBuilder
 * @property {{(onUpdate?: (value: MetricData<T>) => void) => Promise<MetricData<T>>}} fetch - Fetch the item
 * @property {{() => Promise<string>}} fetchCsv - Fetch as CSV
 * @property {{Thenable<T>}} then - Thenable
 */

/** @template T @typedef {{Object}} DateSingleItemBuilder
 * @property {{(onUpdate?: (value: DateMetricData<T>) => void) => Promise<DateMetricData<T>>}} fetch - Fetch the item
 * @property {{() => Promise<string>}} fetchCsv - Fetch as CSV
 * @property {{DateThenable<T>}} then - Thenable
 */

/** @template T @typedef {{Object}} SkippedBuilder
 * @property {{(n: number) => RangeBuilder<T>}} take - Take n items after skipped position
 * @property {{(onUpdate?: (value: MetricData<T>) => void) => Promise<MetricData<T>>}} fetch - Fetch from skipped position to end
 * @property {{() => Promise<string>}} fetchCsv - Fetch as CSV
 * @property {{Thenable<T>}} then - Thenable
 */

/** @template T @typedef {{Object}} DateSkippedBuilder
 * @property {{(n: number) => DateRangeBuilder<T>}} take - Take n items after skipped position
 * @property {{(onUpdate?: (value: DateMetricData<T>) => void) => Promise<DateMetricData<T>>}} fetch - Fetch from skipped position to end
 * @property {{() => Promise<string>}} fetchCsv - Fetch as CSV
 * @property {{DateThenable<T>}} then - Thenable
 */

/** @template T @typedef {{Object}} RangeBuilder
 * @property {{(onUpdate?: (value: MetricData<T>) => void) => Promise<MetricData<T>>}} fetch - Fetch the range
 * @property {{() => Promise<string>}} fetchCsv - Fetch as CSV
 * @property {{Thenable<T>}} then - Thenable
 */

/** @template T @typedef {{Object}} DateRangeBuilder
 * @property {{(onUpdate?: (value: DateMetricData<T>) => void) => Promise<DateMetricData<T>>}} fetch - Fetch the range
 * @property {{() => Promise<string>}} fetchCsv - Fetch as CSV
 * @property {{DateThenable<T>}} then - Thenable
 */

/**
 * @template T
 * @typedef {{Object}} MetricPattern
 * @property {{string}} name - The metric name
 * @property {{Readonly<Partial<Record<Index, MetricEndpointBuilder<T>>>>}} by - Index endpoints as lazy getters
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
 * @returns {{DateMetricEndpointBuilder<T>}}
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
   * @returns {{DateRangeBuilder<T>}}
   */
  const rangeBuilder = (start, end) => ({{
    fetch(onUpdate) {{ return client._fetchMetricData(buildPath(start, end), onUpdate); }},
    fetchCsv() {{ return client.getText(buildPath(start, end, 'csv')); }},
    then(resolve, reject) {{ return this.fetch().then(resolve, reject); }},
  }});

  /**
   * @param {{number}} idx
   * @returns {{DateSingleItemBuilder<T>}}
   */
  const singleItemBuilder = (idx) => ({{
    fetch(onUpdate) {{ return client._fetchMetricData(buildPath(idx, idx + 1), onUpdate); }},
    fetchCsv() {{ return client.getText(buildPath(idx, idx + 1, 'csv')); }},
    then(resolve, reject) {{ return this.fetch().then(resolve, reject); }},
  }});

  /**
   * @param {{number}} start
   * @returns {{DateSkippedBuilder<T>}}
   */
  const skippedBuilder = (start) => ({{
    take(n) {{ return rangeBuilder(start, start + n); }},
    fetch(onUpdate) {{ return client._fetchMetricData(buildPath(start, undefined), onUpdate); }},
    fetchCsv() {{ return client.getText(buildPath(start, undefined, 'csv')); }},
    then(resolve, reject) {{ return this.fetch().then(resolve, reject); }},
  }});

  /** @type {{DateMetricEndpointBuilder<T>}} */
  const endpoint = {{
    get(idx) {{ if (idx instanceof Date) idx = dateToIndex(index, idx); return singleItemBuilder(idx); }},
    slice(start, end) {{
      if (start instanceof Date) start = dateToIndex(index, start);
      if (end instanceof Date) end = dateToIndex(index, end);
      return rangeBuilder(start, end);
    }},
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
    const rawUrl = isString ? options : options.baseUrl;
    this.baseUrl = rawUrl.endsWith('/') ? rawUrl.slice(0, -1) : rawUrl;
    this.timeout = isString ? 5000 : (options.timeout ?? 5000);
    /** @type {{Promise<Cache | null>}} */
    this._cachePromise = _openCache(isString ? undefined : options.cache);
    /** @type {{Cache | null}} */
    this._cache = null;
    this._cachePromise.then(c => this._cache = c);
  }}

  /**
   * @param {{string}} path
   * @returns {{Promise<Response>}}
   */
  async get(path) {{
    const url = `${{this.baseUrl}}${{path}}`;
    const res = await fetch(url, {{ signal: AbortSignal.timeout(this.timeout) }});
    if (!res.ok) throw new BrkError(`HTTP ${{res.status}}: ${{url}}`, res.status);
    return res;
  }}

  /**
   * Make a GET request - races cache vs network, first to resolve calls onUpdate
   * @template T
   * @param {{string}} path
   * @param {{(value: T) => void}} [onUpdate] - Called when data is available (may be called twice: cache then network)
   * @returns {{Promise<T>}}
   */
  async getJson(path, onUpdate) {{
    const url = `${{this.baseUrl}}${{path}}`;
    const cache = this._cache ?? await this._cachePromise;

    let resolved = false;
    /** @type {{Response | null}} */
    let cachedRes = null;

    // Race cache vs network - first to resolve calls onUpdate
    const cachePromise = cache?.match(url).then(async (res) => {{
      cachedRes = res ?? null;
      if (!res) return null;
      const json = await res.json();
      if (!resolved && onUpdate) {{
        resolved = true;
        onUpdate(json);
      }}
      return json;
    }});

    const networkPromise = this.get(path).then(async (res) => {{
      const cloned = res.clone();
      const json = await res.json();
      // Skip update if ETag matches and cache already delivered
      if (cachedRes?.headers.get('ETag') === res.headers.get('ETag')) {{
        if (!resolved && onUpdate) {{
          resolved = true;
          onUpdate(json);
        }}
        return json;
      }}
      resolved = true;
      if (onUpdate) {{
        onUpdate(json);
      }}
      if (cache) _runIdle(() => cache.put(url, cloned));
      return json;
    }});

    try {{
      return await networkPromise;
    }} catch (e) {{
      // Network failed - wait for cache
      const cachedJson = await cachePromise?.catch(() => null);
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
   * @param {{(value: DateMetricData<T>) => void}} [onUpdate]
   * @returns {{Promise<DateMetricData<T>>}}
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
   * Convert a Date to an index value for date-based indexes.
   * @param {{Index}} index - The index type
   * @param {{globalThis.Date}} d - The date to convert
   * @returns {{number}}
   */
  dateToIndex(index, d) {{
    return dateToIndex(index, d);
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

    // Generate index array constants (e.g., _i1 = ["day1", "height"])
    for (i, pattern) in patterns.iter().enumerate() {
        write!(output, "const _i{} = /** @type {{const}} */ ([", i + 1).unwrap();
        for (j, index) in pattern.indexes.iter().enumerate() {
            if j > 0 {
                write!(output, ", ").unwrap();
            }
            write!(output, "\"{}\"", index.name()).unwrap();
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
  const by = {{}};
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
    /** @returns {{readonly Index[]}} */
    indexes() {{ return indexes; }},
    /** @param {{Index}} index @returns {{MetricEndpointBuilder<T>|undefined}} */
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
                let builder = if idx.is_date_based() {
                    "DateMetricEndpointBuilder"
                } else {
                    "MetricEndpointBuilder"
                };
                format!("readonly {}: {}<T>", idx.name(), builder)
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
            "function create{}(client, name) {{ return /** @type {{{}<T>}} */ (_mp(client, name, _i{})); }}",
            pattern.name,
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
