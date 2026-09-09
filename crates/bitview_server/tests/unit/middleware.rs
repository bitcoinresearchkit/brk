use axum::{
    Router,
    body::{Body, to_bytes},
    http::{
        Method, Request, Response, StatusCode,
        header::{ALLOW, CONTENT_TYPE, ETAG},
    },
    middleware::from_fn,
};
use serde_json::{Value, from_slice};
use tower::ServiceExt;

use crate::{json_error, response_time};

#[tokio::test]
async fn middleware_preserves_errors_and_action_cache_policy() {
    for method in [Method::GET, Method::POST] {
        for (status, content_type, body, expected_code) in [
            (StatusCode::OK, "text/plain", "ok", None),
            (
                StatusCode::NOT_FOUND,
                "text/plain",
                "missing",
                Some("not_found"),
            ),
            (
                StatusCode::METHOD_NOT_ALLOWED,
                "text/plain",
                "",
                Some("method_not_allowed"),
            ),
            (
                StatusCode::GATEWAY_TIMEOUT,
                "text/plain",
                "",
                Some("timeout"),
            ),
            (
                StatusCode::BAD_REQUEST,
                "text/plain",
                "bad input",
                Some("bad_request"),
            ),
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "text/plain",
                "failed",
                Some("internal_error"),
            ),
            (
                StatusCode::BAD_REQUEST,
                "application/problem+json; charset=utf-8",
                "{\"kept\":true}",
                None,
            ),
        ] {
            let router = Router::new()
                .fallback(move || async move {
                    let mut response = Response::builder()
                        .status(status)
                        .header(CONTENT_TYPE, content_type)
                        .header(ALLOW, "GET, HEAD")
                        .header(ETAG, "\"fixture\"")
                        .body(Body::from(body))
                        .unwrap();
                    response.extensions_mut().insert(42_u32);
                    response
                })
                .layer(from_fn(json_error::respond))
                .layer(from_fn(response_time::respond));
            let response = router
                .oneshot(
                    Request::builder()
                        .method(method.clone())
                        .uri("/")
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();
            assert_eq!(response.status(), status);
            assert_eq!(response.extensions().get::<u32>(), Some(&42));
            assert_eq!(response.headers()[ALLOW], "GET, HEAD");
            assert!(response.headers().contains_key("x-response-time"));
            if method == Method::POST {
                assert_eq!(response.headers()["cache-control"], "no-store");
                assert!(!response.headers().contains_key(ETAG));
            }
            let bytes = to_bytes(response.into_body(), 4096).await.unwrap();
            if let Some(code) = expected_code {
                let json: Value = from_slice(&bytes).unwrap();
                assert_eq!(json["error"]["code"], code);
                if status == StatusCode::GATEWAY_TIMEOUT {
                    assert_eq!(
                        json["error"]["message"],
                        if method == Method::POST {
                            "Request timed out; submission outcome may be unknown"
                        } else {
                            "Request timed out"
                        }
                    );
                }
            } else {
                assert_eq!(&bytes[..], body.as_bytes());
            }
        }
    }
}
