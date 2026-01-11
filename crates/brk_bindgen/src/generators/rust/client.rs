//! Rust base client and pattern factory generation.

use std::fmt::Write;

use crate::{
    ClientMetadata, GenericSyntax, IndexSetPattern, PatternField, RustSyntax,
    StructuralPattern, generate_parameterized_field, generate_tree_path_field,
    index_to_field_name, to_snake_case,
};

/// Generate import statements.
pub fn generate_imports(output: &mut String) {
    writeln!(
        output,
        r#"use std::sync::Arc;
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

    /// Make a GET request.
    pub fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {{
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

        response
            .json()
            .map_err(|e| BrkError {{ message: e.to_string() }})
    }}
}}

/// Build metric name with optional prefix.
#[inline]
fn _m(acc: &str, s: &str) -> String {{
    if acc.is_empty() {{ s.to_string() }} else {{ format!("{{acc}}_{{s}}") }}
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
    /// Get an endpoint for a specific index, if supported.
    fn get(&self, index: Index) -> Option<Endpoint<T>>;
}}

"#
    )
    .unwrap();
}

/// Generate the Endpoint struct.
pub fn generate_endpoint(output: &mut String) {
    writeln!(
        output,
        r#"/// An endpoint for a specific metric + index combination.
pub struct Endpoint<T> {{
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    index: Index,
    _marker: std::marker::PhantomData<T>,
}}

impl<T: DeserializeOwned> Endpoint<T> {{
    pub fn new(client: Arc<BrkClientBase>, name: Arc<str>, index: Index) -> Self {{
        Self {{
            client,
            name,
            index,
            _marker: std::marker::PhantomData,
        }}
    }}

    /// Fetch all data points for this metric/index.
    pub fn get(&self) -> Result<MetricData<T>> {{
        self.client.get(&self.path())
    }}

    /// Fetch data points within a range.
    pub fn range(&self, from: Option<i64>, to: Option<i64>) -> Result<MetricData<T>> {{
        let mut params = Vec::new();
        if let Some(f) = from {{ params.push(format!("from={{}}", f)); }}
        if let Some(t) = to {{ params.push(format!("to={{}}", t)); }}
        let p = self.path();
        let path = if params.is_empty() {{
            p
        }} else {{
            format!("{{}}?{{}}", p, params.join("&"))
        }};
        self.client.get(&path)
    }}

    /// Get the endpoint path.
    pub fn path(&self) -> String {{
        format!("/api/metric/{{}}/{{}}", self.name, self.index.serialize_long())
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

    writeln!(output, "// Index accessor structs\n").unwrap();

    for pattern in patterns {
        let by_name = format!("{}By", pattern.name);

        // Generate the "By" struct with lazy endpoint methods
        writeln!(output, "/// Container for index endpoint methods.").unwrap();
        writeln!(output, "pub struct {}<T> {{", by_name).unwrap();
        writeln!(output, "    client: Arc<BrkClientBase>,").unwrap();
        writeln!(output, "    name: Arc<str>,").unwrap();
        writeln!(output, "    _marker: std::marker::PhantomData<T>,").unwrap();
        writeln!(output, "}}\n").unwrap();

        // Generate impl with methods for each index
        writeln!(output, "impl<T: DeserializeOwned> {}<T> {{", by_name).unwrap();
        for index in &pattern.indexes {
            let method_name = index_to_field_name(index);
            writeln!(output, "    pub fn {}(&self) -> Endpoint<T> {{", method_name).unwrap();
            writeln!(
                output,
                "        Endpoint::new(self.client.clone(), self.name.clone(), Index::{})",
                index
            )
            .unwrap();
            writeln!(output, "    }}").unwrap();
        }
        writeln!(output, "}}\n").unwrap();

        // Generate the main accessor struct
        writeln!(
            output,
            "/// Index accessor for metrics with {} indexes.",
            pattern.indexes.len()
        )
        .unwrap();
        writeln!(output, "pub struct {}<T> {{", pattern.name).unwrap();
        writeln!(output, "    client: Arc<BrkClientBase>,").unwrap();
        writeln!(output, "    name: Arc<str>,").unwrap();
        writeln!(output, "    pub by: {}<T>,", by_name).unwrap();
        writeln!(output, "}}\n").unwrap();

        // Generate impl block with constructor
        writeln!(output, "impl<T: DeserializeOwned> {}<T> {{", pattern.name).unwrap();
        writeln!(
            output,
            "    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {{"
        )
        .unwrap();
        writeln!(output, "        let name: Arc<str> = name.into();").unwrap();
        writeln!(output, "        Self {{").unwrap();
        writeln!(output, "            client: client.clone(),").unwrap();
        writeln!(output, "            name: name.clone(),").unwrap();
        writeln!(output, "            by: {} {{", by_name).unwrap();
        writeln!(output, "                client,").unwrap();
        writeln!(output, "                name,").unwrap();
        writeln!(output, "                _marker: std::marker::PhantomData,").unwrap();
        writeln!(output, "            }}").unwrap();
        writeln!(output, "        }}").unwrap();
        writeln!(output, "    }}").unwrap();
        writeln!(output).unwrap();
        writeln!(output, "    /// Get the metric name.").unwrap();
        writeln!(output, "    pub fn name(&self) -> &str {{").unwrap();
        writeln!(output, "        &self.name").unwrap();
        writeln!(output, "    }}").unwrap();
        writeln!(output, "}}\n").unwrap();

        // Implement AnyMetricPattern trait
        writeln!(output, "impl<T> AnyMetricPattern for {}<T> {{", pattern.name).unwrap();
        writeln!(output, "    fn name(&self) -> &str {{").unwrap();
        writeln!(output, "        &self.name").unwrap();
        writeln!(output, "    }}").unwrap();
        writeln!(output).unwrap();
        writeln!(output, "    fn indexes(&self) -> &'static [Index] {{").unwrap();
        writeln!(output, "        &[").unwrap();
        for index in &pattern.indexes {
            writeln!(output, "            Index::{},", index).unwrap();
        }
        writeln!(output, "        ]").unwrap();
        writeln!(output, "    }}").unwrap();
        writeln!(output, "}}\n").unwrap();

        // Implement MetricPattern<T> trait
        writeln!(output, "impl<T: DeserializeOwned> MetricPattern<T> for {}<T> {{", pattern.name).unwrap();
        writeln!(output, "    fn get(&self, index: Index) -> Option<Endpoint<T>> {{").unwrap();
        writeln!(output, "        match index {{").unwrap();
        for index in &pattern.indexes {
            let method_name = index_to_field_name(index);
            writeln!(output, "            Index::{} => Some(self.by.{}()),", index, method_name).unwrap();
        }
        writeln!(output, "            _ => None,").unwrap();
        writeln!(output, "        }}").unwrap();
        writeln!(output, "    }}").unwrap();
        writeln!(output, "}}\n").unwrap();
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
        let is_parameterizable = pattern.is_parameterizable();
        let generic_params = if pattern.is_generic { "<T>" } else { "" };

        writeln!(output, "/// Pattern struct for repeated tree structure.").unwrap();
        writeln!(output, "pub struct {}{} {{", pattern.name, generic_params).unwrap();

        for field in &pattern.fields {
            let field_name = to_snake_case(&field.name);
            let type_annotation =
                field_type_with_generic(field, metadata, pattern.is_generic, None);
            writeln!(output, "    pub {}: {},", field_name, type_annotation).unwrap();
        }

        writeln!(output, "}}\n").unwrap();

        // Generate impl block with constructor
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

        if is_parameterizable {
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
        } else {
            writeln!(
                output,
                "    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {{"
            )
            .unwrap();
        }
        writeln!(output, "        Self {{").unwrap();

        let syntax = RustSyntax;
        for field in &pattern.fields {
            if is_parameterizable {
                generate_parameterized_field(output, &syntax, field, pattern, metadata, "            ");
            } else {
                generate_tree_path_field(output, &syntax, field, metadata, "            ");
            }
        }

        writeln!(output, "        }}").unwrap();
        writeln!(output, "    }}").unwrap();
        writeln!(output, "}}\n").unwrap();
    }
}

/// Get Rust type annotation for a field with optional generic value type.
pub fn field_type_with_generic(
    field: &PatternField,
    metadata: &ClientMetadata,
    is_generic: bool,
    generic_value_type: Option<&str>,
) -> String {
    metadata.field_type_annotation(field, is_generic, generic_value_type, GenericSyntax::RUST)
}
