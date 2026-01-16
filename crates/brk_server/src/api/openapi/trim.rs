use serde_json::{Map, Value};

/// Trims an OpenAPI spec JSON to reduce size for LLM consumption.
/// Removes redundant fields while preserving essential API information.
///
/// Transformations applied (in order):
/// 1. Remove error responses (304, 400, 404, 500)
/// 2. Compact responses to "returns": "Type"
/// 3. Remove per-endpoint tags and style
/// 4. Simplify parameter schema to type, remove param descriptions
/// 5. Remove summary
/// 6. Remove examples, replace $ref with type
/// 7. Flatten single-item allOf
/// 8. Flatten anyOf to type array
/// 9. Remove format
/// 10. Remove property descriptions
/// 11. Simplify properties to direct types
pub fn trim_openapi_json(json: &str) -> String {
    let mut spec: Value = serde_json::from_str(json).expect("Invalid OpenAPI JSON");
    trim_value(&mut spec);
    serde_json::to_string(&spec).unwrap()
}

fn trim_value(value: &mut Value) {
    match value {
        Value::Object(obj) => {
            // Step 1: Remove error responses
            if let Some(Value::Object(responses)) = obj.get_mut("responses") {
                for code in &["304", "400", "404", "500"] {
                    responses.remove(*code);
                }
            }

            // Step 2: Compact responses to "returns": "Type"
            if let Some(Value::Object(responses)) = obj.remove("responses")
                && let Some(returns) = extract_return_type(&responses)
            {
                obj.insert("returns".to_string(), Value::String(returns));
            }

            // Step 3: Remove per-endpoint tags and style
            // (only remove "tags" if it's an array, not if it's the top-level tags definition)
            if let Some(Value::Array(_)) = obj.get("tags") {
                // This is a per-endpoint tags array like ["Addresses"], remove it
                obj.remove("tags");
            }
            obj.remove("style");

            // Step 4: Simplify parameters (schema to type, remove descriptions)
            if let Some(Value::Array(params)) = obj.get_mut("parameters") {
                for param in params {
                    simplify_parameter(param);
                }
            }

            // Step 5: Remove summary
            obj.remove("summary");

            // Step 6: Remove examples, replace $ref with type
            obj.remove("example");
            obj.remove("examples");
            if let Some(Value::String(ref_path)) = obj.remove("$ref") {
                let type_name = ref_path.split('/').next_back().unwrap_or("any");
                obj.insert("type".to_string(), Value::String(type_name.to_string()));
            }

            // Step 7: Flatten single-item allOf
            if let Some(Value::Array(all_of)) = obj.remove("allOf")
                && all_of.len() == 1
                && let Some(Value::Object(inner)) = all_of.into_iter().next()
            {
                for (k, v) in inner {
                    obj.insert(k, v);
                }
            }

            // Step 8: Flatten anyOf to type array
            if let Some(Value::Array(any_of)) = obj.remove("anyOf") {
                let types: Vec<Value> = any_of
                    .into_iter()
                    .filter_map(|item| {
                        if let Value::Object(o) = item {
                            if let Some(Value::String(ref_path)) = o.get("$ref") {
                                return Some(Value::String(
                                    ref_path.split('/').next_back().unwrap_or("any").to_string(),
                                ));
                            }
                            o.get("type").cloned()
                        } else {
                            None
                        }
                    })
                    .collect();
                if !types.is_empty() {
                    obj.insert("type".to_string(), Value::Array(types));
                }
            }

            // Step 9: Remove format
            obj.remove("format");

            // Step 10 & 11: Simplify properties (remove descriptions, simplify to direct types)
            if let Some(Value::Object(props)) = obj.get_mut("properties") {
                simplify_properties(props);
            }

            // Recurse into remaining values
            for (_, v) in obj.iter_mut() {
                trim_value(v);
            }
        }
        Value::Array(arr) => {
            for item in arr {
                trim_value(item);
            }
        }
        _ => {}
    }
}

fn extract_return_type(responses: &Map<String, Value>) -> Option<String> {
    let resp_200 = responses.get("200")?;
    let content = resp_200.get("content")?;
    let json_content = content.get("application/json")?;
    let schema = json_content.get("schema")?;
    Some(schema_to_type_string(schema))
}

