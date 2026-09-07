use crate::{VSize, get_weighted_percentile, get_weighted_percentiles};

fn reference(values: &[(u64, VSize)], percentile: f64) -> u64 {
    let total: u64 = values.iter().map(|(_, weight)| u64::from(*weight)).sum();
    let target = (total as f64 * percentile).round() as u64;
    let mut cumulative = 0;
    for &(value, weight) in values {
        cumulative += u64::from(weight);
        if cumulative >= target {
            return value;
        }
    }
    values.last().unwrap().0
}

#[test]
fn batched_ranks_match_independent_scans() {
    let ranks = [0.0, 0.05, 0.10, 0.25, 0.50, 0.50, 0.75, 0.90, 0.95, 1.0];
    for len in [1, 2, 3, 7, 100, 2500] {
        for seed in 0..16 {
            let values: Vec<_> = (0..len)
                .map(|i| (i / 3, VSize::new((i * 13 + seed * 7) % 101)))
                .collect();
            assert_eq!(
                get_weighted_percentiles(&values, ranks),
                ranks.map(|p| reference(&values, p))
            );
            for p in ranks {
                assert_eq!(get_weighted_percentile(&values, p), reference(&values, p));
            }
        }
    }
}

#[test]
fn zero_weights_rounding_and_extreme_targets_keep_first_crossing() {
    for weights in [
        vec![0, 0, 0],
        vec![0, 1, 0],
        vec![1, 0, 1],
        vec![1, 2, 0],
        vec![u64::MAX - 1, 0, 1],
    ] {
        let values: Vec<_> = weights
            .into_iter()
            .enumerate()
            .map(|(i, w)| (i as u64, VSize::new(w)))
            .collect();
        let ranks = [
            f64::NEG_INFINITY,
            -1.0,
            0.0,
            0.1,
            0.25,
            0.5,
            0.75,
            1.0,
            2.0,
            f64::INFINITY,
        ];
        assert_eq!(
            get_weighted_percentiles(&values, ranks),
            ranks.map(|p| reference(&values, p))
        );
        assert_eq!(
            get_weighted_percentile(&values, f64::NAN),
            reference(&values, f64::NAN)
        );
        assert_eq!(get_weighted_percentiles(&values, []), [0u64; 0]);
    }
}

#[test]
#[should_panic(expected = "empty slice")]
fn empty_population_panics() {
    get_weighted_percentiles::<u64, 1>(&[], [0.5]);
}

#[test]
#[should_panic(expected = "nondecreasing")]
fn decreasing_targets_are_rejected() {
    get_weighted_percentiles(&[(1u64, VSize::new(100))], [0.9, 0.1]);
}

#[test]
#[ignore = "same-input optimized percentile benchmark"]
fn benchmark_weighted_percentiles() {
    benchmark_ranks([0.5]);
    benchmark_ranks([0.10, 0.25, 0.50, 0.75, 0.90]);
    benchmark_ranks([0.0, 0.10, 0.25, 0.50, 0.75, 0.90, 1.0]);
    benchmark_ranks([0.05, 0.10, 0.25, 0.50, 0.75, 0.90, 0.95]);
}

fn benchmark_ranks<const N: usize>(ranks: [f64; N]) {
    use std::{hint::black_box, time::Instant};
    for len in [1, 16, 2500, 15000] {
        let values: Vec<_> = (0..len)
            .map(|i| (i / 3, VSize::new((i * 13 + 7) % 101)))
            .collect();
        let mut old = Vec::new();
        let mut new = Vec::new();
        let iterations = (1_000_000 / len).clamp(100, 10_000);
        for round in 0..13 {
            for variant in if round % 2 == 0 {
                [false, true]
            } else {
                [true, false]
            } {
                let begin = Instant::now();
                for _ in 0..iterations {
                    let result = if variant {
                        get_weighted_percentiles(black_box(&values), black_box(ranks))
                    } else {
                        black_box(ranks).map(|p| reference(black_box(&values), p))
                    };
                    black_box(result);
                }
                if round >= 2 {
                    let timings = if variant { &mut new } else { &mut old };
                    timings.push(begin.elapsed().as_nanos() as f64 / iterations as f64);
                }
            }
        }
        old.sort_by(f64::total_cmp);
        new.sort_by(f64::total_cmp);
        assert_eq!(
            get_weighted_percentiles(&values, ranks),
            ranks.map(|p| reference(&values, p))
        );
        println!(
            "len={len} ranks={N} first_rank={} old_ns={:.1} new_ns={:.1} change={:.1}%",
            ranks[0],
            old[5],
            new[5],
            (new[5] / old[5] - 1.0) * 100.0
        );
    }
}
