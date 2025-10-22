use std::{mem, sync::Arc, time::Duration};

use bitcoin::consensus::encode;
use bitcoincore_rpc::json::{GetBlockHeaderResult, GetBlockResult, GetTxOutResult};
use bitcoincore_rpc::{Client as CoreClient, Error as RpcError, RpcApi};
use brk_error::Result;
use brk_structs::{BlockHash, Height, Sats, Transaction, TxIn, TxOut, TxStatus, Txid, Vout};

pub use bitcoincore_rpc::Auth;

mod inner;

use inner::ClientInner;

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

    pub fn get_block<'a, H>(&self, hash: &'a H) -> Result<bitcoin::Block>
    where
        &'a H: Into<&'a bitcoin::BlockHash>,
    {
        self.call(|c| c.get_block(hash.into())).map_err(Into::into)
    }

    /// Returns the numbers of block in the longest chain.
    pub fn get_last_height(&self) -> Result<Height> {
        self.call(|c| c.get_block_count())
            .map(Height::from)
            .map_err(Into::into)
    }

    /// Get block hash at a given height
    pub fn get_block_hash<H>(&self, height: H) -> Result<BlockHash>
    where
        H: Into<u64> + Copy,
    {
        self.call(|c| c.get_block_hash(height.into()))
            .map(BlockHash::from)
            .map_err(Into::into)
    }

    pub fn get_block_info<'a, H>(&self, hash: &'a H) -> Result<GetBlockResult>
    where
        &'a H: Into<&'a bitcoin::BlockHash>,
    {
        self.call(move |c| c.get_block_info(hash.into()))
            .map_err(Into::into)
    }

    pub fn get_block_header_info<'a, H>(&self, hash: &'a H) -> Result<GetBlockHeaderResult>
    where
        &'a H: Into<&'a bitcoin::BlockHash>,
    {
        self.call(|c| c.get_block_header_info(hash.into()))
            .map_err(Into::into)
    }

    pub fn get_transaction<'a, T>(&self, txid: &'a T) -> Result<Transaction>
    where
        &'a T: Into<&'a bitcoin::Txid>,
    {
        let mut tx = self.get_raw_transaction(txid, None as Option<&'a BlockHash>)?;

        let input = mem::take(&mut tx.input)
            .into_iter()
            .map(|txin| -> Result<TxIn> {
                let txout_result = self.get_tx_out(
                    (&txin.previous_output.txid).into(),
                    txin.previous_output.vout.into(),
                    Some(true),
                )?;

                let is_coinbase = txout_result.as_ref().is_none_or(|r| r.coinbase);

                let txout = if let Some(txout_result) = txout_result {
                    Some(TxOut::from((
                        txout_result.script_pub_key.script()?,
                        Sats::from(txout_result.value.to_sat()),
                    )))
                } else {
                    None
                };

                Ok(TxIn {
                    is_coinbase,
                    prevout: txout,
                    txid: txin.previous_output.txid.into(),
                    vout: txin.previous_output.vout.into(),
                    script_sig: txin.script_sig,
                    script_sig_asm: (),
                    sequence: txin.sequence.into(),
                    inner_redeem_script_asm: (),
                })
            })
            .collect::<Result<Vec<_>>>()?;

        let mut tx = Transaction {
            index: None,
            txid: tx.compute_txid().into(),
            version: tx.version.into(),
            total_sigop_cost: tx.total_sigop_cost(|_| None),
            weight: tx.weight().into(),
            lock_time: tx.lock_time.into(),
            total_size: tx.total_size(),
            fee: Sats::default(),
            input,
            output: tx.output.into_iter().map(TxOut::from).collect(),
            status: TxStatus::UNCOMFIRMED,
        };

        tx.compute_fee();

        Ok(tx)
    }

    pub fn get_tx_out(
        &self,
        txid: &Txid,
        vout: Vout,
        include_mempool: Option<bool>,
    ) -> Result<Option<GetTxOutResult>> {
        self.call(|c| c.get_tx_out(txid.into(), vout.into(), include_mempool))
            .map_err(Into::into)
    }

    /// Get txids of all transactions in a memory pool
    pub fn get_raw_mempool(&self) -> Result<Vec<Txid>> {
        self.call(|c| c.get_raw_mempool())
            .map(|v| unsafe { mem::transmute(v) })
            .map_err(Into::into)
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
        self.call(|c| c.get_raw_transaction_hex(txid.into(), block_hash.map(|h| h.into())))
            .map_err(Into::into)
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
                        .map(BlockHash::from)
                        .ok_or("Genesis block has no previous block")?;

                    loop {
                        if self.is_in_main_chain(&current_hash)? {
                            // Found a block in the main chain
                            let current_info = self.get_block_header_info(&current_hash)?;
                            return Ok(current_info.height as u64);
                        }

                        // Continue walking backwards
                        let current_info = self.get_block_header_info(&current_hash)?;
                        current_hash = current_info
                            .previous_block_hash
                            .map(BlockHash::from)
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
