use axum::{body::Body, http::Response, response::IntoResponse};
use reqwest::StatusCode;

use super::header_map::HeaderMapExtended;

pub trait ResponseExtended
where
    Self: Sized,
{
    fn new_not_modified() -> Self;
}

impl ResponseExtended for Response<Body> {
    fn new_not_modified() -> Response<Body> {
        let mut response = (StatusCode::NOT_MODIFIED, "").into_response();
        let headers = response.headers_mut();
        headers.insert_cors();
        response
    }
}
