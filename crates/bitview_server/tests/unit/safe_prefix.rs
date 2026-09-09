use std::time::{Duration, Instant};

use brk_error::{Error, Result};
use brk_types::{Addr, Height as TypesHeight};
use serde_json::to_value;
use tokio::time::timeout;

use super::{
    chain_fixture::run,
    server_routes::{exchange_bytes, exchange_with_etag},
};

fn body(response: &[u8]) -> &[u8] {
    let end = response
        .windows(4)
        .position(|bytes| bytes == b"\r\n\r\n")
        .unwrap()
        + 4;
    &response[end..]
}

#[test]
fn immutable_reads_use_published_prefix_during_append() {
    run(|state, address| async move {
        let (height, hash, gate) = state.sync(|q| {
            (
                q.height(),
                q.tip_blockhash(),
                q.indexer().publication().clone(),
            )
        });
        let txid = state.sync(|q| {
            q.resolve_block_snapshot(&hash)
                .and_then(|resolved| resolved.anchor_txids(q))
                .unwrap()[0]
        });
        let addr = state.sync(|q| {
            let bytes = q
                .transaction_json_resolved(q.resolve_transaction(&txid).unwrap())
                .unwrap();
            let transaction: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
            let script: bitcoin::ScriptBuf =
                serde_json::from_value(transaction["vout"][0]["scriptpubkey"].clone()).unwrap();
            Addr::try_from(&script).unwrap()
        });
        let mut paths = vec![
            "/api/server/sync".to_owned(),
            "/api/blocks".to_owned(),
            format!("/api/block-height/{height}"),
            format!("/api/block/{hash}"),
            format!("/api/block/{hash}/header"),
            format!("/api/block/{hash}/txids"),
            format!("/api/block/{hash}/txid/0"),
            format!("/api/block/{hash}/txs/0"),
            format!("/api/block/{hash}/raw"),
            format!("/api/tx/{txid}"),
            format!("/api/tx/{txid}/status"),
            format!("/api/tx/{txid}/raw"),
            format!("/api/tx/{txid}/hex"),
            format!("/api/tx/{txid}/merkle-proof"),
            format!("/api/tx/{txid}/merkleblock-proof"),
            "/api/v1/mining/blocks/sizes-weights/24h".to_owned(),
            "/api/v1/mining/blocks/timestamp/4294967295".to_owned(),
            format!("/api/address/{addr}/txs/chain"),
            format!("/api/address/{addr}/txs"),
        ];
        #[cfg(feature = "series")]
        paths.extend([
            "/api/series/timestamp/height?start=0&end=2".to_owned(),
            "/api/series/timestamp/height/latest".to_owned(),
            "/api/series/timestamp/height/len".to_owned(),
        ]);
        let mut expected = Vec::new();
        for path in &paths {
            expected.push(exchange_bytes(address, "GET", path, "\"old\"", 4_100_000).await);
        }
        // Closing append/compute publication must not hide the old immutable
        // prefix. Rollback is independently excluded by the pinned lengths.
        gate.begin_update();
        state.sync(|q| q.difficulty_adjustment().unwrap());
        for (path, expected) in paths.into_iter().zip(expected) {
            let response = timeout(
                Duration::from_secs(1),
                exchange_bytes(address, "GET", &path, "\"old\"", 4_100_000),
            )
            .await
            .unwrap();
            assert!(response.starts_with(b"HTTP/1.1 200"), "{path}");
            assert_eq!(body(&response), body(&expected), "{path}");
            let head = timeout(
                Duration::from_secs(1),
                exchange_bytes(address, "HEAD", &path, "\"old\"", 4_100_000),
            )
            .await
            .unwrap();
            assert!(head.starts_with(b"HTTP/1.1 200"), "{path}");
            assert!(body(&head).is_empty(), "{path}");
            for method in ["GET", "HEAD"] {
                let response = timeout(
                    Duration::from_secs(1),
                    exchange_with_etag(address, method, &path, "*"),
                )
                .await
                .unwrap();
                assert!(response.starts_with("HTTP/1.1 304"), "{path}: {response}");
            }
        }
        gate.finish_update();
    });
}

