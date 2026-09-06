use super::*;
use crate::openapi::{extract_endpoints, parse_openapi_json};

#[test]
fn typed_text_post_preserves_the_declared_result_and_checked_in_method() {
    let spec = parse_openapi_json(r##"{
        "openapi":"3.1.0", "info":{"title":"Test","version":"1"},
        "components":{"schemas":{"Txid":{"type":"string"}}},
        "paths":{"/api/tx":{"post":{
            "operationId":"post_tx",
            "requestBody":{"required":true,"content":{"text/plain":{"schema":{"type":"string"}}}},
            "responses":{"200":{"description":"Success","content":{"text/plain":{"schema":{"$ref":"#/components/schemas/Txid"}}}}}
        }}}
    }"##).unwrap();
    let endpoint = extract_endpoints(&spec).pop().unwrap();
    let mut output = String::new();
    generate_post_method(&mut output, &endpoint);
    assert!(output.contains("Result<Txid>"));
    assert!(output.contains("post_text"));
    assert!(output.contains("parse::<Txid>"));
    assert!(!output.contains("post_json"));
    let method = &output[output.find("pub fn post_tx").unwrap()..];
    let compact = |text: &str| text.split_whitespace().collect::<String>();
    let client = include_str!("../../../../../bitview_client/src/generated.rs");
    // Formatting can add a trailing comma in the error struct literal.
    assert!(
        compact(client)
            .replace(",}", "}")
            .contains(&compact(method).replace(",}", "}"))
    );
}
