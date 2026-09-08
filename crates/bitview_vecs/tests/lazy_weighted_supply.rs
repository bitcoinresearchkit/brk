mod common;

use crate::common::CACHE_BUDGET;
use bitview_cohort::{AgeRange, AgeRangeId};
use bitview_compute::WeightedCohortState;
use bitview_vecs::{ColumnarPerBlock, LazySpotValuePerBlock};
use brk_types::{BoundedRatio, Cents, Day1, Height, Sats, Version};
use tempfile::tempdir;
use vecdb::{
    AnyStoredVec, CachedColumnarVec, CachedReadableVec, CachedVec, Database, ImportableVec, PcoVec,
    ReadOnlyClone, ReadableCloneableVec, ReadableColumnarVec, ReadableVec, WritableVec,
};

#[test]
fn lazy_sides_preserve_stored_rounding_and_follow_source_rewrites() {
    let directory = tempdir().unwrap();
    let db = Database::open(directory.path()).unwrap();
    let mut indexes = common::indexes(&db);
    indexes.first_height.day1 = CachedVec::wrap(common::stored::<Day1, _>(
        &db,
        "daily_first_height",
        [0usize, 2, 4].map(Height::from),
    ))
    .read_only_boxed_clone();
    let mut supply = ColumnarPerBlock::<Sats, AgeRangeId, ()>::forced_import(
        &db,
        "supply",
        Version::ONE,
        |_| (),
    )
    .unwrap();
    let mut weights = ColumnarPerBlock::<BoundedRatio, AgeRangeId, ()>::forced_import(
        &db,
        "weights",
        Version::ONE,
        |_| (),
    )
    .unwrap();
    let mut spot = PcoVec::<Height, Cents>::forced_import(&db, "spot", Version::ONE).unwrap();
    let inputs = [
        (Sats::from(123_456_789_u64), BoundedRatio::from(0.321)),
        (Sats::from(1_u64), BoundedRatio::from(0.5)),
        (Sats::from(99_u64), BoundedRatio::ONE),
        (Sats::from(99_u64), BoundedRatio::ZERO),
        (Sats::from(99_u64), BoundedRatio::NAN),
    ];
    for (s, w) in inputs {
        supply.push(AgeRange::from_fn(|_| s));
        weights.push(AgeRange::from_fn(|_| w));
        spot.push(Cents::from(1_000_000_u64));
    }
    supply.write().unwrap();
    weights.write().unwrap();
    spot.write().unwrap();
    let supply_cache = CachedColumnarVec::new(
        supply.height.read_only_clone(),
        Version::ONE,
        CachedVec::wrap,
    );
    let weight_cache = CachedColumnarVec::new(
        weights.height.read_only_clone(),
        Version::ONE,
        CachedVec::wrap,
    );
    let spot = CachedVec::wrap(spot.read_only_clone()).cached_boxed_clone();
    for &id in AgeRangeId::ALL {
        let raw = supply_cache
            .column("supply", Version::ONE, id)
            .read_only_boxed_clone();
        let weight = weight_cache.cached_column(id).cached_boxed_clone();
        let weighted = LazySpotValuePerBlock::from_weighted_supply::<false>(
            "awake_supply",
            Version::ONE,
            &raw,
            &weight,
            &indexes,
            &spot,
        );
        let complement = LazySpotValuePerBlock::from_weighted_supply::<true>(
            "dormant_supply",
            Version::ONE,
            &raw,
            &weight,
            &indexes,
            &spot,
        );
        for (height, (s, w)) in inputs.iter().copied().enumerate() {
            let expected = WeightedCohortState::split_supply(s, w);
            assert_eq!(
                weighted.sats.height.collect_one_at(height),
                Some(expected.0)
            );
            assert_eq!(
                complement.sats.height.collect_one_at(height),
                Some(expected.1)
            );
        }
        // Both floors are zero: subtraction would incorrectly return one sat.
        assert_eq!(weighted.sats.height.collect_one_at(1), Some(Sats::ZERO));
        assert_eq!(complement.sats.height.collect_one_at(1), Some(Sats::ZERO));
        // Stock resolutions select the final block, including the partial day.
        for (day, height) in [1usize, 3, 4].into_iter().enumerate() {
            let expected = WeightedCohortState::split_supply(inputs[height].0, inputs[height].1);
            assert_eq!(
                weighted.sats.day1.collect_one_at(day),
                Some(Some(expected.0))
            );
            assert_eq!(
                complement.sats.day1.collect_one_at(day),
                Some(Some(expected.1))
            );
        }
        supply_cache.invalidate();
        weight_cache.invalidate();
        // Production invalidates the shared budget before rewrites, including
        // derived resolution caches, not only the raw age inputs.
        CACHE_BUDGET.invalidate();
        supply.truncate_if_needed_at(4).unwrap();
        weights.truncate_if_needed_at(4).unwrap();
        supply.push(AgeRange::from_fn(|_| Sats::from(101_u64)));
        weights.push(AgeRange::from_fn(|_| BoundedRatio::ONE));
        supply.write().unwrap();
        weights.write().unwrap();
        assert_eq!(
            weighted.sats.height.collect_one_at(4),
            Some(Sats::from(101_u64))
        );
        assert_eq!(complement.sats.height.collect_one_at(4), Some(Sats::ZERO));
        assert_eq!(
            weighted.sats.day1.collect_one_at(2),
            Some(Some(Sats::from(101_u64)))
        );
        assert_eq!(
            complement.sats.day1.collect_one_at(2),
            Some(Some(Sats::ZERO))
        );
        // Restore before validating the next age column.
        supply_cache.invalidate();
        weight_cache.invalidate();
        CACHE_BUDGET.invalidate();
        supply.truncate_if_needed_at(4).unwrap();
        weights.truncate_if_needed_at(4).unwrap();
        supply.push(AgeRange::from_fn(|_| inputs[4].0));
        weights.push(AgeRange::from_fn(|_| inputs[4].1));
        supply.write().unwrap();
        weights.write().unwrap();
    }
}