#[test]
fn unpublished_reorg_tail_is_unavailable_then_distinguishes_absence() {
    use super::chain_fixture::{default_first, run_genesis};
    use bitview_plugin::UpdateContext;
    use bitview_plugin_indexer::HasIndexer;
    use bitview_runtime::ComputePluginSet;
    use brk_exit::Exit;
    use std::sync::atomic::Ordering;

    run_genesis(default_first(), |mut fixture| async move {
        fixture.publish(1, 1);
        let old_hash = fixture.query.sync(|q| q.tip_blockhash());
        let old_txid = fixture.chain[1].txdata[0].compute_txid();
        let gate = fixture.plugins.indexer().publication().clone();
        gate.begin_update();
        fixture.active.store(2, Ordering::SeqCst);
        fixture
            .plugins
            .compute(UpdateContext::new(&Exit::default()))
            .unwrap();
        assert_eq!(
            fixture.query.sync(|q| q.indexer().safe_lengths().height),
            TypesHeight::new(1)
        );

        let address = fixture.address;
        let unavailable = timeout(
            Duration::from_secs(1),
            exchange_with_etag(address, "GET", "/api/block-height/1", "*"),
        )
        .await
        .unwrap();
        assert!(unavailable.starts_with("HTTP/1.1 503"), "{unavailable}");
        assert!(!unavailable.contains("\r\netag:"));
        // A temporary reorg tail must not publish cacheable absence for txs.
        for suffix in [
            "",
            "/raw",
            "/hex",
            "/status",
            "/merkle-proof",
            "/merkleblock-proof",
        ] {
            let response =
                exchange_with_etag(address, "GET", &format!("/api/tx/{old_txid}{suffix}"), "*")
                    .await;
            assert!(response.starts_with("HTTP/1.1 503"), "{response}");
            assert!(response.contains("\r\ncache-control: no-store\r\n"));
        }
        let prefix = exchange_with_etag(address, "GET", "/api/block-height/0", "\"old\"").await;
        assert!(prefix.starts_with("HTTP/1.1 200"), "{prefix}");
        fixture.plugins.commit().unwrap();
        gate.finish_update();
        let response = exchange_with_etag(address, "GET", "/api/block-height/1", "\"old\"").await;
        assert!(response.starts_with("HTTP/1.1 200"), "{response}");
        let new_hash = fixture.query.sync(|q| q.tip_blockhash());
        assert_ne!(old_hash, new_hash);
        assert!(response.ends_with(&new_hash.to_string()), "{response}");
        for path in [
            format!("/api/block/{old_hash}"),
            format!("/api/tx/{old_txid}"),
            "/api/block-height/2".to_owned(),
        ] {
            let response = exchange_with_etag(address, "GET", &path, "*").await;
            assert!(response.starts_with("HTTP/1.1 404"), "{response}");
        }
    });
}

#[test]
fn append_publication_does_not_wait_for_or_change_retained_snapshots() {
    use super::chain_fixture::{default_first, run_genesis};
    use brk_types::Height;
    use std::{sync::mpsc, thread};

    run_genesis(default_first(), |mut fixture| async move {
        fixture.publish(1, 1);
        let query = fixture.query.clone();
        let old_hash = query.sync(|q| q.tip_blockhash());
        let expected_ids = query.sync(|q| {
            q.resolve_block_snapshot(&old_hash)
                .and_then(|resolved| resolved.anchor_txids(q))
                .unwrap()
        });
        let addr = Addr::try_from(&fixture.chain[1].txdata[0].output[0].script_pubkey).unwrap();
        let expected_utxos = query.sync(|q| q.addr_utxos(addr.clone(), 1000).unwrap());
        thread::scope(|scope| {
            let rows = query.sync(|q| q.resolve_blocks(None, 1).unwrap());
            let txids = query.sync(|q| q.resolve_blocks(None, 1).unwrap());
            let mixed = query.sync(|q| q.resolve_addr_txs(&addr, 50, 25, 50).unwrap());
            let utxos = query.sync(|q| q.resolve_addr_utxos(&addr, 1000).unwrap());
            let (finished, done) = mpsc::channel();
            let fixture = &mut fixture;
            let writer = thread::Builder::new()
                .stack_size(8 * 1024 * 1024)
                .spawn_scoped(scope, move || {
                    fixture.publish(4, 2);
                    finished.send(()).unwrap();
                })
                .unwrap();
            let result = done.recv_timeout(Duration::from_secs(30));
            // Consume/drop all old pins before joining even if publication
            // timed out, so a regression fails rather than hanging cleanup.
            let retained_rows = query.sync(|q| rows.build(q).unwrap());
            let retained_ids = query.sync(|q| txids.anchor_txids(q).unwrap());
            let retained_mixed = query.sync(|q| q.addr_txs_resolved(mixed).unwrap());
            let retained_utxos = query.sync(|q| q.addr_utxos_resolved(utxos, 1000).unwrap().0);
            writer.join().unwrap();
            result.expect("append publication waited for retained snapshots");
            assert_eq!(retained_rows[0].id, old_hash);
            assert_eq!(retained_ids, expected_ids);
            assert!(
                retained_mixed
                    .iter()
                    .any(|tx| expected_ids.contains(&tx.txid))
            );
            assert_eq!(
                to_value(retained_utxos).unwrap(),
                to_value(expected_utxos).unwrap()
            );
            assert_eq!(query.sync(|q| q.height()), Height::new(2));
            assert_ne!(query.sync(|q| q.tip_blockhash()), old_hash);
        });
    });
}

