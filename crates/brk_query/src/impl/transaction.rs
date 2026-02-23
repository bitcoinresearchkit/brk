use std::io::Cursor;

use bitcoin::{consensus::Decodable, hex::DisplayHex};
use brk_error::{Error, Result};
use brk_types::{
    Sats, Transaction, TxIn, TxInIndex, TxIndex, TxOut, TxOutspend, TxStatus, Txid, TxidParam,
    TxidPrefix, Vin, Vout, Weight,
};
use vecdb::{ReadableVec, VecIndex};

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
        let height = indexer.vecs.transactions.height.collect_one(txindex).unwrap();
        let block_hash = indexer.vecs.blocks.blockhash.read_once(height)?;
        let block_time = indexer.vecs.blocks.timestamp.collect_one(height).unwrap();

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
            .transactions
            .first_txoutindex
            .read_once(txindex)?;
        let txoutindex = first_txoutindex + vout;

        // Look up spend status
        let computer = self.computer();
        let txinindex = computer.outputs.spent.txinindex.read_once(txoutindex)?;

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
            .transactions
            .first_txoutindex
            .read_once(txindex)?;
        let next_first_txoutindex = indexer
            .vecs
            .transactions
            .first_txoutindex
            .read_once(txindex.incremented())?;
        let output_count = usize::from(next_first_txoutindex) - usize::from(first_txoutindex);

        // Get spend status for each output
        let computer = self.computer();
        let txinindex_reader = computer.outputs.spent.txinindex.reader();

        let mut outspends = Vec::with_capacity(output_count);
        for i in 0..output_count {
            let txoutindex = first_txoutindex + Vout::from(i);
            let txinindex = txinindex_reader.get(usize::from(txoutindex));

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

        // Get tx metadata using collect_one for PcoVec, read_once for BytesVec
        let txid = indexer.vecs.transactions.txid.read_once(txindex)?;
        let height = indexer.vecs.transactions.height.collect_one(txindex).unwrap();
        let version = indexer.vecs.transactions.txversion.collect_one(txindex).unwrap();
        let lock_time = indexer.vecs.transactions.rawlocktime.collect_one(txindex).unwrap();
        let total_size = indexer.vecs.transactions.total_size.collect_one(txindex).unwrap();
        let first_txinindex = indexer
            .vecs
            .transactions
            .first_txinindex
            .collect_one(txindex)
            .unwrap();
        let position = computer.positions.tx_position.collect_one(txindex).unwrap();

        // Get block info for status
        let block_hash = indexer.vecs.blocks.blockhash.read_once(height)?;
        let block_time = indexer.vecs.blocks.timestamp.collect_one(height).unwrap();

        // Read and decode the raw transaction from blk file
        let buffer = reader.read_raw_bytes(position, *total_size as usize)?;
        let mut cursor = Cursor::new(buffer);
        let tx = bitcoin::Transaction::consensus_decode(&mut cursor)
            .map_err(|_| Error::Parse("Failed to decode transaction".into()))?;

        // Create readers for random access lookups
        let txid_reader = indexer.vecs.transactions.txid.reader();
        let first_txoutindex_reader = indexer.vecs.transactions.first_txoutindex.reader();
        let value_reader = indexer.vecs.outputs.value.reader();

        // Batch-read outpoints for all inputs (avoids per-input PcoVec page decompression)
        let outpoints: Vec<_> = indexer.vecs.inputs.outpoint.collect_range_at(
            usize::from(first_txinindex),
            usize::from(first_txinindex) + tx.input.len(),
        );

        // Build inputs with prevout information
        let input: Vec<TxIn> = tx
            .input
            .iter()
            .enumerate()
            .map(|(i, txin)| {
                let outpoint = outpoints[i];

                let is_coinbase = outpoint.is_coinbase();

                // Get prevout info if not coinbase
                let (prev_txid, prev_vout, prevout) = if is_coinbase {
                    (Txid::COINBASE, Vout::MAX, None)
                } else {
                    let prev_txindex = outpoint.txindex();
                    let prev_vout = outpoint.vout();
                    let prev_txid = txid_reader.get(prev_txindex.to_usize());

                    // Calculate the txoutindex for the prevout
                    let prev_first_txoutindex = first_txoutindex_reader.get(prev_txindex.to_usize());
                    let prev_txoutindex = prev_first_txoutindex + prev_vout;

                    // Get the value of the prevout
                    let prev_value = value_reader.get(usize::from(prev_txoutindex));

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

        let total_size = indexer.vecs.transactions.total_size.collect_one(txindex).unwrap();
        let position = computer.positions.tx_position.collect_one(txindex).unwrap();

        let buffer = reader.read_raw_bytes(position, *total_size as usize)?;

        Ok(buffer.to_lower_hex_string())
    }

    fn outspend_details(&self, txinindex: TxInIndex) -> Result<TxOutspend> {
        let indexer = self.indexer();

        // Look up spending txindex directly
        let spending_txindex = indexer.vecs.inputs.txindex.collect_one(txinindex).unwrap();

        // Calculate vin
        let spending_first_txinindex = indexer
            .vecs
            .transactions
            .first_txinindex
            .collect_one(spending_txindex)
            .unwrap();
        let vin = Vin::from(usize::from(txinindex) - usize::from(spending_first_txinindex));

        // Get spending tx details
        let spending_txid = indexer.vecs.transactions.txid.read_once(spending_txindex)?;
        let spending_height = indexer
            .vecs
            .transactions
            .height
            .collect_one(spending_txindex)
            .unwrap();
        let block_hash = indexer.vecs.blocks.blockhash.read_once(spending_height)?;
        let block_time = indexer.vecs.blocks.timestamp.collect_one(spending_height).unwrap();

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
