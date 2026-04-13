use std::{thread::sleep, time::Duration};

use bitcoincore_rpc::{Client as CoreClient, Error as RpcError, RpcApi, jsonrpc};
use brk_error::{Error, Result};
use brk_types::Sats;
use parking_lot::RwLock;
use serde_json::value::RawValue;
use tracing::info;

use super::{Auth, BlockHeaderInfo, BlockInfo, BlockchainInfo, RawMempoolEntry, TxOutInfo};

/// Per-batch request count for `get_block_hashes_range`. Sized so the
/// JSON request body stays well under a megabyte and bitcoind doesn't
/// spend too long on a single batch before yielding results.
const BATCH_CHUNK: usize = 2000;

fn to_rpc_auth(auth: &Auth) -> bitcoincore_rpc::Auth {
    match auth {
        Auth::None => bitcoincore_rpc::Auth::None,
        Auth::UserPass(u, p) => bitcoincore_rpc::Auth::UserPass(u.clone(), p.clone()),
        Auth::CookieFile(path) => bitcoincore_rpc::Auth::CookieFile(path.clone()),
    }
}

#[derive(Debug)]
pub struct ClientInner {
    url: String,
    auth: Auth,
    client: RwLock<CoreClient>,
    max_retries: usize,
    retry_delay: Duration,
}

impl ClientInner {
    pub fn new(url: &str, auth: Auth, max_retries: usize, retry_delay: Duration) -> Result<Self> {
        let rpc_auth = to_rpc_auth(&auth);
        let client = Self::retry(max_retries, retry_delay, || {
            CoreClient::new(url, rpc_auth.clone()).map_err(Into::into)
        })?;

        Ok(Self {
            url: url.to_string(),
            auth,
            client: RwLock::new(client),
            max_retries,
            retry_delay,
        })
    }

    fn recreate(&self) -> Result<()> {
        *self.client.write() = CoreClient::new(&self.url, to_rpc_auth(&self.auth))?;
        Ok(())
    }

    fn is_retriable(error: &RpcError) -> bool {
        matches!(
            error,
            RpcError::JsonRpc(jsonrpc::Error::Rpc(e))
                if e.code == -32600 || e.code == 401 || e.code == -28
        ) || matches!(error, RpcError::JsonRpc(jsonrpc::Error::Transport(_)))
    }

    fn retry<F, T>(max_retries: usize, delay: Duration, mut f: F) -> Result<T>
    where
        F: FnMut() -> Result<T>,
    {
        let mut last_error = None;

        for attempt in 0..=max_retries {
            if attempt > 0 {
                info!(
                    "Retrying to connect to Bitcoin Core (attempt {}/{})",
                    attempt, max_retries
                );
                sleep(delay);
            }

            match f() {
                Ok(value) => {
                    if attempt > 0 {
                        info!(
                            "Successfully connected to Bitcoin Core after {} retries",
                            attempt
                        );
                    }
                    return Ok(value);
                }
                Err(e) => {
                    if attempt == 0 {
                        info!("Could not connect to Bitcoin Core, retrying: {}", e);
                    }
                    last_error = Some(e);
                }
            }
        }

        let err = last_error.unwrap();
        info!(
            "Failed to connect to Bitcoin Core after {} attempts",
            max_retries + 1
        );
        Err(err)
    }

    pub fn call_with_retry<F, T>(&self, f: F) -> Result<T, RpcError>
    where
        F: Fn(&CoreClient) -> Result<T, RpcError>,
    {
        for attempt in 0..=self.max_retries {
            if attempt > 0 {
                info!(
                    "Trying to reconnect to Bitcoin Core (attempt {}/{})",
                    attempt, self.max_retries
                );
                self.recreate().ok();
                sleep(self.retry_delay);
            }

            match f(&self.client.read()) {
                Ok(value) => {
                    if attempt > 0 {
                        info!(
                            "Successfully reconnected to Bitcoin Core after {} attempts",
                            attempt
                        );
                    }
                    return Ok(value);
                }
                Err(e) if Self::is_retriable(&e) => {
                    if attempt == 0 {
                        info!("Lost connection to Bitcoin Core, reconnecting...");
                    }
                }
                Err(e) => return Err(e),
            }
        }

        info!(
            "Could not reconnect to Bitcoin Core after {} attempts",
            self.max_retries + 1
        );
        Err(RpcError::JsonRpc(jsonrpc::Error::Rpc(
            jsonrpc::error::RpcError {
                code: -1,
                message: "Max retries exceeded".to_string(),
                data: None,
            },
        )))
    }

    pub fn call_once<F, T>(&self, f: F) -> Result<T, RpcError>
    where
        F: Fn(&CoreClient) -> Result<T, RpcError>,
    {
        f(&self.client.read())
    }

    // --- Wrapped methods returning shared types ---

    pub fn get_blockchain_info(&self) -> Result<BlockchainInfo> {
        let r = self.call_with_retry(|c| c.get_blockchain_info())?;
        Ok(BlockchainInfo {
            headers: r.headers,
            blocks: r.blocks,
        })
    }

    pub fn get_block(&self, hash: &bitcoin::BlockHash) -> Result<bitcoin::Block> {
        Ok(self.call_with_retry(|c| c.get_block(hash))?)
    }

    pub fn get_block_count(&self) -> Result<u64> {
        Ok(self.call_with_retry(|c| c.get_block_count())?)
    }

