use std::{
    fmt::{self, Debug},
    io,
};

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    #[cfg(feature = "zerocopy")]
    ZeroCopyError,
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::IO(value)
    }
}

#[cfg(feature = "zerocopy")]
impl<A, B> From<zerocopy::error::SizeError<A, B>> for Error {
    fn from(_: zerocopy::error::SizeError<A, B>) -> Self {
        Self::ZeroCopyError
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::IO(error) => Debug::fmt(&error, f),
            #[cfg(feature = "zerocopy")]
            Error::ZeroCopyError => write!(f, "Zero copy convert error"),
        }
    }
}

impl std::error::Error for Error {}
