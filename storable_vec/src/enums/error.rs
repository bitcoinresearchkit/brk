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
    UnsafeSliceSerde(unsafe_slice_serde::Error),
    IndexTooHigh,
    IndexTooLow,
    ExpectFileToHaveIndex,
    ExpectVecToHaveIndex,
    FailedKeyTryIntoUsize,
    UnsupportedUnflushedState,
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::IO(value)
    }
}

impl From<unsafe_slice_serde::Error> for Error {
    fn from(value: unsafe_slice_serde::Error) -> Self {
        Self::UnsafeSliceSerde(value)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::WrongEndian => write!(f, "Wrong endian"),
            Error::DifferentVersion { found, expected } => {
                write!(f, "Different version; found: {found:?}, expected: {expected:?}")
            }
            Error::MmapsVecIsTooSmall => write!(f, "Mmaps vec is too small"),
            Error::IO(error) => Debug::fmt(&error, f),
            Error::UnsafeSliceSerde(error) => Debug::fmt(&error, f),
            Error::IndexTooHigh => write!(f, "Index too high"),
            Error::IndexTooLow => write!(f, "Index too low"),
            Error::ExpectFileToHaveIndex => write!(f, "Expect file to have index"),
            Error::ExpectVecToHaveIndex => write!(f, "Expect vec to have index"),
            Error::FailedKeyTryIntoUsize => write!(f, "Failed to convert key to usize"),
            Error::UnsupportedUnflushedState => {
                write!(f, "Unsupported unflush state, please flush before using this function")
            }
        }
    }
}

impl std::error::Error for Error {}
