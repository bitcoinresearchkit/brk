//! Rust API method generation.

use std::fmt::Write;

use crate::{Endpoint, VERSION, generators::write_description, to_snake_case};

use super::types::js_type_to_rust;

/// Generate the main BrkClient struct.
pub fn generate_main_client(output: &mut String, endpoints: &[Endpoint]) {
    writeln!(
        output,
        r#"/// Main BRK client with metrics tree and API methods.
pub struct BrkClient {{
    base: Arc<BrkClientBase>,
    metrics: MetricsTree,
}}

impl BrkClient {{
    /// Client version.
    pub const VERSION: &'static str = "v{VERSION}";

    /// Create a new client with the given base URL.
    pub fn new(base_url: impl Into<String>) -> Self {{
        let base = Arc::new(BrkClientBase::new(base_url));
        let metrics = MetricsTree::new(base.clone(), String::new());
        Self {{ base, metrics }}
    }}

    /// Create a new client with options.
    pub fn with_options(options: BrkClientOptions) -> Self {{
        let base = Arc::new(BrkClientBase::with_options(options));
        let metrics = MetricsTree::new(base.clone(), String::new());
        Self {{ base, metrics }}
    }}

    /// Get the metrics tree for navigating metrics.
    pub fn metrics(&self) -> &MetricsTree {{
        &self.metrics
    }}

    /// Create a dynamic metric endpoint builder for any metric/index combination.
    ///
    /// Use this for programmatic access when the metric name is determined at runtime.
    /// For type-safe access, use the `metrics()` tree instead.
    ///
    /// # Example
    /// ```ignore
    /// let data = client.metric("realized_price", Index::Height)
    ///     .last(10)
    ///     .json::<f64>()?;
    /// ```
    pub fn metric(&self, metric: impl Into<Metric>, index: Index) -> MetricEndpointBuilder<serde_json::Value> {{
        MetricEndpointBuilder::new(
            self.base.clone(),
            Arc::from(metric.into().as_str()),
            index,
        )
    }}
"#,
        VERSION = VERSION
    )
    .unwrap();

    generate_api_methods(output, endpoints);

    writeln!(output, "}}").unwrap();
}

/// Generate API methods from OpenAPI endpoints.
pub fn generate_api_methods(output: &mut String, endpoints: &[Endpoint]) {
    for endpoint in endpoints {
        if !endpoint.should_generate() {
            continue;
        }

        let method_name = endpoint_to_method_name(endpoint);
        let base_return_type = endpoint
            .response_type
            .as_deref()
            .map(js_type_to_rust)
            .unwrap_or_else(|| "serde_json::Value".to_string());

        let return_type = if endpoint.supports_csv {
            format!("FormatResponse<{}>", base_return_type)
        } else {
            base_return_type.clone()
        };

        writeln!(
            output,
            "    /// {}",
            endpoint.summary.as_deref().unwrap_or(&method_name)
        )
        .unwrap();
        if let Some(desc) = &endpoint.description
            && endpoint.summary.as_ref() != Some(desc)
        {
            writeln!(output, "    ///").unwrap();
            write_description(output, desc, "    /// ", "    ///");
        }
        // Add endpoint path
        writeln!(output, "    ///").unwrap();
        writeln!(output, "    /// Endpoint: `{} {}`", endpoint.method.to_uppercase(), endpoint.path).unwrap();

        let params = build_method_params(endpoint);
        writeln!(
            output,
            "    pub fn {}(&self{}) -> Result<{}> {{",
            method_name, params, return_type
        )
        .unwrap();

        let (path, index_arg) = build_path_template(endpoint);

        if endpoint.query_params.is_empty() {
            writeln!(output, "        self.base.get_json(&format!(\"{}\"{}))", path, index_arg).unwrap();
        } else {
            writeln!(output, "        let mut query = Vec::new();").unwrap();
            for param in &endpoint.query_params {
                if param.required {
                    writeln!(
                        output,
                        "        query.push(format!(\"{}={{}}\", {}));",
                        param.name, param.name
                    )
                    .unwrap();
                } else {
                    writeln!(
                        output,
                        "        if let Some(v) = {} {{ query.push(format!(\"{}={{}}\", v)); }}",
                        param.name, param.name
                    )
                    .unwrap();
                }
            }
            writeln!(output, "        let query_str = if query.is_empty() {{ String::new() }} else {{ format!(\"?{{}}\", query.join(\"&\")) }};").unwrap();
            writeln!(output, "        let path = format!(\"{}{{}}\"{}, query_str);", path, index_arg).unwrap();

            if endpoint.supports_csv {
                writeln!(output, "        if format == Some(Format::CSV) {{").unwrap();
                writeln!(output, "            self.base.get_text(&path).map(FormatResponse::Csv)").unwrap();
                writeln!(output, "        }} else {{").unwrap();
                writeln!(output, "            self.base.get_json(&path).map(FormatResponse::Json)").unwrap();
                writeln!(output, "        }}").unwrap();
            } else {
                writeln!(output, "        self.base.get_json(&path)").unwrap();
            }
        }

        writeln!(output, "    }}\n").unwrap();
    }
}

fn endpoint_to_method_name(endpoint: &Endpoint) -> String {
    to_snake_case(&endpoint.operation_name())
}

fn build_method_params(endpoint: &Endpoint) -> String {
    let mut params = Vec::new();
    for param in &endpoint.path_params {
        let rust_type = param_type_to_rust(&param.param_type);
        params.push(format!(", {}: {}", param.name, rust_type));
    }
    for param in &endpoint.query_params {
        let rust_type = param_type_to_rust(&param.param_type);
        if param.required {
            params.push(format!(", {}: {}", param.name, rust_type));
        } else {
            params.push(format!(", {}: Option<{}>", param.name, rust_type));
        }
    }
    params.join("")
}

/// Convert parameter type to Rust type for function signatures.
fn param_type_to_rust(param_type: &str) -> String {
    match param_type {
        "string" | "*" => "&str".to_string(),
        "integer" | "number" => "i64".to_string(),
        "boolean" => "bool".to_string(),
        other => other.to_string(), // Domain types like Index, Metric, Format
    }
}

/// Build path template and extra format args for Index params.
fn build_path_template(endpoint: &Endpoint) -> (String, &'static str) {
    let has_index_param = endpoint.path_params.iter().any(|p| p.name == "index" && p.param_type == "Index");
    if has_index_param {
        (endpoint.path.replace("{index}", "{}"), ", index.serialize_long()")
    } else {
        (endpoint.path.clone(), "")
    }
}
