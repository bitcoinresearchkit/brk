use axum::{
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
};

use super::header_map::HeaderMapExtended;

pub trait ResponseExtended
where
    Self: Sized,
{
    fn new_not_modified() -> Self;
    fn new_json_from_bytes(bytes: Vec<u8>) -> Self;
}

impl ResponseExtended for Response<Body> {
    fn new_not_modified() -> Response<Body> {
        let mut response = (StatusCode::NOT_MODIFIED, "").into_response();
        let headers = response.headers_mut();
        headers.insert_cors();
        response
    }

    fn new_json_from_bytes(bytes: Vec<u8>) -> Self {
        Response::builder()
            .header("content-type", "application/json")
            .body(bytes.into())
            .unwrap()
    }
}
