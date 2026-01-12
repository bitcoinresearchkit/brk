//! JavaScript API method generation.

use std::fmt::Write;

use crate::{Endpoint, Parameter, generators::{MANUAL_GENERIC_TYPES, write_description}, to_camel_case};

/// Generate API methods for the BrkClient class.
pub fn generate_api_methods(output: &mut String, endpoints: &[Endpoint]) {
    for endpoint in endpoints {
        if !endpoint.should_generate() {
            continue;
        }

        let method_name = endpoint_to_method_name(endpoint);
        let base_return_type =
            normalize_return_type(endpoint.response_type.as_deref().unwrap_or("*"));
        let return_type = if endpoint.supports_csv {
            format!("{} | string", base_return_type)
        } else {
            base_return_type
        };

        writeln!(output, "  /**").unwrap();
        if let Some(summary) = &endpoint.summary {
            writeln!(output, "   * {}", summary).unwrap();
        }
        if let Some(desc) = &endpoint.description
            && endpoint.summary.as_ref() != Some(desc)
        {
            writeln!(output, "   *").unwrap();
            write_description(output, desc, "   * ", "   *");
        }

        // Add endpoint path
        writeln!(output, "   *").unwrap();
        writeln!(output, "   * Endpoint: `{} {}`", endpoint.method.to_uppercase(), endpoint.path).unwrap();

        if !endpoint.path_params.is_empty() || !endpoint.query_params.is_empty() {
            writeln!(output, "   *").unwrap();
        }

        for param in &endpoint.path_params {
            let desc = format_param_desc(param.description.as_deref());
            writeln!(
                output,
                "   * @param {{{}}} {}{}",
                param.param_type, param.name, desc
            )
            .unwrap();
        }
        for param in &endpoint.query_params {
            let optional = if param.required { "" } else { "=" };
            let desc = format_param_desc(param.description.as_deref());
            writeln!(
                output,
                "   * @param {{{}{}}} [{}]{}",
                param.param_type, optional, param.name, desc
            )
            .unwrap();
        }

        writeln!(output, "   * @returns {{Promise<{}>}}", return_type).unwrap();
        writeln!(output, "   */").unwrap();

        let params = build_method_params(endpoint);
        writeln!(output, "  async {}({}) {{", method_name, params).unwrap();

        let path = build_path_template(&endpoint.path, &endpoint.path_params);

        if endpoint.query_params.is_empty() {
            writeln!(output, "    return this.getJson(`{}`);", path).unwrap();
        } else {
            writeln!(output, "    const params = new URLSearchParams();").unwrap();
            for param in &endpoint.query_params {
                if param.required {
                    writeln!(
                        output,
                        "    params.set('{}', String({}));",
                        param.name, param.name
                    )
                    .unwrap();
                } else {
                    writeln!(
                        output,
                        "    if ({} !== undefined) params.set('{}', String({}));",
                        param.name, param.name, param.name
                    )
                    .unwrap();
                }
            }
            writeln!(output, "    const query = params.toString();").unwrap();
            writeln!(output, "    const path = `{}${{query ? '?' + query : ''}}`;", path).unwrap();

            if endpoint.supports_csv {
                writeln!(output, "    if (format === 'csv') {{").unwrap();
                writeln!(output, "      return this.getText(path);").unwrap();
                writeln!(output, "    }}").unwrap();
                writeln!(output, "    return this.getJson(path);").unwrap();
            } else {
                writeln!(output, "    return this.getJson(path);").unwrap();
            }
        }

        writeln!(output, "  }}\n").unwrap();
    }
}

fn endpoint_to_method_name(endpoint: &Endpoint) -> String {
    to_camel_case(&endpoint.operation_name())
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

fn build_path_template(path: &str, path_params: &[Parameter]) -> String {
    let mut result = path.to_string();
    for param in path_params {
        let placeholder = format!("{{{}}}", param.name);
        let interpolation = format!("${{{}}}", param.name);
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

/// Format param description with dash prefix, or empty string if no description.
fn format_param_desc(desc: Option<&str>) -> String {
    match desc {
        Some(d) if !d.is_empty() => format!(" - {}", d),
        _ => String::new(),
    }
}
