use axum::response::Response;
use brk_error::Result;
use serde::Serialize;

use crate::{Error, extended::ResponseExtended};

pub trait ResultExtended<T> {
    fn to_json_response(self, etag: &str) -> Response
    where
        T: Serialize;
    fn to_text_response(self, etag: &str) -> Response
    where
        T: AsRef<str>;
    fn to_bytes_response(self, etag: &str) -> Response
    where
        T: Into<Vec<u8>>;
}

impl<T> ResultExtended<T> for Result<T> {
    fn to_json_response(self, etag: &str) -> Response
    where
        T: Serialize,
    {
        match self {
            Ok(value) => Response::new_json(&value, etag),
            Err(e) => Error::from(e).into_response_with_etag(etag),
        }
    }

    fn to_text_response(self, etag: &str) -> Response
    where
        T: AsRef<str>,
    {
        match self {
            Ok(value) => Response::new_text(value.as_ref(), etag),
            Err(e) => Error::from(e).into_response_with_etag(etag),
        }
    }

    fn to_bytes_response(self, etag: &str) -> Response
    where
        T: Into<Vec<u8>>,
    {
        match self {
            Ok(value) => Response::new_bytes(value.into(), etag),
            Err(e) => Error::from(e).into_response_with_etag(etag),
        }
    }
}
