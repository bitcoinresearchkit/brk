use serde_json::Value;

use super::*;
use crate::{extract_endpoints, parse_openapi_json};

#[test]
fn parameter_lists_keep_separators_even_for_empty_sanitized_names() {
    let spec = parse_openapi_json(r#"{"openapi":"3.1.0","info":{"title":"Fixture","version":"1"},"paths":{"/api/test":{"get":{"responses":{"200":{"description":"Success"}}}}}}"#).unwrap();
    let mut endpoint = extract_endpoints(&spec).pop().unwrap();
    let names = ["", "[]", "hash", "ids[]", "é"];
    for a in names {
        for b in names {
            for c in names {
                let parameters = [a, b, c].map(|name| Parameter {
                    name: name.into(),
                    param_type: "string".into(),
                    required: true,
                    description: None,
                    schema: Value::Null,
                });
                for split in 0..=3 {
                    endpoint.path_params = parameters[..split].to_vec();
                    endpoint.query_params = parameters[split..].to_vec();
                    assert_eq!(
                        build_method_params(&endpoint),
                        [a, b, c].map(sanitize_ident).join(", ")
                    );
                }
            }
        }
    }
    endpoint.path_params.clear();
    endpoint.query_params.clear();
    assert_eq!(build_method_params(&endpoint), "");
}
