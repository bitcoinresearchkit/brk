use axum::{
    body::{HttpBody, to_bytes},
    http::{
        HeaderName,
        header::{CACHE_CONTROL, CONTENT_LENGTH, CONTENT_TYPE, ETAG, VARY},
    },
};

use super::*;

#[tokio::test]
async fn not_modified_is_empty_and_preserves_cache_metadata() {
    let response = Response::new_not_modified(&CacheParams::deploy());

    assert_eq!(response.status(), StatusCode::NOT_MODIFIED);
    assert!(response.headers().contains_key(ETAG));
    assert!(response.headers().contains_key(CACHE_CONTROL));
    assert!(
        response
            .headers()
            .contains_key(HeaderName::from_static("cdn-cache-control"))
    );
    assert_eq!(
        response.headers().get(VARY),
        Some(&HeaderValue::from_static("Accept-Encoding"))
    );
    assert!(!response.headers().contains_key(CONTENT_TYPE));
    assert!(!response.headers().contains_key(CONTENT_LENGTH));
    assert_eq!(response.body().size_hint().exact(), None);
    assert!(
        to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap()
            .is_empty()
    );
}
