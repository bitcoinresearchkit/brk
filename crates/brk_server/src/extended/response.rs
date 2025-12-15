use axum::{
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
};
use serde::Serialize;

use super::header_map::HeaderMapExtended;
use crate::cache::CacheParams;

pub trait ResponseExtended
where
    Self: Sized,
{
    fn new_not_modified() -> Self;
    fn new_json<T>(value: T, etag: &str) -> Self
    where
        T: Serialize;
    fn new_json_with<T>(status: StatusCode, value: T, etag: &str) -> Self
    where
        T: Serialize;
    fn new_json_cached<T>(value: T, params: &CacheParams) -> Self
    where
        T: Serialize;
    fn new_text(value: &str, etag: &str) -> Self;
    fn new_text_with(status: StatusCode, value: &str, etag: &str) -> Self;
    fn new_text_cached(value: &str, params: &CacheParams) -> Self;
    fn new_bytes(value: Vec<u8>, etag: &str) -> Self;
    fn new_bytes_with(status: StatusCode, value: Vec<u8>, etag: &str) -> Self;
    fn new_bytes_cached(value: Vec<u8>, params: &CacheParams) -> Self;
}

impl ResponseExtended for Response<Body> {
    fn new_not_modified() -> Response<Body> {
        let mut response = (StatusCode::NOT_MODIFIED, "").into_response();
        let headers = response.headers_mut();
        headers.insert_cors();
        response
    }

    fn new_json<T>(value: T, etag: &str) -> Self
    where
        T: Serialize,
    {
        Self::new_json_with(StatusCode::default(), value, etag)
    }

    fn new_json_with<T>(status: StatusCode, value: T, etag: &str) -> Self
    where
        T: Serialize,
    {
        let bytes = serde_json::to_vec(&value).unwrap();
        let mut response = Response::builder().body(bytes.into()).unwrap();
        *response.status_mut() = status;
        let headers = response.headers_mut();
        headers.insert_cors();
        headers.insert_content_type_application_json();
        headers.insert_cache_control_must_revalidate();
        headers.insert_etag(etag);
        response
    }

    fn new_text(value: &str, etag: &str) -> Self {
        Self::new_text_with(StatusCode::default(), value, etag)
    }

    fn new_text_with(status: StatusCode, value: &str, etag: &str) -> Self {
        let mut response = Response::builder()
            .body(value.to_string().into())
            .unwrap();
        *response.status_mut() = status;
        let headers = response.headers_mut();
        headers.insert_cors();
        headers.insert_content_type_text_plain();
        headers.insert_cache_control_must_revalidate();
        headers.insert_etag(etag);
        response
    }

    fn new_bytes(value: Vec<u8>, etag: &str) -> Self {
        Self::new_bytes_with(StatusCode::default(), value, etag)
    }

    fn new_bytes_with(status: StatusCode, value: Vec<u8>, etag: &str) -> Self {
        let mut response = Response::builder().body(value.into()).unwrap();
        *response.status_mut() = status;
        let headers = response.headers_mut();
        headers.insert_cors();
        headers.insert_content_type_octet_stream();
        headers.insert_cache_control_must_revalidate();
        headers.insert_etag(etag);
        response
    }

    fn new_json_cached<T>(value: T, params: &CacheParams) -> Self
    where
        T: Serialize,
    {
        let bytes = serde_json::to_vec(&value).unwrap();
        let mut response = Response::builder().body(bytes.into()).unwrap();
        let headers = response.headers_mut();
        headers.insert_cors();
        headers.insert_content_type_application_json();
        headers.insert_cache_control(&params.cache_control);
        if let Some(etag) = &params.etag {
            headers.insert_etag(etag);
        }
        response
    }

    fn new_text_cached(value: &str, params: &CacheParams) -> Self {
        let mut response = Response::builder()
            .body(value.to_string().into())
            .unwrap();
        let headers = response.headers_mut();
        headers.insert_cors();
        headers.insert_content_type_text_plain();
        headers.insert_cache_control(&params.cache_control);
        if let Some(etag) = &params.etag {
            headers.insert_etag(etag);
        }
        response
    }

    fn new_bytes_cached(value: Vec<u8>, params: &CacheParams) -> Self {
        let mut response = Response::builder().body(value.into()).unwrap();
        let headers = response.headers_mut();
        headers.insert_cors();
        headers.insert_content_type_octet_stream();
        headers.insert_cache_control(&params.cache_control);
        if let Some(etag) = &params.etag {
            headers.insert_etag(etag);
        }
        response
    }
}
