//! Rust base client and pattern factory generation.

use std::fmt::Write;

use crate::{
    ClientMetadata, GenericSyntax, IndexSetPattern, RustSyntax, StructuralPattern,
    generate_parameterized_field, index_to_field_name, to_snake_case,
};

/// Generate import statements.
pub fn generate_imports(output: &mut String) {
    writeln!(
        output,
        r#"use std::sync::Arc;
use std::ops::{{Bound, RangeBounds}};
use serde::de::DeserializeOwned;
pub use brk_cohort::*;
pub use brk_types::*;

"#
    )
    .unwrap();
}

/// Generate the base BrkClientBase struct and error types.
pub fn generate_base_client(output: &mut String) {
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

    fn get(&self, path: &str) -> Result<minreq::Response> {{
        let base = self.base_url.trim_end_matches('/');
        let url = format!("{{}}{{}}", base, path);
        let response = minreq::get(&url)
            .with_timeout(self.timeout_secs)
            .send()
            .map_err(|e| BrkError {{ message: e.to_string() }})?;

        if response.status_code >= 400 {{
            return Err(BrkError {{
                message: format!("HTTP {{}}", response.status_code),
            }});
        }}

        Ok(response)
    }}

    /// Make a GET request and deserialize JSON response.
    pub fn get_json<T: DeserializeOwned>(&self, path: &str) -> Result<T> {{
        self.get(path)?
            .json()
            .map_err(|e| BrkError {{ message: e.to_string() }})
    }}

    /// Make a GET request and return raw text response.
    pub fn get_text(&self, path: &str) -> Result<String> {{
        self.get(path)?
            .as_str()
            .map(|s| s.to_string())
            .map_err(|e| BrkError {{ message: e.to_string() }})
    }}
}}

/// Build metric name with suffix.
#[inline]
fn _m(acc: &str, s: &str) -> String {{
    if s.is_empty() {{ acc.to_string() }}
    else if acc.is_empty() {{ s.to_string() }}
    else {{ format!("{{acc}}_{{s}}") }}
}}

/// Build metric name with prefix.
#[inline]
fn _p(prefix: &str, acc: &str) -> String {{
    if acc.is_empty() {{ prefix.to_string() }} else {{ format!("{{prefix}}_{{acc}}") }}
}}

"#
    )
    .unwrap();
}

/// Generate the MetricPattern trait.
pub fn generate_metric_pattern_trait(output: &mut String) {
    writeln!(
        output,
        r#"/// Non-generic trait for metric patterns (usable in collections).
pub trait AnyMetricPattern {{
    /// Get the metric name.
    fn name(&self) -> &str;

    /// Get the list of available indexes for this metric.
    fn indexes(&self) -> &'static [Index];
}}

/// Generic trait for metric patterns with endpoint access.
pub trait MetricPattern<T>: AnyMetricPattern {{
    /// Get an endpoint builder for a specific index, if supported.
    fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>>;
}}

"#
    )
    .unwrap();
}

