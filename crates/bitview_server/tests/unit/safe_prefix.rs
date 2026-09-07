use std::time::Duration;

use bitview_plugin::Plugin;
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
fn immutable_block_reads_use_published_prefix_during_append() {
    run(|state, address| async move {
        let (height, hash, gate) =
            state.sync(|q| (q.height(), q.tip_blockhash(), q.indexer().gate().clone()));
        let paths = [
            "/api/server/sync".to_owned(),
            "/api/blocks".to_owned(),
            format!("/api/block-height/{height}"),
            format!("/api/block/{hash}"),
            format!("/api/block/{hash}/header"),
            format!("/api/block/{hash}/txids"),
            format!("/api/block/{hash}/txid/0"),
            format!("/api/block/{hash}/txs/0"),
            format!("/api/block/{hash}/raw"),
        ];
        let mut expected = Vec::new();
        for path in &paths {
            expected.push(exchange_bytes(address, "GET", path, "\"old\"", 4_100_000).await);
        }
        // Closing append/compute publication must not hide the old immutable
        // prefix. Rollback is independently excluded by the pinned lengths.
        gate.begin_update();
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
fn reorg_tail_waits_for_publication_then_distinguishes_absence() {
    use super::chain_fixture::{default_first, run_genesis};
    use bitview_plugin::UpdateContext;
    use bitview_plugin_indexer::HasIndexer;
    use bitview_runtime::ComputePluginSet;
    use brk_exit::Exit;
    use std::sync::atomic::Ordering;

    run_genesis(default_first(), |mut fixture| async move {
        fixture.publish(1, 1);
        let old_hash = fixture.query.sync(|q| q.tip_blockhash());
        let gate = fixture.plugins.indexer().gate().clone();
        gate.begin_update();
        fixture.active.store(2, Ordering::SeqCst);
        fixture
            .plugins
            .compute(UpdateContext::new(&Exit::default()))
            .unwrap();
        assert_eq!(
            fixture.query.sync(|q| q.indexer().safe_lengths().height),
            brk_types::Height::new(1)
        );

        let address = fixture.address;
        let mut pending = tokio::spawn(exchange_with_etag(
            address,
            "GET",
            "/api/block-height/1",
            "\"old\"",
        ));
        assert!(
            timeout(Duration::from_millis(100), &mut pending)
                .await
                .is_err()
        );
        let prefix = exchange_with_etag(address, "GET", "/api/block-height/0", "\"old\"").await;
        assert!(prefix.starts_with("HTTP/1.1 200"), "{prefix}");
        fixture.plugins.commit().unwrap();
        gate.finish_update();
        let response = timeout(Duration::from_secs(2), pending)
            .await
            .unwrap()
            .unwrap();
        assert!(response.starts_with("HTTP/1.1 200"), "{response}");
        let new_hash = fixture.query.sync(|q| q.tip_blockhash());
        assert_ne!(old_hash, new_hash);
        assert!(response.ends_with(&new_hash.to_string()), "{response}");
        for path in [
            format!("/api/block/{old_hash}"),
            "/api/block-height/2".to_owned(),
        ] {
            let response = exchange_with_etag(address, "GET", &path, "*").await;
            assert!(response.starts_with("HTTP/1.1 404"), "{response}");
        }
    });
}

#[test]
fn request_deadline_bounds_gate_waits_and_skips_expired_work() {
    run(|state, _| async move {
        let query = state.query.with_deadline(std::time::Instant::now());
        let result = query
            .run(|_| -> brk_error::Result<()> { panic!("expired work ran") })
            .await;
        assert!(matches!(result, Err(brk_error::Error::ReadTimeout)));

        let gate = state.sync(|q| q.indexer().gate().clone());
        gate.begin_update();
        let started = std::time::Instant::now();
        let query = state
            .query
            .with_deadline(started + Duration::from_millis(50));
        let result = query.run(|q| q.blocks_v1(None, 1)).await;
        gate.finish_update();
        assert!(matches!(result, Err(brk_error::Error::ReadTimeout)));
        assert!(started.elapsed() < Duration::from_secs(1));
    });
}
