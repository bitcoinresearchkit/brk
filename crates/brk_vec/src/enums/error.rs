use std::{
    fmt::{self, Debug},
    io,
};

use crate::Version;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
pub enum Error {
    WrongEndian,
    DifferentVersion { found: Version, expected: Version },
    MmapsVecIsTooSmall,
    IO(io::Error),
    ZeroCopyError,
    IndexTooHigh,
    EmptyVec,
    IndexTooLow,
    ExpectFileToHaveIndex,
    ExpectVecToHaveIndex,
    FailedKeyTryIntoUsize,
    UnsupportedUnflushedState,
    RangeFromAfterTo(usize, usize),
    DifferentCompressionMode,
    ToSerdeJsonValueError(serde_json::Error),
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::IO(value)
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
        Self::ToSerdeJsonValueError(error)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::WrongEndian => write!(f, "Wrong endian"),
            Error::DifferentVersion { found, expected } => {
                write!(
                    f,
                    "Different version; found: {found:?}, expected: {expected:?}"
                )
            }
            Error::MmapsVecIsTooSmall => write!(f, "Mmaps vec is too small"),
            Error::IO(error) => Debug::fmt(&error, f),
            Error::IndexTooHigh => write!(f, "Index too high"),
            Error::IndexTooLow => write!(f, "Index too low"),
            Error::ExpectFileToHaveIndex => write!(f, "Expect file to have index"),
            Error::ExpectVecToHaveIndex => write!(f, "Expect vec to have index"),
            Error::FailedKeyTryIntoUsize => write!(f, "Failed to convert key to usize"),
            Error::UnsupportedUnflushedState => {
                write!(
                    f,
                    "Unsupported unflush state, please flush before using this function"
                )
            }
            Error::ZeroCopyError => write!(f, "Zero copy convert error"),
            Error::RangeFromAfterTo(from, to) => write!(f, "Range, from {from} is after to {to}"),
            Error::DifferentCompressionMode => write!(f, "Different compression mode chosen"),
            Error::EmptyVec => write!(f, "The Vec is empty, maybe wait for a bit"),
            Error::ToSerdeJsonValueError(error) => Debug::fmt(&error, f),
        }
    }
}

impl std::error::Error for Error {}
