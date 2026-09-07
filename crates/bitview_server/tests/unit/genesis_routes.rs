use std::{net::Ipv4Addr, sync::atomic::Ordering, time::Duration};

#[cfg(feature = "series")]
use brk_types::Index;
use serde_json::{Value, from_str, json};
use tokio::{spawn, time::timeout};

#[cfg(feature = "series")]
use super::series_admission;
use super::{
    chain_fixture::{default_first, run_genesis},
    server_routes::exchange_with_etag,
};
use crate::{Server, ServerConfig};

#[test]
fn genesis_health_sync_and_series_limits() {
    run_genesis(default_first(), |fixture| async move {
        let query = &fixture.query;
        let address = fixture.address;
        let inspection_state = &fixture.state;
        let directory = &fixture.directory;
        let tip = &fixture.tip;
        let block = &fixture.chain[0];
        timeout(Duration::from_secs(10), async {
                let health = exchange_with_etag(address, "GET", "/health", "*").await;
                assert!(health.starts_with("HTTP/1.1 200"), "{health}");
                assert!(health.contains("\r\ncache-control: no-store\r\n"));
                assert!(health.contains("\r\ncdn-cache-control: no-store\r\n"));
                assert!(!health.contains("\r\netag:"));
                let health: Value = from_str(health.split_once("\r\n\r\n").unwrap().1).unwrap();
                assert_eq!(health["last_indexed_at_unix"], block.header.time);
                let response = exchange_with_etag(address, "GET", "/api/server/sync", "\"old\"").await;
                assert!(response.starts_with("HTTP/1.1 200"), "{response}");
                let body: Value = from_str(response.split_once("\r\n\r\n").unwrap().1).unwrap();
                assert_eq!(body, json!({"indexed_height": 0, "computed_height": 0,
                    "tip_height": 0, "blocks_behind": 0, "last_indexed_at_unix": 1231006505,
                    "last_indexed_at": "2009-01-03T18:15:05Z"}));
                for (key, value) in body.as_object().unwrap() {
                    assert_eq!(&health[key], value);
                }
                let etag = response.lines().find_map(|line| line.strip_prefix("etag: ")).unwrap();
                for method in ["GET", "HEAD"] {
                    let response = exchange_with_etag(address, method, "/api/server/sync", etag).await;
                    assert!(response.starts_with("HTTP/1.1 304"), "{response}");
                    assert!(response.ends_with("\r\n\r\n"));
                    assert!(response.contains("\r\ncdn-cache-control: public, max-age=1, must-revalidate\r\n"), "{response}");
                }
                for path in ["/health", "/api/server/sync"] {
                    let response = exchange_with_etag(address, "HEAD", path, "\"old\"").await;
                    assert!(response.starts_with("HTTP/1.1 200"), "{response}");
                    assert!(response.ends_with("\r\n\r\n"));
                }
                let mut etag = etag.to_owned();
                #[cfg(feature = "series")]
                {
                    assert_eq!(query.sync(|q| q.len(&"timestamp".into(), Index::Height)).unwrap(), 1);
                    assert_eq!(query.sync(|q| q.len(&"timestamp_monotonic".into(), Index::Height)).unwrap(), 0);
                    series_admission::check(inspection_state).await;
                }
                #[cfg(feature = "series")]
                for (names, columns, available) in [
                    ("timestamp", 1, 1),
                    ("timestamp,timestamp", 2, 1),
                    ("timestamp,timestamp_monotonic", 2, 0),
                    ("timestamp_monotonic,timestamp", 2, 0),
                ] {
                    for limit in [0, 1] {
                        let end = limit.min(available);
                        for format in ["json", "csv"] {
                            let path = format!("/api/series/bulk?series={names}&index=height&limit={limit}&format={format}");
                            let response = exchange_with_etag(address, "GET", &path, "\"old\"").await;
                            assert!(response.starts_with("HTTP/1.1 200"), "{response}");
                            let body = response.split_once("\r\n\r\n").unwrap().1;
                            if format == "json" {
                                let body: Value = from_str(body).unwrap();
                                let series = body.as_array().expect("bulk is always an array");
                                assert_eq!(series.len(), columns);
                                for series in series {
                                    assert_eq!(series["start"], 0);
                                    assert_eq!(series["end"], end);
                                    assert_eq!(series["data"], if end == 0 { json!([]) } else { json!([block.header.time]) });
                                }
                                assert!(!response.contains("content-disposition:"));
                            } else {
                                let mut expected = format!("{names}\n");
                                if end != 0 {
                                    expected.push_str(&vec![block.header.time.to_string(); columns].join(","));
                                    expected.push('\n');
                                }
                                assert_eq!(body, expected);
                                assert!(response.contains("content-type: text/csv\r\n"));
                                assert!(response.contains(&format!("content-disposition: attachment; filename=\"{}-Height.csv\"\r\n", names.replace(',', "_"))));
                            }
                            let tag = response.lines().find_map(|line| line.strip_prefix("etag: ")).unwrap();
                            for method in ["GET", "HEAD"] {
                                let response = exchange_with_etag(address, method, &path, tag).await;
                                assert!(response.starts_with("HTTP/1.1 304"), "{response}");
                                assert!(response.ends_with("\r\n\r\n"));
                                assert!(!response.contains("content-disposition:"));
                                assert!(!response.contains("content-type:"));
                            }
                        }
                    }
                }
                for height in [2, 0] {
                    tip.store(height, Ordering::SeqCst);
                    let response = exchange_with_etag(address, "GET", "/api/server/sync", &etag).await;
                    assert!(response.starts_with("HTTP/1.1 200"), "{response}");
                    let body: Value = from_str(response.split_once("\r\n\r\n").unwrap().1).unwrap();
                    assert_eq!(body["indexed_height"], 0);
                    assert_eq!(body["tip_height"], height);
                    assert_eq!(body["blocks_behind"], height);
                    etag = response.lines().find_map(|line| line.strip_prefix("etag: ")).unwrap().to_owned();
                }
            }).await.unwrap();

        #[cfg(feature = "series")]
        {
            let limited = Server::bind(
                query,
                ServerConfig {
                    bind: Ipv4Addr::LOCALHOST.into(),
                    port: 0.into(),
                    max_weight: 8,
                    data_path: directory.path().to_owned(),
                    ..ServerConfig::default()
                },
            )
            .await
            .unwrap();
            let limited_address = limited.listener.local_addr().unwrap();
            let limited_serving = spawn(limited.serve());
            timeout(Duration::from_secs(10), async {
                for columns in [1, 2, 3] {
                    let names = vec!["timestamp"; columns].join(",");
                    for format in ["json", "csv"] {
                        let path = format!(
                            "/api/series/bulk?series={names}&index=height&limit=1&format={format}"
                        );
                        let response = exchange_with_etag(address, "GET", &path, "\"old\"").await;
                        assert!(response.starts_with("HTTP/1.1 200"), "{response}");
                        let etag = response
                            .lines()
                            .find_map(|line| line.strip_prefix("etag: "))
                            .unwrap();
                        for condition in ["\"old\"", etag, "*"] {
                            for method in ["GET", "HEAD"] {
                                let response =
                                    exchange_with_etag(limited_address, method, &path, condition)
                                        .await;
                                let status = if columns == 3 {
                                    400
                                } else if condition == "\"old\"" {
                                    200
                                } else {
                                    304
                                };
                                assert!(
                                    response.starts_with(&format!("HTTP/1.1 {status}")),
                                    "{response}"
                                );
                                if columns == 3 {
                                    assert!(!response.contains("\r\netag:"));
                                    assert!(
                                        response
                                            .contains("content-type: application/problem+json\r\n")
                                    );
                                    if method == "GET" {
                                        let body: Value =
                                            from_str(response.split_once("\r\n\r\n").unwrap().1)
                                                .unwrap();
                                        assert_eq!(body["error"]["code"], "weight_exceeded");
                                    }
                                }
                                if method == "HEAD" || status == 304 {
                                    assert!(response.ends_with("\r\n\r\n"));
                                }
                            }
                        }
                    }
                }
            })
            .await
            .unwrap();
            limited_serving.abort();
        }
    });
}
