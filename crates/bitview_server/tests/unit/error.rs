use super::*;
use axum::http::{
    HeaderName,
    header::{CACHE_CONTROL, ETAG},
};

fn assert_cache_control(response: &Response, expected: &'static str) {
    let expected = HeaderValue::from_static(expected);
    assert_eq!(response.headers().get(CACHE_CONTROL), Some(&expected));
    assert_eq!(
        response
            .headers()
            .get(HeaderName::from_static("cdn-cache-control")),
        Some(&expected)
    );
    assert!(!response.headers().contains_key(ETAG));
}

#[test]
fn unknown_address_is_briefly_cacheable_without_a_validator() {
    let response = Error::from(BrkError::UnknownAddr).into_response();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    assert_cache_control(&response, "public, max-age=1, must-revalidate");
}

#[test]
fn invalid_address_is_immutable_without_a_validator() {
    let response = Error::from(BrkError::InvalidAddr).into_response();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    assert_cache_control(&response, "public, max-age=31536000, immutable");
}

#[test]
fn state_updating_is_a_retryable_service_unavailable_response() {
    let error = Error::from(BrkError::StateUpdating);
    assert_eq!(error.status, StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(error.code, "state_updating");

    let response = error.into_response();
    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(
        response.headers().get(header::RETRY_AFTER),
        Some(&HeaderValue::from_static("1"))
    );
    assert_cache_control(&response, "no-store");
}
