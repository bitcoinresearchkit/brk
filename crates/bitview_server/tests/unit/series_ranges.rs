use std::sync::atomic::Ordering;

use bitview_plugin::UpdateContext;
use bitview_runtime::update;
use brk_exit::Exit;
use brk_types::Timestamp;
use serde_json::{Value, from_str};

use super::{
    chain_fixture::{default_first, run_genesis},
    series_publication,
    server_routes::exchange_with_etag,
};

#[cfg(feature = "series")]
fn assert_same_series_payload(response: &str, reference: &str) {
    // `stamp` records request-generation time, not selection identity.
    // Sequential equivalent queries can cross a wall-clock second.
    let mut bodies = [response, reference]
        .map(|response| from_str::<Value>(response.split_once("\r\n\r\n").unwrap().1).unwrap());
    for body in &mut bodies {
        assert!(
            body.as_object_mut()
                .unwrap()
                .remove("stamp")
                .unwrap()
                .is_string()
        );
    }
    assert_eq!(bodies[0], bodies[1]);
}

#[test]
fn date_and_timestamp_ranges_follow_reorganizations() {
    run_genesis(default_first(), |mut fixture| async move {
        fixture.publish(2, 1);
        let address = fixture.address;
        let chain = fixture.chain.clone();
        let active = fixture.active.clone();
        let tip = fixture.tip.clone();
        let plugins = &mut fixture.plugins;
        #[cfg(feature = "series")]
        {
            for (index, date) in [
                ("month1", "7470-05-01"),
                ("month3", "2073-01-01"),
                ("month6", "2137-01-01"),
                ("year1", "2265-01-01"),
                ("year10", "4560-01-01"),
            ] {
                let route = format!("/api/series/price_close/{index}");
                let valid = exchange_with_etag(
                    address,
                    "GET",
                    &format!("{route}?start=2009-01-03&limit=1"),
                    "\"old\"",
                )
                .await;
                assert!(valid.starts_with("HTTP/1.1 200"), "{route}: {valid}");
                for field in ["start", "end"] {
                    for method in ["GET", "HEAD"] {
                        let response = exchange_with_etag(
                            address,
                            method,
                            &format!("{route}?{field}={date}"),
                            "*",
                        )
                        .await;
                        assert!(response.starts_with("HTTP/1.1 400"), "{route}: {response}");
                        assert!(!response.contains("\r\netag:"));
                        if method == "HEAD" {
                            assert!(response.ends_with("\r\n\r\n"));
                        }
                    }
                }
            }
            for field in ["start", "end"] {
                let route = "/api/series/price_close/week1";
                for (bound, index) in [
                    ("2009-01-04", 0),
                    ("2009-01-04T23:59:59Z", 0),
                    ("2009-01-05", 1),
                    ("2009-01-05T00:00:00Z", 1),
                ] {
                    let reference = exchange_with_etag(
                        address,
                        "GET",
                        &format!("{route}?{field}={index}&limit=1"),
                        "\"old\"",
                    )
                    .await;
                    let path = format!("{route}?{field}={bound}&limit=1");
                    let response = exchange_with_etag(address, "GET", &path, "\"old\"").await;
                    assert!(reference.starts_with("HTTP/1.1 200"), "{reference}");
                    assert!(response.starts_with("HTTP/1.1 200"), "{path}: {response}");
                    assert_same_series_payload(&response, &reference);
                    let etag = response
                        .lines()
                        .find_map(|line| line.strip_prefix("etag: "))
                        .unwrap();
                    for method in ["GET", "HEAD"] {
                        let response = exchange_with_etag(address, method, &path, etag).await;
                        assert!(response.starts_with("HTTP/1.1 304"), "{response}");
                        assert!(response.ends_with("\r\n\r\n"));
                    }
                }
                for method in ["GET", "HEAD"] {
                    let response = exchange_with_etag(
                        address,
                        method,
                        &format!("{route}?{field}=9999-12-31"),
                        "*",
                    )
                    .await;
                    assert!(response.starts_with("HTTP/1.1 400"), "{response}");
                    assert!(!response.contains("\r\netag:"));
                    if method == "HEAD" {
                        assert!(response.ends_with("\r\n\r\n"));
                    }
                }
            }
            for field in ["start", "end"] {
                let route = "/api/series/price_close/day3";
                let reference = exchange_with_etag(
                    address,
                    "GET",
                    &format!("{route}?{field}=0&limit=1"),
                    "\"old\"",
                )
                .await;
                assert!(reference.starts_with("HTTP/1.1 200"), "{reference}");
                for bound in ["1970-01-01", "1970-01-01T00:00:00Z", "2008-12-31T23:59:59Z"] {
                    let path = format!("{route}?{field}={bound}&limit=1");
                    let response = exchange_with_etag(address, "GET", &path, "\"old\"").await;
                    assert!(response.starts_with("HTTP/1.1 200"), "{path}: {response}");
                    assert_same_series_payload(&response, &reference);
                    let etag = response
                        .lines()
                        .find_map(|line| line.strip_prefix("etag: "))
                        .unwrap();
                    for method in ["GET", "HEAD"] {
                        let response = exchange_with_etag(address, method, &path, etag).await;
                        assert!(response.starts_with("HTTP/1.1 304"), "{response}");
                        assert!(response.ends_with("\r\n\r\n"));
                    }
                }
            }
            for bound in [
                "2026-02-29",
                "2026-02-30",
                "2026-02-31",
                "2009-02-30",
                "2023-02-29",
                "123%C3%A901-01",
                "1969-12-31",
                "2106-02-08",
                "2188-06-09",
                "9999-12-31",
                "999999999999999999999999",
            ] {
                for field in ["start", "end"] {
                    for route in [
                        "/api/series/timestamp/height?",
                        "/api/series/timestamp/height/data?",
                        "/api/series/bulk?series=timestamp&index=height&",
                    ] {
                        let path = format!("{route}{field}={bound}");
                        for method in ["GET", "HEAD"] {
                            let response = exchange_with_etag(address, method, &path, "*").await;
                            assert!(response.starts_with("HTTP/1.1 400"), "{path}: {response}");
                            assert!(!response.contains("\r\netag:"));
                            if method == "HEAD" {
                                assert!(response.ends_with("\r\n\r\n"));
                            }
                        }
                    }
                }
            }
            // A two-block fork moves the timestamp boundary from height 1 to
            // height 2 without changing the tip height or vector length.
            for prefix in ["series"] {
                for bound in ["2009-01-03", "2009-01-03T18:30:05Z"] {
                    for field in ["start", "end"] {
                        let path = format!("/api/{prefix}/txid/tx_index?{field}={bound}");
                        for method in ["GET", "HEAD"] {
                            let response = exchange_with_etag(address, method, &path, "*").await;
                            assert!(response.starts_with("HTTP/1.1 400"), "{response}");
                            assert!(!response.contains("\r\netag:"));
                            if method == "GET" {
                                assert!(
                                    response
                                        .contains("date/timestamp ranges not supported for index"),
                                    "{response}"
                                );
                            } else {
                                assert!(response.ends_with("\r\n\r\n"));
                            }
                        }
                    }
                }
            }
            tip.store(2, Ordering::SeqCst);
            let boundary = (chain[1].header.time + chain[2].header.time) / 2;
            let path = format!(
                "/api/series/timestamp/height?start={}&limit=1",
                Timestamp::new(boundary).to_iso8601()
            );
            let mut timestamp_etag = "\"old\"".to_owned();
            for (branch, parent_time) in [(3, chain[2].header.time), (4, chain[1].header.time)] {
                let expected_start = if parent_time >= boundary { 1 } else { 2 };
                active.store(branch, Ordering::SeqCst);
                update(plugins, UpdateContext::new(&Exit::default())).unwrap();
                let response = exchange_with_etag(address, "GET", &path, &timestamp_etag).await;
                assert!(response.starts_with("HTTP/1.1 200"), "{response}");
                let body: Value = from_str(response.split_once("\r\n\r\n").unwrap().1).unwrap();
                assert_eq!(
                    body["start"], expected_start,
                    "timestamp range must follow the current fork: {body}"
                );
                timestamp_etag = response
                    .lines()
                    .find_map(|line| line.strip_prefix("etag: "))
                    .unwrap()
                    .to_owned();
                for method in ["GET", "HEAD"] {
                    let response =
                        exchange_with_etag(address, method, &path, &timestamp_etag).await;
                    assert!(response.starts_with("HTTP/1.1 304"), "{response}");
                    assert!(response.ends_with("\r\n\r\n"));
                }
            }
            for (bound, expected_start) in [
                ("1970-01-01".to_owned(), 0),
                ("2009-01-03".to_owned(), 0),
                (Timestamp::new(chain[0].header.time).to_iso8601(), 0),
                (Timestamp::new(chain[1].header.time).to_iso8601(), 1),
                (
                    Timestamp::new((chain[1].header.time + chain[4].header.time) / 2).to_iso8601(),
                    2,
                ),
                (Timestamp::new(chain[4].header.time).to_iso8601(), 2),
                // Preserve the existing after-tip clamp, not an empty range.
                ("2009-01-04".to_owned(), 2),
                ("2106-02-07".to_owned(), 2),
            ] {
                for prefix in ["series"] {
                    let path = format!("/api/{prefix}/timestamp/height?start={bound}&limit=1");
                    let response = exchange_with_etag(address, "GET", &path, "\"old\"").await;
                    assert!(response.starts_with("HTTP/1.1 200"), "{response}");
                    let body: Value = from_str(response.split_once("\r\n\r\n").unwrap().1).unwrap();
                    let expected_value = [
                        chain[0].header.time,
                        chain[1].header.time,
                        chain[4].header.time,
                    ][expected_start];
                    assert_eq!(body["start"], expected_start, "{path}: {body}");
                    assert_eq!(body["data"][0], expected_value, "{path}: {body}");
                    let etag = response
                        .lines()
                        .find_map(|line| line.strip_prefix("etag: "))
                        .unwrap();
                    for method in ["GET", "HEAD"] {
                        let response = exchange_with_etag(address, method, &path, etag).await;
                        assert!(response.starts_with("HTTP/1.1 304"), "{response}");
                        assert!(response.ends_with("\r\n\r\n"));
                    }
                }
            }
        }
    });
}

#[test]
fn daily_bucket_publication_remains_coherent() {
    run_genesis(default_first(), |mut fixture| async move {
        fixture.publish(6, 4);
        series_publication::check(
            &fixture.state,
            fixture.address,
            fixture.chain[5].header.time,
        )
        .await;
    });
}
