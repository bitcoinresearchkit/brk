mod common;

use bitview_collections::WindowId;
use bitview_vecs::{
    ColumnarPerBlock, LazyColumnPriceWithRatioPerBlock, LazyPriceWithRatioPerBlock,
    PriceWithRatioPerBlock,
};
use brk_types::{Cents, Height, PriceRatio, Version};
use common::{CACHE_BUDGET, indexes, stored};
use vecdb::{
    AnySerializableVec, AnyStoredVec, CachedVec, ColumnId, Database, ReadOnlyClone, ReadableVec,
    WritableVec,
};

#[test]
fn price_ratios_preserve_zero_nan_saturation_and_empty_days() {
    let directory = tempfile::tempdir().unwrap();
    let db = Database::open(directory.path()).unwrap();
    let mut indexes = indexes(&db);
    indexes.first_height.day1 = CachedVec::wrap(stored(
        &db,
        "ratio_days",
        [0usize, 1, 1, 3].map(Height::from),
    ))
    .read_only_boxed_clone();
    let spot = CachedVec::wrap(stored::<Height, _>(
        &db,
        "spot",
        [100u64, 200, 300, 400, 1_000_000, 400].map(Cents::from),
    ))
    .read_only_cached_boxed_clone();
    let version = Version::new(3);
    let prices = [
        Cents::ZERO,
        Cents::new(50),
        Cents::new(100),
        Cents::new(200),
        Cents::new(1),
        Cents::NAN,
    ];
    let mut imported = PriceWithRatioPerBlock::forced_import(
        &CACHE_BUDGET,
        &db,
        "imported",
        version,
        &indexes,
        &spot,
    )
    .unwrap();
    let mut columns =
        ColumnarPerBlock::<Cents, WindowId, ()>::forced_import(&db, "columns", version, |_| ())
            .unwrap();
    for price in prices {
        imported.cents.height.push(price);
        columns.push(WindowId::from_fn(|column| {
            if column == WindowId::Week1 {
                price
            } else {
                Cents::new(999)
            }
        }));
    }
    imported.cents.height.write().unwrap();
    columns.write().unwrap();
    let lazy = LazyPriceWithRatioPerBlock::from_height_source(
        "lazy",
        version,
        &imported.cents.height,
        &indexes,
        &spot,
    );
    let columnar = LazyColumnPriceWithRatioPerBlock::new(
        &CACHE_BUDGET,
        "columnar",
        version,
        &columns.height.read_only_clone(),
        WindowId::Week1,
        &indexes,
        &spot,
    );
    macro_rules! check {
        ($view:ident) => {{
            let ppm = &$view.relative.ppm;
            assert_eq!(
                ppm.height.collect(),
                [
                    PriceRatio::NAN,
                    PriceRatio::from(4.0),
                    PriceRatio::from(3.0),
                    PriceRatio::from(2.0),
                    PriceRatio::MAX,
                    PriceRatio::NAN,
                ]
            );
            assert_eq!(
                ppm.day1.collect(),
                [
                    Some(PriceRatio::NAN),
                    None,
                    Some(PriceRatio::from(3.0)),
                    Some(PriceRatio::NAN),
                ]
            );
            let mut json = Vec::new();
            ppm.height.write_json(Some(4), Some(6), &mut json).unwrap();
            assert_eq!(json, b"[4294967294,null]");
            assert_eq!(
                f32::from($view.relative.ratio.height.collect_one_at(4).unwrap()),
                f32::from(PriceRatio::MAX)
            );
            assert!(
                $view
                    .relative
                    .ratio
                    .height
                    .collect_one_at(5)
                    .unwrap()
                    .is_nan()
            );
        }};
    }
    check!(imported);
    check!(lazy);
    check!(columnar);
}
