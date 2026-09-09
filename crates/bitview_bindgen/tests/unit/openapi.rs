use serde_json::json;

use super::*;

#[test]
fn schema_extraction_preserves_owned_nested_values_and_invalid_input_fallbacks() {
    for input in [
        "invalid",
        "null",
        "[]",
        "{}",
        r#"{"components":false}"#,
        r#"{"components":{"schemas":[]}}"#,
    ] {
        assert!(extract_schemas(input).is_empty(), "{input}");
    }
    let expected = json!({
        "Z": false,
        "Nested": {"properties": {"value": {"anyOf": [{"type":"string"}, {"$ref":"#/components/schemas/Z"}]}}},
        "A": {"enum": ["é", "", "value"]}
    });
    let input = json!({"components": {"schemas": expected}, "ignored": [1,2,3]}).to_string();
    let schemas = extract_schemas(&input);
    drop(input);
    assert_eq!(
        schemas.keys().map(String::as_str).collect::<Vec<_>>(),
        ["A", "Nested", "Z"]
    );
    for (name, schema) in expected.as_object().unwrap() {
        assert_eq!(&schemas[name], schema);
    }
}

#[test]
fn extracts_mcp_ignore_without_disabling_client_generation() {
    let spec = parse_openapi_json(
        r#"{
            "openapi": "3.1.0",
            "info": { "title": "Test", "version": "1" },
            "paths": {
                "/api/items": {
                    "get": {
                        "operationId": "get_items",
                        "x-mcp-ignore": true,
                        "responses": {
                            "200": { "description": "Successful response" }
                        }
                    }
                }
            }
        }"#,
    )
    .unwrap();
    let endpoint = extract_endpoints(&spec).into_iter().next().unwrap();

    assert!(endpoint.mcp_ignored);
    assert!(endpoint.should_generate());
}

#[test]
fn extracts_request_body_media_type() {
    let spec = parse_openapi_json(
        r#"{
            "openapi": "3.1.0",
            "info": { "title": "Test", "version": "1" },
            "paths": {
                "/api/items": {
                    "post": {
                        "operationId": "post_items",
                        "requestBody": {
                            "required": true,
                            "content": {
                                "application/json": {
                                    "schema": { "type": "object" }
                                }
                            }
                        },
                        "responses": {
                            "200": { "description": "Successful response" }
                        }
                    }
                }
            }
        }"#,
    )
    .unwrap();
    let body = extract_endpoints(&spec)
        .into_iter()
        .next()
        .unwrap()
        .request_body
        .unwrap();

    assert_eq!(body.body_type, "Object");
    assert_eq!(body.content_type, "application/json");
    assert!(body.required);
}

#[test]
fn extracts_every_openapi_http_method() {
    let spec = parse_openapi_json(
        r#"{
            "openapi": "3.1.0",
            "info": { "title": "Test", "version": "1" },
            "paths": {
                "/api/trace": {
                    "trace": {
                        "operationId": "trace_items",
                        "responses": {
                            "200": { "description": "Successful response" }
                        }
                    }
                }
            }
        }"#,
    )
    .unwrap();
    let endpoint = extract_endpoints(&spec).into_iter().next().unwrap();

    assert_eq!(endpoint.method, "TRACE");
}
