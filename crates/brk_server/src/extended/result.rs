use axum::http::StatusCode;
use brk_error::{Error, Result};

pub trait ResultExtended<T> {
    fn with_status(self) -> Result<T, (StatusCode, String)>;
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
}
