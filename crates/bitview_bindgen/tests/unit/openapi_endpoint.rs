use crate::{extract_endpoints, parse_openapi_json};

#[test]
fn operation_names_follow_path_segments_and_explicit_overrides() {
    let spec = parse_openapi_json(r#"{"openapi":"3.1.0","info":{"title":"Fixture","version":"1"},"paths":{"/api/test":{"get":{"responses":{"200":{"description":"Success"}}}}}}"#).unwrap();
    let mut endpoint = extract_endpoints(&spec).pop().unwrap();
    for (path, expected) in [
        ("/api/", "get_"),
        ("/api/block-hash/{hash}", "get_block_hash"),
        ("/api/block/{height}/txs", "get_block_by_height_txs"),
        ("//api//price-close//", "get_price_close"),
        ("/api/é/{id}", "get_é_by_id"),
    ] {
        endpoint.path = path.into();
        assert_eq!(endpoint.operation_name(), expected);
    }
    for name in ["explicitName", ""] {
        endpoint.operation_id = Some(name.into());
        assert_eq!(endpoint.operation_name(), name);
    }
}
