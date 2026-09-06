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
