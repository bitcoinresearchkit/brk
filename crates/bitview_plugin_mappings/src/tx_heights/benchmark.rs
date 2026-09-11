use std::{hint::black_box, time::Instant};

use brk_types::{Height, TxIndex};
use rangeindex::{RangeMap, SharedRangeMap};

fn next_random(state: &mut u64) -> usize {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state as usize
}

fn measure(mut read: impl FnMut() -> Vec<Height>, expected: &[Height]) -> f64 {
    let start = Instant::now();
    let values = black_box(read());
    let elapsed = start.elapsed().as_secs_f64() * 1e3;
    assert_eq!(values, expected);
    elapsed
}

fn report(label: &str, samples: &mut [f64]) {
    samples.sort_by(f64::total_cmp);
    println!(
        "{label}: mean {:.3} ms, median {:.3} ms",
        samples.iter().sum::<f64>() / samples.len() as f64,
        (samples[samples.len() / 2 - 1] + samples[samples.len() / 2]) / 2.0,
    );
}

/// Same synthetic spending requests and output allocation in every variant.
/// Measures only the distribution reader's transaction-to-height lookup stage.
#[test]
#[ignore = "manual paired distribution height lookup benchmark"]
fn distribution_height_lookups() {
    const BLOCKS: usize = 966_505;
    const REQUESTS: usize = 262_144;
    const ROUNDS: usize = 20;
    let mut random = 0x9e37_79b9_7f4a_7c15;
    let mut tx_index = 0usize;
    let starts: Vec<_> = (0..BLOCKS)
        .map(|_| {
            let start = TxIndex::from(tx_index);
            tx_index += 1 + next_random(&mut random) % 3_000;
            start
        })
        .collect();
    let shared = SharedRangeMap::<_, Height>::new(starts.clone());
    println!(
        "synthetic {BLOCKS} block boundaries, {REQUESTS} spending inputs, {ROUNDS} alternating rounds; duplicate boundary payload {} bytes",
        starts.len() * size_of::<TxIndex>(),
    );
    for pattern in ["random", "recent_mix", "repeated_tx", "interleaved_hot"] {
        let mut requests = Vec::with_capacity(REQUESTS);
        let mut expected = Vec::with_capacity(REQUESTS);
        let mut last_height = 0;
        for index in 0..REQUESTS {
            let random = next_random(&mut random);
            let height = match pattern {
                "recent_mix" if !random.is_multiple_of(5) => BLOCKS - 1 - random % 4_096,
                "repeated_tx" if !index.is_multiple_of(16) => last_height,
                "interleaved_hot" => BLOCKS - 1 - index % 256,
                _ => random % BLOCKS,
            };
            last_height = height;
            requests.push(starts[height]);
            expected.push(Height::from(height));
        }
        let mut cached = RangeMap::<_, Height>::from(starts.clone());
        let mut samples = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];
        for round in 0..ROUNDS + 2 {
            for offset in 0..4 {
                let variant = (round + offset) % 4;
                let elapsed = measure(
                    || match variant {
                        0 => requests.iter().map(|&tx| cached.get(tx).unwrap()).collect(),
                        1 => {
                            let map = shared.read();
                            requests
                                .iter()
                                .map(|&tx| map.get_shared(tx).unwrap())
                                .collect()
                        }
                        2 => {
                            let map = shared.read();
                            let mut cursor = map.cursor();
                            requests.iter().map(|&tx| cursor.get(tx).unwrap()).collect()
                        }
                        _ => {
                            let map = shared.read();
                            let mut cursor = map.cached_cursor();
                            requests.iter().map(|&tx| cursor.get(tx).unwrap()).collect()
                        }
                    },
                    &expected,
                );
                if round >= 2 {
                    samples[variant].push(elapsed);
                }
            }
        }
        for (name, samples) in [
            "owned_cached",
            "shared_binary",
            "shared_cursor",
            "shared_cached",
        ]
        .into_iter()
        .zip(samples.iter_mut())
        {
            report(&format!("{pattern}/{name}"), samples);
        }
    }
}
