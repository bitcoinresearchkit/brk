use aide::{openapi::Operation, transform::TransformOperation};

use super::TransformResponseExtended;

#[test]
fn error_schema_matches_wire_media_type() {
    let mut operation = Operation::default();
    let _ = TransformOperation::new(&mut operation)
        .bad_request()
        .server_errors();
    let operation = serde_json::to_value(operation).unwrap();
    assert_eq!(operation["responses"].as_object().unwrap().len(), 4);
    for status in ["400", "500", "503", "504"] {
        let content = operation["responses"][status]["content"]
            .as_object()
            .unwrap();
        assert_eq!(content.len(), 1);
        assert!(content.contains_key("application/problem+json"));
    }
}
