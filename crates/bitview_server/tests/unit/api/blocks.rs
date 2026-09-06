use crate::CdnCacheMode;

use std::{hint::black_box, time::Instant};

use axum::{
    http::{
        HeaderMap, HeaderValue,
        header::{ETAG, IF_NONE_MATCH},
    },
    response::Response,
};
use bitcoin::{
    BlockHash as BitcoinBlockHash,
    hashes::{Hash, HashEngine, sha256},
};
use brk_types::{BlockHash, Dollars};

use super::{BASE_BLOCK_SCHEMA, V1_BLOCK_SCHEMA, block_height_hint, blocks_v1_params};
use crate::extended::ResponseExtended;
use crate::{CacheParams, CacheStrategy};

#[test]
fn single_block_height_is_only_a_bounded_hint() {
    for (value, expected) in [
        ("W/\"block-v1-4-1-digest\"", Some(1)),
        ("\"block-v1-4-2-digest\"", Some(2)),
        ("\"other\", W/\"block-v1-4-3-digest\"", Some(3)),
        ("\"block-v1-4-4-x\", \"block-v1-4-5-y\"", Some(4)),
        ("\"block-v1-4-4294967296-x\"", None),
        ("\"block-v1-2-1-x\"", None),
        ("\"block-v1-3-1-x\"", None),
        ("\"blocks-v1-4-1-x\"", None),
        ("block-v1-4-1-x", None),
        ("*", None),
    ] {
        let mut headers = HeaderMap::new();
        headers.insert(IF_NONE_MATCH, HeaderValue::from_static(value));
        assert_eq!(
            block_height_hint(&headers, V1_BLOCK_SCHEMA).map(u32::from),
            expected
        );
    }
    let mut headers = HeaderMap::new();
    headers.insert(
        IF_NONE_MATCH,
        HeaderValue::from_static("W/\"block-v3-123-hash\""),
    );
    assert_eq!(
        block_height_hint(&headers, BASE_BLOCK_SCHEMA).map(u32::from),
        Some(123)
    );
    assert_eq!(block_height_hint(&headers, V1_BLOCK_SCHEMA), None);
}

#[test]
fn v1_identity_covers_full_tip_and_ordered_body_prices() {
    let tip = BlockHash::default();
    let prices = [Dollars::from(10.0), Dollars::from(20.0)];
    let tag = |tip, prices: &[Dollars]| {
        Response::new_not_modified(&blocks_v1_params(tip, prices)).headers()[ETAG].clone()
    };
    let expected = tag(Some(tip), &prices);
    assert!(expected.to_str().unwrap().starts_with("W/\"blocks-v1-4-"));
    assert_eq!(expected, tag(Some(tip), &prices));
    assert_ne!(expected, tag(Some(tip), &[prices[1], prices[0]]));
    assert_ne!(expected, tag(Some(tip), &[prices[0], Dollars::from(20.01)]));
    assert_ne!(expected, tag(Some(tip), &prices[..1]));
    let mut changed = [0; 32];
    changed[31] = 1;
    assert_ne!(
        expected,
        tag(
            Some(BitcoinBlockHash::from_byte_array(changed).into()),
            &prices
        )
    );
    assert_ne!(tag(None, &[]), tag(Some(tip), &[]));
}

#[test]
#[ignore = "V1 validator encoding comparison; no I/O"]
fn benchmark_v1_validator_encoding() {
    let tip = BlockHash::default();
    for count in [2, 15] {
        let prices: Vec<_> = (0..count).map(|n| Dollars::from(10.0 + n as f64)).collect();
        let mut samples = [Vec::new(), Vec::new()];
        for round in 0..24 {
            for variant in [round % 2, 1 - round % 2] {
                let started = Instant::now();
                for _ in 0..1000 {
                    let params = if variant == 0 {
                        let mut engine = sha256::Hash::engine();
                        for price in black_box(&prices) {
                            engine.input(&f64::from(*price).to_bits().to_le_bytes());
                        }
                        let hash = sha256::Hash::from_engine(engine);
                        CacheParams::resolve(
                            &CacheStrategy::Live(
                                format!("blocks-v1-2-{}-{hash}", black_box(tip)).into(),
                            ),
                            CdnCacheMode::Live,
                        )
                    } else {
                        blocks_v1_params(Some(black_box(tip)), black_box(&prices))
                    };
                    black_box(params);
                }
                if round >= 4 {
                    samples[variant].push(started.elapsed() / 1000);
                }
            }
        }
        for times in &mut samples {
            times.sort_unstable();
        }
        eprintln!(
            "V1 validator {count} prices: split {:?}, combined {:?}",
            samples[0][10], samples[1][10]
        );
    }
}
