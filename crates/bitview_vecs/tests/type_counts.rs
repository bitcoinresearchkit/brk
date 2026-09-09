use bitview_cohort::{OutputTypeId, SpendableTypeId};
use bitview_collections::WindowId;
use bitview_vecs::{ColumnarPerBlock, CountTotal, OutputTypeCounts, SpendableTypeCounts};
use brk_types::{Height, PartsPerMillion32, StoredU16, StoredU64, Version};
use tempfile::tempdir;
use vecdb::{AnyStoredVec, CachedVec, ColumnId, Database, ReadOnlyClone, ReadableVec, WritableVec};

mod common;

#[test]
fn type_domains_share_the_engine_without_sharing_the_wrong_denominator() {
    let dir = tempdir().unwrap();
    let db = Database::open(dir.path()).unwrap();
    let indexes = common::indexes(&db);
    let starts = common::stored::<Height, _>(&db, "starts", [Height::ZERO; 3]);
    let windows = WindowId::series(|_| &starts);
    let mut totals = common::stored::<Height, _>(&db, "totals", [1_u64, 4, 8].map(StoredU64::from));
    let cached = CachedVec::wrap(totals.read_only_clone());
    let version = Version::new(11);
    let selected = SpendableTypeId::ALL[0].output_type();

    let mut inputs = ColumnarPerBlock::<StoredU16, SpendableTypeId, ()>::forced_import(
        &common::CACHE_BUDGET,
        &db,
        "inputs",
        version,
        |_| (),
    )
    .unwrap();
    let mut outputs = ColumnarPerBlock::<StoredU16, OutputTypeId, ()>::forced_import(
        &common::CACHE_BUDGET,
        &db,
        "outputs",
        version,
        |_| (),
    )
    .unwrap();
    for count in [0_u16, 1, 1] {
        inputs.push(SpendableTypeId::from_fn(|id| {
            StoredU16::new(if id.output_type() == selected {
                count
            } else {
                0
            })
        }));
        outputs.push(OutputTypeId::from_fn(|id| {
            StoredU16::new(if id.output_type() == selected {
                count
            } else {
                0
            })
        }));
    }
    inputs.write().unwrap();
    outputs.write().unwrap();
    let input = SpendableTypeCounts::from_columnar_count_source(
        CountTotal::from_transformed_source(
            "non_coinbase",
            version,
            &cached,
            |height, total| total - StoredU64::from(height.incremented()),
            &indexes,
            &windows,
        ),
        |name| format!("{name}_inputs"),
        version,
        &inputs.height.read_only_clone(),
        &indexes,
        &windows,
    );
    let output = OutputTypeCounts::from_columnar_count_source(
        CountTotal::from_source("all", version, &cached, &indexes, &windows),
        |name| format!("{name}_outputs"),
        version,
        &outputs.height.read_only_clone(),
        &indexes,
        &windows,
    );
    assert_eq!(input.by_type.iter().count(), 11);
    assert_eq!(output.by_type.iter().count(), 12);
    let input_shares = input.lazy_shares(
        version,
        |name| format!("{name}_input_share"),
        &windows,
        &indexes,
    );
    let output_shares = output.lazy_shares(
        version,
        |name| format!("{name}_output_share"),
        &windows,
        &indexes,
    );
    assert_eq!(
        input.all.cumulative.height.collect(),
        [0_u64, 2, 5].map(StoredU64::from)
    );
    assert_eq!(
        output.all.cumulative.height.collect(),
        [1_u64, 4, 8].map(StoredU64::from)
    );
    assert_eq!(
        input_shares
            .get(selected)
            .cumulative
            .ppm
            .height
            .collect_one_at(2),
        Some(PartsPerMillion32::from(0.4))
    );
    assert_eq!(
        output_shares
            .get(selected)
            .cumulative
            .ppm
            .height
            .collect_one_at(2),
        Some(PartsPerMillion32::from(0.25))
    );

    // Only the source owner needs to invalidate rewritten totals.
    totals.truncate_if_needed_at(2).unwrap();
    totals.push(StoredU64::from(10_u64));
    totals.write().unwrap();
    cached.invalidate();
    assert_eq!(
        input_shares
            .get(selected)
            .cumulative
            .ppm
            .height
            .collect_one_at(2),
        Some(PartsPerMillion32::from(2.0 / 7.0))
    );
    assert_eq!(
        output_shares
            .get(selected)
            .cumulative
            .ppm
            .height
            .collect_one_at(2),
        Some(PartsPerMillion32::from(0.2))
    );
}
