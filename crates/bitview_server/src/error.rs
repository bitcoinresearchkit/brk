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

fn error_details(error: &BrkError) -> (StatusCode, &'static str) {
    match error {
        BrkError::InvalidAddr => (StatusCode::BAD_REQUEST, "invalid_addr"),
        BrkError::InvalidTxid => (StatusCode::BAD_REQUEST, "invalid_txid"),
        BrkError::InvalidNetwork => (StatusCode::BAD_REQUEST, "invalid_network"),
        BrkError::UnsupportedType(_) => (StatusCode::BAD_REQUEST, "unsupported_type"),
        BrkError::Parse(_) => (StatusCode::BAD_REQUEST, "parse_error"),
        BrkError::NoSeries => (StatusCode::BAD_REQUEST, "no_series"),
        BrkError::SeriesUnsupportedIndex { .. } => {
            (StatusCode::BAD_REQUEST, "series_unsupported_index")
        }
        BrkError::WeightExceeded { .. } => (StatusCode::BAD_REQUEST, "weight_exceeded"),
        BrkError::TooManyUtxos => (StatusCode::BAD_REQUEST, "too_many_utxos"),
        BrkError::UnknownAddr => (StatusCode::NOT_FOUND, "unknown_addr"),
        BrkError::UnknownTxid => (StatusCode::NOT_FOUND, "unknown_txid"),
        BrkError::NotFound(_) => (StatusCode::NOT_FOUND, "not_found"),
        BrkError::OutOfRange(_) => (StatusCode::NOT_FOUND, "out_of_range"),
        BrkError::UnindexableDate => (StatusCode::NOT_FOUND, "unindexable_date"),
        BrkError::NoData => (StatusCode::NOT_FOUND, "no_data"),
        BrkError::SeriesNotFound(_) => (StatusCode::NOT_FOUND, "series_not_found"),
        BrkError::MempoolNotAvailable => (StatusCode::SERVICE_UNAVAILABLE, "mempool_not_available"),
        BrkError::StateUpdating => (StatusCode::SERVICE_UNAVAILABLE, "state_updating"),
        BrkError::AuthFailed => (StatusCode::FORBIDDEN, "auth_failed"),
        _ => (StatusCode::INTERNAL_SERVER_ERROR, "internal_error"),
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

    pub fn not_found(msg: impl Into<String>) -> Self {
        Self::new(StatusCode::NOT_FOUND, "not_found", msg)
    }

    pub fn internal(msg: impl Into<String>) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, "internal_error", msg)
    }

    #[cfg(any(feature = "chain", feature = "series", feature = "urpd", test))]
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
}

impl From<BrkError> for Error {
    fn from(e: BrkError) -> Self {
        let (status, code) = error_details(&e);
        Self {
            status,
            code,
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
        let body = to_vec(&ErrorBody::new(
            error_type(self.status),
            self.code,
            self.message,
            DOC_URL,
        ))
        .unwrap();
        let mut response = (
            self.status,
            [(header::CONTENT_TYPE, "application/problem+json")],
            body,
        )
            .into_response();
        let availability = match self.code {
            "state_updating" => Some(ReadAvailability::Publication),
            "overloaded" => Some(ReadAvailability::Capacity),
            _ => None,
        };
        if let Some(availability) = availability {
            response
                .headers_mut()
                .insert(header::RETRY_AFTER, HeaderValue::from_static("1"));
            response.extensions_mut().insert(availability);
        }
        CacheParams::apply_error_cache_control(response.headers_mut(), policy);
        response
    }
}

#[cfg(test)]
#[path = "../tests/unit/error.rs"]
mod tests;
