use std::{hint::black_box, time::Instant};

use bitview_vecs::RangeMapVec;
use brk_types::{
    Date, Day1, Day3, Height, Hour1, Hour4, Hour12, Minute10, Minute30, Timestamp, Version,
};
use rangeindex::SharedRangeMap;
use vecdb::{AnyVec, LazyVec, ReadableCloneableVec, ReadableVec, VecIndex, VecValue};

use super::ResolutionVecs;
use crate::height::Vecs as HeightMappings;

/// Measures mapping work only, not HTTP, disk decoding, or serialization.
#[test]
#[ignore = "manual full-height resident mapping benchmark"]
fn resident_day_mapping_costs() {
    const BLOCKS: usize = 966_486;
    const DAYS: usize = 6_463;
    const READS: usize = 10_000;
    const APPENDS: usize = 1_000;
    let start = *Timestamp::from(Date::new(2009, 1, 1));
    let timestamp = |height: usize| {
        Timestamp::new(
            start + (height as u64 * ((DAYS - 1) * 86_400) as u64 / BLOCKS as u64) as u32,
        )
    };
    let timestamp_source = SharedRangeMap::new((0..BLOCKS).map(timestamp).collect());
    let timestamps =
        RangeMapVec::<Height, Timestamp>::new("timestamps", Version::ONE, timestamp_source.clone());
    let mapping: LazyVec<Height, Day1, Height, Timestamp> = LazyVec::init(
        "day1",
        Version::ONE,
        timestamps.read_only_boxed_clone(),
        |_, timestamp| HeightMappings::day1_from_timestamp(timestamp),
    );
    let mut rebuilds = Vec::new();
    for _ in 0..7 {
        let start = Instant::now();
        black_box(ResolutionVecs::new(&mapping));
        rebuilds.push(start.elapsed());
    }
    rebuilds.sort_unstable();
    let mut resolution = ResolutionVecs::new(&mapping);
    let values = resolution.first_height.collect();
    let expected: Vec<_> = (0..resolution.first_height.len())
        .map(|day| {
            let height = (day * 86_400 * BLOCKS).div_ceil((DAYS - 1) * 86_400);
            Height::from(height)
        })
        .collect();
    assert_eq!(values, expected);

    let read_start = Instant::now();
    for _ in 0..READS {
        black_box(resolution.first_height.collect());
    }
    let read = read_start.elapsed();
    let mut append = 0.0;
    for height in BLOCKS..BLOCKS + APPENDS {
        timestamp_source.update_at(height, [timestamp(height)]);
        let start = Instant::now();
        resolution.update(Height::from(height));
        append += start.elapsed().as_secs_f64();
    }
    assert_eq!(
        resolution.first_height.collect(),
        ResolutionVecs::new(&mapping).first_height.collect()
    );
    println!(
        "{BLOCKS} heights: startup rebuild median {:.3} ms; all {} boundaries {:.3} us/read; one-block update {:.3} us; initial boundary payload {} bytes",
        rebuilds[3].as_secs_f64() * 1e3,
        values.len(),
        read.as_secs_f64() * 1e6 / READS as f64,
        append * 1e6 / APPENDS as f64,
        values.len() * size_of::<Height>(),
    );
}

fn compare_reverse<I: VecIndex + VecValue>(
    name: &str,
    timestamps: &RangeMapVec<Height, Timestamp>,
    convert: fn(Height, Timestamp) -> I,
) {
    let raw = LazyVec::init(
        name,
        Version::ONE,
        timestamps.read_only_boxed_clone(),
        convert,
    );
    let resident = ResolutionVecs::new(&raw).height_lookup();
    let sparse: Vec<_> = (0..timestamps.len()).step_by(149).collect();
    for dense in [true, false] {
        let read = |mapped| {
            if dense {
                if mapped {
                    resident.collect()
                } else {
                    raw.collect()
                }
            } else if mapped {
                resident.read_sorted_at(&sparse)
            } else {
                raw.read_sorted_at(&sparse)
            }
        };
        let expected = read(false);
        let mut samples = [Vec::new(), Vec::new()];
        for round in 0..14 {
            for offset in 0..2 {
                let variant = (round + offset) % 2;
                let start = Instant::now();
                let values = black_box(read(variant == 1));
                let elapsed = start.elapsed().as_secs_f64() * 1e3;
                assert_eq!(values, expected);
                if round >= 2 {
                    samples[variant].push(elapsed);
                }
            }
        }
        for (variant, samples) in ["timestamp", "range_map"]
            .into_iter()
            .zip(samples.iter_mut())
        {
            samples.sort_by(f64::total_cmp);
            println!(
                "{name}/{}/{variant}: {} values, mean {:.3} ms, median {:.3} ms",
                if dense { "full" } else { "sparse" },
                expected.len(),
                samples.iter().sum::<f64>() / samples.len() as f64,
                (samples[5] + samples[6]) / 2.0
            );
        }
    }
}

/// Fully resident timestamp input makes this a conservative comparison against
/// arithmetic/date conversion; excludes storage decoding, JSON, and HTTP.
#[test]
#[ignore = "manual paired full-height and sparse reverse mapping benchmark"]
fn reverse_mapping_read_costs() {
    const BLOCKS: usize = 966_505;
    const DAYS: usize = 6_463;
    let epoch = *Timestamp::from(Date::new(2009, 1, 1));
    let timestamps = RangeMapVec::<Height, _>::new(
        "timestamps",
        Version::ONE,
        SharedRangeMap::new(
            (0..BLOCKS)
                .map(|height| {
                    Timestamp::new(
                        epoch
                            + (height as u64 * ((DAYS - 1) * 86_400) as u64 / (BLOCKS - 1) as u64)
                                as u32,
                    )
                })
                .collect(),
        ),
    );
    compare_reverse("minute10", &timestamps, |_, t| Minute10::from_timestamp(t));
    compare_reverse("minute30", &timestamps, |_, t| Minute30::from_timestamp(t));
    compare_reverse("hour1", &timestamps, |_, t| Hour1::from_timestamp(t));
    compare_reverse("hour4", &timestamps, |_, t| Hour4::from_timestamp(t));
    compare_reverse("hour12", &timestamps, |_, t| Hour12::from_timestamp(t));
    compare_reverse("day1", &timestamps, |_, t| {
        HeightMappings::day1_from_timestamp(t)
    });
    compare_reverse("day3", &timestamps, |_, t| Day3::from_timestamp(t));
    compare_reverse("week1", &timestamps, |_, t| {
        HeightMappings::week1_from_timestamp(t)
    });
    compare_reverse("month1", &timestamps, |_, t| {
        HeightMappings::month1_from_timestamp(t)
    });
    compare_reverse("month3", &timestamps, |_, t| {
        HeightMappings::month3_from_timestamp(t)
    });
    compare_reverse("month6", &timestamps, |_, t| {
        HeightMappings::month6_from_timestamp(t)
    });
    compare_reverse("year1", &timestamps, |_, t| {
        HeightMappings::year1_from_timestamp(t)
    });
    compare_reverse("year10", &timestamps, |_, t| {
        HeightMappings::year10_from_timestamp(t)
    });
}
