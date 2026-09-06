use std::{hint::black_box, time::Instant};

use axum::{
    body::to_bytes,
    http::{
        HeaderMap, StatusCode,
        header::{CACHE_CONTROL, ETAG, IF_NONE_MATCH},
    },
    response::Response,
};

use crate::{CacheParams, extended::ResponseExtended};

use super::PreparedJson;

#[tokio::test]
async fn content_not_package_version_controls_revalidation() {
    let first = PreparedJson::new(["first"]);
    let identical = PreparedJson::new(["first"]);
    let changed = PreparedJson::new(["second"]);
    let response = first.respond(&HeaderMap::new());
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.headers()[CACHE_CONTROL],
        "public, max-age=1, must-revalidate"
    );
    assert_eq!(
        response.headers()["cdn-cache-control"],
        response.headers()[CACHE_CONTROL]
    );
    let mut headers = HeaderMap::new();
    headers.insert(IF_NONE_MATCH, response.headers()[ETAG].clone());
    assert_eq!(
        to_bytes(response.into_body(), 1024).await.unwrap(),
        "[\"first\"]"
    );
    let response = identical.respond(&headers);
    assert_eq!(response.status(), StatusCode::NOT_MODIFIED);
    assert!(
        to_bytes(response.into_body(), 1024)
            .await
            .unwrap()
            .is_empty()
    );
    let response = changed.respond(&headers);
    assert_eq!(response.status(), StatusCode::OK);
    assert_ne!(response.headers()[ETAG], headers[IF_NONE_MATCH]);
    assert_eq!(
        to_bytes(response.into_body(), 1024).await.unwrap(),
        "[\"second\"]"
    );
}

#[test]
#[ignore = "response assembly comparison; excludes startup hashing and network I/O"]
fn benchmark_prepared_json() {
    let prepared = PreparedJson::new(["catalog"]);
    let empty = HeaderMap::new();
    let mut validators = [HeaderMap::new(), HeaderMap::new()];
    for (headers, response) in validators.iter_mut().zip([
        Response::json_bytes(&empty, &CacheParams::deploy(), || prepared.bytes.clone()),
        prepared.respond(&empty),
    ]) {
        headers.insert(IF_NONE_MATCH, response.headers()[ETAG].clone());
    }
    for conditional in [false, true] {
        let mut times = [Vec::new(), Vec::new()];
        for round in 0..24 {
            for index in [round % 2, 1 - round % 2] {
                let headers = if conditional {
                    &validators[index]
                } else {
                    &empty
                };
                let started = Instant::now();
                for _ in 0..1000 {
                    let response = if index == 0 {
                        Response::json_bytes(black_box(headers), &CacheParams::deploy(), || {
                            prepared.bytes.clone()
                        })
                    } else {
                        prepared.respond(black_box(headers))
                    };
                    assert_eq!(response.status() == StatusCode::NOT_MODIFIED, conditional);
                    black_box(response);
                }
                if round >= 4 {
                    times[index].push(started.elapsed() / 1000);
                }
            }
        }
        for samples in &mut times {
            samples.sort_unstable();
        }
        eprintln!(
            "conditional={conditional}: deploy {:?}, prepared {:?} median/response",
            times[0][10], times[1][10]
        );
    }
}
