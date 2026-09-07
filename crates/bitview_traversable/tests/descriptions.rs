use std::collections::BTreeMap;

use bitview_traversable::{IndexMap, Traversable, TreeNode};
use vecdb::AnyExportableVec;

struct TestLeaf(&'static str);

impl Traversable for TestLeaf {
    fn to_tree_node(&self) -> TreeNode {
        TreeNode::branch(IndexMap::new())
    }

    fn iter_any_exportable(&self) -> impl Iterator<Item = &dyn AnyExportableVec> {
        std::iter::empty()
    }

    fn collect_series_descriptions<'a>(
        &'a self,
        description_fragments: &mut Vec<&'static str>,
        descriptions: &mut BTreeMap<&'a str, Vec<&'static str>>,
    ) {
        descriptions.insert(self.0, description_fragments.clone());
    }
}

#[derive(bitview_traversable_derive::Traversable)]
struct TestTree {
    /// Reported in the child representation.
    #[traversable(rename = "metric")]
    pub represented: TestLeaf,
    inherited: TestLeaf,
}

#[derive(bitview_traversable_derive::Traversable)]
struct TestRoot {
    /// The exact metric definition. Continues on a second line.
    #[traversable(flatten)]
    metrics: TestTree,
}

#[test]
fn joins_every_documented_field_on_the_path_in_order() {
    let tree = TestRoot {
        metrics: TestTree {
            represented: TestLeaf("represented"),
            inherited: TestLeaf("inherited"),
        },
    };
    let mut description_fragments = Vec::new();
    let mut descriptions = BTreeMap::new();

    tree.collect_series_descriptions(&mut description_fragments, &mut descriptions);

    assert_eq!(
        descriptions.get("represented").unwrap(),
        &[
            "The exact metric definition. Continues on a second line.",
            "Reported in the child representation.",
        ]
    );
    assert_eq!(
        descriptions.get("inherited").unwrap(),
        &["The exact metric definition. Continues on a second line."]
    );
    assert!(description_fragments.is_empty());
}

#[derive(bitview_traversable_derive::Traversable)]
struct WrappedTree {
    #[traversable(wrap = "outer/inner", rename = "renamed")]
    named: TestLeaf,
    #[traversable(wrap = "outer")]
    _plain: TestLeaf,
    r#type: TestLeaf,
    #[traversable(wrap = "optional/path")]
    _maybe: Option<TestLeaf>,
    #[traversable(wrap = "", rename = "")]
    empty: TestLeaf,
}

#[test]
fn wrapping_preserves_names_paths_and_optional_fields() {
    let tree = WrappedTree {
        named: TestLeaf("named"),
        _plain: TestLeaf("plain"),
        r#type: TestLeaf("type"),
        _maybe: Some(TestLeaf("maybe")),
        empty: TestLeaf("empty"),
    };
    fn keys(node: &TreeNode) -> Vec<String> {
        let TreeNode::Branch(branch) = node else {
            panic!("expected branch")
        };
        branch.keys().cloned().collect()
    }
    fn child<'a>(node: &'a TreeNode, key: &str) -> &'a TreeNode {
        let TreeNode::Branch(branch) = node else {
            panic!("expected branch")
        };
        &branch[key]
    }
    let node = tree.to_tree_node();
    assert_eq!(keys(&node), ["outer", "type", "optional", ""]);
    assert_eq!(keys(child(&node, "outer")), ["inner", "plain"]);
    assert_eq!(keys(child(child(&node, "outer"), "inner")), ["renamed"]);
    assert_eq!(keys(child(child(&node, "optional"), "path")), ["maybe"]);
    assert_eq!(keys(child(&node, "")), [""]);
    let absent = WrappedTree {
        _maybe: None,
        ..tree
    }
    .to_tree_node();
    assert_eq!(keys(&absent), ["outer", "type", ""]);
}
