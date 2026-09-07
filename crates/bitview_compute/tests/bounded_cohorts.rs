mod common;

use bitview_cohort::{AgeRange, AgeRangeId};
use bitview_compute::{BoundedToF64, ColumnarPerBlock, LazyColumnPerBlock, LazyPerBlock};
use brk_types::{BoundedRatio, Version};
use vecdb::{ColumnId, Database, ReadOnlyClone, ReadableCloneableVec, ReadableVec};

use common::indexes;

#[test]
fn bounded_age_columns_preserve_endpoints_views_and_reopen() {
    let directory = tempfile::tempdir().unwrap();
    let db = Database::open(directory.path()).unwrap();
    let indexes = indexes(&db);
    let mut source = ColumnarPerBlock::<BoundedRatio, AgeRangeId, ()>::forced_import(
        &db,
        "age_bounded",
        Version::ONE,
        |_| (),
    )
    .unwrap();
    let values = AgeRange::from_fn(|id| {
        BoundedRatio::from(id.index() as f64 / AgeRangeId::ALL.len() as f64)
    });
    for row in [
        values.clone(),
        AgeRange::from_fn(|_| BoundedRatio::ONE),
        AgeRange::from_fn(|_| BoundedRatio::NAN),
    ] {
        source.push(row);
    }
    source.write().unwrap();
    let columns = source.height.read_only_clone();
    for &id in AgeRangeId::ALL {
        let column = LazyColumnPerBlock::new("bounded", Version::ONE, &columns, id, &indexes);
        let view = LazyPerBlock::from_resolutions::<BoundedToF64>(
            "weight",
            Version::ONE,
            column.height.read_only_boxed_clone(),
            &column.resolutions,
        );
        let complement = LazyPerBlock::from_resolutions::<BoundedToF64<true>>(
            "complement",
            Version::ONE,
            column.height.read_only_boxed_clone(),
            &column.resolutions,
        );
        let raw = *id.select(&values);
        assert_eq!(
            f64::from(view.height.collect_one_at(0).unwrap()),
            f64::from(raw)
        );
        assert_eq!(
            f64::from(complement.height.collect_one_at(0).unwrap()),
            f64::from(raw.complement())
        );
        assert_eq!(f64::from(view.height.collect_one_at(1).unwrap()), 1.0);
        assert_eq!(f64::from(complement.height.collect_one_at(1).unwrap()), 0.0);
        assert!(view.height.collect_one_at(2).unwrap().is_nan());
        assert!(complement.height.collect_one_at(2).unwrap().is_nan());
    }
    drop(columns);
    drop(source);
    let reopened = ColumnarPerBlock::<BoundedRatio, AgeRangeId, ()>::forced_import(
        &db,
        "age_bounded",
        Version::ONE,
        |_| (),
    )
    .unwrap();
    let reopened = reopened.height.collect_one_at(0).unwrap();
    for &id in AgeRangeId::ALL {
        assert_eq!(id.select(&reopened), id.select(&values));
    }
}
