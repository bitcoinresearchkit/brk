use std::{
    fmt::{self, Debug},
    io,
};

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
pub enum Error {
    MmapsVecIsTooSmall,
    IO(io::Error),
    UnsafeSliceSerde(unsafe_slice_serde::Error),
    IndexTooHigh,
    ExpectFileToHaveIndex,
    ExpectVecToHaveIndex,
    FailedKeyTryIntoUsize,
}

impl fmt::Display for Error {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::MmapsVecIsTooSmall => write!(f, "Mmaps vec is too small"),
            Error::IO(error) => Debug::fmt(&error, f),
            Error::UnsafeSliceSerde(error) => Debug::fmt(&error, f),
            Error::IndexTooHigh => write!(f, "Index too high"),
            Error::ExpectFileToHaveIndex => write!(f, "Expect file to have index"),
            Error::ExpectVecToHaveIndex => write!(f, "Expect vec to have index"),
            Error::FailedKeyTryIntoUsize => write!(f, "Failed to convert key to usize"),
        }
    }
}

impl std::error::Error for Error {}
