use std::collections::BTreeSet;

use bitview_catalog::{SeriesLeaf, SeriesLeafWithSchema, TreeBranch, TreeNode};
use brk_types::Index;
use indexmap::IndexMap;
use schemars::JsonSchema;
use serde_json::json;

fn family(index: Index) -> TreeNode {
    let mut node = TreeNode::branch(
        ["min", "max"]
            .map(|field| {
                (
                    field.to_string(),
                    TreeNode::Leaf(SeriesLeafWithSchema::new(
                        SeriesLeaf::new(
                            format!("value_{field}"),
                            "Sats".into(),
                            BTreeSet::from([index]),
                        ),
                        json!({"type": "integer"}),
                    )),
                )
            })
            .into(),
    )
    .with_field_suffixes()
    .with_source("fixture::Stats");
    let TreeNode::Branch(branch) = &mut node else {
        unreachable!()
    };
    for key in ["min", "max"] {
        branch.field_types.insert(key.into(), "fixture::Stats::A");
    }
    node
}

fn branch(node: &TreeNode) -> &TreeBranch {
    let TreeNode::Branch(branch) = node else {
        panic!("expected branch")
    };
    branch
}

#[test]
fn naming_is_internal_and_keeps_the_original_map_schema() {
    let node = family(Index::Height);
    let encoded = serde_json::to_value(&node).unwrap();
    assert_eq!(
        encoded,
        serde_json::to_value(&branch(&node).children).unwrap()
    );
    let decoded: TreeNode = serde_json::from_value(encoded).unwrap();
    assert_eq!(node, decoded);
    assert!(!branch(&decoded).field_suffixes);
    assert!(branch(&decoded).source.is_none());
    assert!(branch(&decoded).field_types.is_empty());
    assert_eq!(
        TreeBranch::schema_id(),
        IndexMap::<String, TreeNode>::schema_id()
    );
    assert_eq!(
        TreeBranch::schema_name(),
        IndexMap::<String, TreeNode>::schema_name()
    );
    assert_eq!(
        TreeBranch::inline_schema(),
        IndexMap::<String, TreeNode>::inline_schema()
    );
    assert_eq!(
        schemars::schema_for!(TreeBranch),
        schemars::schema_for!(IndexMap<String, TreeNode>),
    );
}

#[test]
fn naming_survives_resolution_merges_and_wrapping() {
    let merged = TreeNode::branch(IndexMap::from([
        ("height".into(), family(Index::Height)),
        ("day".into(), family(Index::Day1)),
    ]))
    .merge_branches()
    .unwrap();
    assert!(branch(&merged).field_suffixes);
    assert_eq!(branch(&merged).source, Some("fixture::Stats"));
    assert_eq!(branch(&merged).field_types.len(), 2);
    let TreeNode::Leaf(min) = &branch(&merged)["min"] else {
        panic!("expected leaf")
    };
    assert_eq!(min.indexes(), &BTreeSet::from([Index::Height, Index::Day1]));
    let wrapped = TreeNode::wrap("stats", merged);
    assert!(!branch(&wrapped).field_suffixes);
    assert!(branch(&branch(&wrapped)["stats"]).field_suffixes);

    let mut root = IndexMap::from([("stats".into(), family(Index::Height))]);
    TreeNode::merge_node(&mut root, "stats".into(), family(Index::Day1)).unwrap();
    assert!(branch(&root["stats"]).field_suffixes);
}

#[test]
fn combining_a_family_with_undeclared_fields_drops_the_contract() {
    let mut ordinary = family(Index::Day1);
    let TreeNode::Branch(children) = &mut ordinary else {
        unreachable!()
    };
    children.field_suffixes = false;
    let merged = TreeNode::branch(IndexMap::from([
        ("known".into(), family(Index::Height)),
        ("unknown".into(), ordinary.clone()),
    ]))
    .merge_branches()
    .unwrap();
    assert!(!branch(&merged).field_suffixes);

    let mut root = IndexMap::from([("stats".into(), family(Index::Height))]);
    TreeNode::merge_node(&mut root, "stats".into(), ordinary).unwrap();
    assert!(!branch(&root["stats"]).field_suffixes);
}

#[test]
fn conflicting_sources_and_field_declarations_are_not_invented_by_merging() {
    let mut other = family(Index::Day1).with_source("ignored::TransparentWrapper");
    assert_eq!(branch(&other).source, Some("fixture::Stats"));
    let TreeNode::Branch(fields) = &mut other else {
        unreachable!()
    };
    fields.source = Some("fixture::Other");
    fields.field_types.insert("min".into(), "fixture::Other::A");
    let mut root = IndexMap::from([("stats".into(), family(Index::Height))]);
    TreeNode::merge_node(&mut root, "stats".into(), other).unwrap();
    let merged = branch(&root["stats"]);
    assert!(merged.source.is_none());
    assert!(!merged.field_types.contains_key("min"));
    assert_eq!(merged.field_types["max"], "fixture::Stats::A");
}
