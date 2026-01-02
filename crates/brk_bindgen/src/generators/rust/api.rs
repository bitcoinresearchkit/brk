//! Rust API method generation.

use std::fmt::Write;

use crate::{Endpoint, VERSION, to_snake_case};

use super::types::js_type_to_rust;

/// Generate the main BrkClient struct.
pub fn generate_main_client(output: &mut String, endpoints: &[Endpoint]) {
    writeln!(
        output,
        r#"/// Main BRK client with catalog tree and API methods.
pub struct BrkClient {{
    base: Arc<BrkClientBase>,
    tree: CatalogTree,
}}

impl BrkClient {{
    /// Client version.
    pub const VERSION: &'static str = "v{VERSION}";

    /// Create a new client with the given base URL.
    pub fn new(base_url: impl Into<String>) -> Self {{
        let base = Arc::new(BrkClientBase::new(base_url));
        let tree = CatalogTree::new(base.clone(), String::new());
        Self {{ base, tree }}
    }}

    /// Create a new client with options.
    pub fn with_options(options: BrkClientOptions) -> Self {{
        let base = Arc::new(BrkClientBase::with_options(options));
        let tree = CatalogTree::new(base.clone(), String::new());
        Self {{ base, tree }}
    }}

    /// Get the catalog tree for navigating metrics.
    pub fn tree(&self) -> &CatalogTree {{
        &self.tree
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
        let return_type = endpoint
            .response_type
            .as_deref()
            .map(js_type_to_rust)
            .unwrap_or_else(|| "serde_json::Value".to_string());

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
            writeln!(output, "    /// {}", desc).unwrap();
        }

        let params = build_method_params(endpoint);
        writeln!(
            output,
            "    pub fn {}(&self{}) -> Result<{}> {{",
            method_name, params, return_type
        )
        .unwrap();

        let path = build_path_template(&endpoint.path);

        if endpoint.query_params.is_empty() {
            writeln!(output, "        self.base.get(&format!(\"{}\"))", path).unwrap();
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
            writeln!(
                output,
                "        self.base.get(&format!(\"{}{{}}\", query_str))",
                path
            )
            .unwrap();
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
        params.push(format!(", {}: &str", param.name));
    }
    for param in &endpoint.query_params {
        if param.required {
            params.push(format!(", {}: &str", param.name));
        } else {
            params.push(format!(", {}: Option<&str>", param.name));
        }
    }
    params.join("")
}

/// OpenAPI path placeholders `{param}` are already valid Rust format string syntax.
fn build_path_template(path: &str) -> &str {
    path
}
