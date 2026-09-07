use std::{collections::BTreeSet, convert::Infallible};

use bitview_compute::DistributionStats;
use bitview_traversable::{SeriesLeaf, SeriesLeafWithSchema, Traversable, TreeNode};
use brk_types::Index;
use serde_json::json;
use vecdb::AnyExportableVec;

struct Leaf(String);

impl Traversable for Leaf {
    fn to_tree_node(&self) -> TreeNode {
        TreeNode::Leaf(SeriesLeafWithSchema::new(
            SeriesLeaf::new(
                self.0.clone(),
                "Sats".into(),
                BTreeSet::from([Index::Height]),
            ),
            json!({"type": "integer"}),
        ))
    }

    fn iter_any_exportable(&self) -> impl Iterator<Item = &dyn AnyExportableVec> {
        std::iter::empty()
    }
}

#[test]
fn distribution_stats_declares_the_names_used_by_its_constructor() {
    let stats = DistributionStats::try_from_fn(|suffix| {
        Ok::<_, Infallible>(Leaf(format!("metric_{suffix}")))
    })
    .unwrap();
    let TreeNode::Branch(branch) = stats.to_tree_node() else {
        panic!("expected family")
    };
    assert!(branch.field_suffixes);
    assert!(branch.source.unwrap().ends_with("::DistributionStats"));
    assert_eq!(
        branch.field_types.values().collect::<BTreeSet<_>>().len(),
        1
    );
    assert_eq!(branch.len(), DistributionStats::<Leaf>::SUFFIXES.len());
    for (field, node) in branch {
        let TreeNode::Leaf(leaf) = node else {
            panic!("expected leaf")
        };
        assert_eq!(leaf.name(), format!("metric_{field}"));
    }
}

#[test]
fn resolution_expansion_preserves_catalog_identity_and_field_declarations() {
    macro_rules! family {
        (
            periods { $($field:ident: $index:ident => $param:ident,)* }
            epochs { $($epoch:ident: $epoch_index:ident => $epoch_param:ident,)* }
        ) => {
            bitview_compute::PerResolution {
                $($field: Leaf(stringify!($field).into()),)*
                $($epoch: Leaf(stringify!($epoch).into()),)*
            }
        };
    }
    let family = bitview_compute::with_resolution_fields!(family);
    let TreeNode::Branch(branch) = family.to_tree_node() else {
        panic!("expected resolution family")
    };
    let source = "bitview_compute::containers::per_resolution::PerResolution";
    assert_eq!(branch.source, Some(source));
    let expected = [
        ("minute10", "M10"),
        ("minute30", "M30"),
        ("hour1", "H1"),
        ("hour4", "H4"),
        ("hour12", "H12"),
        ("day1", "D1"),
        ("day3", "D3"),
        ("week1", "W1"),
        ("month1", "Mo1"),
        ("month3", "Mo3"),
        ("month6", "Mo6"),
        ("year1", "Y1"),
        ("year10", "Y10"),
        ("halving", "HE"),
        ("epoch", "DE"),
    ];
    assert_eq!(branch.len(), expected.len());
    for (field, parameter) in expected {
        assert_eq!(branch.field_types[field], format!("{source}::{parameter}"));
    }
}
