#![doc = include_str!("../README.md")]

use std::{io, path::PathBuf, result, time};

use thiserror::Error;

pub type Result<T, E = Error> = result::Result<T, E>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    IO(#[from] io::Error),

    #[cfg(feature = "bitcoincore-rpc")]
    #[error(transparent)]
    BitcoinRPC(#[from] bitcoincore_rpc::Error),

    #[cfg(feature = "corepc")]
    #[error(transparent)]
    CorepcRPC(#[from] corepc_client::client_sync::Error),

    #[cfg(feature = "jiff")]
    #[error(transparent)]
    Jiff(#[from] jiff::Error),

    #[cfg(feature = "fjall")]
    #[error(transparent)]
    Fjall(#[from] fjall::Error),

    #[cfg(feature = "vecdb")]
    #[error(transparent)]
    VecDB(#[from] vecdb::Error),

    #[cfg(feature = "vecdb")]
    #[error(transparent)]
    RawDB(#[from] vecdb::RawDBError),

    #[cfg(feature = "minreq")]
    #[error(transparent)]
    Minreq(#[from] minreq::Error),

    #[error(transparent)]
    SystemTimeError(#[from] time::SystemTimeError),

    #[cfg(feature = "bitcoin")]
    #[error(transparent)]
    BitcoinConsensusEncode(#[from] bitcoin::consensus::encode::Error),

    #[cfg(feature = "bitcoin")]
    #[error(transparent)]
    BitcoinBip34Error(#[from] bitcoin::block::Bip34Error),

    #[cfg(feature = "bitcoin")]
    #[error(transparent)]
    BitcoinHexError(#[from] bitcoin::consensus::encode::FromHexError),

    #[cfg(feature = "bitcoin")]
    #[error(transparent)]
    BitcoinFromScriptError(#[from] bitcoin::address::FromScriptError),

    #[cfg(feature = "bitcoin")]
    #[error(transparent)]
    BitcoinHexToArrayError(#[from] bitcoin::hex::HexToArrayError),

    #[cfg(feature = "pco")]
    #[error(transparent)]
    Pco(#[from] pco::errors::PcoError),

    #[cfg(feature = "serde_json")]
    #[error(transparent)]
    SerdeJSON(#[from] serde_json::Error),

    #[cfg(feature = "tokio")]
    #[error(transparent)]
    TokioJoin(#[from] tokio::task::JoinError),

    #[error("ZeroCopy error")]
    ZeroCopyError,

    #[error("Wrong length, expected: {expected}, received: {received}")]
    WrongLength { expected: usize, received: usize },

    #[error("Wrong address type")]
    WrongAddressType,

    #[error("Date cannot be indexed, must be 2009-01-03, 2009-01-09 or greater")]
    UnindexableDate,

    #[error("Quick cache error")]
    QuickCacheError,

    #[error("The provided address appears to be invalid")]
    InvalidAddress,

    #[error("Invalid network")]
    InvalidNetwork,

    #[error("The provided TXID appears to be invalid")]
    InvalidTxid,

    #[error("Mempool data is not available")]
    MempoolNotAvailable,

    #[error("Address not found in the blockchain (no transaction history)")]
    UnknownAddress,

    #[error("Failed to find the TXID in the blockchain")]
    UnknownTxid,

    #[error("Unsupported type ({0})")]
    UnsupportedType(String),

    // Generic errors with context
    #[error("{0}")]
    NotFound(String),

    #[error("{0}")]
    OutOfRange(String),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Internal error: {0}")]
    Internal(&'static str),

    #[error("Authentication failed")]
    AuthFailed,

    // Metric-specific errors
    #[error("'{metric}' not found{}", suggestion.as_ref().map(|s| format!(", did you mean '{s}'?")).unwrap_or_default())]
    MetricNotFound {
        metric: String,
        suggestion: Option<String>,
    },

    #[error("'{metric}' doesn't support the requested index. Supported indexes: {supported}")]
    MetricUnsupportedIndex { metric: String, supported: String },

    #[error("No metrics specified")]
    NoMetrics,

    #[error("Request weight {requested} exceeds maximum {max}")]
    WeightExceeded { requested: usize, max: usize },

    #[error("Fetch failed after retries: {0}")]
    FetchFailed(String),

    #[error("HTTP {status}: {url}")]
    HttpStatus { status: u16, url: String },

    #[error("Version mismatch at {path:?}: expected {expected}, found {found}")]
    VersionMismatch {
        path: PathBuf,
        expected: usize,
        found: usize,
    },
}


impl Error {
    /// Returns true if this error is due to a file lock (another process has the database open).
    /// Lock errors are transient and should not trigger data deletion.
    #[cfg(feature = "vecdb")]
    pub fn is_lock_error(&self) -> bool {
        matches!(self, Error::VecDB(e) if e.is_lock_error())
    }

    /// Returns true if this error indicates data corruption or version incompatibility.
    /// These errors may require resetting/deleting the data to recover.
    #[cfg(feature = "vecdb")]
    pub fn is_data_error(&self) -> bool {
        matches!(self, Error::VecDB(e) if e.is_data_error())
            || matches!(self, Error::VersionMismatch { .. })
    }

    /// Returns true if this network/fetch error indicates a permanent/blocking condition
    /// that won't be resolved by retrying (e.g., DNS failure, connection refused, blocked endpoint).
    /// Returns false for transient errors worth retrying (timeouts, rate limits, server errors).
    pub fn is_network_permanently_blocked(&self) -> bool {
        match self {
            #[cfg(feature = "minreq")]
            Error::Minreq(e) => is_minreq_error_permanent(e),
            Error::IO(e) => is_io_error_permanent(e),
            // 403 Forbidden suggests IP/geo blocking; 429 and 5xx are transient
            Error::HttpStatus { status, .. } => *status == 403,
            // Other errors are data/parsing related, not network - treat as transient
            _ => false,
        }
    }
}

#[cfg(feature = "minreq")]
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