/// Generate the MetricEndpointBuilder structs with typestate pattern.
pub fn generate_endpoint(output: &mut String) {
    writeln!(
        output,
        r#"/// Shared endpoint configuration.
#[derive(Clone)]
struct EndpointConfig {{
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    index: Index,
    start: Option<i64>,
    end: Option<i64>,
}}

impl EndpointConfig {{
    fn new(client: Arc<BrkClientBase>, name: Arc<str>, index: Index) -> Self {{
        Self {{ client, name, index, start: None, end: None }}
    }}

    fn path(&self) -> String {{
        format!("/api/metric/{{}}/{{}}", self.name, self.index.serialize_long())
    }}

    fn build_path(&self, format: Option<&str>) -> String {{
        let mut params = Vec::new();
        if let Some(s) = self.start {{ params.push(format!("start={{}}", s)); }}
        if let Some(e) = self.end {{ params.push(format!("end={{}}", e)); }}
        if let Some(fmt) = format {{ params.push(format!("format={{}}", fmt)); }}
        let p = self.path();
        if params.is_empty() {{ p }} else {{ format!("{{}}?{{}}", p, params.join("&")) }}
    }}

    fn get_json<T: DeserializeOwned>(&self, format: Option<&str>) -> Result<T> {{
        self.client.get_json(&self.build_path(format))
    }}

    fn get_text(&self, format: Option<&str>) -> Result<String> {{
        self.client.get_text(&self.build_path(format))
    }}
}}

/// Initial builder for metric endpoint queries.
///
/// Use method chaining to specify the data range, then call `fetch()` or `fetch_csv()` to execute.
///
/// # Examples
/// ```ignore
/// // Fetch all data
/// let data = endpoint.fetch()?;
///
/// // Get single item at index 5
/// let data = endpoint.get(5).fetch()?;
///
/// // Get first 10 using range
/// let data = endpoint.range(..10).fetch()?;
///
/// // Get range [100, 200)
/// let data = endpoint.range(100..200).fetch()?;
///
/// // Get first 10 (convenience)
/// let data = endpoint.take(10).fetch()?;
///
/// // Get last 10
/// let data = endpoint.last(10).fetch()?;
///
/// // Iterator-style chaining
/// let data = endpoint.skip(100).take(10).fetch()?;
/// ```
pub struct MetricEndpointBuilder<T> {{
    config: EndpointConfig,
    _marker: std::marker::PhantomData<T>,
}}

impl<T: DeserializeOwned> MetricEndpointBuilder<T> {{
    pub fn new(client: Arc<BrkClientBase>, name: Arc<str>, index: Index) -> Self {{
        Self {{ config: EndpointConfig::new(client, name, index), _marker: std::marker::PhantomData }}
    }}

    /// Select a specific index position.
    pub fn get(mut self, index: usize) -> SingleItemBuilder<T> {{
        self.config.start = Some(index as i64);
        self.config.end = Some(index as i64 + 1);
        SingleItemBuilder {{ config: self.config, _marker: std::marker::PhantomData }}
    }}

    /// Select a range using Rust range syntax.
    ///
    /// # Examples
    /// ```ignore
    /// endpoint.range(..10)      // first 10
    /// endpoint.range(100..110)  // indices 100-109
    /// endpoint.range(100..)     // from 100 to end
    /// ```
    pub fn range<R: RangeBounds<usize>>(mut self, range: R) -> RangeBuilder<T> {{
        self.config.start = match range.start_bound() {{
            Bound::Included(&n) => Some(n as i64),
            Bound::Excluded(&n) => Some(n as i64 + 1),
            Bound::Unbounded => None,
        }};
        self.config.end = match range.end_bound() {{
            Bound::Included(&n) => Some(n as i64 + 1),
            Bound::Excluded(&n) => Some(n as i64),
            Bound::Unbounded => None,
        }};
        RangeBuilder {{ config: self.config, _marker: std::marker::PhantomData }}
    }}

    /// Take the first n items.
    pub fn take(self, n: usize) -> RangeBuilder<T> {{
        self.range(..n)
    }}

    /// Take the last n items.
    pub fn last(mut self, n: usize) -> RangeBuilder<T> {{
        if n == 0 {{
            self.config.end = Some(0);
        }} else {{
            self.config.start = Some(-(n as i64));
        }}
        RangeBuilder {{ config: self.config, _marker: std::marker::PhantomData }}
    }}

    /// Skip the first n items. Chain with `take(n)` to get a range.
    pub fn skip(mut self, n: usize) -> SkippedBuilder<T> {{
        self.config.start = Some(n as i64);
        SkippedBuilder {{ config: self.config, _marker: std::marker::PhantomData }}
    }}

    /// Fetch all data as parsed JSON.
    pub fn fetch(self) -> Result<MetricData<T>> {{
        self.config.get_json(None)
    }}

    /// Fetch all data as CSV string.
    pub fn fetch_csv(self) -> Result<String> {{
        self.config.get_text(Some("csv"))
    }}

    /// Get the base endpoint path.
    pub fn path(&self) -> String {{
        self.config.path()
    }}
}}

/// Builder for single item access.
pub struct SingleItemBuilder<T> {{
    config: EndpointConfig,
    _marker: std::marker::PhantomData<T>,
}}

impl<T: DeserializeOwned> SingleItemBuilder<T> {{
    /// Fetch the single item.
    pub fn fetch(self) -> Result<MetricData<T>> {{
        self.config.get_json(None)
    }}

    /// Fetch the single item as CSV.
    pub fn fetch_csv(self) -> Result<String> {{
        self.config.get_text(Some("csv"))
    }}
}}

/// Builder after calling `skip(n)`. Chain with `take(n)` to specify count.
pub struct SkippedBuilder<T> {{
    config: EndpointConfig,
    _marker: std::marker::PhantomData<T>,
}}

impl<T: DeserializeOwned> SkippedBuilder<T> {{
    /// Take n items after the skipped position.
    pub fn take(mut self, n: usize) -> RangeBuilder<T> {{
        let start = self.config.start.unwrap_or(0);
        self.config.end = Some(start + n as i64);
        RangeBuilder {{ config: self.config, _marker: std::marker::PhantomData }}
    }}

    /// Fetch from the skipped position to the end.
    pub fn fetch(self) -> Result<MetricData<T>> {{
        self.config.get_json(None)
    }}

    /// Fetch from the skipped position to the end as CSV.
    pub fn fetch_csv(self) -> Result<String> {{
        self.config.get_text(Some("csv"))
    }}
}}

/// Builder with range fully specified.
pub struct RangeBuilder<T> {{
    config: EndpointConfig,
    _marker: std::marker::PhantomData<T>,
}}

impl<T: DeserializeOwned> RangeBuilder<T> {{
    /// Fetch the range as parsed JSON.
    pub fn fetch(self) -> Result<MetricData<T>> {{
        self.config.get_json(None)
    }}

    /// Fetch the range as CSV string.
    pub fn fetch_csv(self) -> Result<String> {{
        self.config.get_text(Some("csv"))
    }}
}}

"#
    )
    .unwrap();
}

