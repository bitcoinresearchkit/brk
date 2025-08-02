use std::{
    fmt::{self, Debug},
    io, result, time,
};

use crate::Version;

pub type Result<T, E = Error> = result::Result<T, E>;

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    SerdeJson(serde_json::Error),
    SystemTimeError(time::SystemTimeError),
    PCO(pco::errors::PcoError),
    ZeroCopyError,

    WrongEndian,
    DifferentVersion { found: Version, expected: Version },
    IndexTooHigh,
    ExpectVecToHaveIndex,
    FailedKeyTryIntoUsize,
    DifferentCompressionMode,
    WrongLength,
    Str(&'static str),
    String(String),
}

impl From<time::SystemTimeError> for Error {
    fn from(value: time::SystemTimeError) -> Self {
        Self::SystemTimeError(value)
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::IO(value)
    }
}

impl From<pco::errors::PcoError> for Error {
    fn from(value: pco::errors::PcoError) -> Self {
        Self::PCO(value)
    }
}

impl<A, B, C> From<zerocopy::error::ConvertError<A, B, C>> for Error {
    fn from(_: zerocopy::error::ConvertError<A, B, C>) -> Self {
        Self::ZeroCopyError
    }
}

impl<A, B> From<zerocopy::error::SizeError<A, B>> for Error {
    fn from(_: zerocopy::error::SizeError<A, B>) -> Self {
        Self::ZeroCopyError
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self::SerdeJson(error)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::IO(error) => Debug::fmt(&error, f),
            Error::PCO(error) => Debug::fmt(&error, f),
            Error::SystemTimeError(error) => Debug::fmt(&error, f),
            Error::SerdeJson(error) => Debug::fmt(&error, f),
            Error::ZeroCopyError => write!(f, "ZeroCopy error"),

            Error::WrongEndian => write!(f, "Wrong endian"),
            Error::DifferentVersion { found, expected } => {
                write!(
                    f,
                    "Different version found: {found:?}, expected: {expected:?}"
                )
            }
            Error::IndexTooHigh => write!(f, "Index too high"),
            Error::ExpectVecToHaveIndex => write!(f, "Expect vec to have index"),
            Error::FailedKeyTryIntoUsize => write!(f, "Failed to convert key to usize"),
            Error::DifferentCompressionMode => write!(f, "Different compression mode chosen"),
            Error::WrongLength => write!(f, "Wrong length"),
            Error::Str(s) => write!(f, "{s}"),
            Error::String(s) => write!(f, "{s}"),
        }
    }
}

impl std::error::Error for Error {}
