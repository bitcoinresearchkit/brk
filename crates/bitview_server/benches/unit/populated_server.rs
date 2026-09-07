use std::{hint::black_box, sync::Arc, time::Instant};

use axum::{
    body::{Bytes, to_bytes},
    http::{
        HeaderMap, Method, StatusCode,
        header::{ETAG, IF_NONE_MATCH},
    },
};
use bitcoin::consensus::serialize;
use bitview_types::SeriesName;
use brk_types::{Index, Timestamp};
use serde_json::{from_value, json, to_value, to_vec};
use tokio::sync::Semaphore;
use vecdb::{Formattable, ValueWriter};

use super::chain_fixture::{
    raw_fixture_block, run as run_fixture, run_populated as run_fixture_with_first,
};
#[cfg(feature = "urpd")]
use super::urpd;
use crate::{AppState, CacheParams, CacheStrategy};

#[cfg(feature = "series")]
#[test]
#[ignore = "raw HEAD versus full payload preparation; synthetic block, excludes HTTP and cold storage"]
fn benchmark_raw_head() {
    let first = raw_fixture_block();
    let hash = first.block_hash().into();
    let size = serialize(&first).len();
    run_fixture_with_first(first, move |state, _| async move {
        let mut samples = [Vec::new(), Vec::new()];
        for round in 0..12 {
            for offset in 0..2 {
                let variant = (round + offset) % 2;
                let method = if variant == 0 {
                    Method::GET
                } else {
                    Method::HEAD
                };
                let started = Instant::now();
                for _ in 0..1000 {
                    let response = state
                        .respond_block_raw(HeaderMap::new(), hash, method.clone())
                        .await
                        .unwrap_or_else(|_| panic!("raw response failed"));
                    assert_eq!(response.status(), StatusCode::OK);
                    let bytes = to_bytes(response.into_body(), 4_000_000).await.unwrap();
                    assert_eq!(bytes.len(), if variant == 0 { size } else { 0 });
                    black_box(bytes);
                }
                if round >= 2 {
                    samples[variant].push(started.elapsed() / 1000);
                }
            }
        }
        for times in &mut samples {
            times.sort_unstable();
        }
        eprintln!(
            "raw {size}-byte admitted response: full payload {:?}, metadata HEAD {:?}",
            samples[0][5], samples[1][5]
        );
    });
}

#[cfg(feature = "series")]
#[test]
#[ignore = "populated health/latest/length comparisons; local fixture RPC only"]
fn benchmark_populated_series_scalars() {
    run_fixture(|state, _| async move {
        benchmark_csv(&state);
        benchmark_health(&state).await;
        for length in [false, true] {
            benchmark_scalar(&state, length).await;
        }
    });
}

#[cfg(feature = "urpd")]
#[test]
#[ignore = "URPD input capture versus decoding; synthetic snapshots and fixture RPC only"]
fn benchmark_urpd_inputs() {
    run_fixture(|state, _| async move {
        urpd::benchmark_inputs(&state);
    });
}

