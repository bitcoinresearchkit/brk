use axum::{body, http::StatusCode};

use super::*;

#[tokio::test]
async fn matching_validator_skips_body_work_and_preserves_cdn_policy() {
    let params = CacheParams::resolve(
        &CacheStrategy::Live("unchanged-representation".to_owned().into()),
        CdnCacheMode::Live,
    );
    for condition in [
        "W/\"unchanged-representation\"",
        "\"unchanged-representation\"",
        "\"old\", W/\"unchanged-representation\"",
        "*",
    ] {
        let mut headers = HeaderMap::new();
        headers.insert(header::IF_NONE_MATCH, HeaderValue::from_static(condition));
        let mut prepared = false;
        let response = AppState::respond_with_future(&headers, params.clone(), async {
            prepared = true;
            Ok((Bytes::from_static(b"body"), |_: &mut HeaderMap| {}))
        })
        .await;

        assert!(!prepared, "body prepared for {condition}");
        assert_eq!(response.status(), StatusCode::NOT_MODIFIED);
        assert_eq!(
            response.headers()[header::ETAG],
            "W/\"unchanged-representation\""
        );
        assert_eq!(
            response.headers()["cdn-cache-control"],
            "public, max-age=1, must-revalidate"
        );
        assert!(
            body::to_bytes(response.into_body(), 0)
                .await
                .unwrap()
                .is_empty()
        );
    }
}

#[tokio::test]
async fn conditional_skips_preparation_and_errors_keep_their_policy() {
    for (matched, failed) in [(true, false), (false, false), (false, true)] {
        let mut headers = HeaderMap::new();
        if matched {
            headers.insert(header::IF_NONE_MATCH, HeaderValue::from_static("*"));
        }
        let mut prepared = false;
        let response = AppState::respond_with_future(&headers, CacheParams::deploy(), async {
            prepared = true;
            if failed {
                return Err(BrkError::Internal("preparation failed"));
            }
            Ok((Bytes::from_static(b"body"), |h: &mut HeaderMap| {
                h.insert(header::CONTENT_TYPE, HeaderValue::from_static("text/plain"));
            }))
        })
        .await;
        assert_eq!(prepared, !matched);
        assert_eq!(
            response.status(),
            if matched {
                StatusCode::NOT_MODIFIED
            } else if failed {
                StatusCode::INTERNAL_SERVER_ERROR
            } else {
                StatusCode::OK
            }
        );
        assert_eq!(response.headers().contains_key(header::ETAG), !failed);
        if failed {
            assert_eq!(response.headers()[header::CACHE_CONTROL], "no-store");
        }
    }
}
