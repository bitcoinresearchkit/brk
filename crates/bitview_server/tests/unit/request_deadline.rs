use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

use axum::{Router, http::StatusCode, middleware::from_fn, routing::any};
use brk_error::Error as BrkError;
use tokio::time;
use tower::ServiceExt;

use super::*;

#[tokio::test]
async fn deadline_bounds_the_initial_attempt() {
    let app = Router::new()
        .route(
            "/",
            any(|request: Request<Body>| async move {
                assert!(request.extensions().get::<RequestDeadline>().is_some());
                time::sleep(Duration::from_secs(10)).await;
                StatusCode::OK
            }),
        )
        .layer(from_fn(|request, next| {
            apply_for(request, next, Duration::from_millis(30))
        }));
    let started = Instant::now();
    let response = app.oneshot(Request::new(Body::empty())).await.unwrap();
    assert_eq!(response.status(), StatusCode::GATEWAY_TIMEOUT);
    assert_eq!(response.headers()["cache-control"], "no-store");
    assert!(started.elapsed() < Duration::from_secs(1));
}

#[tokio::test]
async fn http_never_replays_handlers_or_classifies_readiness() {
    for method in ["GET", "HEAD", "POST"] {
        let calls = Arc::new(AtomicUsize::new(0));
        let counted = calls.clone();
        let app = Router::new()
            .route(
                "/",
                any(move || {
                    counted.fetch_add(1, Ordering::SeqCst);
                    async { Error::from(BrkError::StateUpdating).into_response() }
                }),
            )
            .layer(from_fn(apply));
        let response = app
            .oneshot(
                Request::builder()
                    .method(method)
                    .uri("/")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }
}
