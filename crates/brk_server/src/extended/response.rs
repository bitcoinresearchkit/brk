use axum::{
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
};
use serde::Serialize;

use super::header_map::HeaderMapExtended;

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
}
