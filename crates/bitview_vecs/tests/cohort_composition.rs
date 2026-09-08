use bitview_cohort::{UTXOCoreRows, UTXORows};
use bitview_vecs::{
    CumulativeUTXOColumnarMetric, CumulativeUTXOColumnarMetricWithoutAmountOrType,
    UTXOColumnarMetric, UTXOColumnarMetricWithoutAmount, UTXOColumnarMetricWithoutAmountOrType,
};
use brk_types::{StoredU64, Version};
use vecdb::{AnyVec, Database, ReadOnlyClone, ReadableVec, Ro};

#[test]
fn cohort_extensions_preserve_core_storage_identity_and_order() {
    let directories = [
        tempfile::tempdir().unwrap(),
        tempfile::tempdir().unwrap(),
        tempfile::tempdir().unwrap(),
    ];
    let databases = directories
        .each_ref()
        .map(|dir| Database::open(dir.path()).unwrap());
    let version = Version::new(17);
    let mut core = UTXOColumnarMetricWithoutAmountOrType::<StoredU64>::forced_import(
        &databases[0],
        "metric",
        version,
    )
    .unwrap();
    let mut typed = UTXOColumnarMetricWithoutAmount::<StoredU64>::forced_import(
        &databases[1],
        "metric",
        version,
    )
    .unwrap();
    let mut full =
        UTXOColumnarMetric::<StoredU64>::forced_import(&databases[2], "metric", version).unwrap();
    let core_identity: Vec<_> = core
        .collect_vecs_mut()
        .iter()
        .map(|v| (v.name().to_owned(), v.version()))
        .collect();
    let typed_identity: Vec<_> = typed
        .collect_vecs_mut()
        .iter()
        .map(|v| (v.name().to_owned(), v.version()))
        .collect();
    let full_identity: Vec<_> = full
        .collect_vecs_mut()
        .iter()
        .map(|v| (v.name().to_owned(), v.version()))
        .collect();
    assert_eq!(core_identity, typed_identity[..4]);
    assert_eq!(typed_identity, full_identity[..5]);
    assert_eq!(
        full_identity
            .iter()
            .map(|(name, _)| name.as_str())
            .collect::<Vec<_>>(),
        [
            "utxos_metric_by_age_range",
            "metric_by_epoch",
            "metric_by_class",
            "metric_by_entry",
            "metric_by_type",
            "utxos_metric_by_amount_range",
        ]
    );
}

#[test]
fn reader_projection_omits_cumulative_rows_and_reduced_writers_use_only_core_axes() {
    assert_eq!(
        size_of::<CumulativeUTXOColumnarMetric<StoredU64, Ro>>(),
        size_of::<UTXOColumnarMetric<StoredU64, Ro>>()
    );
    assert_eq!(
        size_of::<CumulativeUTXOColumnarMetricWithoutAmountOrType<StoredU64, Ro>>(),
        size_of::<UTXOColumnarMetricWithoutAmountOrType<StoredU64, Ro>>()
    );
    assert!(size_of::<UTXOCoreRows<StoredU64>>() < size_of::<UTXORows<StoredU64>>());

    let dir = tempfile::tempdir().unwrap();
    let db = Database::open(dir.path()).unwrap();
    let mut full =
        CumulativeUTXOColumnarMetric::<StoredU64>::forced_import(&db, "full", Version::ONE)
            .unwrap();
    let mut core = CumulativeUTXOColumnarMetricWithoutAmountOrType::<StoredU64>::forced_import(
        &db,
        "core",
        Version::ONE,
    )
    .unwrap();
    for delta in [1_u64, 2, 4] {
        let rows = UTXORows::default().map(|_: &StoredU64| StoredU64::from(delta));
        core.push_block(rows.core.clone());
        full.push_block(rows);
    }
    for v in full
        .collect_vecs_mut()
        .into_iter()
        .chain(core.collect_vecs_mut())
    {
        v.write().unwrap();
    }
    let reader = full.read_only_clone();
    assert_eq!(reader.matrices.age_range_matrix.len(), 3);
    assert_eq!(
        full.matrices
            .collect_last()
            .unwrap()
            .age_range
            .iter()
            .next()
            .copied(),
        Some(StoredU64::from(7_u64))
    );

    // Exposing mutable storage invalidates the warm writer checkpoint, including
    // a truncate-and-rewrite that returns to the original length.
    for v in full
        .collect_vecs_mut()
        .into_iter()
        .chain(core.collect_vecs_mut())
    {
        v.any_truncate_if_needed_at(2).unwrap();
    }
    let replacement = UTXORows::default().map(|_: &StoredU64| StoredU64::from(10_u64));
    core.push_block(replacement.core.clone());
    full.push_block(replacement);
    for v in full
        .collect_vecs_mut()
        .into_iter()
        .chain(core.collect_vecs_mut())
    {
        v.write().unwrap();
    }
    assert_eq!(
        full.matrices
            .collect_last()
            .unwrap()
            .age_range
            .iter()
            .next()
            .copied(),
        Some(StoredU64::from(13_u64))
    );
    assert_eq!(
        core.matrices
            .collect_last()
            .unwrap()
            .age_range
            .iter()
            .next()
            .copied(),
        Some(StoredU64::from(13_u64))
    );
    assert_eq!(
        reader
            .matrices
            .age_range_matrix
            .collect_last()
            .unwrap()
            .iter()
            .next()
            .copied(),
        Some(StoredU64::from(13_u64))
    );
}

#[test]
fn exact_aggregates_remain_stored_values_not_additive_reconstructions() {
    use bitview_cohort::{Filter, UTXOAggregateRows};
    use bitview_vecs::ExactUTXOColumnarMetric;
    static CACHE: vecdb::CacheBudget = vecdb::CacheBudget::new(1024 * 1024);
    let dir = tempfile::tempdir().unwrap();
    let db = Database::open(dir.path()).unwrap();
    let mut metric =
        ExactUTXOColumnarMetric::<StoredU64>::forced_import(&db, "exact", Version::ONE).unwrap();
    let rows = UTXORows::default().map(|_: &StoredU64| StoredU64::from(1_u64));
    let aggregates = UTXOAggregateRows::default().map(|_: &StoredU64| StoredU64::from(999_u64));
    metric.push(rows, aggregates);
    for v in metric.collect_vecs_mut() {
        v.write().unwrap();
    }
    let exact = metric
        .source(&CACHE, &Filter::All, "all_exact", Version::ONE)
        .unwrap();
    let additive = metric
        .direct
        .additive_source(&CACHE, &Filter::All, "all_additive", Version::ONE)
        .unwrap();
    assert_eq!(exact.collect_one_at(0), Some(StoredU64::from(999_u64)));
    assert_eq!(
        additive.collect_one_at(0),
        Some(StoredU64::from(bitview_cohort::AGE_RANGE_COUNT as u64))
    );
}
