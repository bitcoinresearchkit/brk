use super::*;
use axum::body::Body;

#[test]
fn head_uses_representation_length_and_small_bodies_stay_uncompressed() {
    let predicate = ResponseSizeAbove(1024);
    let mut response = Response::new(Body::empty());
    assert!(!predicate.should_compress(&response));
    response.headers_mut().insert(CONTENT_LENGTH, 1024.into());
    assert!(predicate.should_compress(&response));
    response.headers_mut().insert(CONTENT_LENGTH, 1023.into());
    assert!(!predicate.should_compress(&response));
    response.headers_mut().remove(CONTENT_LENGTH);
    *response.body_mut() = Body::from(vec![0; 1024]);
    assert!(predicate.should_compress(&response));
    *response.status_mut() = StatusCode::NOT_MODIFIED;
    assert!(!predicate.should_compress(&response));
}
