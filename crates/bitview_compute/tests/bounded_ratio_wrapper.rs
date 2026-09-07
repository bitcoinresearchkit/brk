mod common;

use bitview_compute::BoundedRatioPerBlock;
use bitview_traversable::{Traversable, TreeNode};
use brk_types::{BoundedRatio, Version};
use vecdb::{AnySerializableVec, AnyStoredVec, AnyVec, Database, ReadableVec, WritableVec};

use common::indexes;

#[test]
fn bounded_wrapper_groups_storage_and_decimal_view() {
    let directory = tempfile::tempdir().unwrap();
    let db = Database::open(directory.path()).unwrap();
    let indexes = indexes(&db);
    let mut view =
        BoundedRatioPerBlock::forced_import(&db, "loss_share", Version::ONE, &indexes).unwrap();
    let values = [
        BoundedRatio::ZERO,
        BoundedRatio::from(0.5),
        BoundedRatio::ONE,
        BoundedRatio::NAN,
    ];
    for value in values {
        view.bounded.height.push(value);
    }
    view.bounded.height.write().unwrap();
    assert_eq!(view.bounded.height.name(), "loss_share_bounded");
    assert_eq!(view.ratio.height.name(), "loss_share");
    assert_eq!(f64::from(view.ratio.height.collect_one_at(1).unwrap()), 0.5);
    let mut json = Vec::new();
    view.ratio
        .height
        .write_json(Some(0), Some(4), &mut json)
        .unwrap();
    assert_eq!(json, b"[0.0,0.5,1.0,null]");
    let TreeNode::Branch(branch) = view.to_tree_node() else {
        panic!("expected wrapper branch")
    };
    for (name, kind) in [("bounded", "BoundedRatio"), ("ratio", "StoredF64")] {
        let TreeNode::Leaf(leaf) = branch.get(name).unwrap() else {
            panic!("expected vector leaf")
        };
        assert_eq!(leaf.kind(), kind);
    }
    drop(view);
    let reopened =
        BoundedRatioPerBlock::forced_import(&db, "loss_share", Version::ONE, &indexes).unwrap();
    assert_eq!(reopened.bounded.height.collect_range_at(0, 4), values);
}
