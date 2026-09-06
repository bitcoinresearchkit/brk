use std::{hint::black_box, time::Instant};

use super::{BucketAccum, Cents, CentsSats, FxHashMap, UrpdRaw, sorted_buckets};
use crate::{CentsCompact, Sats, UrpdAggregation};

fn raw_buckets(raw: &UrpdRaw, direct: bool) -> Vec<(Cents, BucketAccum)> {
    if direct {
        return sorted_buckets(raw, UrpdAggregation::Raw);
    }
    let mut agg: FxHashMap<Cents, BucketAccum> =
        FxHashMap::with_capacity_and_hasher(raw.map.len(), Default::default());
    for (&price, &supply) in &raw.map {
        let price = Cents::from(price);
        let slot = agg.entry(price).or_default();
        slot.supply += supply;
        slot.realized_cap += CentsSats::from_price_sats(price, supply);
    }
    let mut sorted: Vec<_> = agg.into_iter().collect();
    sorted.sort_unstable_by_key(|&(price, _)| price);
    sorted
}

#[test]
fn raw_buckets_preserve_order_and_exact_accumulators() {
    for prices in [vec![], vec![0], vec![u32::MAX - 1, 1, 0, 100_000]] {
        let raw = UrpdRaw {
            map: prices
                .into_iter()
                .map(|price| (CentsCompact::new(price), Sats::from(u64::from(price))))
                .collect(),
        };
        let old = raw_buckets(&raw, false);
        let new = sorted_buckets(&raw, UrpdAggregation::Raw);
        assert_eq!(old.len(), new.len());
        for ((a, x), (b, y)) in old.iter().zip(&new) {
            assert_eq!(a, b);
            assert_eq!(x.supply, y.supply);
            assert_eq!(x.realized_cap, y.realized_cap);
        }
    }
}

#[test]
#[ignore = "raw URPD bucket preparation comparison; excludes decoding and JSON"]
fn benchmark_raw_buckets() {
    for count in [0_u32, 1, 1_000, 100_000] {
        let raw = UrpdRaw {
            map: (0..count)
                .map(|n| {
                    (
                        CentsCompact::new(n * 101),
                        Sats::from(u64::from(n) * 123_457),
                    )
                })
                .collect(),
        };
        let old = raw_buckets(&raw, false);
        let new = raw_buckets(&raw, true);
        assert_eq!(old.len(), new.len());
        for ((a, x), (b, y)) in old.iter().zip(&new) {
            assert_eq!(a, b);
            assert_eq!(x.supply, y.supply);
            assert_eq!(x.realized_cap, y.realized_cap);
        }
        let batch = if count < 100_000 { 1000 } else { 20 };
        let mut samples = [Vec::new(), Vec::new()];
        for round in 0..12 {
            for variant in [round % 2, 1 - round % 2] {
                let start = Instant::now();
                for _ in 0..batch {
                    black_box(raw_buckets(black_box(&raw), variant == 1));
                }
                if round >= 2 {
                    samples[variant].push(start.elapsed() / batch);
                }
            }
        }
        for sample in &mut samples {
            sample.sort_unstable();
        }
        eprintln!(
            "{count}: hash/sort {:?}, direct {:?}",
            samples[0][5], samples[1][5]
        );
    }
}
