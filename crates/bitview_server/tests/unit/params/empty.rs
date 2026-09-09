use axum::{
    body::to_bytes,
    extract::FromRequestParts,
    http::{Request, StatusCode},
    response::IntoResponse,
};
use serde_json::{Value, from_slice};

use super::Empty;

#[tokio::test]
async fn accepts_only_absent_or_empty_query() {
    for uri in ["/health", "/health?"] {
        let (mut parts, _) = Request::builder().uri(uri).body(()).unwrap().into_parts();
        assert!(Empty::from_request_parts(&mut parts, &()).await.is_ok());
    }
    for uri in ["/health?x=1", "/health?x=1&x=2", "/health?&", "/health?x"] {
        let (mut parts, _) = Request::builder().uri(uri).body(()).unwrap().into_parts();
        let error = Empty::from_request_parts(&mut parts, &())
            .await
            .err()
            .unwrap();
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let body = to_bytes(response.into_body(), 1024).await.unwrap();
        let body: Value = from_slice(&body).unwrap();
        assert_eq!(
            body["error"]["message"],
            "this endpoint does not accept query parameters"
        );
    }
}
