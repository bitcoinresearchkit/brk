use std::{collections::BTreeSet, fs};

use bitview_bindgen::{ClientOutputPaths, generate_clients};
use bitview_catalog::{SeriesLeaf, SeriesLeafWithSchema, TreeNode};
use brk_types::Index;
use indexmap::IndexMap;
use serde_json::json;
use tempfile::tempdir;

#[test]
fn generates_every_output_from_catalog_and_openapi_only() {
    let catalog = TreeNode::branch(IndexMap::from([(
        "metric".into(),
        TreeNode::Leaf(SeriesLeafWithSchema::new(
            SeriesLeaf::new(
                "fixture_metric".into(),
                "Sats".into(),
                BTreeSet::from([Index::Height]),
            ),
            json!({"type": "integer"}),
        )),
    )]));
    let openapi = json!({
        "openapi": "3.1.0",
        "info": {"title": "Fixture", "version": "1"},
        "paths": {"/api/value": {"get": {
            "operationId": "get_value",
            "responses": {"200": {
                "description": "Value",
                "content": {"application/json": {"schema": {"type": "integer"}}}
            }}
        }}}
    })
    .to_string();
    let directory = tempdir().unwrap();
    let root = directory.path();
    let paths = ClientOutputPaths::new()
        .rust(root.join("client.rs"))
        .cli(root.join("cli.rs"))
        .javascript(root.join("client.js"))
        .python(root.join("client.py"))
        .llm(root.join("llm"))
        .llm_manifest(root.join("manifest.json"));

    generate_clients(&catalog, &openapi, &paths).unwrap();

    for name in ["client.rs", "client.js", "client.py"] {
        assert!(
            fs::read_to_string(root.join(name))
                .unwrap()
                .contains("fixture_metric"),
            "{name}"
        );
    }
    for name in [
        "cli.rs",
        "llm/llms.txt",
        "llm/llms-full.txt",
        "manifest.json",
    ] {
        assert!(!fs::read(root.join(name)).unwrap().is_empty(), "{name}");
    }
}
