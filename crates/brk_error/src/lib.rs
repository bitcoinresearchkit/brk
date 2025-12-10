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
    RawDB(vecdb::RawDBError),
    Minreq(minreq::Error),
    SystemTimeError(time::SystemTimeError),
    BitcoinConsensusEncode(bitcoin::consensus::encode::Error),
    BitcoinBip34Error(bitcoin::block::Bip34Error),
    BitcoinHexError(bitcoin::consensus::encode::FromHexError),
    BitcoinFromScriptError(bitcoin::address::FromScriptError),
    BitcoinHexToArrayError(bitcoin::hex::HexToArrayError),
    SerdeJSON(serde_json::Error),
    TokioJoin(tokio::task::JoinError),
    ZeroCopyError,
    Vecs(vecdb::Error),

    WrongLength { expected: usize, received: usize },
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
    #[inline]
    fn from(value: bitcoin::block::Bip34Error) -> Self {
        Self::BitcoinBip34Error(value)
    }
}

impl From<bitcoin::consensus::encode::Error> for Error {
    #[inline]
    fn from(value: bitcoin::consensus::encode::Error) -> Self {
        Self::BitcoinConsensusEncode(value)
    }
}

impl From<bitcoin::consensus::encode::FromHexError> for Error {
    #[inline]
    fn from(value: bitcoin::consensus::encode::FromHexError) -> Self {
        Self::BitcoinHexError(value)
    }
}

impl From<bitcoin::hex::HexToArrayError> for Error {
    #[inline]
    fn from(value: bitcoin::hex::HexToArrayError) -> Self {
        Self::BitcoinHexToArrayError(value)
    }
}

impl From<bitcoin::address::FromScriptError> for Error {
    #[inline]
    fn from(value: bitcoin::address::FromScriptError) -> Self {
        Self::BitcoinFromScriptError(value)
    }
}

impl From<time::SystemTimeError> for Error {
    #[inline]
    fn from(value: time::SystemTimeError) -> Self {
        Self::SystemTimeError(value)
    }
}

impl From<serde_json::Error> for Error {
    #[inline]
    fn from(error: serde_json::Error) -> Self {
        Self::SerdeJSON(error)
    }
}

impl From<tokio::task::JoinError> for Error {
    #[inline]
    fn from(error: tokio::task::JoinError) -> Self {
        Self::TokioJoin(error)
    }
}

impl From<io::Error> for Error {
    #[inline]
    fn from(value: io::Error) -> Self {
        Self::IO(value)
    }
}

impl From<vecdb::Error> for Error {
    #[inline]
    fn from(value: vecdb::Error) -> Self {
        Self::VecDB(value)
    }
}

impl From<vecdb::RawDBError> for Error {
    #[inline]
    fn from(value: vecdb::RawDBError) -> Self {
        Self::RawDB(value)
    }
}

impl From<bitcoincore_rpc::Error> for Error {
    #[inline]
    fn from(value: bitcoincore_rpc::Error) -> Self {
        Self::BitcoinRPC(value)
    }
}

impl From<minreq::Error> for Error {
    #[inline]
    fn from(value: minreq::Error) -> Self {
        Self::Minreq(value)
    }
}

impl From<jiff::Error> for Error {
    #[inline]
    fn from(value: jiff::Error) -> Self {
        Self::Jiff(value)
    }
}

impl From<fjall::Error> for Error {
    #[inline]
    fn from(value: fjall::Error) -> Self {
        Self::Fjall(value)
    }
}

impl From<&'static str> for Error {
    #[inline]
    fn from(value: &'static str) -> Self {
        Self::Str(value)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::BitcoinConsensusEncode(error) => Display::fmt(&error, f),
            Error::BitcoinBip34Error(error) => Display::fmt(&error, f),
            Error::BitcoinFromScriptError(error) => Display::fmt(&error, f),
            Error::BitcoinHexError(error) => Display::fmt(&error, f),
            Error::BitcoinHexToArrayError(error) => Display::fmt(&error, f),
            Error::BitcoinRPC(error) => Display::fmt(&error, f),
            Error::Fjall(error) => Display::fmt(&error, f),
            Error::IO(error) => Display::fmt(&error, f),
            Error::Jiff(error) => Display::fmt(&error, f),
            Error::Minreq(error) => Display::fmt(&error, f),
            Error::RawDB(error) => Display::fmt(&error, f),
            Error::SerdeJSON(error) => Display::fmt(&error, f),
            Error::SystemTimeError(error) => Display::fmt(&error, f),
            Error::TokioJoin(error) => Display::fmt(&error, f),
            Error::VecDB(error) => Display::fmt(&error, f),
            Error::Vecs(error) => Display::fmt(&error, f),
            Error::ZeroCopyError => write!(f, "ZeroCopy error"),
            Error::WrongLength { expected, received } => write!(
                f,
                "Wrong length, expected: {expected}, received: {received}"
            ),
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

impl Error {
    /// Returns true if this network/fetch error indicates a permanent/blocking condition
    /// that won't be resolved by retrying (e.g., DNS failure, connection refused, blocked endpoint).
    /// Returns false for transient errors worth retrying (timeouts, rate limits, server errors).
    pub fn is_network_permanently_blocked(&self) -> bool {
        match self {
            Error::Minreq(e) => is_minreq_error_permanent(e),
            Error::IO(e) => is_io_error_permanent(e),
            // Other errors are data/parsing related, not network - treat as transient
            _ => false,
        }
    }
}

fn is_minreq_error_permanent(e: &minreq::Error) -> bool {
    use minreq::Error::*;
    match e {
        // DNS resolution failure - likely blocked or misconfigured
        IoError(io_err) => is_io_error_permanent(io_err),
        // Check error message for common blocking indicators
        other => {
            let msg = format!("{:?}", other);
            // DNS/connection failures
            msg.contains("nodename nor servname")
                || msg.contains("Name or service not known")
                || msg.contains("No such host")
                || msg.contains("connection refused")
                || msg.contains("Connection refused")
                // SSL/TLS failures (often due to blocking/MITM)
                || msg.contains("certificate")
                || msg.contains("SSL")
                || msg.contains("TLS")
                || msg.contains("handshake")
        }
    }
}

fn is_io_error_permanent(e: &std::io::Error) -> bool {
    use std::io::ErrorKind::*;
    match e.kind() {
        // Permanent errors
        ConnectionRefused | PermissionDenied | AddrNotAvailable => true,
        // Check the error message for DNS failures
        _ => {
            let msg = e.to_string();
            msg.contains("nodename nor servname")
                || msg.contains("Name or service not known")
                || msg.contains("No such host")
        }
    }
}
