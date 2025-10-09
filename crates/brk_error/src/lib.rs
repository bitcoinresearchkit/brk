#![doc = include_str!("../README.md")]

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
    VecDB(vecdb::Error),
    SeqDB(vecdb::SeqDBError),
    Minreq(minreq::Error),
    SystemTimeError(time::SystemTimeError),
    BitcoinConsensusEncode(bitcoin::consensus::encode::Error),
    BitcoinBip34Error(bitcoin::block::Bip34Error),
    SonicRS(sonic_rs::Error),
    ZeroCopyError,
    Vecs(vecdb::Error),

    WrongLength,
    WrongAddressType,
    UnindexableDate,
    QuickCacheError,

    InvalidAddress,
    InvalidNetwork,
    InvalidTxid,
    UnknownAddress,
    UnknownTxid,
    UnsupportedType(String),

    Str(&'static str),
    String(String),
}

impl From<bitcoin::block::Bip34Error> for Error {
    fn from(value: bitcoin::block::Bip34Error) -> Self {
        Self::BitcoinBip34Error(value)
    }
}

impl From<bitcoin::consensus::encode::Error> for Error {
    fn from(value: bitcoin::consensus::encode::Error) -> Self {
        Self::BitcoinConsensusEncode(value)
    }
}

impl From<time::SystemTimeError> for Error {
    fn from(value: time::SystemTimeError) -> Self {
        Self::SystemTimeError(value)
    }
}

impl From<sonic_rs::Error> for Error {
    fn from(error: sonic_rs::Error) -> Self {
        Self::SonicRS(error)
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::IO(value)
    }
}

impl From<vecdb::Error> for Error {
    fn from(value: vecdb::Error) -> Self {
        Self::VecDB(value)
    }
}

impl From<vecdb::SeqDBError> for Error {
    fn from(value: vecdb::SeqDBError) -> Self {
        Self::SeqDB(value)
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

impl From<&'static str> for Error {
    fn from(value: &'static str) -> Self {
        Self::Str(value)
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
            Error::BitcoinConsensusEncode(error) => Display::fmt(&error, f),
            Error::BitcoinBip34Error(error) => Display::fmt(&error, f),
            Error::BitcoinRPC(error) => Display::fmt(&error, f),
            Error::Fjall(error) => Display::fmt(&error, f),
            Error::IO(error) => Display::fmt(&error, f),
            Error::Jiff(error) => Display::fmt(&error, f),
            Error::Minreq(error) => Display::fmt(&error, f),
            Error::SeqDB(error) => Display::fmt(&error, f),
            Error::SonicRS(error) => Display::fmt(&error, f),
            Error::SystemTimeError(error) => Display::fmt(&error, f),
            Error::VecDB(error) => Display::fmt(&error, f),
            Error::Vecs(error) => Display::fmt(&error, f),
            Error::ZeroCopyError => write!(f, "ZeroCopy error"),

            Error::WrongLength => write!(f, "Wrong length"),
            Error::QuickCacheError => write!(f, "Quick cache error"),
            Error::WrongAddressType => write!(f, "Wrong address type"),
            Error::UnindexableDate => write!(
                f,
                "Date cannot be indexed, must be 2009-01-03, 2009-01-09 or greater"
            ),

            Error::InvalidTxid => write!(f, "The provided TXID appears to be invalid"),
            Error::InvalidNetwork => write!(f, "Invalid network"),
            Error::InvalidAddress => write!(f, "The provided address appears to be invalid"),
            Error::UnknownAddress => write!(
                f,
                "Address not found in the blockchain (no transaction history)"
            ),
            Error::UnknownTxid => write!(f, "Failed to find the TXID in the blockchain"),
            Error::UnsupportedType(t) => write!(f, "Unsupported type ({t})"),

            Error::Str(s) => write!(f, "{s}"),
            Error::String(s) => write!(f, "{s}"),
        }
    }
}

impl std::error::Error for Error {}
