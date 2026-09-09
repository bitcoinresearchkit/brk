use axum::{
    body::{Body, to_bytes},
    handler::Handler,
    http::{Request, StatusCode},
};
use serde_json::to_vec;

use super::*;

#[tokio::test]
async fn version_validates_parameters_before_conditionals() {
    for (uri, condition, status) in [
        ("/version", "\"other\"", StatusCode::OK),
        ("/version", "*", StatusCode::NOT_MODIFIED),
        ("/version?", "*", StatusCode::NOT_MODIFIED),
        ("/version?x=1&x=2", "*", StatusCode::BAD_REQUEST),
    ] {
        let request = Request::builder()
            .uri(uri)
            .header("if-none-match", condition)
            .body(Body::empty())
            .unwrap();
        let response = version.call(request, ()).await;
        assert_eq!(response.status(), status);
        if status == StatusCode::BAD_REQUEST {
            continue;
        }
        assert_eq!(
            response.headers()["cache-control"],
            "public, max-age=1, must-revalidate"
        );
        assert_eq!(
            response.headers()["cdn-cache-control"],
            response.headers()["cache-control"]
        );
        let body = to_bytes(response.into_body(), 1024).await.unwrap();
        if status == StatusCode::OK {
            assert_eq!(body.as_ref(), to_vec(VERSION).unwrap());
        } else {
            assert!(body.is_empty());
        }
    }
}
