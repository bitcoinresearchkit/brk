use std::io::Cursor;

use bitcoin::{consensus::Decodable, hex::DisplayHex};
use brk_error::{Error, Result};
use brk_types::{
    Sats, Transaction, TxIn, TxInIndex, TxIndex, TxOut, TxOutspend, TxStatus, Txid, TxidParam,
    TxidPrefix, Vin, Vout, Weight,
};
use vecdb::{GenericStoredVec, TypedVecIterator};

use crate::Query;

impl Query {
    pub fn transaction(&self, TxidParam { txid }: TxidParam) -> Result<Transaction> {
        // First check mempool for unconfirmed transactions
        if let Some(mempool) = self.mempool()
            && let Some(tx_with_hex) = mempool.get_txs().get(&txid)
        {
            return Ok(tx_with_hex.tx().clone());
        }

        // Look up confirmed transaction by txid prefix
        let prefix = TxidPrefix::from(&txid);
        let indexer = self.indexer();
        let Ok(Some(txindex)) = indexer
            .stores
            .txidprefix_to_txindex
            .get(&prefix)
            .map(|opt| opt.map(|cow| cow.into_owned()))
        else {
            return Err(Error::UnknownTxid);
        };

        self.transaction_by_index(txindex)
    }

    pub fn transaction_status(&self, TxidParam { txid }: TxidParam) -> Result<TxStatus> {
        // First check mempool for unconfirmed transactions
        if let Some(mempool) = self.mempool()
            && mempool.get_txs().contains_key(&txid)
        {
            return Ok(TxStatus::UNCONFIRMED);
        }

        // Look up confirmed transaction by txid prefix
        let prefix = TxidPrefix::from(&txid);
        let indexer = self.indexer();
        let Ok(Some(txindex)) = indexer
            .stores
            .txidprefix_to_txindex
            .get(&prefix)
            .map(|opt| opt.map(|cow| cow.into_owned()))
        else {
            return Err(Error::UnknownTxid);
        };

        // Get block info for status
        let height = indexer.vecs.tx.txindex_to_height.read_once(txindex)?;
        let block_hash = indexer.vecs.block.height_to_blockhash.read_once(height)?;
        let block_time = indexer.vecs.block.height_to_timestamp.read_once(height)?;

        Ok(TxStatus {
            confirmed: true,
            block_height: Some(height),
            block_hash: Some(block_hash),
            block_time: Some(block_time),
        })
    }

    pub fn transaction_hex(&self, TxidParam { txid }: TxidParam) -> Result<String> {
        // First check mempool for unconfirmed transactions
        if let Some(mempool) = self.mempool()
            && let Some(tx_with_hex) = mempool.get_txs().get(&txid)
        {
            return Ok(tx_with_hex.hex().to_string());
        }

        // Look up confirmed transaction by txid prefix
        let prefix = TxidPrefix::from(&txid);
        let indexer = self.indexer();
        let Ok(Some(txindex)) = indexer
            .stores
            .txidprefix_to_txindex
            .get(&prefix)
            .map(|opt| opt.map(|cow| cow.into_owned()))
        else {
            return Err(Error::UnknownTxid);
        };

        self.transaction_hex_by_index(txindex)
    }

    pub fn outspend(&self, TxidParam { txid }: TxidParam, vout: Vout) -> Result<TxOutspend> {
        // Mempool outputs are unspent in on-chain terms
        if let Some(mempool) = self.mempool()
            && mempool.get_txs().contains_key(&txid)
        {
            return Ok(TxOutspend::UNSPENT);
        }

        // Look up confirmed transaction
        let prefix = TxidPrefix::from(&txid);
        let indexer = self.indexer();
        let Ok(Some(txindex)) = indexer
            .stores
            .txidprefix_to_txindex
            .get(&prefix)
            .map(|opt| opt.map(|cow| cow.into_owned()))
        else {
            return Err(Error::UnknownTxid);
        };

        // Calculate txoutindex
        let first_txoutindex = indexer
            .vecs
            .tx
            .txindex_to_first_txoutindex
            .read_once(txindex)?;
        let txoutindex = first_txoutindex + vout;

        // Look up spend status
        let computer = self.computer();
        let txinindex = computer
            .txouts
            .txoutindex_to_txinindex
            .read_once(txoutindex)?;

        if txinindex == TxInIndex::UNSPENT {
            return Ok(TxOutspend::UNSPENT);
        }

        self.outspend_details(txinindex)
    }

