use std::{
    env, mem,
    path::{Path, PathBuf},
    sync::Arc,
    thread::sleep,
    time::Duration,
};

use bitcoin::consensus::encode;
use brk_error::{Error, Result};
use brk_types::{BlockHash, Height, MempoolEntryInfo, Sats, Txid, Vout};

pub mod backend;

pub use backend::{Auth, BlockHeaderInfo, BlockInfo, BlockTemplateTx, BlockchainInfo, TxOutInfo};

use backend::ClientInner;
use tracing::{debug, info};

///
/// Bitcoin Core RPC Client
///
/// Thread safe and free to clone
///
#[derive(Debug, Clone)]
pub struct Client(Arc<ClientInner>);

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

    /// Returns a data structure containing various state info regarding
    /// blockchain processing.
    pub fn get_blockchain_info(&self) -> Result<BlockchainInfo> {
        self.0.get_blockchain_info()
    }

    pub fn get_block<'a, H>(&self, hash: &'a H) -> Result<bitcoin::Block>
    where
        &'a H: Into<&'a bitcoin::BlockHash>,
    {
        self.0.get_block(hash.into())
    }

    /// Returns the numbers of block in the longest chain.
    pub fn get_block_count(&self) -> Result<u64> {
        self.0.get_block_count()
    }

    /// Returns the numbers of block in the longest chain.
    pub fn get_last_height(&self) -> Result<Height> {
        self.0.get_block_count().map(Height::from)
    }

    /// Get block hash at a given height
    pub fn get_block_hash<H>(&self, height: H) -> Result<BlockHash>
    where
        H: Into<u64> + Copy,
    {
        self.0.get_block_hash(height.into()).map(BlockHash::from)
    }

    /// Get every canonical block hash for the inclusive height range
    /// `start..=end` in a single JSON-RPC batch request. Returns hashes
    /// in canonical order (`start`, `start+1`, …, `end`). Use this
    /// whenever resolving more than ~2 heights — one HTTP round-trip
    /// beats N sequential `get_block_hash` calls once the per-call
    /// overhead dominates.
    pub fn get_block_hashes_range<H1, H2>(&self, start: H1, end: H2) -> Result<Vec<BlockHash>>
    where
        H1: Into<u64>,
        H2: Into<u64>,
    {
        self.0
            .get_block_hashes_range(start.into(), end.into())
            .map(|v| v.into_iter().map(BlockHash::from).collect())
    }

    pub fn get_block_header<'a, H>(&self, hash: &'a H) -> Result<bitcoin::block::Header>
    where
        &'a H: Into<&'a bitcoin::BlockHash>,
    {
        self.0.get_block_header(hash.into())
    }

    pub fn get_block_info<'a, H>(&self, hash: &'a H) -> Result<BlockInfo>
    where
        &'a H: Into<&'a bitcoin::BlockHash>,
    {
        self.0.get_block_info(hash.into())
    }

    pub fn get_block_header_info<'a, H>(&self, hash: &'a H) -> Result<BlockHeaderInfo>
    where
        &'a H: Into<&'a bitcoin::BlockHash>,
    {
        self.0.get_block_header_info(hash.into())
    }

    pub fn get_transaction<'a, T, H>(
        &self,
        txid: &'a T,
        block_hash: Option<&'a H>,
    ) -> brk_error::Result<bitcoin::Transaction>
    where
        &'a T: Into<&'a bitcoin::Txid>,
        &'a H: Into<&'a bitcoin::BlockHash>,
    {
        let tx = self.get_raw_transaction(txid, block_hash)?;
        Ok(tx)
    }

    pub fn get_mempool_raw_tx(
        &self,
        txid: &Txid,
    ) -> Result<(bitcoin::Transaction, String)> {
        let hex = self.get_raw_transaction_hex(txid, None as Option<&BlockHash>)?;
        let tx = encode::deserialize_hex::<bitcoin::Transaction>(&hex)?;
        Ok((tx, hex))
    }

    pub fn get_tx_out(
        &self,
        txid: &Txid,
        vout: Vout,
        include_mempool: Option<bool>,
    ) -> Result<Option<TxOutInfo>> {
        self.0.get_tx_out(txid.into(), vout.into(), include_mempool)
    }

    /// Get txids of all transactions in a memory pool
    pub fn get_raw_mempool(&self) -> Result<Vec<Txid>> {
        self.0
            .get_raw_mempool()
            .map(|v| unsafe { mem::transmute(v) })
    }

    /// Get all mempool entries with their fee data in a single RPC call
    pub fn get_raw_mempool_verbose(&self) -> Result<Vec<MempoolEntryInfo>> {
        let result = self.0.get_raw_mempool_verbose()?;
        Ok(result
            .into_iter()
            .map(
                |(txid, entry): (bitcoin::Txid, backend::RawMempoolEntry)| MempoolEntryInfo {
                    txid: txid.into(),
                    vsize: entry.vsize,
                    weight: entry.weight,
                    fee: Sats::from(entry.base_fee_sats),
                    ancestor_count: entry.ancestor_count,
                    ancestor_size: entry.ancestor_size,
                    ancestor_fee: Sats::from(entry.ancestor_fee_sats),
                    depends: entry.depends.into_iter().map(Txid::from).collect(),
                },
            )
            .collect())
    }

    pub fn get_raw_transaction<'a, T, H>(
        &self,
        txid: &'a T,
        block_hash: Option<&'a H>,
    ) -> brk_error::Result<bitcoin::Transaction>
    where
        &'a T: Into<&'a bitcoin::Txid>,
        &'a H: Into<&'a bitcoin::BlockHash>,
    {
        let hex = self.get_raw_transaction_hex(txid, block_hash)?;
        let tx = encode::deserialize_hex::<bitcoin::Transaction>(&hex)?;
        Ok(tx)
    }

    pub fn get_raw_transaction_hex<'a, T, H>(
        &self,
        txid: &'a T,
        block_hash: Option<&'a H>,
    ) -> Result<String>
    where
        &'a T: Into<&'a bitcoin::Txid>,
        &'a H: Into<&'a bitcoin::BlockHash>,
    {
        self.0
            .get_raw_transaction_hex(txid.into(), block_hash.map(|h| h.into()))
    }

    pub fn send_raw_transaction(&self, hex: &str) -> Result<Txid> {
        self.0.send_raw_transaction(hex).map(Txid::from)
    }

    /// Transactions (txid + fee) Bitcoin Core would include in the next
    /// block it would mine, via `getblocktemplate`.
    pub fn get_block_template_txs(&self) -> Result<Vec<BlockTemplateTx>> {
        self.0.get_block_template_txs()
    }

    /// Checks if a block is in the main chain (has positive confirmations)
    pub fn is_in_main_chain(&self, hash: &BlockHash) -> Result<bool> {
        let block_info = self.get_block_info(hash)?;
        Ok(block_info.confirmations > 0)
    }

    pub fn get_closest_valid_height(&self, hash: BlockHash) -> Result<(Height, BlockHash)> {
        debug!("Get closest valid height...");

        match self.get_block_header_info(&hash) {
            Ok(block_info) => {
                if self.is_in_main_chain(&hash)? {
                    return Ok((block_info.height.into(), hash));
                }

                let mut hash =
                    block_info
                        .previous_block_hash
                        .map(BlockHash::from)
                        .ok_or(Error::NotFound(
                            "Genesis block has no previous block".into(),
                        ))?;

                loop {
                    if self.is_in_main_chain(&hash)? {
                        let current_info = self.get_block_header_info(&hash)?;
                        return Ok((current_info.height.into(), hash));
                    }

                    let info = self.get_block_header_info(&hash)?;
                    hash = info
                        .previous_block_hash
                        .map(BlockHash::from)
                        .ok_or(Error::NotFound(
                            "Reached genesis without finding main chain".into(),
                        ))?;
                }
            }
            Err(_) => Err(Error::NotFound("Block hash not found in blockchain".into())),
        }
    }

    pub fn wait_for_synced_node(&self) -> Result<()> {
        let is_synced = || -> Result<bool> {
            let info = self.get_blockchain_info()?;
            Ok(info.headers == info.blocks)
        };

        if !is_synced()? {
            info!("Waiting for node to sync...");
            while !is_synced()? {
                sleep(Duration::from_secs(1))
            }
        }

        Ok(())
    }

    #[cfg(feature = "bitcoincore-rpc")]
    pub fn call<F, T>(&self, f: F) -> Result<T, bitcoincore_rpc::Error>
    where
        F: Fn(&bitcoincore_rpc::Client) -> Result<T, bitcoincore_rpc::Error>,
    {
        self.0.call_with_retry(f)
    }

    #[cfg(feature = "bitcoincore-rpc")]
    pub fn call_once<F, T>(&self, f: F) -> Result<T, bitcoincore_rpc::Error>
    where
        F: Fn(&bitcoincore_rpc::Client) -> Result<T, bitcoincore_rpc::Error>,
    {
        self.0.call_once(f)
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