#[cfg(feature = "series")]
fn benchmark_csv(state: &AppState) {
    state.sync(|query| {
        for columns in [1, 2, 32] {
            let names = vec!["timestamp"; columns].join(",");
            for limit in [0, 1] {
                let mut times = [Vec::new(), Vec::new()];
                let selection = from_value(json!({"series": names, "index": "height", "limit": limit, "format": "csv"})).unwrap();
                let expected = query.format_bulk(query.resolve(selection, usize::MAX).unwrap()).unwrap().output.to_string();
                for round in 0..24 {
                    for variant in [round % 2, 1 - round % 2] {
                        let started = Instant::now();
                        for sample in 0..1000 {
                            let selection = from_value(json!({"series": names, "index": "height", "limit": limit, "format": "csv"})).unwrap();
                            let resolved = query.resolve(selection, usize::MAX).unwrap();
                            let bounded_columns = resolved.columns().collect::<Vec<_>>();
                            let csv = {
                                // Identical formatting controls; only empty-range writer setup differs.
                                let mut csv = String::with_capacity(columns * 10);
                                for (i, col) in bounded_columns.iter().enumerate() {
                                    if i > 0 { csv.push(','); }
                                    csv.push_str(col.name());
                                }
                                csv.push('\n');
                                if variant == 1 && resolved.start == resolved.end {
                                    // Candidate returns the header without creating any writers.
                                } else if columns == 1 {
                                    bounded_columns[0].write_csv_column(Some(resolved.start), Some(resolved.end), &mut csv).unwrap();
                                } else {
                                    let from = Some(resolved.start as i64);
                                    let to = Some(resolved.end as i64);
                                    let rows = bounded_columns[0].range_count(from, to);
                                    csv.reserve(rows * columns * 15);
                                    let mut writers: Vec<_> = bounded_columns.iter().map(|col| col.create_writer(from, to)).collect();
                                    for _ in 0..rows {
                                        for (i, writer) in writers.iter_mut().enumerate() {
                                            if i > 0 { csv.push(','); }
                                            writer.write_next(&mut csv).unwrap();
                                        }
                                        csv.push('\n');
                                    }
                                }
                                csv
                            };
                            if round < 4 && sample == 0 {
                                assert_eq!(csv, expected);
                            } else { black_box(csv); }
                        }
                        if round >= 4 { times[variant].push(started.elapsed() / 1000); }
                    }
                }
                for samples in &mut times { samples.sort_unstable(); }
                eprintln!("CSV columns={columns} rows={limit}: previous {:?}, early-empty {:?}", times[0][10], times[1][10]);
            }
        }
    });
}

#[cfg(feature = "series")]
#[test]
#[ignore = "timestamp conditional preparation and worker comparison; excludes HTTP"]
fn benchmark_timestamp_response() {
    run_fixture(move |state, _| async move {
        let timestamp = Timestamp::from(u32::MAX);
        let expected = state.sync(|q| {
            q.mappings().timestamp.monotonic.snapshot();
            q.indexer().vecs().blocks.timestamp.snapshot();
            to_vec(&q.block_by_timestamp(timestamp).unwrap()).unwrap()
        });
        let response = state
            .respond_block_timestamp(HeaderMap::new(), timestamp)
            .await
            .unwrap_or_else(|_| panic!("timestamp response failed"));
        let mut headers = HeaderMap::new();
        headers.insert(IF_NONE_MATCH, response.headers()[ETAG].clone());
        assert_eq!(
            to_bytes(response.into_body(), usize::MAX)
                .await
                .unwrap()
                .as_ref(),
            expected
        );
        let mut samples = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];
        for round in 0..12 {
            for offset in 0..4 {
                let variant = (round + offset) % 4;
                let started = Instant::now();
                for _ in 0..1000 {
                    if variant < 2 {
                        state.sync(|q| {
                            let hash = if variant == 0 {
                                q.block_by_timestamp(timestamp).unwrap().hash
                            } else {
                                q.resolve_block_by_timestamp(timestamp).unwrap().hash()
                            };
                            let params = CacheParams::revalidate(
                                format!("block-timestamp-v2-{hash}").into(),
                            );
                            assert!(params.matches_etag(&headers));
                            black_box(params);
                        });
                    } else {
                        let response = if variant == 2 {
                            state
                                .respond_block_timestamp_eager_for_bench(headers.clone(), timestamp)
                                .await
                                .unwrap_or_else(|_| panic!("eager timestamp response failed"))
                        } else {
                            state
                                .respond_block_timestamp(headers.clone(), timestamp)
                                .await
                                .unwrap_or_else(|_| panic!("timestamp response failed"))
                        };
                        assert_eq!(response.status(), StatusCode::NOT_MODIFIED);
                        black_box(response);
                    }
                }
                if round >= 2 {
                    samples[variant].push(started.elapsed() / 1000);
                }
            }
        }
        for values in &mut samples {
            values.sort_unstable();
        }
        eprintln!(
            "timestamp 304 eager/deferred preparation {:?}/{:?}; worker {:?}/{:?}",
            samples[0][5], samples[1][5], samples[2][5], samples[3][5]
        );
    });
}

