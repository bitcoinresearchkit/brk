fn original(path: &str) -> String {
    let mut parts = Vec::new();
    let mut previous = "";
    for segment in path.split('/').filter(|s| !s.is_empty()) {
        if segment == "api" {
            continue;
        }
        if let Some(param) = segment.strip_prefix('{').and_then(|s| s.strip_suffix('}')) {
            if !previous.replace('-', "_").ends_with(param) {
                parts.push(format!("by_{param}"));
            }
        } else {
            parts.push(segment.replace('-', "_"));
            previous = segment;
        }
    }
    format!("get_{}", parts.join("_"))
}

#[test]
fn operation_names_preserve_path_segment_and_parameter_rules() {
    let spec = crate::parse_openapi_json(r#"{"openapi":"3.1.0","info":{"title":"Fixture","version":"1"},"paths":{"/api/test":{"get":{"responses":{"200":{"description":"Success"}}}}}}"#).unwrap();
    let mut endpoint = crate::extract_endpoints(&spec).pop().unwrap();
    let segments = [
        "",
        "api",
        "block-hash",
        "{hash}",
        "{height}",
        "é",
        "{}",
        "{",
        "{{id}}",
        "x-y",
        "x_y",
        "all",
    ];
    for a in segments {
        for b in segments {
            for c in segments {
                endpoint.path = format!("/{a}/{b}/{c}");
                assert_eq!(endpoint.operation_name(), original(&endpoint.path));
            }
        }
    }
    endpoint.operation_id = Some("explicitName".into());
    assert_eq!(endpoint.operation_name(), "explicitName");
    endpoint.operation_id = Some(String::new());
    assert_eq!(endpoint.operation_name(), "");
}
