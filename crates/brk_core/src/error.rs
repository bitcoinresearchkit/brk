use std::{
    fmt::{self, Debug},
    io,
};

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    Jiff(jiff::Error),
    ZeroCopyError,
    WrongLength,
    WrongAddressType,
    UnindexableDate,
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

impl<A, B> From<zerocopy::error::SizeError<A, B>> for Error {
    fn from(_: zerocopy::error::SizeError<A, B>) -> Self {
        Self::ZeroCopyError
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::IO(error) => Debug::fmt(&error, f),
            Error::Jiff(error) => Debug::fmt(&error, f),
            Error::ZeroCopyError => write!(f, "Zero copy convert error"),
            Error::WrongLength => write!(f, "Wrong length"),
            Error::WrongAddressType => write!(f, "Wrong address type"),
            Error::UnindexableDate => write!(f, "Date cannot be indexed, must be 2009-01-03, 2009-01-09 or greater"),
        }
    }
}

impl std::error::Error for Error {}
