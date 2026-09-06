use std::result::Result as StdResult;

use aide::OperationOutput;
use axum::{
    http::{HeaderValue, StatusCode, header},
    response::{IntoResponse, Response},
};
use brk_error::Error as BrkError;
use serde_json::to_vec;

use crate::{
    cache::{CacheParams, ErrorCachePolicy},
    error_body::ErrorBody,
    read_availability::ReadAvailability,
};

const DOC_URL: &str = "/api";

pub type Result<T> = StdResult<T, Error>;

fn error_type(status: StatusCode) -> &'static str {
    match status {
        StatusCode::BAD_REQUEST => "invalid_request",
        StatusCode::FORBIDDEN => "forbidden",
        StatusCode::NOT_FOUND => "not_found",
        StatusCode::SERVICE_UNAVAILABLE => "unavailable",
        _ => "internal",
    }
}

fn error_status(e: &BrkError) -> StatusCode {
    match e {
        BrkError::InvalidTxid
        | BrkError::InvalidNetwork
        | BrkError::InvalidAddr
        | BrkError::UnsupportedType(_)
        | BrkError::Parse(_)
        | BrkError::NoSeries
        | BrkError::SeriesUnsupportedIndex { .. }
        | BrkError::WeightExceeded { .. }
        | BrkError::TooManyUtxos => StatusCode::BAD_REQUEST,

        BrkError::UnknownAddr
        | BrkError::UnknownTxid
        | BrkError::NotFound(_)
        | BrkError::NoData
        | BrkError::OutOfRange(_)
        | BrkError::UnindexableDate
        | BrkError::SeriesNotFound(_) => StatusCode::NOT_FOUND,

        BrkError::AuthFailed => StatusCode::FORBIDDEN,
        BrkError::MempoolNotAvailable | BrkError::StateUpdating => StatusCode::SERVICE_UNAVAILABLE,

        _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

fn error_code(e: &BrkError) -> &'static str {
    match e {
        BrkError::InvalidAddr => "invalid_addr",
        BrkError::InvalidTxid => "invalid_txid",
        BrkError::InvalidNetwork => "invalid_network",
        BrkError::UnsupportedType(_) => "unsupported_type",
        BrkError::Parse(_) => "parse_error",
        BrkError::NoSeries => "no_series",
        BrkError::SeriesUnsupportedIndex { .. } => "series_unsupported_index",
        BrkError::WeightExceeded { .. } => "weight_exceeded",
        BrkError::TooManyUtxos => "too_many_utxos",
        BrkError::UnknownAddr => "unknown_addr",
        BrkError::UnknownTxid => "unknown_txid",
        BrkError::NotFound(_) => "not_found",
        BrkError::OutOfRange(_) => "out_of_range",
        BrkError::UnindexableDate => "unindexable_date",
        BrkError::NoData => "no_data",
        BrkError::SeriesNotFound(_) => "series_not_found",
        BrkError::MempoolNotAvailable => "mempool_not_available",
        BrkError::StateUpdating => "state_updating",
        BrkError::AuthFailed => "auth_failed",
        _ => "internal_error",
    }
}

fn build_error_body(status: StatusCode, code: &'static str, message: String) -> Vec<u8> {
    to_vec(&ErrorBody::new(error_type(status), code, message, DOC_URL)).unwrap()
}

fn apply_retry_after(code: &str, response: &mut Response) {
    if matches!(code, "state_updating" | "overloaded") {
        response
            .headers_mut()
            .insert(header::RETRY_AFTER, HeaderValue::from_static("1"));
    }
}

/// Server error type that maps to HTTP status codes and structured JSON.
pub struct Error {
    status: StatusCode,
    code: &'static str,
    message: String,
}

impl Error {
    pub fn new(status: StatusCode, code: &'static str, msg: impl Into<String>) -> Self {
        Self {
            status,
            code,
            message: msg.into(),
        }
    }

    pub fn bad_request(msg: impl Into<String>) -> Self {
        Self::new(StatusCode::BAD_REQUEST, "bad_request", msg)
    }

    pub fn forbidden(msg: impl Into<String>) -> Self {
        Self::new(StatusCode::FORBIDDEN, "forbidden", msg)
    }

    pub fn not_found(msg: impl Into<String>) -> Self {
        Self::new(StatusCode::NOT_FOUND, "not_found", msg)
    }

    pub fn not_implemented(msg: impl Into<String>) -> Self {
        Self::new(StatusCode::NOT_IMPLEMENTED, "not_implemented", msg)
    }

    pub fn internal(msg: impl Into<String>) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, "internal_error", msg)
    }

    pub fn overloaded(msg: impl Into<String>) -> Self {
        Self::new(StatusCode::SERVICE_UNAVAILABLE, "overloaded", msg)
    }

    fn cache_policy(&self) -> ErrorCachePolicy {
        match self.code {
            "invalid_addr" | "invalid_network" | "invalid_txid" => ErrorCachePolicy::Immutable,
            _ => match self.status {
                StatusCode::BAD_REQUEST | StatusCode::NOT_FOUND => ErrorCachePolicy::Revalidate,
                _ => ErrorCachePolicy::NoStore,
            },
        }
    }

    fn build_response(self) -> Response {
        let body = build_error_body(self.status, self.code, self.message);
        let mut response = (
            self.status,
            [(header::CONTENT_TYPE, "application/problem+json")],
            body,
        )
            .into_response();
        apply_retry_after(self.code, &mut response);
        match self.code {
            "state_updating" => {
                response
                    .extensions_mut()
                    .insert(ReadAvailability::Publication);
            }
            "overloaded" => {
                response.extensions_mut().insert(ReadAvailability::Capacity);
            }
            _ => {}
        }
        response
    }
}

impl From<BrkError> for Error {
    fn from(e: BrkError) -> Self {
        Self {
            status: error_status(&e),
            code: error_code(&e),
            message: e.to_string(),
        }
    }
}

impl OperationOutput for Error {
    type Inner = ();
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let policy = self.cache_policy();
        let mut response = self.build_response();
        CacheParams::apply_error_cache_control(response.headers_mut(), policy);
        response
    }
}

#[cfg(test)]
#[path = "../tests/unit/error.rs"]
mod tests;
