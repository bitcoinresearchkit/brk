use std::{collections::BTreeMap, io};

use crate::ref_to_type_name;
use oas3::Spec;
use oas3::spec::{ObjectOrReference, Operation, ParameterIn, PathItem, Schema, SchemaTypeSet};
use serde_json::Value;

/// Type schema extracted from OpenAPI components
pub type TypeSchemas = BTreeMap<String, Value>;

/// Endpoint information extracted from OpenAPI spec
#[derive(Debug, Clone)]
pub struct Endpoint {
    /// HTTP method (GET, POST, etc.)
    pub method: String,
    /// Path template (e.g., "/blocks/{hash}")
    pub path: String,
    /// Operation ID (e.g., "getBlockByHash")
    pub operation_id: Option<String>,
    /// Short summary
    pub summary: Option<String>,
    /// Detailed description
    pub description: Option<String>,
    /// Tags for grouping
    pub tags: Vec<String>,
    /// Path parameters
    pub path_params: Vec<Parameter>,
    /// Query parameters
    pub query_params: Vec<Parameter>,
    /// Response type (simplified)
    pub response_type: Option<String>,
    /// Whether this endpoint is deprecated
    pub deprecated: bool,
}

impl Endpoint {
    /// Returns true if this endpoint should be included in client generation.
    /// Only non-deprecated GET endpoints are included.
    pub fn should_generate(&self) -> bool {
        self.method == "GET" && !self.deprecated
    }

    /// Returns the operation ID or generates one from the path.
    /// The returned string uses the raw case from the spec (typically camelCase).
    pub fn operation_name(&self) -> String {
        if let Some(op_id) = &self.operation_id {
            return op_id.clone();
        }
        // Generate from path: /api/block/{hash} -> "get_block"
        // Skip "api" prefix, convert hyphens to underscores, avoid redundant param names
        let mut parts: Vec<String> = Vec::new();
        let mut prev_segment = "";

        for segment in self.path.split('/').filter(|s| !s.is_empty()) {
            if segment == "api" {
                continue;
            }
            if let Some(param) = segment.strip_prefix('{').and_then(|s| s.strip_suffix('}')) {
                // Only add "by_{param}" if the previous segment doesn't already contain the param name
                let prev_normalized = prev_segment.replace('-', "_");
                if !prev_normalized.ends_with(param) {
                    parts.push(format!("by_{}", param));
                }
            } else {
                let normalized = segment.replace('-', "_");
                parts.push(normalized);
                prev_segment = segment;
            }
        }
        format!("get_{}", parts.join("_"))
    }
}

/// Parameter information
#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub required: bool,
    pub param_type: String,
    pub description: Option<String>,
}

