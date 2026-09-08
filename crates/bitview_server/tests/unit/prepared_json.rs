use axum::{
    body::to_bytes,
    http::{
        HeaderMap, StatusCode,
        header::{CACHE_CONTROL, ETAG, IF_NONE_MATCH},
    },
};

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
