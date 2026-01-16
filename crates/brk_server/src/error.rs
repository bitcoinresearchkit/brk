use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use brk_error::Error as BrkError;

/// Server result type with Error that implements IntoResponse.
pub type Result<T> = std::result::Result<T, Error>;

/// Server error type that maps to HTTP status codes.
pub struct Error(StatusCode, String);

impl Error {
    pub fn bad_request(msg: impl Into<String>) -> Self {
        Self(StatusCode::BAD_REQUEST, msg.into())
    }

    pub fn forbidden(msg: impl Into<String>) -> Self {
        Self(StatusCode::FORBIDDEN, msg.into())
    }

    pub fn not_found(msg: impl Into<String>) -> Self {
        Self(StatusCode::NOT_FOUND, msg.into())
    }

    pub fn internal(msg: impl Into<String>) -> Self {
        Self(StatusCode::INTERNAL_SERVER_ERROR, msg.into())
    }
}

impl From<BrkError> for Error {
    fn from(e: BrkError) -> Self {
        let status = match &e {
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
            | BrkError::MetricNotFound { .. } => StatusCode::NOT_FOUND,

            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        Self(status, e.to_string())
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (self.0, self.1).into_response()
    }
}
