#![doc = include_str!("../README.md")]

use std::{io, path::PathBuf, result, time};

use thiserror::Error;

pub type Result<T, E = Error> = result::Result<T, E>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    IO(#[from] io::Error),

    #[error(transparent)]
    BitcoinRPC(#[from] bitcoincore_rpc::Error),

    #[error(transparent)]
    Jiff(#[from] jiff::Error),

    #[error(transparent)]
    Fjall(#[from] fjall::Error),

    #[error(transparent)]
    VecDB(#[from] vecdb::Error),

    #[error(transparent)]
    RawDB(#[from] vecdb::RawDBError),

    #[error(transparent)]
    Minreq(#[from] minreq::Error),

    #[error(transparent)]
    SystemTimeError(#[from] time::SystemTimeError),

    #[error(transparent)]
    BitcoinConsensusEncode(#[from] bitcoin::consensus::encode::Error),

    #[error(transparent)]
    BitcoinBip34Error(#[from] bitcoin::block::Bip34Error),

    #[error(transparent)]
    BitcoinHexError(#[from] bitcoin::consensus::encode::FromHexError),

    #[error(transparent)]
    BitcoinFromScriptError(#[from] bitcoin::address::FromScriptError),

    #[error(transparent)]
    BitcoinHexToArrayError(#[from] bitcoin::hex::HexToArrayError),

    #[error(transparent)]
    SerdeJSON(#[from] serde_json::Error),

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
    /// Returns true if this network/fetch error indicates a permanent/blocking condition
    /// that won't be resolved by retrying (e.g., DNS failure, connection refused, blocked endpoint).
    /// Returns false for transient errors worth retrying (timeouts, rate limits, server errors).
    pub fn is_network_permanently_blocked(&self) -> bool {
        match self {
            Error::Minreq(e) => is_minreq_error_permanent(e),
            Error::IO(e) => is_io_error_permanent(e),
            // 403 Forbidden suggests IP/geo blocking; 429 and 5xx are transient
            Error::HttpStatus { status, .. } => *status == 403,
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
