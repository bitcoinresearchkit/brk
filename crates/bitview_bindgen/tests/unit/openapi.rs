use super::*;

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