/// Parse OpenAPI spec from JSON string
///
/// Pre-processes the JSON to handle oas3 limitations:
/// - Removes unsupported siblings from `$ref` objects (oas3 only supports `summary` and `description`)
pub fn parse_openapi_json(json: &str) -> io::Result<Spec> {
    let mut value: Value =
        serde_json::from_str(json).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    // Clean up for oas3 compatibility
    clean_for_oas3(&mut value);

    let cleaned_json =
        serde_json::to_string(&value).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    oas3::from_json(&cleaned_json).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

/// Extract type schemas from OpenAPI JSON
pub fn extract_schemas(json: &str) -> TypeSchemas {
    let Ok(value) = serde_json::from_str::<Value>(json) else {
        return BTreeMap::new();
    };

    value
        .get("components")
        .and_then(|c| c.get("schemas"))
        .and_then(|s| s.as_object())
        .map(|schemas| {
            schemas
                .iter()
                .map(|(name, schema)| (name.clone(), schema.clone()))
                .collect()
        })
        .unwrap_or_default()
}

/// Clean up OpenAPI spec for oas3 compatibility.
/// - Removes unsupported siblings from $ref objects (oas3 only supports summary and description)
/// - Converts boolean schemas to object schemas (oas3 doesn't handle `"schema": true`)
fn clean_for_oas3(value: &mut Value) {
    match value {
        Value::Object(map) => {
            // Handle $ref with unsupported siblings
            if map.contains_key("$ref") {
                map.retain(|k, _| k == "$ref" || k == "summary" || k == "description");
            } else {
                // Convert boolean schemas to empty object schemas
                if let Some(schema) = map.get_mut("schema")
                    && schema.is_boolean()
                {
                    *schema = Value::Object(serde_json::Map::new());
                }
                for v in map.values_mut() {
                    clean_for_oas3(v);
                }
            }
        }
        Value::Array(arr) => {
            for v in arr {
                clean_for_oas3(v);
            }
        }
        _ => {}
    }
}

/// Extract all endpoints from OpenAPI spec
pub fn extract_endpoints(spec: &Spec) -> Vec<Endpoint> {
    let mut endpoints = Vec::new();

    let Some(paths) = &spec.paths else {
        return endpoints;
    };

    for (path, path_item) in paths {
        for (method, operation) in get_operations(path_item) {
            if let Some(endpoint) = extract_endpoint(path, &method, operation) {
                endpoints.push(endpoint);
            }
        }
    }

    endpoints
}

fn get_operations(path_item: &PathItem) -> Vec<(String, &Operation)> {
    let mut ops = Vec::new();
    if let Some(op) = &path_item.get {
        ops.push(("GET".to_string(), op));
    }
    if let Some(op) = &path_item.post {
        ops.push(("POST".to_string(), op));
    }
    if let Some(op) = &path_item.put {
        ops.push(("PUT".to_string(), op));
    }
    if let Some(op) = &path_item.delete {
        ops.push(("DELETE".to_string(), op));
    }
    if let Some(op) = &path_item.patch {
        ops.push(("PATCH".to_string(), op));
    }
    ops
}

fn extract_endpoint(path: &str, method: &str, operation: &Operation) -> Option<Endpoint> {
    let path_params = extract_parameters(operation, ParameterIn::Path);
    let query_params = extract_parameters(operation, ParameterIn::Query);

    let response_type = extract_response_type(operation);

    Some(Endpoint {
        method: method.to_string(),
        path: path.to_string(),
        operation_id: operation.operation_id.clone(),
        summary: operation.summary.clone(),
        description: operation.description.clone(),
        tags: operation.tags.clone(),
        path_params,
        query_params,
        response_type,
        deprecated: operation.deprecated.unwrap_or(false),
    })
}

fn extract_parameters(operation: &Operation, location: ParameterIn) -> Vec<Parameter> {
    operation
        .parameters
        .iter()
        .filter_map(|p| match p {
            ObjectOrReference::Object(param) if param.location == location => {
                let param_type = param
                    .schema
                    .as_ref()
                    .and_then(|s| match s {
                        ObjectOrReference::Ref { ref_path, .. } => {
                            ref_to_type_name(ref_path).map(|s| s.to_string())
                        }
                        ObjectOrReference::Object(obj_schema) => schema_to_type_name(obj_schema),
                    })
                    .unwrap_or_else(|| "string".to_string());
                Some(Parameter {
                    name: param.name.clone(),
                    required: param.required.unwrap_or(false),
                    param_type,
                    description: param.description.clone(),
                })
            }
            _ => None,
        })
        .collect()
}

fn extract_response_type(operation: &Operation) -> Option<String> {
    let responses = operation.responses.as_ref()?;

    // Look for 200 OK response
    let response = responses.get("200")?;

    match response {
        ObjectOrReference::Object(response) => {
            // Look for JSON content
            let content = response.content.get("application/json")?;

            match &content.schema {
                Some(ObjectOrReference::Ref { ref_path, .. }) => {
                    // Extract type name from reference like "#/components/schemas/Block"
                    Some(ref_to_type_name(ref_path)?.to_string())
                }
                Some(ObjectOrReference::Object(schema)) => schema_to_type_name(schema),
                None => None,
            }
        }
        ObjectOrReference::Ref { .. } => None,
    }
}

fn schema_type_from_schema(schema: &Schema) -> Option<String> {
    match schema {
        Schema::Boolean(_) => Some("boolean".to_string()),
        Schema::Object(obj_or_ref) => match obj_or_ref.as_ref() {
            ObjectOrReference::Object(obj_schema) => schema_to_type_name(obj_schema),
            ObjectOrReference::Ref { ref_path, .. } => {
                // Return the type name as-is (e.g., "Height", "Address")
                // These should have definitions generated from schemas
                ref_to_type_name(ref_path).map(|s| s.to_string())
            }
        },
    }
}

fn schema_to_type_name(schema: &oas3::spec::ObjectSchema) -> Option<String> {
    let schema_type = schema.schema_type.as_ref()?;

    match schema_type {
        SchemaTypeSet::Single(t) => match t {
            oas3::spec::SchemaType::String => Some("string".to_string()),
            oas3::spec::SchemaType::Number => Some("number".to_string()),
            oas3::spec::SchemaType::Integer => Some("number".to_string()),
            oas3::spec::SchemaType::Boolean => Some("boolean".to_string()),
            oas3::spec::SchemaType::Array => {
                let inner = match &schema.items {
                    Some(boxed_schema) => schema_type_from_schema(boxed_schema),
                    None => Some("*".to_string()),
                };
                inner.map(|t| format!("{}[]", t))
            }
            oas3::spec::SchemaType::Object => Some("Object".to_string()),
            oas3::spec::SchemaType::Null => Some("null".to_string()),
        },
        SchemaTypeSet::Multiple(_) => Some("*".to_string()),
    }
}
