use bitcoin::BlockHash;
use bitcoincore_rpc::json::GetBlockResult;
use bitcoincore_rpc::{Client as CoreClient, Error as RpcError, RpcApi};
use brk_error::Result;
use std::sync::Arc;
use std::time::Duration;

pub use bitcoincore_rpc::Auth;

mod inner;

use inner::ClientInner;

///
/// Bitcoin Core RPC Client
///
/// Free to clone (Arc)
///
#[derive(Clone)]
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

    pub fn get_block_info(&self, hash: &BlockHash) -> Result<GetBlockResult> {
        self.call(|c| c.get_block_info(hash)).map_err(Into::into)
    }

    /// Checks if a block is in the main chain (has positive confirmations)
    pub fn is_in_main_chain(&self, hash: &BlockHash) -> Result<bool> {
        let block_info = self.get_block_info(hash)?;
        Ok(block_info.confirmations > 0)
    }

    pub fn get_closest_valid_height(&self, hash: BlockHash) -> Result<u64> {
        // First, try to get block info for the hash
        match self.get_block_info(&hash) {
            Ok(block_info) => {
                // Check if this block is in the main chain
                if self.is_in_main_chain(&hash)? {
                    // Block is in the main chain
                    Ok(block_info.height as u64)
                } else {
                    // Confirmations is -1, meaning it's on a fork
                    // We need to find where it diverged from the main chain

                    // Get the previous block hash and walk backwards
                    let mut current_hash = block_info
                        .previousblockhash
                        .ok_or("Genesis block has no previous block")?;

                    loop {
                        if self.is_in_main_chain(&current_hash)? {
                            // Found a block in the main chain
                            let current_info = self.get_block_info(&current_hash)?;
                            return Ok(current_info.height as u64);
                        }

                        // Continue walking backwards
                        let current_info = self.get_block_info(&current_hash)?;
                        current_hash = current_info
                            .previousblockhash
                            .ok_or("Reached genesis without finding main chain")?;
                    }
                }
            }
            Err(_) => {
                // Block not found in the node's database at all
                Err("Block hash not found in blockchain".into())
            }
        }
    }

    pub fn call<F, T>(&self, f: F) -> Result<T, RpcError>
    where
        F: Fn(&CoreClient) -> Result<T, RpcError>,
    {
        self.0.call_with_retry(f)
    }

    pub fn call_once<F, T>(&self, f: F) -> Result<T, RpcError>
    where
        F: Fn(&CoreClient) -> Result<T, RpcError>,
    {
        self.0.call_once(f)
    }
}