fn schema_to_type_string(schema: &Value) -> String {
    if let Some(Value::String(ref_path)) = schema.get("$ref") {
        return ref_path.split('/').next_back().unwrap_or("any").to_string();
    }
    if let Some(Value::String(t)) = schema.get("type") {
        if t == "array"
            && let Some(items) = schema.get("items")
        {
            return format!("array[{}]", schema_to_type_string(items));
        }
        return t.clone();
    }
    "any".to_string()
}

fn simplify_parameter(param: &mut Value) {
    if let Value::Object(obj) = param {
        // Remove description
        obj.remove("description");

        // Extract type from schema
        if let Some(schema) = obj.remove("schema") {
            let type_val = extract_type_from_schema(&schema);
            obj.insert("type".to_string(), type_val);
        }
    }
}

fn extract_type_from_schema(schema: &Value) -> Value {
    if let Value::Object(obj) = schema {
        // Handle anyOf (optional fields)
        if let Some(Value::Array(any_of)) = obj.get("anyOf") {
            let types: Vec<Value> = any_of
                .iter()
                .filter_map(|item| {
                    if let Value::Object(o) = item {
                        if let Some(Value::String(ref_path)) = o.get("$ref") {
                            return Some(Value::String(
                                ref_path.split('/').next_back().unwrap_or("any").to_string(),
                            ));
                        }
                        o.get("type").cloned()
                    } else {
                        None
                    }
                })
                .collect();
            if types.len() == 1 {
                return types.into_iter().next().unwrap();
            }
            return Value::Array(types);
        }

        // Handle $ref
        if let Some(Value::String(ref_path)) = obj.get("$ref") {
            return Value::String(ref_path.split('/').next_back().unwrap_or("any").to_string());
        }

        // Handle type
        if let Some(t) = obj.get("type") {
            return t.clone();
        }
    }
    Value::String("any".to_string())
}

fn simplify_properties(props: &mut Map<String, Value>) {
    let keys: Vec<String> = props.keys().cloned().collect();
    for key in keys {
        if let Some(prop_value) = props.get_mut(&key)
            && let Value::Object(prop_obj) = prop_value
        {
            // Remove description
            prop_obj.remove("description");

            // Check if we can simplify to just the type
            let simplified = simplify_property_value(prop_obj);
            *prop_value = simplified;
        }
    }
}

