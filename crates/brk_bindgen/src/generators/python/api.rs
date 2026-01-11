//! Python API method generation.

use std::fmt::Write;

use crate::{Endpoint, Parameter, escape_python_keyword, generators::MANUAL_GENERIC_TYPES, to_snake_case};

use super::client::generate_class_constants;
use super::types::js_type_to_python;

/// Generate the main client class
pub fn generate_main_client(output: &mut String, endpoints: &[Endpoint]) {
    writeln!(output, "class BrkClient(BrkClientBase):").unwrap();
    writeln!(
        output,
        "    \"\"\"Main BRK client with metrics tree and API methods.\"\"\""
    )
    .unwrap();
    writeln!(output).unwrap();

    // Generate class-level constants
    generate_class_constants(output);

    writeln!(
        output,
        "    def __init__(self, base_url: str = 'http://localhost:3000', timeout: float = 30.0):"
    )
    .unwrap();
    writeln!(output, "        super().__init__(base_url, timeout)").unwrap();
    writeln!(output, "        self.metrics = MetricsTree(self)").unwrap();
    writeln!(output).unwrap();

    // Generate API methods
    generate_api_methods(output, endpoints);
}

/// Generate API methods from OpenAPI endpoints
pub fn generate_api_methods(output: &mut String, endpoints: &[Endpoint]) {
    for endpoint in endpoints {
        if !endpoint.should_generate() {
            continue;
        }

        let method_name = endpoint_to_method_name(endpoint);
        let base_return_type = normalize_return_type(
            &endpoint
                .response_type
                .as_deref()
                .map(js_type_to_python)
                .unwrap_or_else(|| "Any".to_string()),
        );

        let return_type = if endpoint.supports_csv {
            format!("Union[{}, str]", base_return_type)
        } else {
            base_return_type
        };

        // Build method signature
        let params = build_method_params(endpoint);
        writeln!(
            output,
            "    def {}(self{}) -> {}:",
            method_name, params, return_type
        )
        .unwrap();

        // Docstring
        match (&endpoint.summary, &endpoint.description) {
            (Some(summary), Some(desc)) if summary != desc => {
                writeln!(output, "        \"\"\"{}.", summary.trim_end_matches('.')).unwrap();
                writeln!(output).unwrap();
                writeln!(output, "        {}\"\"\"", desc).unwrap();
            }
            (Some(summary), _) => {
                writeln!(output, "        \"\"\"{}\"\"\"", summary).unwrap();
            }
            (None, Some(desc)) => {
                writeln!(output, "        \"\"\"{}\"\"\"", desc).unwrap();
            }
            (None, None) => {}
        }

        // Build path
        let path = build_path_template(&endpoint.path, &endpoint.path_params);

        if endpoint.query_params.is_empty() {
            if endpoint.path_params.is_empty() {
                writeln!(output, "        return self.get_json('{}')", path).unwrap();
            } else {
                writeln!(output, "        return self.get_json(f'{}')", path).unwrap();
            }
        } else {
            writeln!(output, "        params = []").unwrap();
            for param in &endpoint.query_params {
                // Use safe name for Python variable, original name for API query parameter
                let safe_name = escape_python_keyword(&param.name);
                if param.required {
                    writeln!(
                        output,
                        "        params.append(f'{}={{{}}}')",
                        param.name, safe_name
                    )
                    .unwrap();
                } else {
                    writeln!(
                        output,
                        "        if {} is not None: params.append(f'{}={{{}}}')",
                        safe_name, param.name, safe_name
                    )
                    .unwrap();
                }
            }
            writeln!(output, "        query = '&'.join(params)").unwrap();
            writeln!(
                output,
                "        path = f'{}{{\"?\" + query if query else \"\"}}'",
                path
            )
            .unwrap();

            if endpoint.supports_csv {
                writeln!(output, "        if format == 'csv':").unwrap();
                writeln!(output, "            return self.get_text(path)").unwrap();
                writeln!(output, "        return self.get_json(path)").unwrap();
            } else {
                writeln!(output, "        return self.get_json(path)").unwrap();
            }
        }

        writeln!(output).unwrap();
    }
}

fn endpoint_to_method_name(endpoint: &Endpoint) -> String {
    to_snake_case(&endpoint.operation_name())
}

fn build_method_params(endpoint: &Endpoint) -> String {
    let mut params = Vec::new();
    // Path params are always required
    for param in &endpoint.path_params {
        let safe_name = escape_python_keyword(&param.name);
        let py_type = js_type_to_python(&param.param_type);
        params.push(format!(", {}: {}", safe_name, py_type));
    }
    // Required query params must come before optional ones (Python syntax requirement)
    for param in &endpoint.query_params {
        if param.required {
            let safe_name = escape_python_keyword(&param.name);
            let py_type = js_type_to_python(&param.param_type);
            params.push(format!(", {}: {}", safe_name, py_type));
        }
    }
    for param in &endpoint.query_params {
        if !param.required {
            let safe_name = escape_python_keyword(&param.name);
            let py_type = js_type_to_python(&param.param_type);
            params.push(format!(", {}: Optional[{}] = None", safe_name, py_type));
        }
    }
    params.join("")
}

fn build_path_template(path: &str, path_params: &[Parameter]) -> String {
    let mut result = path.to_string();
    for param in path_params {
        let placeholder = format!("{{{}}}", param.name);
        // Use escaped name for Python variable interpolation in f-string
        let safe_name = escape_python_keyword(&param.name);
        let interpolation = format!("{{{}}}", safe_name);
        result = result.replace(&placeholder, &interpolation);
    }
    result
}

/// Replace generic types with their Any variants in return types.
fn normalize_return_type(return_type: &str) -> String {
    let mut result = return_type.to_string();
    for type_name in MANUAL_GENERIC_TYPES {
        result = result.replace(type_name, &format!("Any{}", type_name));
    }
    result
}
