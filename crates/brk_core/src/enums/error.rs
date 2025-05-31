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
    Jiff(jiff::Error),
    Fjall(fjall::Error),
    SystemTimeError(time::SystemTimeError),
    ZeroCopyError,

    WrongEndian,
    DifferentVersion { found: Version, expected: Version },
    MmapsVecIsTooSmall,
    IndexTooHigh,
    EmptyVec,
    IndexTooLow,
    ExpectFileToHaveIndex,
    ExpectVecToHaveIndex,
    FailedKeyTryIntoUsize,
    UnsupportedUnflushedState,
    RangeFromAfterTo(usize, usize),
    DifferentCompressionMode,
    WrongLength,
    WrongAddressType,
    UnindexableDate,

    String(&'static str),
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

impl From<jiff::Error> for Error {
    fn from(value: jiff::Error) -> Self {
        Self::Jiff(value)
    }
}

impl From<fjall::Error> for Error {
    fn from(value: fjall::Error) -> Self {
        Self::Fjall(value)
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
            Error::SystemTimeError(error) => Debug::fmt(&error, f),
            Error::SerdeJson(error) => Debug::fmt(&error, f),
            Error::Jiff(error) => Debug::fmt(&error, f),
            Error::Fjall(error) => Debug::fmt(&error, f),
            Error::ZeroCopyError => write!(f, "ZeroCopy error"),

            Error::WrongEndian => write!(f, "Wrong endian"),
            Error::DifferentVersion { found, expected } => {
                write!(
                    f,
                    "Different version; found: {found:?}, expected: {expected:?}"
                )
            }
            Error::MmapsVecIsTooSmall => write!(f, "Mmaps vec is too small"),
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
            Error::RangeFromAfterTo(from, to) => write!(f, "Range, from {from} is after to {to}"),
            Error::DifferentCompressionMode => write!(f, "Different compression mode chosen"),
            Error::EmptyVec => write!(f, "The Vec is empty, maybe wait for a bit"),
            Error::WrongLength => write!(f, "Wrong length"),
            Error::WrongAddressType => write!(f, "Wrong address type"),
            Error::UnindexableDate => write!(
                f,
                "Date cannot be indexed, must be 2009-01-03, 2009-01-09 or greater"
            ),

            Error::String(s) => write!(f, "{s}"),
        }
    }
}

impl std::error::Error for Error {}