    pub fn outspends(&self, TxidParam { txid }: TxidParam) -> Result<Vec<TxOutspend>> {
        // Mempool outputs are unspent in on-chain terms
        if let Some(mempool) = self.mempool()
            && let Some(tx_with_hex) = mempool.get_txs().get(&txid)
        {
            let output_count = tx_with_hex.tx().output.len();
            return Ok(vec![TxOutspend::UNSPENT; output_count]);
        }

        // Look up confirmed transaction
        let prefix = TxidPrefix::from(&txid);
        let indexer = self.indexer();
        let Ok(Some(txindex)) = indexer
            .stores
            .txidprefix_to_txindex
            .get(&prefix)
            .map(|opt| opt.map(|cow| cow.into_owned()))
        else {
            return Err(Error::UnknownTxid);
        };

        // Get output range
        let first_txoutindex = indexer
            .vecs
            .tx
            .txindex_to_first_txoutindex
            .read_once(txindex)?;
        let next_first_txoutindex = indexer
            .vecs
            .tx
            .txindex_to_first_txoutindex
            .read_once(txindex.incremented())?;
        let output_count = usize::from(next_first_txoutindex) - usize::from(first_txoutindex);

        // Get spend status for each output
        let computer = self.computer();
        let mut txoutindex_to_txinindex_iter = computer.txouts.txoutindex_to_txinindex.iter()?;

        let mut outspends = Vec::with_capacity(output_count);
        for i in 0..output_count {
            let txoutindex = first_txoutindex + Vout::from(i);
            let txinindex = txoutindex_to_txinindex_iter.get_unwrap(txoutindex);

            if txinindex == TxInIndex::UNSPENT {
                outspends.push(TxOutspend::UNSPENT);
            } else {
                outspends.push(self.outspend_details(txinindex)?);
            }
        }

        Ok(outspends)
    }

    // === Helper methods ===

