use std::time::Duration;

use axum::{
    body::Body,
    http::{Method, Request},
    middleware::Next,
    response::Response,
};
use http_body::Body as HttpBody;
use tokio::time::{Instant, sleep, timeout_at};

/// Internal response metadata, never inferred from an upstream status or body.
#[derive(Clone, Copy)]
pub enum ReadAvailability {
    Publication,
    Capacity,
}

const WAIT: Duration = Duration::from_secs(4);

/// Absorb transient publication/admission gaps at the HTTP read boundary.
/// Each attempt resolves and validates fresh state; no stale response is served.
/// Route-owned permits and read guards are released before the asynchronous wait.
pub async fn wait(request: Request<Body>, next: Next) -> Response {
    wait_for(request, next, WAIT).await
}

async fn wait_for(request: Request<Body>, next: Next, budget: Duration) -> Response {
    // Never replay actions or consume/reconstruct a caller's request payload.
    if !matches!(*request.method(), Method::GET | Method::HEAD) || !request.body().is_end_stream() {
        return next.run(request).await;
    }
    let (parts, body) = request.into_parts();
    let deadline = Instant::now() + budget;
    let mut response = next
        .clone()
        .run(Request::from_parts(parts.clone(), body))
        .await;
    let mut delay = Duration::from_millis(10);
    while response.extensions().get::<ReadAvailability>().is_some() {
        if Instant::now() + delay >= deadline {
            break;
        }
        sleep(delay).await;
        match timeout_at(
            deadline,
            next.clone()
                .run(Request::from_parts(parts.clone(), Body::empty())),
        )
        .await
        {
            Ok(retried) => response = retried,
            Err(_) => break,
        }
        delay = (delay * 2).min(Duration::from_millis(100));
    }
    response
}

#[cfg(test)]
#[path = "../tests/unit/read_availability.rs"]
mod tests;
