use std::{hint::black_box, time::Instant};

use bitview_transforms::price_ratio;
use bitview_vecs::{DailyMappings, LazyDailyPriceWithRatio, RangeMapLookupVec};
use brk_types::{Cents, Date, Day1, Height, Timestamp, Version};
use tempfile::tempdir;
use vecdb::{AnySerializableVec, Database, LazyVec, ReadableCloneableVec, ReadableVec};

use crate::common;

/// Exercises the complete daily ratio view and JSON writer, without HTTP.
#[test]
#[ignore = "manual full-history daily price ratio benchmark"]
fn full_history_daily_price_ratio() {
    const BLOCKS: usize = 966_505;
    const DAYS: usize = 6_463;
    let directory = tempdir().unwrap();
    let db = Database::open(directory.path()).unwrap();
    let epoch = *Timestamp::from(Date::from(Day1::from(0usize)));
    let timestamp = |height: usize| {
        Timestamp::new(
            epoch + (height as u64 * ((DAYS - 1) * 86_400) as u64 / (BLOCKS - 1) as u64) as u32,
        )
    };
    let timestamps =
        common::stored::<Height, _>(&db, "benchmark_timestamps", (0..BLOCKS).map(timestamp));
    let height_days = LazyVec::init(
        "day1",
        Version::ZERO,
        timestamps.read_only_boxed_clone(),
        |_, timestamp| Day1::try_from(Date::from(timestamp)).unwrap(),
    );
    let mut boundaries = Vec::new();
    for (height, day) in height_days.collect().into_iter().enumerate() {
        boundaries.resize(usize::from(day) + 1, Height::from(height));
    }
    assert_eq!(boundaries.len(), DAYS);
    let mut indexes = common::indexes(&db);
    indexes.first_height.day1 = common::first_heights("first_height", boundaries.clone());
    indexes.height_day1 = RangeMapLookupVec::new(indexes.first_height.day1.mapping(), &height_days)
        .read_only_boxed_clone();
    let daily_price = |day: usize| Cents::new(5_000_000 + day as u64 * 100);
    let spot_price = |height: usize| Cents::new(6_000_000 + height as u64 * 5);
    let prices = common::stored::<Day1, _>(&db, "benchmark_prices", (0..DAYS).map(daily_price));
    let spot = common::stored::<Height, _>(&db, "benchmark_spot", (0..BLOCKS).map(spot_price));
    let mappings = DailyMappings::new(&indexes);
    let view = LazyDailyPriceWithRatio::from_day1_source(
        "benchmark_price",
        Version::ONE,
        &prices,
        &indexes,
        &mappings,
        &spot,
    );
    let expected: Vec<_> = (0..DAYS)
        .map(|day| {
            let end = boundaries.get(day + 1).map_or(BLOCKS, |h| usize::from(*h));
            Some(price_ratio(spot_price(end - 1), daily_price(day)))
        })
        .collect();
    assert_eq!(view.relative.ppm.day1.collect(), expected);
    let mut expected_json = Vec::new();
    view.relative
        .ppm
        .day1
        .write_json(None, None, &mut expected_json)
        .unwrap();
    let mut samples = Vec::new();
    for _ in 0..30 {
        let mut json = Vec::new();
        let start = Instant::now();
        view.relative
            .ppm
            .day1
            .write_json(None, None, black_box(&mut json))
            .unwrap();
        samples.push(start.elapsed().as_secs_f64() * 1e3);
        assert_eq!(json, expected_json);
    }
    samples.sort_by(f64::total_cmp);
    println!(
        "synthetic {BLOCKS} heights / {DAYS} days; full ratio + JSON, 30 warmed calls: mean {:.3} ms, median {:.3} ms, min {:.3} ms, max {:.3} ms",
        samples.iter().sum::<f64>() / samples.len() as f64,
        (samples[14] + samples[15]) / 2.0,
        samples[0],
        samples[29],
    );
}
