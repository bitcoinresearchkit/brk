use std::{thread::sleep, time::Duration};

use brk_error::Result;
use brk_types::Sats;
use corepc_client::client_sync::Auth as CorepcAuth;
use parking_lot::RwLock;
use tracing::info;

use super::{Auth, BlockHeaderInfo, BlockInfo, BlockchainInfo, RawMempoolEntry, TxOutInfo};

type CoreClient = corepc_client::client_sync::v30::Client;
type CoreError = corepc_client::client_sync::Error;

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
        let client = Self::retry(max_retries, retry_delay, || {
            Self::create_client(url, &auth).map_err(Into::into)
        })?;

        Ok(Self {
            url: url.to_string(),
            auth,
            client: RwLock::new(client),
            max_retries,
            retry_delay,
        })
    }

    fn create_client(url: &str, auth: &Auth) -> Result<CoreClient, CoreError> {
        let corepc_auth = match auth {
            Auth::None => CorepcAuth::None,
            Auth::UserPass(u, p) => CorepcAuth::UserPass(u.clone(), p.clone()),
            Auth::CookieFile(path) => CorepcAuth::CookieFile(path.clone()),
        };
        match corepc_auth {
            CorepcAuth::None => Ok(CoreClient::new(url)),
            other => CoreClient::new_with_auth(url, other),
        }
    }

    fn recreate(&self) -> Result<()> {
        *self.client.write() = Self::create_client(&self.url, &self.auth)?;
        Ok(())
    }

    fn is_retriable(error: &CoreError) -> bool {
        match error {
            CoreError::JsonRpc(corepc_jsonrpc::error::Error::Rpc(e)) => {
                e.code == -32600 || e.code == 401 || e.code == -28
            }
            CoreError::JsonRpc(corepc_jsonrpc::error::Error::Transport(_)) => true,
            _ => false,
        }
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

    fn call_with_retry<F, T>(&self, f: F) -> Result<T, CoreError>
    where
        F: Fn(&CoreClient) -> Result<T, CoreError>,
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
        Err(CoreError::JsonRpc(corepc_jsonrpc::error::Error::Rpc(
            corepc_jsonrpc::error::RpcError {
                code: -1,
                message: "Max retries exceeded".to_string(),
                data: None,
            },
        )))
    }

    // --- Wrapped methods returning shared types ---

    pub fn get_blockchain_info(&self) -> Result<BlockchainInfo> {
        let r = self.call_with_retry(|c| c.get_blockchain_info())?;
        Ok(BlockchainInfo {
            headers: r.headers as u64,
            blocks: r.blocks as u64,
        })
    }

    pub fn get_block(&self, hash: &bitcoin::BlockHash) -> Result<bitcoin::Block> {
        Ok(self.call_with_retry(|c| c.get_block(*hash))?)
    }

    pub fn get_block_count(&self) -> Result<u64> {
        let r = self.call_with_retry(|c| c.get_block_count())?;
        Ok(r.0)
    }

    pub fn get_block_hash(&self, height: u64) -> Result<bitcoin::BlockHash> {
        let r = self.call_with_retry(|c| c.get_block_hash(height))?;
        Ok(r.block_hash()?)
    }

    pub fn get_block_header(&self, hash: &bitcoin::BlockHash) -> Result<bitcoin::block::Header> {
        let r = self.call_with_retry(|c| c.get_block_header(hash))?;
        r.block_header()
            .map_err(|_| CoreError::UnexpectedStructure.into())
    }

    pub fn get_block_info(&self, hash: &bitcoin::BlockHash) -> Result<BlockInfo> {
        let r = self.call_with_retry(|c| c.get_block_verbose_one(*hash))?;
        Ok(BlockInfo {
            height: r.height as usize,
            confirmations: r.confirmations,
        })
    }

    pub fn get_block_header_info(&self, hash: &bitcoin::BlockHash) -> Result<BlockHeaderInfo> {
        let r = self.call_with_retry(|c| c.get_block_header_verbose(hash))?;
        let previous_block_hash = r
            .previous_block_hash
            .map(|s| s.parse::<bitcoin::BlockHash>())
            .transpose()
            .map_err(|_| {
                corepc_client::client_sync::Error::UnexpectedStructure
            })?;
        Ok(BlockHeaderInfo {
            height: r.height as usize,
            confirmations: r.confirmations,
            previous_block_hash,
        })
    }

    pub fn get_tx_out(
        &self,
        txid: &bitcoin::Txid,
        vout: u32,
        include_mempool: Option<bool>,
    ) -> Result<Option<TxOutInfo>> {
        // corepc's typed get_tx_out doesn't support include_mempool, so use raw call
        let r: Option<TxOutResponse> = self.call_with_retry(|c| {
            let mut args = vec![
                serde_json::to_value(txid).map_err(CoreError::from)?,
                serde_json::to_value(vout).map_err(CoreError::from)?,
            ];
            if let Some(mempool) = include_mempool {
                args.push(serde_json::to_value(mempool).map_err(CoreError::from)?);
            }
            c.call("gettxout", &args)
        })?;

        match r {
            Some(r) => {
                let script_pub_key =
                    bitcoin::ScriptBuf::from_hex(&r.script_pub_key.hex).map_err(|_| {
                        corepc_client::client_sync::Error::UnexpectedStructure
                    })?;
                let sats = (r.value * 100_000_000.0).round() as u64;
                Ok(Some(TxOutInfo {
                    coinbase: r.coinbase,
                    value: Sats::from(sats),
                    script_pub_key,
                }))
            }
            None => Ok(None),
        }
    }

    pub fn get_raw_mempool(&self) -> Result<Vec<bitcoin::Txid>> {
        let r = self.call_with_retry(|c| c.get_raw_mempool())?;
        r.0.iter()
            .map(|s| {
                s.parse::<bitcoin::Txid>().map_err(|_| {
                    corepc_client::client_sync::Error::UnexpectedStructure.into()
                })
            })
            .collect()
    }

    pub fn get_raw_mempool_verbose(&self) -> Result<Vec<(bitcoin::Txid, RawMempoolEntry)>> {
        let r = self.call_with_retry(|c| c.get_raw_mempool_verbose())?;
        r.0.into_iter()
            .map(|(txid_str, entry)| {
                let txid = txid_str.parse::<bitcoin::Txid>().map_err(|_| {
                    corepc_client::client_sync::Error::UnexpectedStructure
                })?;
                let depends = entry
                    .depends
                    .iter()
                    .map(|s| {
                        s.parse::<bitcoin::Txid>().map_err(|_| {
                            corepc_client::client_sync::Error::UnexpectedStructure
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                Ok((
                    txid,
                    RawMempoolEntry {
                        vsize: entry.vsize as u64,
                        weight: entry.weight as u64,
                        base_fee_sats: (entry.fees.base * 100_000_000.0).round() as u64,
                        ancestor_count: entry.ancestor_count as u64,
                        ancestor_size: entry.ancestor_size as u64,
                        ancestor_fee_sats: (entry.fees.ancestor * 100_000_000.0).round() as u64,
                        depends,
                    },
                ))
            })
            .collect()
    }

    pub fn get_raw_transaction_hex(
        &self,
        txid: &bitcoin::Txid,
        block_hash: Option<&bitcoin::BlockHash>,
    ) -> Result<String> {
        // corepc's get_raw_transaction doesn't support block_hash param, use raw call
        let r: String = self.call_with_retry(|c| {
            let mut args: Vec<serde_json::Value> = vec![
                serde_json::to_value(txid).map_err(CoreError::from)?,
                serde_json::Value::Bool(false),
            ];
            if let Some(bh) = block_hash {
                args.push(serde_json::to_value(bh).map_err(CoreError::from)?);
            }
            c.call("getrawtransaction", &args)
        })?;
        Ok(r)
    }
}

// Local deserialization structs for raw RPC responses

#[derive(serde::Deserialize)]
struct TxOutResponse {
    coinbase: bool,
    value: f64,
    #[serde(rename = "scriptPubKey")]
    script_pub_key: TxOutScriptPubKey,
}

#[derive(serde::Deserialize)]
struct TxOutScriptPubKey {
    hex: String,
}
