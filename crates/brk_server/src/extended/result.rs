use axum::{http::StatusCode, response::Response};
use brk_error::{Error, Result};
use serde::Serialize;

use crate::extended::ResponseExtended;

pub trait ResultExtended<T> {
    fn with_status(self) -> Result<T, (StatusCode, String)>;
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
    fn with_status(self) -> Result<T, (StatusCode, String)> {
        self.map_err(|e| {
            (
                match e {
                    Error::InvalidTxid
                    | Error::InvalidNetwork
                    | Error::InvalidAddress
                    | Error::UnsupportedType(_) => StatusCode::BAD_REQUEST,
                    Error::UnknownAddress | Error::UnknownTxid => StatusCode::NOT_FOUND,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                },
                e.to_string(),
            )
        })
    }

    fn to_json_response(self, etag: &str) -> Response
    where
        T: Serialize,
    {
        match self.with_status() {
            Ok(value) => Response::new_json(&value, etag),
            Err((status, message)) => Response::new_json_with(status, &message, etag),
        }
    }

    fn to_text_response(self, etag: &str) -> Response
    where
        T: AsRef<str>,
    {
        match self.with_status() {
            Ok(value) => Response::new_text(value.as_ref(), etag),
            Err((status, message)) => Response::new_text_with(status, &message, etag),
        }
    }

    fn to_bytes_response(self, etag: &str) -> Response
    where
        T: Into<Vec<u8>>,
    {
        match self.with_status() {
            Ok(value) => Response::new_bytes(value.into(), etag),
            Err((status, message)) => Response::new_bytes_with(status, message.into_bytes(), etag),
        }
    }
}
