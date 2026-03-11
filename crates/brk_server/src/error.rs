use axum::{
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};
use brk_error::Error as BrkError;
use schemars::JsonSchema;
use serde::Serialize;

use crate::extended::HeaderMapExtended;

/// Server result type with Error that implements IntoResponse.
pub type Result<T> = std::result::Result<T, Error>;

const DOC_URL: &str = "/api";

#[derive(Serialize, JsonSchema)]
pub(crate) struct ErrorBody {
    error: ErrorDetail,
}

#[derive(Serialize, JsonSchema)]
struct ErrorDetail {
    /// Error category: "invalid_request", "forbidden", "not_found", "unavailable", or "internal"
    #[schemars(with = "String")]
    r#type: &'static str,
    /// Machine-readable error code (e.g. "invalid_address", "metric_not_found")
    #[schemars(with = "String")]
    code: &'static str,
    /// Human-readable description
    message: String,
    /// Link to API documentation
    #[schemars(with = "String")]
    doc_url: &'static str,
}

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
        | BrkError::InvalidAddress
        | BrkError::UnsupportedType(_)
        | BrkError::Parse(_)
        | BrkError::NoMetrics
        | BrkError::MetricUnsupportedIndex { .. }
        | BrkError::WeightExceeded { .. } => StatusCode::BAD_REQUEST,

        BrkError::UnknownAddress
        | BrkError::UnknownTxid
        | BrkError::NotFound(_)
        | BrkError::NoData
        | BrkError::MetricNotFound(_) => StatusCode::NOT_FOUND,

        BrkError::AuthFailed => StatusCode::FORBIDDEN,
        BrkError::MempoolNotAvailable => StatusCode::SERVICE_UNAVAILABLE,

        _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

fn error_code(e: &BrkError) -> &'static str {
    match e {
        BrkError::InvalidAddress => "invalid_address",
        BrkError::InvalidTxid => "invalid_txid",
        BrkError::InvalidNetwork => "invalid_network",
        BrkError::UnsupportedType(_) => "unsupported_type",
        BrkError::Parse(_) => "parse_error",
        BrkError::NoMetrics => "no_metrics",
        BrkError::MetricUnsupportedIndex { .. } => "metric_unsupported_index",
        BrkError::WeightExceeded { .. } => "weight_exceeded",
        BrkError::UnknownAddress => "unknown_address",
        BrkError::UnknownTxid => "unknown_txid",
        BrkError::NotFound(_) => "not_found",
        BrkError::NoData => "no_data",
        BrkError::MetricNotFound(_) => "metric_not_found",
        BrkError::MempoolNotAvailable => "mempool_not_available",
        BrkError::AuthFailed => "auth_failed",
        _ => "internal_error",
    }
}

fn build_error_body(status: StatusCode, code: &'static str, message: String) -> Vec<u8> {
    serde_json::to_vec(&ErrorBody {
        error: ErrorDetail {
            r#type: error_type(status),
            code,
            message,
            doc_url: DOC_URL,
        },
    })
    .unwrap()
}

/// Server error type that maps to HTTP status codes and structured JSON.
pub struct Error {
    status: StatusCode,
    code: &'static str,
    message: String,
}

impl Error {
    pub(crate) fn new(status: StatusCode, code: &'static str, msg: impl Into<String>) -> Self {
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

    pub fn internal(msg: impl Into<String>) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, "internal_error", msg)
    }

    pub(crate) fn into_response_with_etag(self, etag: &str) -> Response {
        let mut response = self.into_response();
        let headers = response.headers_mut();
        headers.insert_etag(etag);
        headers.insert_cache_control_must_revalidate();
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

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let body = build_error_body(self.status, self.code, self.message);
        (
            self.status,
            [(header::CONTENT_TYPE, "application/json")],
            body,
        )
            .into_response()
    }
}
