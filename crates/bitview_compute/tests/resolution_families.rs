mod common;

use bitview_compute::{DerivedResolutions, Identity, Resolutions};
use bitview_traversable::Traversable;
use brk_types::{Height, StoredU64, Version};
use vecdb::{CachedVec, Database, ReadableVec};

#[test]
fn resolution_families_preserve_order_empty_buckets_and_both_derived_constructors() {
    let temp = tempfile::tempdir().unwrap();
    let db = Database::open(temp.path()).unwrap();
    let mut indexes = common::indexes(&db);
    macro_rules! initialize_mappings {
        (
            periods { $($field:ident: $index:ident => $param:ident,)* }
            epochs { $($epoch:ident: $epoch_index:ident => $epoch_param:ident,)* }
        ) => {
            $(indexes.cached_first_height.$field = CachedVec::wrap(common::stored(
                &db, stringify!($field), [0_usize, 2, 2].map(Height::from),
            )).read_only_cached_boxed_clone();)*
            $(indexes.cached_first_height.$epoch = CachedVec::wrap(common::stored(
                &db, stringify!($epoch), [Height::ZERO],
            )).read_only_cached_boxed_clone();)*
        };
    }
    bitview_compute::with_resolution_fields!(initialize_mappings);
    let source = common::stored::<Height, _>(&db, "source", [10, 20, 30].map(StoredU64::new));
    let resolutions = Resolutions::from_height_source("metric", source, Version::ONE, &indexes);
    let direct = DerivedResolutions::from_derived_computed::<Identity<StoredU64>>(
        "metric",
        Version::ONE,
        &resolutions,
    );
    let chained = DerivedResolutions::from_lazy::<Identity<StoredU64>, StoredU64>(
        "metric",
        Version::ONE,
        &direct,
    );

    let expected_indexes = [
        "minute10", "minute30", "hour1", "hour4", "hour12", "day1", "day3", "week1", "month1",
        "month3", "month6", "year1", "year10", "halving", "epoch",
    ];
    for actual in [
        resolutions.iter_any_exportable().collect::<Vec<_>>(),
        direct.iter_any_exportable().collect(),
        chained.iter_any_exportable().collect(),
    ] {
        assert_eq!(
            actual
                .iter()
                .map(|v| v.index_type_to_string())
                .collect::<Vec<_>>(),
            expected_indexes,
        );
    }

    macro_rules! check_values {
        (
            periods { $($field:ident: $index:ident => $param:ident,)* }
            epochs { $($epoch:ident: $epoch_index:ident => $epoch_param:ident,)* }
        ) => {
            $(for values in [
                resolutions.$field.collect_range_at(0, 3),
                direct.$field.collect_range_at(0, 3),
                chained.$field.collect_range_at(0, 3),
            ] {
                assert_eq!(values, [Some(StoredU64::new(20)), None, Some(StoredU64::new(30))]);
            })*
            $(for values in [
                resolutions.$epoch.collect_range_at(0, 1),
                direct.$epoch.collect_range_at(0, 1),
                chained.$epoch.collect_range_at(0, 1),
            ] {
                assert_eq!(values, [StoredU64::new(30)]);
            })*
        };
    }
    bitview_compute::with_resolution_fields!(check_values);
    assert_eq!(
        serde_json::to_value(resolutions.to_tree_node()).unwrap(),
        serde_json::to_value(direct.to_tree_node()).unwrap(),
    );
    assert_eq!(
        serde_json::to_value(direct.to_tree_node()).unwrap(),
        serde_json::to_value(chained.to_tree_node()).unwrap(),
    );
}
