use bitview_cohort::{
    AgeRangeId, AmountRange, AmountRangeId, ClassId, CohortContext, EntryId, EpochId, OverAgeId,
    OverAmountId, SpendableTypeId, UTXOAggregateId, UnderAgeId, UnderAmountId,
};
use bitview_traversable::{Traversable, TreeNode};
use bitview_vecs::{ColumnarAmount, ColumnarPerBlock, ExactUTXOColumns};
use brk_types::{Height, StoredU64, Version};
use vecdb::{
    AnyStoredVec, AnyVec, ColumnId, ColumnarVec, Database, EagerVec, ImportableVec, PcoVec,
    ReadOnlyClone, ReadableVec, Ro, WritableVec,
};

static CACHE: vecdb::CacheBudget = vecdb::CacheBudget::new(1024 * 1024);

fn seed_legacy_axis<C: ColumnId>(db: &Database, name: &str, version: Version) -> (String, Version) {
    let mut source = EagerVec::<ColumnarVec<PcoVec<Height, StoredU64>, C>>::forced_import(
        db,
        name,
        version + Version::ONE,
    )
    .unwrap();
    source.push(C::from_fn(|id| StoredU64::from(id.index() as u64 + 17)));
    source.write().unwrap();
    (source.name().to_owned(), source.version())
}

#[test]
fn composed_axes_reopen_existing_storage_without_renaming_or_resetting() {
    let dir = tempfile::tempdir().unwrap();
    let db = Database::open(dir.path()).unwrap();
    let version = Version::new(31);
    let legacy = vec![
        seed_legacy_axis::<AgeRangeId>(&db, "utxos_existing_by_age_range", version),
        seed_legacy_axis::<EpochId>(&db, "existing_by_epoch", version),
        seed_legacy_axis::<ClassId>(&db, "existing_by_class", version),
        seed_legacy_axis::<EntryId>(&db, "existing_by_entry", version),
        seed_legacy_axis::<SpendableTypeId>(&db, "existing_by_type", version),
        seed_legacy_axis::<AmountRangeId>(&db, "utxos_existing_by_amount_range", version),
        seed_legacy_axis::<UTXOAggregateId>(&db, "existing_by_aggregate", version),
        seed_legacy_axis::<UnderAgeId>(&db, "utxos_existing_by_under_age", version),
        seed_legacy_axis::<OverAgeId>(&db, "utxos_existing_by_over_age", version),
        seed_legacy_axis::<UnderAmountId>(&db, "utxos_existing_by_under_amount", version),
        seed_legacy_axis::<OverAmountId>(&db, "utxos_existing_by_over_amount", version),
    ];
    let mut columns =
        ExactUTXOColumns::<StoredU64>::forced_import(&db, "existing", version).unwrap();
    assert_eq!(columns.min_len(), 1);
    let identity: Vec<_> = columns
        .collect_vecs_mut()
        .iter()
        .map(|v| {
            assert_eq!(v.len(), 1);
            (v.name().to_owned(), v.version())
        })
        .collect();
    assert_eq!(identity, legacy);
    let row = columns.direct.collect_last().unwrap();
    for &id in AgeRangeId::ALL {
        assert_eq!(
            id.select(&row.age_range),
            &StoredU64::from(id.index() as u64 + 17)
        );
    }
    assert_eq!(
        columns
            .overlapping
            .aggregate
            .height
            .collect_last()
            .unwrap()
            .all,
        StoredU64::from(17_u64)
    );

    let TreeNode::Branch(root) = columns.to_tree_node() else {
        panic!("expected columns")
    };
    assert_eq!(
        root.keys().map(String::as_str).collect::<Vec<_>>(),
        [
            "age_range",
            "epoch",
            "class",
            "entry",
            "type",
            "amount_range",
            "overlapping"
        ]
    );
    let TreeNode::Branch(age) = &root["age_range"] else {
        panic!("expected axis")
    };
    assert_eq!(
        age.keys().map(String::as_str).collect::<Vec<_>>(),
        ["height"]
    );
    let TreeNode::Branch(overlap) = &root["overlapping"] else {
        panic!("expected overlapping axes")
    };
    assert_eq!(
        overlap.keys().map(String::as_str).collect::<Vec<_>>(),
        [
            "aggregate",
            "under_age",
            "over_age",
            "under_amount",
            "over_amount"
        ]
    );
}

#[test]
fn amount_composition_keeps_checkpoint_invalidation_and_reader_projection() {
    let dir = tempfile::tempdir().unwrap();
    let db = Database::open(dir.path()).unwrap();
    let mut amounts = ColumnarAmount::<StoredU64, ()>::forced_import(
        &CACHE,
        &db,
        "amount_source",
        CohortContext::Addr,
        "amount",
        Version::ONE,
        |_, _| (),
    )
    .unwrap();
    let row = |value| AmountRange::from_fn(|_| StoredU64::from(value));
    for value in [2_u64, 3] {
        amounts.push_cumulative(&row(value));
    }
    amounts.stored_mut().write().unwrap();
    amounts.stored_mut().any_truncate_if_needed_at(1).unwrap();
    amounts.push_cumulative(&row(10));
    amounts.push_cumulative(&row(1));
    amounts.stored_mut().write().unwrap();
    assert!(
        amounts
            .height
            .collect_last()
            .unwrap()
            .iter()
            .all(|value| *value == StoredU64::from(13_u64))
    );
    let reader = amounts.read_only_clone();
    assert_eq!(reader.height.len(), 3);
    assert_eq!(
        size_of::<ColumnarAmount<StoredU64, (), Ro>>(),
        size_of::<ColumnarPerBlock<StoredU64, AmountRangeId, bitview_cohort::Amount<()>, Ro>>()
    );
    let TreeNode::Branch(root) = reader.to_tree_node() else {
        panic!("expected amount views")
    };
    assert_eq!(
        root.keys().map(String::as_str).collect::<Vec<_>>(),
        ["range", "under", "over", "height"]
    );
}
