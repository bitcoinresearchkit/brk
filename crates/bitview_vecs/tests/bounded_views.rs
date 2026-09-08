mod common;

use bitview_transforms::BoundedToF64;
use bitview_vecs::{DailyMappings, LazyDailyMetric, LazyPerBlock};
use brk_types::{BoundedRatio, Day1, Height, Version};
use common::{indexes, stored};
use vecdb::{AnySerializableVec, Database, ReadableCloneableVec, ReadableVec};

use crate::common::CACHE_BUDGET;

#[test]
fn bounded_sources_keep_decimal_block_and_daily_views() {
    let directory = tempfile::tempdir().unwrap();
    let db = Database::open(directory.path()).unwrap();
    let mut indexes = indexes(&db);
    let values = [BoundedRatio::ZERO, BoundedRatio::ONE, BoundedRatio::NAN];
    let blocks = CACHE_BUDGET.wrap(stored::<Height, _>(&db, "loss_share_bounded", values));
    let block_view = LazyPerBlock::from_height_source::<BoundedToF64>(
        "loss_share",
        Version::ONE,
        &blocks,
        &indexes,
    );
    let mut json = Vec::new();
    block_view
        .height
        .write_json(Some(0), Some(3), &mut json)
        .unwrap();
    assert_eq!(json, b"[0.0,1.0,null]");

    indexes.height_day1 =
        stored(&db, "height_days", [0usize, 1, 2].map(Day1::from)).read_only_boxed_clone();
    let daily = stored::<Day1, _>(&db, "threshold_bounded", values);
    let daily_view = LazyDailyMetric::from_source::<BoundedToF64>(
        "threshold",
        Version::ONE,
        daily.read_only_boxed_clone(),
        &DailyMappings::new(&indexes),
    );
    assert_eq!(f64::from(daily_view.day1.collect_one_at(1).unwrap()), 1.0);
    assert!(daily_view.day1.collect_one_at(2).unwrap().is_nan());
    assert_eq!(
        f64::from(daily_view.views.height.collect_one_at(1).unwrap().unwrap()),
        1.0
    );
}