    pub fn transaction_by_index(&self, txindex: TxIndex) -> Result<Transaction> {
        let indexer = self.indexer();
        let reader = self.reader();
        let computer = self.computer();

        // Get tx metadata using read_once for single lookups
        let txid = indexer.vecs.tx.txindex_to_txid.read_once(txindex)?;
        let height = indexer.vecs.tx.txindex_to_height.read_once(txindex)?;
        let version = indexer.vecs.tx.txindex_to_txversion.read_once(txindex)?;
        let lock_time = indexer.vecs.tx.txindex_to_rawlocktime.read_once(txindex)?;
        let total_size = indexer.vecs.tx.txindex_to_total_size.read_once(txindex)?;
        let first_txinindex = indexer
            .vecs
            .tx
            .txindex_to_first_txinindex
            .read_once(txindex)?;
        let position = computer.blks.txindex_to_position.read_once(txindex)?;

        // Get block info for status
        let block_hash = indexer.vecs.block.height_to_blockhash.read_once(height)?;
        let block_time = indexer.vecs.block.height_to_timestamp.read_once(height)?;

        // Read and decode the raw transaction from blk file
        let buffer = reader.read_raw_bytes(position, *total_size as usize)?;
        let mut cursor = Cursor::new(buffer);
        let tx = bitcoin::Transaction::consensus_decode(&mut cursor)
            .map_err(|_| Error::Parse("Failed to decode transaction".into()))?;

        // For iterating through inputs, we need iterators (multiple lookups)
        let mut txindex_to_txid_iter = indexer.vecs.tx.txindex_to_txid.iter()?;
        let mut txindex_to_first_txoutindex_iter =
            indexer.vecs.tx.txindex_to_first_txoutindex.iter()?;
        let mut txinindex_to_outpoint_iter = indexer.vecs.txin.txinindex_to_outpoint.iter()?;
        let mut txoutindex_to_value_iter = indexer.vecs.txout.txoutindex_to_value.iter()?;

        // Build inputs with prevout information
        let input: Vec<TxIn> = tx
            .input
            .iter()
            .enumerate()
            .map(|(i, txin)| {
                let txinindex = first_txinindex + i;
                let outpoint = txinindex_to_outpoint_iter.get_unwrap(txinindex);

                let is_coinbase = outpoint.is_coinbase();

                // Get prevout info if not coinbase
                let (prev_txid, prev_vout, prevout) = if is_coinbase {
                    (Txid::COINBASE, Vout::MAX, None)
                } else {
                    let prev_txindex = outpoint.txindex();
                    let prev_vout = outpoint.vout();
                    let prev_txid = txindex_to_txid_iter.get_unwrap(prev_txindex);

                    // Calculate the txoutindex for the prevout
                    let prev_first_txoutindex =
                        txindex_to_first_txoutindex_iter.get_unwrap(prev_txindex);
                    let prev_txoutindex = prev_first_txoutindex + prev_vout;

                    // Get the value of the prevout
                    let prev_value = txoutindex_to_value_iter.get_unwrap(prev_txoutindex);

                    let prevout = Some(TxOut::from((
                        bitcoin::ScriptBuf::new(), // Placeholder - would need to reconstruct
                        prev_value,
                    )));

                    (prev_txid, prev_vout, prevout)
                };

                TxIn {
                    txid: prev_txid,
                    vout: prev_vout,
                    prevout,
                    script_sig: txin.script_sig.clone(),
                    script_sig_asm: (),
                    is_coinbase,
                    sequence: txin.sequence.0,
                    inner_redeem_script_asm: (),
                }
            })
            .collect();

        // Calculate weight before consuming tx.output
        let weight = Weight::from(tx.weight());

        // Calculate sigop cost
        let total_sigop_cost = tx.total_sigop_cost(|_| None);

        // Build outputs
        let output: Vec<TxOut> = tx.output.into_iter().map(TxOut::from).collect();

        // Build status
        let status = TxStatus {
            confirmed: true,
            block_height: Some(height),
            block_hash: Some(block_hash),
            block_time: Some(block_time),
        };

        let mut transaction = Transaction {
            index: Some(txindex),
            txid,
            version,
            lock_time,
            total_size: *total_size as usize,
            weight,
            total_sigop_cost,
            fee: Sats::ZERO, // Will be computed below
            input,
            output,
            status,
        };

        // Compute fee from inputs - outputs
        transaction.compute_fee();

        Ok(transaction)
    }

    fn transaction_hex_by_index(&self, txindex: TxIndex) -> Result<String> {
        let indexer = self.indexer();
        let reader = self.reader();
        let computer = self.computer();

        let total_size = indexer.vecs.tx.txindex_to_total_size.read_once(txindex)?;
        let position = computer.blks.txindex_to_position.read_once(txindex)?;

        let buffer = reader.read_raw_bytes(position, *total_size as usize)?;

        Ok(buffer.to_lower_hex_string())
    }

    fn outspend_details(&self, txinindex: TxInIndex) -> Result<TxOutspend> {
        let indexer = self.indexer();

        // Look up spending txindex directly
        let spending_txindex = indexer
            .vecs
            .txin
            .txinindex_to_txindex
            .read_once(txinindex)?;

        // Calculate vin
        let spending_first_txinindex = indexer
            .vecs
            .tx
            .txindex_to_first_txinindex
            .read_once(spending_txindex)?;
        let vin = Vin::from(usize::from(txinindex) - usize::from(spending_first_txinindex));

        // Get spending tx details
        let spending_txid = indexer
            .vecs
            .tx
            .txindex_to_txid
            .read_once(spending_txindex)?;
        let spending_height = indexer
            .vecs
            .tx
            .txindex_to_height
            .read_once(spending_txindex)?;
        let block_hash = indexer
            .vecs
            .block
            .height_to_blockhash
            .read_once(spending_height)?;
        let block_time = indexer
            .vecs
            .block
            .height_to_timestamp
            .read_once(spending_height)?;

        Ok(TxOutspend {
            spent: true,
            txid: Some(spending_txid),
            vin: Some(vin),
            status: Some(TxStatus {
                confirmed: true,
                block_height: Some(spending_height),
                block_hash: Some(block_hash),
                block_time: Some(block_time),
            }),
        })
    }
}
