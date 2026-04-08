use axum::{
    body::Body,
    http::{HeaderMap, Response, StatusCode, header},
    response::IntoResponse,
};
use brk_types::Etag;
use serde::Serialize;

use super::header_map::HeaderMapExtended;
use crate::cache::CacheParams;

pub trait ResponseExtended
where
    Self: Sized,
{
    fn new_not_modified(etag: &Etag, cache_control: &str) -> Self;
    fn new_not_modified_with(params: &CacheParams) -> Self;
    fn new_json_cached<T>(value: T, params: &CacheParams) -> Self
    where
        T: Serialize;
    fn static_json<T>(headers: &HeaderMap, value: T) -> Self
    where
        T: Serialize;
    fn static_bytes(
        headers: &HeaderMap,
        bytes: &'static [u8],
        content_type: &'static str,
        content_encoding: &'static str,
    ) -> Self;
}

impl ResponseExtended for Response<Body> {
    fn new_not_modified(etag: &Etag, cache_control: &str) -> Response<Body> {
        let mut response = (StatusCode::NOT_MODIFIED, "").into_response();
        let headers = response.headers_mut();
        headers.insert_etag(etag.as_str());
        headers.insert_cache_control(cache_control);
        response
    }

    fn new_not_modified_with(params: &CacheParams) -> Response<Body> {
        let etag = Etag::from(params.etag_str());
        Self::new_not_modified(&etag, params.cache_control)
    }

    fn new_json_cached<T>(value: T, params: &CacheParams) -> Self
    where
        T: Serialize,
    {
        let bytes = serde_json::to_vec(&value).unwrap();
        let mut response = Response::builder().body(bytes.into()).unwrap();
        let headers = response.headers_mut();
        headers.insert_content_type_application_json();
        headers.insert_cache_control(params.cache_control);
        if let Some(etag) = &params.etag {
            headers.insert_etag(etag);
        }
        response
    }

    fn static_json<T>(headers: &HeaderMap, value: T) -> Self
    where
        T: Serialize,
    {
        let params = CacheParams::static_version();
        if params.matches_etag(headers) {
            return Self::new_not_modified_with(&params);
        }
        Self::new_json_cached(value, &params)
    }

    fn static_bytes(
        headers: &HeaderMap,
        bytes: &'static [u8],
        content_type: &'static str,
        content_encoding: &'static str,
    ) -> Self {
        let params = CacheParams::static_version();
        if params.matches_etag(headers) {
            return Self::new_not_modified_with(&params);
        }
        let mut response = Response::new(Body::from(bytes));
        let h = response.headers_mut();
        h.insert(header::CONTENT_TYPE, content_type.parse().unwrap());
        h.insert(header::CONTENT_ENCODING, content_encoding.parse().unwrap());
        h.insert_cache_control(params.cache_control);
        if let Some(etag) = &params.etag {
            h.insert_etag(etag);
        }
        response
    }
}

#[cfg(test)]
mod tests {
    use axum::{
        body::to_bytes,
        http::{
            HeaderMap, StatusCode,
            header::{CACHE_CONTROL, CONTENT_TYPE, ETAG, IF_NONE_MATCH},
        },
    };
    use serde_json::json;

    use super::ResponseExtended;
    use crate::{VERSION, cache::CacheParams};

    #[tokio::test]
    async fn new_json_cached_sets_headers_and_body() {
        let params = CacheParams::version();
        let response = axum::http::Response::<axum::body::Body>::new_json_cached(
            json!({ "ok": true }),
            &params,
        );

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get(CONTENT_TYPE).unwrap(),
            "application/json"
        );
        assert_eq!(
            response.headers().get(CACHE_CONTROL).unwrap(),
            "public, max-age=1, must-revalidate",
        );
        assert_eq!(
            response.headers().get(ETAG).unwrap(),
            format!("\"{VERSION}\"").as_str(),
        );

        let body = to_bytes(response.into_body(), 1024).await.unwrap();
        assert_eq!(
            serde_json::from_slice::<serde_json::Value>(&body).unwrap(),
            json!({ "ok": true })
        );
    }

    #[tokio::test]
    async fn static_json_returns_not_modified_for_matching_unquoted_etag() {
        let mut headers = HeaderMap::new();
        headers.insert(IF_NONE_MATCH, VERSION.parse().unwrap());

        let response =
            axum::http::Response::<axum::body::Body>::static_json(&headers, json!({ "ok": true }));

        assert_eq!(response.status(), StatusCode::NOT_MODIFIED);
        let body = to_bytes(response.into_body(), 1024).await.unwrap();
        assert!(body.is_empty());
    }

    #[tokio::test]
    async fn static_json_returns_not_modified_for_matching_quoted_etag() {
        let mut headers = HeaderMap::new();
        headers.insert(IF_NONE_MATCH, format!("\"{VERSION}\"").parse().unwrap());

        let response =
            axum::http::Response::<axum::body::Body>::static_json(&headers, json!({ "ok": true }));

        assert_eq!(response.status(), StatusCode::NOT_MODIFIED);
        let body = to_bytes(response.into_body(), 1024).await.unwrap();
        assert!(body.is_empty());
    }
}
