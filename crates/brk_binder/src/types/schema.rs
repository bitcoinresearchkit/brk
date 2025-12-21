//! JSON Schema utilities.

use serde_json::Value;

/// Unwrap allOf with a single element, returning the inner schema.
/// Schemars uses allOf for composition, but often with just one $ref.
pub fn unwrap_allof(schema: &Value) -> &Value {
    if let Some(all_of) = schema.get("allOf").and_then(|v| v.as_array())
        && all_of.len() == 1
    {
        return &all_of[0];
    }
    schema
}

/// Check if a schema represents an enum type.
/// Enums have either an "enum" array or "oneOf" without properties.
pub fn is_enum_schema(schema: &Value) -> bool {
    schema.get("enum").is_some()
        || (schema.get("oneOf").is_some() && schema.get("properties").is_none())
}

/// Extract inner type from a wrapper generic like `Close<Dollars>` -> `Dollars`.
/// Also handles malformed types like `Dollars>` (from vecdb's short_type_name).
pub fn extract_inner_type(type_str: &str) -> String {
    // Handle proper generic wrappers like `Close<Dollars>` -> `Dollars`
    if let Some(start) = type_str.find('<')
        && let Some(end) = type_str.rfind('>')
        && start < end
    {
        return type_str[start + 1..end].to_string();
    }
    // Handle malformed types like `Dollars>` (trailing > without <)
    if type_str.ends_with('>') && !type_str.contains('<') {
        return type_str.trim_end_matches('>').to_string();
    }
    type_str.to_string()
}

/// Extract JSON type from a schema ("integer", "number", "string", etc).
pub fn schema_to_json_type(schema: &Value) -> String {
    schema
        .get("type")
        .and_then(|v| v.as_str())
        .unwrap_or("object")
        .to_string()
}