    pub fn get_block_hash(&self, height: u64) -> Result<bitcoin::BlockHash> {
        Ok(self.call_with_retry(|c| c.get_block_hash(height))?)
    }

    /// Batched canonical height → block hash lookup over the inclusive
    /// range `start..=end`. See the corepc backend for the rationale and
    /// chunking strategy; this mirror uses bitcoincore-rpc's
    /// `get_jsonrpc_client` accessor.
    pub fn get_block_hashes_range(
        &self,
        start: u64,
        end: u64,
    ) -> Result<Vec<bitcoin::BlockHash>> {
        if end < start {
            return Ok(Vec::new());
        }
        let total = (end - start + 1) as usize;
        let mut hashes = Vec::with_capacity(total);

        let mut chunk_start = start;
        while chunk_start <= end {
            let chunk_end = (chunk_start + BATCH_CHUNK as u64 - 1).min(end);
            self.batch_get_block_hashes(chunk_start, chunk_end, &mut hashes)?;
            chunk_start = chunk_end + 1;
        }
        Ok(hashes)
    }

    fn batch_get_block_hashes(
        &self,
        start: u64,
        end: u64,
        out: &mut Vec<bitcoin::BlockHash>,
    ) -> Result<()> {
        let params: Vec<Box<RawValue>> = (start..=end)
            .map(|h| {
                RawValue::from_string(format!("[{h}]")).map_err(|e| Error::Parse(e.to_string()))
            })
            .collect::<Result<Vec<_>>>()?;

        let client = self.client.read();
        let jsonrpc_client = client.get_jsonrpc_client();
        let requests: Vec<jsonrpc::Request> = params
            .iter()
            .map(|p| jsonrpc_client.build_request("getblockhash", Some(p)))
            .collect();

        let responses = jsonrpc_client
            .send_batch(&requests)
            .map_err(|e| Error::Parse(format!("getblockhash batch failed: {e}")))?;

        for response in responses {
            let response = response.ok_or(Error::Internal("Missing response in JSON-RPC batch"))?;
            let hex: String = response
                .result()
                .map_err(|e| Error::Parse(format!("getblockhash batch result: {e}")))?;
            out.push(
                hex.parse::<bitcoin::BlockHash>()
                    .map_err(|e| Error::Parse(format!("invalid block hash hex: {e}")))?,
            );
        }
        Ok(())
    }

    pub fn get_block_header(&self, hash: &bitcoin::BlockHash) -> Result<bitcoin::block::Header> {
        Ok(self.call_with_retry(|c| c.get_block_header(hash))?)
    }

    pub fn get_block_info(&self, hash: &bitcoin::BlockHash) -> Result<BlockInfo> {
        let r = self.call_with_retry(|c| c.get_block_info(hash))?;
        Ok(BlockInfo {
            height: r.height,
            confirmations: r.confirmations as i64,
        })
    }

    pub fn get_block_header_info(&self, hash: &bitcoin::BlockHash) -> Result<BlockHeaderInfo> {
        let r = self.call_with_retry(|c| c.get_block_header_info(hash))?;
        Ok(BlockHeaderInfo {
            height: r.height,
            confirmations: r.confirmations as i64,
            previous_block_hash: r.previous_block_hash,
        })
    }

    pub fn get_tx_out(
        &self,
        txid: &bitcoin::Txid,
        vout: u32,
        include_mempool: Option<bool>,
    ) -> Result<Option<TxOutInfo>> {
        let r = self.call_with_retry(|c| c.get_tx_out(txid, vout, include_mempool))?;
        match r {
            Some(r) => Ok(Some(TxOutInfo {
                coinbase: r.coinbase,
                value: Sats::from(r.value.to_sat()),
                script_pub_key: r.script_pub_key.script()?,
            })),
            None => Ok(None),
        }
    }

    pub fn get_raw_mempool(&self) -> Result<Vec<bitcoin::Txid>> {
        Ok(self.call_with_retry(|c| c.get_raw_mempool())?)
    }

    pub fn get_raw_mempool_verbose(&self) -> Result<Vec<(bitcoin::Txid, RawMempoolEntry)>> {
        let r = self.call_with_retry(|c| c.get_raw_mempool_verbose())?;
        Ok(r.into_iter()
            .map(|(txid, entry)| {
                (
                    txid,
                    RawMempoolEntry {
                        vsize: entry.vsize,
                        weight: entry.weight.unwrap_or(entry.vsize * 4),
                        base_fee_sats: entry.fees.base.to_sat(),
                        ancestor_count: entry.ancestor_count,
                        ancestor_size: entry.ancestor_size,
                        ancestor_fee_sats: entry.fees.ancestor.to_sat(),
                        depends: entry.depends.into_iter().collect(),
                    },
                )
            })
            .collect())
    }

    pub fn get_raw_transaction_hex(
        &self,
        txid: &bitcoin::Txid,
        block_hash: Option<&bitcoin::BlockHash>,
    ) -> Result<String> {
        Ok(self.call_with_retry(|c| c.get_raw_transaction_hex(txid, block_hash))?)
    }

    pub fn send_raw_transaction(&self, hex: &str) -> Result<bitcoin::Txid> {
        Ok(self.call_once(|c| c.send_raw_transaction(hex))?)
    }
}