fn simplify_property_value(obj: &mut Map<String, Value>) -> Value {
    // Remove validation constraints
    for key in &["default", "minItems", "maxItems", "uniqueItems"] {
        obj.remove(*key);
    }

    // Handle $ref - convert to type (runs before recursion would)
    if let Some(Value::String(ref_path)) = obj.remove("$ref") {
        let type_name = ref_path.split('/').next_back().unwrap_or("any");
        return Value::String(type_name.to_string());
    }

    // Handle single-item allOf - flatten and extract type
    if let Some(Value::Array(all_of)) = obj.remove("allOf")
        && all_of.len() == 1
        && let Some(Value::Object(inner)) = all_of.into_iter().next()
    {
        if let Some(Value::String(ref_path)) = inner.get("$ref") {
            let type_name = ref_path.split('/').next_back().unwrap_or("any");
            return Value::String(type_name.to_string());
        }
        if let Some(t) = inner.get("type") {
            return t.clone();
        }
    }

    // Handle anyOf - flatten to type array (runs before recursion would)
    if let Some(Value::Array(any_of)) = obj.remove("anyOf") {
        let types: Vec<Value> = any_of
            .into_iter()
            .filter_map(|item| {
                if let Value::Object(o) = item {
                    if let Some(Value::String(ref_path)) = o.get("$ref") {
                        return Some(Value::String(
                            ref_path.split('/').next_back().unwrap_or("any").to_string(),
                        ));
                    }
                    o.get("type").cloned()
                } else {
                    None
                }
            })
            .collect();
        return Value::Array(types);
    }

    // If only "type" remains, return just the type value
    if obj.len() == 1
        && let Some(t) = obj.get("type")
    {
        return t.clone();
    }

    // Handle array with items
    if obj.get("type") == Some(&Value::String("array".to_string()))
        && let Some(items) = obj.get("items")
        && let Value::Object(items_obj) = items
        && items_obj.len() == 1
    {
        // Items can have either "type" or "$ref"
        if let Some(Value::String(item_type)) = items_obj.get("type") {
            return Value::String(format!("array[{}]", item_type));
        }
        if let Some(Value::String(ref_path)) = items_obj.get("$ref") {
            let type_name = ref_path.split('/').next_back().unwrap_or("any");
            return Value::String(format!("array[{}]", type_name));
        }
    }

    Value::Object(obj.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trim_property_anyof() {
        let input = r##"{
            "type": "object",
            "properties": {
                "index": {
                    "anyOf": [
                        {"type": "TxIndex"},
                        {"type": "null"}
                    ]
                }
            }
        }"##;

        let result = trim_openapi_json(input);
        let parsed: Value = serde_json::from_str(&result).unwrap();

        // Property should be simplified to array, not {"type": [...]}
        let index = &parsed["properties"]["index"];
        assert!(index.is_array(), "Expected array, got: {}", index);
        assert_eq!(index[0], "TxIndex");
        assert_eq!(index[1], "null");
    }

    #[test]
    fn test_trim_parameter_anyof() {
        let input = r##"{
            "parameters": [{
                "in": "query",
                "name": "after_txid",
                "schema": {
                    "anyOf": [
                        {"$ref": "#/components/schemas/Txid"},
                        {"type": "null"}
                    ]
                }
            }]
        }"##;

        let result = trim_openapi_json(input);
        let parsed: Value = serde_json::from_str(&result).unwrap();

        // Parameter should have type array including null
        let param = &parsed["parameters"][0];
        assert_eq!(param["name"], "after_txid");
        assert_eq!(param["type"][0], "Txid");
        assert_eq!(param["type"][1], "null");
    }

    #[test]
    fn test_trim_property_ref() {
        let input = r##"{
            "type": "object",
            "properties": {
                "txid": {
                    "$ref": "#/components/schemas/Txid"
                }
            }
        }"##;

        let result = trim_openapi_json(input);
        let parsed: Value = serde_json::from_str(&result).unwrap();

        // Property with $ref should be simplified to just the type name
        assert_eq!(parsed["properties"]["txid"], "Txid");
    }

    #[test]
    fn test_trim_nested_component_schema() {
        // This matches the actual API structure: components > schemas > Type > properties
        let input = r##"{
            "components": {
                "schemas": {
                    "AddressStats": {
                        "type": "object",
                        "properties": {
                            "address": {
                                "$ref": "#/components/schemas/Address"
                            },
                            "chain_stats": {
                                "$ref": "#/components/schemas/AddressChainStats"
                            }
                        }
                    }
                }
            }
        }"##;

        let result = trim_openapi_json(input);
        let parsed: Value = serde_json::from_str(&result).unwrap();

        let props = &parsed["components"]["schemas"]["AddressStats"]["properties"];
        assert_eq!(props["address"], "Address", "address should be simplified");
        assert_eq!(
            props["chain_stats"], "AddressChainStats",
            "chain_stats should be simplified"
        );
    }

    #[test]
    fn test_trim_property_allof_with_ref() {
        // Real API uses allOf wrapper around $ref
        let input = r##"{
            "type": "object",
            "properties": {
                "address": {
                    "description": "Bitcoin address string",
                    "allOf": [
                        {"$ref": "#/components/schemas/Address"}
                    ]
                }
            }
        }"##;

        let result = trim_openapi_json(input);
        let parsed: Value = serde_json::from_str(&result).unwrap();

        assert_eq!(parsed["properties"]["address"], "Address");
    }

    #[test]
    fn test_trim_property_array_with_ref() {
        let input = r##"{
            "type": "object",
            "properties": {
                "vin": {
                    "type": "array",
                    "items": {
                        "$ref": "#/components/schemas/TxIn"
                    }
                }
            }
        }"##;

        let result = trim_openapi_json(input);
        let parsed: Value = serde_json::from_str(&result).unwrap();

        // Array with $ref items should be simplified to "array[Type]"
        assert_eq!(parsed["properties"]["vin"], "array[TxIn]");
    }

    #[test]
    fn test_trim_responses_to_returns() {
        let input = r##"{
            "responses": {
                "200": {
                    "content": {
                        "application/json": {
                            "schema": {"$ref": "#/components/schemas/Block"}
                        }
                    }
                },
                "400": {"description": "Bad request"},
                "500": {"description": "Error"}
            }
        }"##;

        let result = trim_openapi_json(input);
        let parsed: Value = serde_json::from_str(&result).unwrap();

        assert_eq!(parsed["returns"], "Block");
        assert!(parsed.get("responses").is_none());
    }
}
