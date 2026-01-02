//! JavaScript type definitions generation.

use std::fmt::Write;

use serde_json::Value;

use crate::{TypeSchemas, ref_to_type_name, to_camel_case};

/// Generate JSDoc type definitions from OpenAPI schemas.
pub fn generate_type_definitions(output: &mut String, schemas: &TypeSchemas) {
    if schemas.is_empty() {
        return;
    }

    writeln!(output, "// Type definitions\n").unwrap();

    for (name, schema) in schemas {
        let js_type = schema_to_js_type(schema, Some(name));

        if is_primitive_alias(schema) {
            writeln!(output, "/** @typedef {{{}}} {} */", js_type, name).unwrap();
        } else if let Some(props) = schema.get("properties").and_then(|p| p.as_object()) {
            writeln!(output, "/**").unwrap();
            writeln!(output, " * @typedef {{Object}} {}", name).unwrap();
            for (prop_name, prop_schema) in props {
                let prop_type = schema_to_js_type(prop_schema, Some(name));
                let required = schema
                    .get("required")
                    .and_then(|r| r.as_array())
                    .map(|arr| arr.iter().any(|v| v.as_str() == Some(prop_name)))
                    .unwrap_or(false);
                let optional = if required { "" } else { "=" };
                let safe_name = to_camel_case(prop_name);
                writeln!(
                    output,
                    " * @property {{{}{}}} {}",
                    prop_type, optional, safe_name
                )
                .unwrap();
            }
            writeln!(output, " */").unwrap();
        } else {
            writeln!(output, "/** @typedef {{{}}} {} */", js_type, name).unwrap();
        }
    }
    writeln!(output).unwrap();
}

fn is_primitive_alias(schema: &Value) -> bool {
    schema.get("properties").is_none()
        && schema.get("items").is_none()
        && schema.get("anyOf").is_none()
        && schema.get("oneOf").is_none()
        && schema.get("enum").is_none()
}

fn json_type_to_js(ty: &str, schema: &Value, current_type: Option<&str>) -> String {
    match ty {
        "integer" | "number" => "number".to_string(),
        "boolean" => "boolean".to_string(),
        "string" => "string".to_string(),
        "null" => "null".to_string(),
        "array" => {
            let item_type = schema
                .get("items")
                .map(|s| schema_to_js_type(s, current_type))
                .unwrap_or_else(|| "*".to_string());
            format!("{}[]", item_type)
        }
        "object" => {
            if let Some(add_props) = schema.get("additionalProperties") {
                let value_type = schema_to_js_type(add_props, current_type);
                return format!("{{ [key: string]: {} }}", value_type);
            }
            "Object".to_string()
        }
        _ => "*".to_string(),
    }
}

/// Convert a JSON schema to a JavaScript type string.
pub fn schema_to_js_type(schema: &Value, current_type: Option<&str>) -> String {
    if let Some(all_of) = schema.get("allOf").and_then(|v| v.as_array()) {
        for item in all_of {
            let resolved = schema_to_js_type(item, current_type);
            if resolved != "*" {
                return resolved;
            }
        }
    }

    if let Some(ref_path) = schema.get("$ref").and_then(|r| r.as_str()) {
        return ref_to_type_name(ref_path).unwrap_or("*").to_string();
    }

    if let Some(enum_values) = schema.get("enum").and_then(|e| e.as_array()) {
        let literals: Vec<String> = enum_values
            .iter()
            .filter_map(|v| v.as_str())
            .map(|s| format!("\"{}\"", s))
            .collect();
        if !literals.is_empty() {
            return format!("({})", literals.join("|"));
        }
    }

    if let Some(ty) = schema.get("type") {
        if let Some(type_array) = ty.as_array() {
            let types: Vec<String> = type_array
                .iter()
                .filter_map(|t| t.as_str())
                .filter(|t| *t != "null")
                .map(|t| json_type_to_js(t, schema, current_type))
                .collect();
            let has_null = type_array.iter().any(|t| t.as_str() == Some("null"));

            if types.len() == 1 {
                let base_type = &types[0];
                return if has_null {
                    format!("?{}", base_type)
                } else {
                    base_type.clone()
                };
            } else if !types.is_empty() {
                let union = format!("({})", types.join("|"));
                return if has_null {
                    format!("?{}", union)
                } else {
                    union
                };
            }
        }

        if let Some(ty_str) = ty.as_str() {
            return json_type_to_js(ty_str, schema, current_type);
        }
    }

    if let Some(variants) = schema
        .get("anyOf")
        .or_else(|| schema.get("oneOf"))
        .and_then(|v| v.as_array())
    {
        let types: Vec<String> = variants
            .iter()
            .map(|v| schema_to_js_type(v, current_type))
            .collect();
        let filtered: Vec<_> = types.iter().filter(|t| *t != "*").collect();
        if !filtered.is_empty() {
            return format!(
                "({})",
                filtered
                    .iter()
                    .map(|s| s.as_str())
                    .collect::<Vec<_>>()
                    .join("|")
            );
        }
        return format!("({})", types.join("|"));
    }

    if let Some(format) = schema.get("format").and_then(|f| f.as_str()) {
        return match format {
            "int32" | "int64" => "number".to_string(),
            "float" | "double" => "number".to_string(),
            "date" | "date-time" => "string".to_string(),
            _ => "*".to_string(),
        };
    }

    "*".to_string()
}