#[test]
fn retained_block_snapshots_survive_a_queued_real_reorg() {
    use super::chain_fixture::{default_first, run_genesis};
    use bitcoin::consensus::serialize;
    use bitview_plugin::UpdateContext;
    use bitview_runtime::update;
    use brk_exit::Exit;
    use brk_types::BlockTxIndex;
    use std::{sync::atomic::Ordering, thread, time::Instant};

    run_genesis(default_first(), |mut fixture| async move {
        fixture.publish(1, 1);
        let query = fixture.query.clone();
        let old_hash = query.sync(|q| q.tip_blockhash());
        let expected_raw = serialize(&fixture.chain[1]);
        let addr = Addr::try_from(&fixture.chain[1].txdata[0].output[0].script_pubkey).unwrap();
        fixture.active.store(2, Ordering::SeqCst);

        thread::scope(|scope| {
            // Keep this inside the scope closure so an assertion failure drops
            // the pin before scope cleanup joins the writer.
            let pin = query.sync(|q| {
                q.indexer()
                    .pin_safe_lengths_for(Duration::from_secs(1))
                    .unwrap()
            });
            let rows = query.sync(|q| q.resolve_blocks(None, 1).unwrap());
            let txids = query.sync(|q| q.resolve_blocks(None, 1).unwrap());
            let txs = query.sync(|q| q.resolve_blocks(None, 1).unwrap());
            let header = query.sync(|q| q.resolve_blocks(None, 1).unwrap());
            let raw = query.sync(|q| q.resolve_blocks(None, 1).unwrap());
            let mixed = query.sync(|q| q.resolve_addr_txs(&addr, 50, 25, 50).unwrap());
            let utxos = query.sync(|q| q.resolve_addr_utxos(&addr, 1000).unwrap());
            let plugins = &mut fixture.plugins;
            let writer = thread::Builder::new()
                .stack_size(8 * 1024 * 1024)
                .spawn_scoped(scope, move || {
                    update(plugins, UpdateContext::new(&Exit::default()))
                })
                .unwrap();

            let deadline = Instant::now() + Duration::from_secs(5);
            while query.sync(|q| q.indexer().try_pin_safe_lengths().is_some()) {
                assert!(
                    Instant::now() < deadline,
                    "reorg never queued its prefix write"
                );
                thread::sleep(Duration::from_millis(1));
            }
            assert!(!writer.is_finished());
            // Tip reads may nest beneath a retained pin, including when a
            // writer is queued. Snapshot consumers must borrow, not reacquire.
            assert_eq!(query.sync(|q| q.tip_blockhash()), old_hash);
            assert_eq!(query.sync(|q| rows.build(q).unwrap())[0].id, old_hash);
            let ids = query.sync(|q| txids.anchor_txids(q).unwrap());
            let transactions =
                query.sync(|q| txs.anchor_txs(q, BlockTxIndex::default(), 25).unwrap());
            assert_eq!(
                transactions.iter().map(|tx| tx.txid).collect::<Vec<_>>(),
                ids
            );
            assert_eq!(
                query.sync(|q| header.anchor_header_hex(q).unwrap()).len(),
                160
            );
            assert_eq!(query.sync(|q| raw.anchor_raw(q).unwrap()), expected_raw);
            assert!(
                !query
                    .sync(|q| q.addr_txs_resolved(mixed).unwrap())
                    .is_empty()
            );
            assert!(
                !query
                    .sync(|q| q.addr_utxos_resolved(utxos, 1000).unwrap().0)
                    .is_empty()
            );
            assert_eq!(query.sync(|q| q.tip_blockhash()), old_hash);
            drop(pin);
            writer.join().unwrap().unwrap();
        });

        assert_ne!(query.sync(|q| q.tip_blockhash()), old_hash);
        let response = exchange_with_etag(
            fixture.address,
            "GET",
            &format!("/api/block/{old_hash}"),
            "*",
        )
        .await;
        assert!(response.starts_with("HTTP/1.1 404"), "{response}");
    });
}

#[test]
fn request_deadline_bounds_gate_waits_and_skips_expired_work() {
    run(|state, _| async move {
        let query = state.query.with_deadline(Instant::now());
        let result = query
            .run(|_| -> Result<()> { panic!("expired work ran") })
            .await;
        assert!(matches!(result, Err(Error::ReadTimeout)));

        let gate = state.sync(|q| q.indexer().publication().clone());
        gate.begin_update();
        let started = Instant::now();
        let query = state
            .query
            .with_deadline(started + Duration::from_millis(50));
        let result = query
            .run(|q| {
                q.resolve_blocks_v1(None, 1)
                    .and_then(|resolved| resolved.build(q))
            })
            .await;
        gate.finish_update();
        assert!(matches!(result, Err(Error::ReadTimeout)));
        assert!(started.elapsed() < Duration::from_secs(1));
    });
}