/// Generate index accessor structs.
pub fn generate_index_accessors(output: &mut String, patterns: &[IndexSetPattern]) {
    if patterns.is_empty() {
        return;
    }

    // Generate static index arrays
    writeln!(output, "// Static index arrays").unwrap();
    for (i, pattern) in patterns.iter().enumerate() {
        write!(output, "const _I{}: &[Index] = &[", i + 1).unwrap();
        for (j, index) in pattern.indexes.iter().enumerate() {
            if j > 0 {
                write!(output, ", ").unwrap();
            }
            write!(output, "Index::{}", index).unwrap();
        }
        writeln!(output, "];").unwrap();
    }
    writeln!(output).unwrap();

    // Generate helper function
    writeln!(
        output,
        r#"#[inline]
fn _ep<T: DeserializeOwned>(c: &Arc<BrkClientBase>, n: &Arc<str>, i: Index) -> MetricEndpointBuilder<T> {{
    MetricEndpointBuilder::new(c.clone(), n.clone(), i)
}}
"#
    )
    .unwrap();

    // Generate index accessor structs
    writeln!(output, "// Index accessor structs\n").unwrap();

    for (i, pattern) in patterns.iter().enumerate() {
        let by_name = format!("{}By", pattern.name);
        let idx_const = format!("_I{}", i + 1);

        // Generate the "By" struct
        writeln!(output, "pub struct {}<T> {{ client: Arc<BrkClientBase>, name: Arc<str>, _marker: std::marker::PhantomData<T> }}", by_name).unwrap();
        writeln!(output, "impl<T: DeserializeOwned> {}<T> {{", by_name).unwrap();
        for index in &pattern.indexes {
            let method_name = index_to_field_name(index);
            writeln!(
                output,
                "    pub fn {}(&self) -> MetricEndpointBuilder<T> {{ _ep(&self.client, &self.name, Index::{}) }}",
                method_name, index
            )
            .unwrap();
        }
        writeln!(output, "}}\n").unwrap();

        // Generate the main accessor struct
        writeln!(
            output,
            "pub struct {}<T> {{ name: Arc<str>, pub by: {}<T> }}",
            pattern.name, by_name
        )
        .unwrap();
        writeln!(output, "impl<T: DeserializeOwned> {}<T> {{", pattern.name).unwrap();
        writeln!(
            output,
            "    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {{ let name: Arc<str> = name.into(); Self {{ name: name.clone(), by: {} {{ client, name, _marker: std::marker::PhantomData }} }} }}",
            by_name
        )
        .unwrap();
        writeln!(output, "    pub fn name(&self) -> &str {{ &self.name }}").unwrap();
        writeln!(output, "}}\n").unwrap();

        // Implement AnyMetricPattern trait
        writeln!(
            output,
            "impl<T> AnyMetricPattern for {}<T> {{ fn name(&self) -> &str {{ &self.name }} fn indexes(&self) -> &'static [Index] {{ {} }} }}",
            pattern.name, idx_const
        )
        .unwrap();

        // Implement MetricPattern<T> trait
        writeln!(
            output,
            "impl<T: DeserializeOwned> MetricPattern<T> for {}<T> {{ fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> {{ {}.contains(&index).then(|| _ep(&self.by.client, &self.by.name, index)) }} }}\n",
            pattern.name, idx_const
        )
        .unwrap();
    }
}

/// Generate structural pattern structs.
pub fn generate_pattern_structs(
    output: &mut String,
    patterns: &[StructuralPattern],
    metadata: &ClientMetadata,
) {
    if patterns.is_empty() {
        return;
    }

    writeln!(output, "// Reusable pattern structs\n").unwrap();

    for pattern in patterns {
        let generic_params = if pattern.is_generic { "<T>" } else { "" };

        // Generate struct definition
        writeln!(output, "/// Pattern struct for repeated tree structure.").unwrap();
        writeln!(output, "pub struct {}{} {{", pattern.name, generic_params).unwrap();

        for field in &pattern.fields {
            let field_name = to_snake_case(&field.name);
            let type_annotation = metadata.field_type_annotation(
                field,
                pattern.is_generic,
                None,
                GenericSyntax::RUST,
            );
            writeln!(output, "    pub {}: {},", field_name, type_annotation).unwrap();
        }

        writeln!(output, "}}\n").unwrap();

        // Generate impl block with constructor for ALL patterns
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

        writeln!(
            output,
            "    /// Create a new pattern node with accumulated metric name."
        )
        .unwrap();
        writeln!(
            output,
            "    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {{"
        )
        .unwrap();
        writeln!(output, "        Self {{").unwrap();

        let syntax = RustSyntax;
        for field in &pattern.fields {
            generate_parameterized_field(output, &syntax, field, pattern, metadata, "            ");
        }

        writeln!(output, "        }}").unwrap();
        writeln!(output, "    }}").unwrap();
        writeln!(output, "}}\n").unwrap();
    }
}
