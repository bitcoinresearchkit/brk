use axum::{Json, http::StatusCode};
use brk_error::{Error, Result};

pub trait ResultExtended<T> {
    fn to_server_result(self) -> Result<T, (StatusCode, Json<String>)>;
}

impl<T> ResultExtended<T> for Result<T> {
    fn to_server_result(self) -> Result<T, (StatusCode, Json<String>)> {
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
                Json(e.to_string()),
            )
        })
    }
}
