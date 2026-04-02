use axum::{
    body::Body,
    http::{HeaderMap, Response, StatusCode},
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
    fn new_json_cached<T>(value: T, params: &CacheParams) -> Self
    where
        T: Serialize;
    fn static_json<T>(headers: &HeaderMap, value: T) -> Self
    where
        T: Serialize;
}

impl ResponseExtended for Response<Body> {
    fn new_not_modified() -> Response<Body> {
        let mut response = (StatusCode::NOT_MODIFIED, "").into_response();
        let _headers = response.headers_mut();
        response
    }

    fn new_json_cached<T>(value: T, params: &CacheParams) -> Self
    where
        T: Serialize,
    {
        let bytes = serde_json::to_vec(&value).unwrap();
        let mut response = Response::builder().body(bytes.into()).unwrap();
        let headers = response.headers_mut();
        headers.insert_content_type_application_json();
        headers.insert_cache_control(&params.cache_control);
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
            return Self::new_not_modified();
        }
        Self::new_json_cached(value, &params)
    }
}
