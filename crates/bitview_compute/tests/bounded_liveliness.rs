mod common;

use bitview_compute::{BoundedOddsF64, BoundedToF64, LazyPerBlock, PerBlock};
use brk_exit::Exit;
use brk_types::{BoundedRatio, Height, StoredF64, Version};
use vecdb::{AnyStoredVec, AnyVec, Database, ReadableVec};

use common::{indexes, stored};

#[test]
fn bounded_liveliness_storage_and_complement_views() {
    let directory = tempfile::tempdir().unwrap();
    let db = Database::open(directory.path()).unwrap();
    let indexes = indexes(&db);
    let destroyed = stored(
        &db,
        "destroyed",
        [0.0, 1.0, 1.0, 3.0, 0.0].map(StoredF64::from),
    );
    let created = stored(
        &db,
        "created",
        [1.0, 3.0, 1.0, 4.0, 0.0].map(StoredF64::from),
    );
    let mut source = PerBlock::<BoundedRatio>::forced_import(
        &db,
        "liveliness_bounded_source",
        Version::ONE,
        &indexes,
    )
    .unwrap();
    source
        .height
        .compute_transform2(
            Height::ZERO,
            &destroyed,
            &created,
            |(h, d, c, ..)| (h, BoundedRatio::from(f64::from(d) / f64::from(c))),
            &Exit::default(),
        )
        .unwrap();
    source.height.write().unwrap();
    let liveliness =
        LazyPerBlock::from_resolutions::<BoundedToF64>("liveliness", Version::ONE, &source);
    let vaultedness =
        LazyPerBlock::from_resolutions::<BoundedToF64<true>>("vaultedness", Version::ONE, &source);
    let odds = LazyPerBlock::from_resolutions::<BoundedOddsF64>(
        "activity_to_vaultedness",
        Version::ONE,
        &source,
    );
    assert_eq!(liveliness.height.name(), "liveliness");
    assert_eq!(vaultedness.height.name(), "vaultedness");
    for (i, exact) in [0.0, 1.0 / 3.0, 1.0, 0.75].into_iter().enumerate() {
        let raw = source.height.collect_one_at(i).unwrap();
        let live = f64::from(liveliness.height.collect_one_at(i).unwrap());
        let vault = f64::from(vaultedness.height.collect_one_at(i).unwrap());
        assert_eq!(raw.inner() + raw.complement().inner(), BoundedRatio::SCALE);
        assert_eq!(vault, f64::from(raw.complement()));
        assert!(live <= exact && exact - live < 1.0 / f64::from(BoundedRatio::SCALE));
        assert_eq!(
            f64::from(odds.height.collect_one_at(i).unwrap()),
            raw.inner() as f64 / raw.complement().inner() as f64
        );
    }
    assert!(liveliness.height.collect_one_at(4).unwrap().is_nan());
    assert!(vaultedness.height.collect_one_at(4).unwrap().is_nan());
    assert!(odds.height.collect_one_at(4).unwrap().is_nan());
    assert_eq!(
        destroyed.collect_range_at(0, 5),
        [0.0, 1.0, 1.0, 3.0, 0.0].map(StoredF64::from)
    );
    let expected = source.height.collect_range_at(0, 5);
    drop(odds);
    drop(vaultedness);
    drop(liveliness);
    drop(source);
    let reopened = PerBlock::<BoundedRatio>::forced_import(
        &db,
        "liveliness_bounded_source",
        Version::ONE,
        &indexes,
    )
    .unwrap();
    assert_eq!(reopened.height.collect_range_at(0, 5), expected);
}
