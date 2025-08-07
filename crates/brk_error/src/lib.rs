use std::{
    fmt::{self, Debug, Display},
    io, result, time,
};

pub type Result<T, E = Error> = result::Result<T, E>;

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    BitcoinRPC(bitcoincore_rpc::Error),
    Jiff(jiff::Error),
    Fjall(fjall::Error),
    Minreq(minreq::Error),
    SystemTimeError(time::SystemTimeError),
    SerdeJson(serde_json::Error),
    ZeroCopyError,
    Vecs(brk_vecs::Error),

    WrongLength,
    WrongAddressType,
    UnindexableDate,
    QuickCacheError,
    Str(&'static str),
    String(String),
}

impl From<time::SystemTimeError> for Error {
    fn from(value: time::SystemTimeError) -> Self {
        Self::SystemTimeError(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self::SerdeJson(error)
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::IO(value)
    }
}

impl From<bitcoincore_rpc::Error> for Error {
    fn from(value: bitcoincore_rpc::Error) -> Self {
        Self::BitcoinRPC(value)
    }
}

impl From<minreq::Error> for Error {
    fn from(value: minreq::Error) -> Self {
        Self::Minreq(value)
    }
}

impl From<brk_vecs::Error> for Error {
    fn from(value: brk_vecs::Error) -> Self {
        Self::Vecs(value)
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

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::IO(error) => Display::fmt(&error, f),
            Error::Minreq(error) => Display::fmt(&error, f),
            Error::SerdeJson(error) => Display::fmt(&error, f),
            Error::Vecs(error) => Display::fmt(&error, f),
            Error::BitcoinRPC(error) => Display::fmt(&error, f),
            Error::SystemTimeError(error) => Display::fmt(&error, f),
            Error::Jiff(error) => Display::fmt(&error, f),
            Error::Fjall(error) => Display::fmt(&error, f),
            Error::ZeroCopyError => write!(f, "ZeroCopy error"),

            Error::WrongLength => write!(f, "Wrong length"),
            Error::QuickCacheError => write!(f, "Quick cache error"),
            Error::WrongAddressType => write!(f, "Wrong address type"),
            Error::UnindexableDate => write!(
                f,
                "Date cannot be indexed, must be 2009-01-03, 2009-01-09 or greater"
            ),

            Error::Str(s) => write!(f, "{s}"),
            Error::String(s) => write!(f, "{s}"),
        }
    }
}

impl std::error::Error for Error {}