#[cfg(feature = "series")]
async fn benchmark_health(state: &AppState) {
    let admission = Arc::new(Semaphore::new(1));
    let expected = to_value(state.run(|q| q.local_sync_status()).await.unwrap()).unwrap();
    let mut times = [Vec::new(), Vec::new()];
    for round in 0..24 {
        for variant in [round % 2, 1 - round % 2] {
            let started = Instant::now();
            for sample in 0..1000 {
                let permit = if variant == 1 {
                    Some(admission.clone().acquire_owned().await.unwrap())
                } else {
                    None
                };
                let status = state
                    .run(move |q| {
                        let _permit = permit;
                        q.local_sync_status()
                    })
                    .await
                    .unwrap();
                if round < 4 && sample == 0 {
                    assert_eq!(to_value(status).unwrap(), expected);
                } else {
                    black_box(status);
                }
            }
            if round >= 4 {
                times[variant].push(started.elapsed() / 1000);
            }
        }
    }
    for samples in &mut times {
        samples.sort_unstable();
    }
    eprintln!(
        "health local query: unbounded {:?}, admitted {:?}",
        times[0][10], times[1][10]
    );
}

#[cfg(feature = "series")]
async fn benchmark_scalar(state: &AppState, length: bool) {
    let response = if length {
        state
            .respond_json_content(&HeaderMap::new(), |q| {
                q.len(&"timestamp".into(), Index::Height)
            })
            .await
    } else {
        state
            .respond_json_content(&HeaderMap::new(), |q| {
                q.latest(&"timestamp".into(), Index::Height)
            })
            .await
    };
    let etag = response.headers()[ETAG].clone();
    for conditional in [false, true] {
        let variants = if length { 3 } else { 2 };
        let mut times = vec![Vec::new(); variants];
        let mut expected = None;
        for round in 0..24 {
            for offset in 0..variants {
                let variant = (round + offset) % variants;
                let started = Instant::now();
                for sample in 0..1000 {
                    let mut headers = HeaderMap::new();
                    if conditional {
                        let validator = if variant == 2 {
                            // This fixture has published genesis and one child.
                            "W/\"len1-2\"".parse().unwrap()
                        } else {
                            etag.clone()
                        };
                        headers.insert(IF_NONE_MATCH, validator);
                    }
                    let series = SeriesName::from("timestamp");
                    let response = if variant == 0 {
                        if length {
                            state
                                .respond_json_content(&headers, move |q| {
                                    q.len(&series, Index::Height)
                                })
                                .await
                        } else {
                            state
                                .respond_json_content(&headers, move |q| {
                                    q.latest(&series, Index::Height)
                                })
                                .await
                        }
                    } else if variant == 2 {
                        let permit = state
                            .series_bodies
                            .data_query
                            .clone()
                            .acquire_owned()
                            .await
                            .unwrap();
                        let length = state
                            .run(move |q| {
                                let _permit = permit;
                                q.len(&series, Index::Height)
                            })
                            .await
                            .unwrap();
                        let strategy = CacheStrategy::Live(format!("len1-{length}").into());
                        state.respond_json_value(&headers, strategy, length)
                    } else {
                        let permit = state
                            .series_bodies
                            .data_query
                            .clone()
                            .acquire_owned()
                            .await
                            .unwrap();
                        let bytes = state
                            .run(move |q| {
                                let _permit = permit;
                                let bytes = if length {
                                    let mut bytes = Vec::new();
                                    q.len(&series, Index::Height)?.fmt_json(&mut bytes);
                                    bytes
                                } else {
                                    q.latest_json(&series, Index::Height)?
                                };
                                Ok(Bytes::from(bytes))
                            })
                            .await
                            .unwrap();
                        state.respond_json_content_bytes(&headers, bytes)
                    };
                    assert_eq!(
                        response.status(),
                        if conditional {
                            StatusCode::NOT_MODIFIED
                        } else {
                            StatusCode::OK
                        }
                    );
                    if round < 4 && sample == 0 {
                        let mut headers = response.headers().clone();
                        if length {
                            headers.remove(ETAG);
                        }
                        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
                        let actual = (headers, body);
                        if let Some(expected) = &expected {
                            assert_eq!(&actual, expected);
                        } else {
                            expected = Some(actual);
                        }
                    } else {
                        black_box(response);
                    }
                }
                if round >= 4 {
                    times[variant].push(started.elapsed() / 1000);
                }
            }
        }
        for samples in &mut times {
            samples.sort_unstable();
        }
        eprintln!(
            "length={length} conditional={conditional}: unbounded {:?}, admitted {:?}",
            times[0][10], times[1][10]
        );
        if length {
            eprintln!("exact length validator: {:?}", times[2][10]);
        }
    }
}
