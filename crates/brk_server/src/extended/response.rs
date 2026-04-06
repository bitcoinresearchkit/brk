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
