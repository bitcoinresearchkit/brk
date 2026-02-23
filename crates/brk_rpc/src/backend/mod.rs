use std::path::PathBuf;

use bitcoin::ScriptBuf;
use brk_types::Sats;

#[derive(Debug, Clone)]
pub struct BlockchainInfo {
    pub headers: u64,
    pub blocks: u64,
}

#[derive(Debug, Clone)]
pub struct BlockInfo {
    pub height: usize,
    pub confirmations: i64,
}

#[derive(Debug, Clone)]
pub struct BlockHeaderInfo {
    pub height: usize,
    pub confirmations: i64,
    pub previous_block_hash: Option<bitcoin::BlockHash>,
}

#[derive(Debug, Clone)]
pub struct TxOutInfo {
    pub coinbase: bool,
    pub value: Sats,
    pub script_pub_key: ScriptBuf,
}

#[derive(Debug, Clone)]
pub struct RawMempoolEntry {
    pub vsize: u64,
    pub weight: u64,
    pub base_fee_sats: u64,
    pub ancestor_count: u64,
    pub ancestor_size: u64,
    pub ancestor_fee_sats: u64,
    pub depends: Vec<bitcoin::Txid>,
}

#[derive(Clone, Debug)]
pub enum Auth {
    None,
    UserPass(String, String),
    CookieFile(PathBuf),
}

#[cfg(feature = "bitcoincore-rpc")]
pub mod bitcoincore;

#[cfg(feature = "corepc")]
pub mod corepc;

// Default ClientInner: prefer bitcoincore-rpc when both are enabled
#[cfg(feature = "bitcoincore-rpc")]
pub use bitcoincore::ClientInner;

#[cfg(all(feature = "corepc", not(feature = "bitcoincore-rpc")))]
pub use corepc::ClientInner;

#[cfg(not(any(feature = "bitcoincore-rpc", feature = "corepc")))]
compile_error!("brk_rpc requires either the `bitcoincore-rpc` or `corepc` feature");
