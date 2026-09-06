use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

use axum::{
    Router, body::to_bytes, http::StatusCode, middleware::from_fn, response::IntoResponse,
    routing::any,
};
use brk_error::Error as BrkError;
use tokio::{spawn, task::yield_now};
use tower::ServiceExt;

use super::*;
use crate::Error;

fn request(method: &str) -> Request<Body> {
    Request::builder()
        .method(method)
        .uri("/api/test?value=1")
        .header("if-none-match", "\"current\"")
        .body(Body::empty())
        .unwrap()
}

fn router(attempts: Arc<AtomicUsize>, capacity: bool, recover: bool) -> Router {
    Router::new()
        .route(
            "/api/test",
            any(move |request: Request<Body>| {
                let attempt = attempts.fetch_add(1, Ordering::SeqCst);
                async move {
                    assert_eq!(request.uri().query(), Some("value=1"));
                    assert_eq!(request.headers()["if-none-match"], "\"current\"");
                    if recover && attempt >= 2 {
                        (StatusCode::NOT_MODIFIED, [("etag", "\"current\"")]).into_response()
                    } else if capacity {
                        Error::overloaded("test capacity").into_response()
                    } else {
                        Error::from(BrkError::StateUpdating).into_response()
                    }
                }
            }),
        )
        .layer(from_fn(|request, next| {
            wait_for(request, next, Duration::from_millis(90))
        }))
}

#[tokio::test]
async fn transient_read_failures_recover_and_preserve_validators() {
    for method in ["GET", "HEAD"] {
        for capacity in [false, true] {
            let attempts = Arc::new(AtomicUsize::new(0));
            let response = router(attempts.clone(), capacity, true)
                .oneshot(request(method))
                .await
                .unwrap();
            assert_eq!(response.status(), StatusCode::NOT_MODIFIED);
            assert_eq!(response.headers()["etag"], "\"current\"");
            assert_eq!(attempts.load(Ordering::SeqCst), 3);
            assert!(
                to_bytes(response.into_body(), 1024)
                    .await
                    .unwrap()
                    .is_empty()
            );
        }
    }
}

#[tokio::test]
async fn persistent_unavailability_is_bounded_and_not_cacheable() {
    for capacity in [false, true] {
        let attempts = Arc::new(AtomicUsize::new(0));
        let started = Instant::now();
        let response = router(attempts.clone(), capacity, false)
            .oneshot(request("GET"))
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
        assert!(started.elapsed() < Duration::from_secs(1));
        assert!(attempts.load(Ordering::SeqCst) > 1);
        assert!(!response.headers().contains_key("etag"));
        assert_eq!(response.headers()["cache-control"], "no-store");
        assert_eq!(response.headers()["retry-after"], "1");
    }
}

#[tokio::test]
async fn actions_and_request_payloads_are_never_replayed() {
    for method in ["POST", "PUT", "DELETE", "GET"] {
        let attempts = Arc::new(AtomicUsize::new(0));
        let mut request = request(method);
        if method == "GET" {
            *request.body_mut() = Body::from("do not discard this payload");
        }
        let response = router(attempts.clone(), false, true)
            .oneshot(request)
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
        assert_eq!(attempts.load(Ordering::SeqCst), 1);
    }
}

#[tokio::test]
async fn unrelated_errors_are_not_retried() {
    for status in [
        StatusCode::BAD_REQUEST,
        StatusCode::NOT_FOUND,
        StatusCode::SERVICE_UNAVAILABLE,
    ] {
        let attempts = Arc::new(AtomicUsize::new(0));
        let counted = attempts.clone();
        let router = Router::new()
            .route(
                "/api/test",
                any(move || {
                    counted.fetch_add(1, Ordering::SeqCst);
                    async move { status }
                }),
            )
            .layer(from_fn(wait));
        assert_eq!(
            router.oneshot(request("GET")).await.unwrap().status(),
            status
        );
        assert_eq!(attempts.load(Ordering::SeqCst), 1);
    }
}

#[tokio::test]
async fn cancellation_stops_future_attempts() {
    let attempts = Arc::new(AtomicUsize::new(0));
    let task = spawn(router(attempts.clone(), false, false).oneshot(request("GET")));
    while attempts.load(Ordering::SeqCst) == 0 {
        yield_now().await;
    }
    task.abort();
    let _ = task.await;
    let stopped = attempts.load(Ordering::SeqCst);
    sleep(Duration::from_millis(120)).await;
    assert_eq!(attempts.load(Ordering::SeqCst), stopped);
}
