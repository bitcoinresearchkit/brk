mod common;

use bitview_collections::WindowId;
use bitview_traversable::{Traversable, TreeNode};
use bitview_vecs::{
    ColumnarPerBlock, IndexSources, LazyColumnPriceWithRatioPerBlock, LazyIndexedVec,
    LazyPriceWithRatioPerBlock, LazyRatioPerBlock, PriceWithRatioPerBlock,
};
use brk_types::{Cents, Height, PriceRatio, Version};
use common::{indexes, stored};
use vecdb::{
    AnySerializableVec, AnyStoredVec, AnyVec, CachedBoxedVec, CachedVec, ColumnId, Database,
    ReadOnlyClone, ReadableBoxedVec, ReadableCloneableVec, ReadableVec, WritableVec,
};

use crate::common::CACHE_BUDGET;

fn original_ratio(
    name: &str,
    version: Version,
    price: ReadableBoxedVec<Height, Cents>,
    spot: &CachedBoxedVec<Height, Cents>,
    indexes: &IndexSources,
) -> LazyRatioPerBlock<PriceRatio> {
    let version = version + Version::new(5);
    let source = LazyIndexedVec::new(
        &format!("{name}_ratio_ppm_source"),
        version,
        &price,
        spot,
        |_, price, spot| {
            if price == Cents::ZERO {
                PriceRatio::NAN
            } else {
                PriceRatio::from(f64::from(spot) / f64::from(price))
            }
        },
    );
    LazyRatioPerBlock::from_height_source(
        &format!("{name}_ratio"),
        version,
        &CACHE_BUDGET.wrap(source),
        indexes,
    )
}

#[test]
fn cached_price_constructors_preserve_ratio_sources_names_and_versions() {
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
        &crate::common::CACHE_BUDGET,
        &db,
        "imported",
        version,
        &indexes,
        &spot,
    )
    .unwrap();
    for price in prices {
        imported.cents.height.push(price);
    }
    imported.cents.height.write().unwrap();
    let lazy = LazyPriceWithRatioPerBlock::from_height_source(
        "lazy",
        version,
        &imported.cents.height,
        &indexes,
        &spot,
    );
    let direct = LazyPriceWithRatioPerBlock::from_height_source(
        "direct",
        version,
        &imported.cents.height,
        &indexes,
        &spot,
    );
    let mut columns =
        ColumnarPerBlock::<Cents, WindowId, ()>::forced_import(&db, "matrix", version, |_| ())
            .unwrap();
    for price in prices {
        columns.push(WindowId::from_fn(|column| {
            if column == WindowId::Week1 {
                price
            } else {
                Cents::from(999u64)
            }
        }));
    }
    columns.write().unwrap();
    let columnar = LazyColumnPriceWithRatioPerBlock::new(
        &crate::common::CACHE_BUDGET,
        "columnar",
        version,
        &columns.height.read_only_clone(),
        WindowId::Week1,
        &indexes,
        &spot,
    );
    macro_rules! check {
        ($view:ident, $name:literal) => {{
            let expected = original_ratio(
                $name,
                version,
                $view.cents.height.read_only_boxed_clone(),
                &spot,
                &indexes,
            );
            assert!(
                $view
                    .relative
                    .ppm
                    .height
                    .collect_one_at(0)
                    .unwrap()
                    .is_nan()
            );
            let TreeNode::Leaf(leaf) = $view.relative.ppm.to_tree_node() else {
                panic!("expected ratio leaf");
            };
            assert_eq!(leaf.kind(), "PriceRatio");
            let mut json = Vec::new();
            $view
                .relative
                .ppm
                .height
                .write_json(Some(4), Some(6), &mut json)
                .unwrap();
            assert_eq!(json, b"[4294967294,null]");
            assert_eq!(
                $view.relative.ppm.height.collect_one_at(4),
                Some(PriceRatio::MAX)
            );
            assert!(
                $view
                    .relative
                    .ppm
                    .height
                    .collect_one_at(5)
                    .unwrap()
                    .is_nan()
            );
            assert_eq!($view.cents.height.collect_one_at(4), Some(Cents::new(1)));
            assert_eq!(
                f32::from($view.relative.ratio.height.collect_one_at(4).unwrap()),
                f32::from(PriceRatio::MAX)
            );
            assert!(f32::from($view.relative.ratio.height.collect_one_at(5).unwrap()).is_nan());
            assert_eq!(
                $view.relative.ppm.height.collect_range_at(1, 4),
                [4.0, 3.0, 2.0].map(PriceRatio::from)
            );
            assert_eq!($view.relative.ppm.height.name(), expected.ppm.height.name());
            assert_eq!(
                $view.relative.ppm.height.version(),
                expected.ppm.height.version()
            );
            assert_eq!(
                $view.relative.ppm.day1.collect(),
                expected.ppm.day1.collect()
            );
            assert_eq!(
                $view.relative.ratio.height.collect_range_at(1, 4),
                expected.ratio.height.collect_range_at(1, 4)
            );
            assert_eq!(
                $view.relative.ratio.height.name(),
                expected.ratio.height.name()
            );
            assert_eq!(
                $view.relative.ratio.height.version(),
                expected.ratio.height.version()
            );
        }};
    }
    check!(imported, "imported");
    check!(lazy, "lazy");
    check!(direct, "direct");
    check!(columnar, "columnar");
}
