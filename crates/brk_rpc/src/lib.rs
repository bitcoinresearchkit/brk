use std::{
    env,
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};

use bitcoin::ScriptBuf;
use brk_error::Result;
use brk_types::{BlockHash, Sats, Txid, Weight};

mod client;
mod methods;

use client::ClientInner;
pub use methods::MempoolState;

#[derive(Debug, Clone)]
pub struct BlockInfo {
    pub height: usize,
    pub confirmations: i64,
}

#[derive(Debug, Clone)]
pub struct BlockHeaderInfo {
    pub height: usize,
    pub confirmations: i64,
    pub previous_block_hash: Option<BlockHash>,
}

#[derive(Debug, Clone)]
pub struct TxOutInfo {
    pub coinbase: bool,
    pub value: Sats,
    pub script_pub_key: ScriptBuf,
}

/// One transaction from `getblocktemplate`. Carries the full decoded
/// body and stats so block 0 can be projected without a follow-up
/// `getmempoolentry`/`getrawtransaction` per tx; that follow-up was the
/// source of the GBT/listing race that used to skip cycles.
#[derive(Debug, Clone)]
pub struct BlockTemplateTx {
    pub txid: Txid,
    pub fee: Sats,
    pub weight: Weight,
    /// Parent txids also in this template (Core's own ancestor
    /// accounting, resolved from the wire-level 1-based indices).
    pub depends: Vec<Txid>,
    pub tx: bitcoin::Transaction,
}

#[derive(Clone, Debug)]
pub enum Auth {
    None,
    UserPass(String, String),
    CookieFile(PathBuf),
}

/// Bitcoin Core RPC client. Thread-safe and cheap to clone.
#[derive(Debug, Clone)]
pub struct Client(pub(crate) Arc<ClientInner>);

impl Client {
    pub fn new(url: &str, auth: Auth) -> Result<Self> {
        Self::new_with(url, auth, 1_000_000, Duration::from_secs(1))
    }

    pub fn new_with(
        url: &str,
        auth: Auth,
        max_retries: usize,
        retry_delay: Duration,
    ) -> Result<Self> {
        Ok(Self(Arc::new(ClientInner::new(
            url,
            auth,
            max_retries,
            retry_delay,
        )?)))
    }

    pub fn default_url() -> &'static str {
        "http://localhost:8332"
    }

    pub fn default_bitcoin_path() -> PathBuf {
        if env::consts::OS == "macos" {
            Self::default_mac_bitcoin_path()
        } else {
            Self::default_linux_bitcoin_path()
        }
    }

    pub fn default_linux_bitcoin_path() -> PathBuf {
        Path::new(&env::var("HOME").unwrap()).join(".bitcoin")
    }

    pub fn default_mac_bitcoin_path() -> PathBuf {
        Path::new(&env::var("HOME").unwrap())
            .join("Library")
            .join("Application Support")
            .join("Bitcoin")
    }
}
