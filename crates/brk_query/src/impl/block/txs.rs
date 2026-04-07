use std::io::Cursor;

use bitcoin::{consensus::Decodable, hex::DisplayHex};
use brk_error::{Error, Result};
use brk_types::{
    BlockHash, Height, OutputType, Sats, Timestamp, Transaction, TxIn, TxIndex, TxOut, TxStatus,
    Txid, Vout, Weight,
};
use vecdb::{AnyVec, ReadableVec, VecIndex};

use super::BLOCK_TXS_PAGE_SIZE;
use crate::Query;

impl Query {
    pub fn block_txids(&self, hash: &BlockHash) -> Result<Vec<Txid>> {
        let height = self.height_by_hash(hash)?;
        self.block_txids_by_height(height)
    }

    pub fn block_txs(&self, hash: &BlockHash, start_index: TxIndex) -> Result<Vec<Transaction>> {
        let height = self.height_by_hash(hash)?;
        let (first, tx_count) = self.block_tx_range(height)?;
        let start: usize = start_index.into();
        if start >= tx_count {
            return Ok(Vec::new());
        }
        let count = BLOCK_TXS_PAGE_SIZE.min(tx_count - start);
        let indices: Vec<TxIndex> = (first + start..first + start + count)
            .map(TxIndex::from)
            .collect();
        self.transactions_by_indices(&indices)
    }

    pub fn block_txid_at_index(&self, hash: &BlockHash, index: TxIndex) -> Result<Txid> {
        let height = self.height_by_hash(hash)?;
        self.block_txid_at_index_by_height(height, index.into())
    }

    // === Helper methods ===

    pub(crate) fn block_txids_by_height(&self, height: Height) -> Result<Vec<Txid>> {
        let (first, tx_count) = self.block_tx_range(height)?;
        Ok(self
            .indexer()
            .vecs
            .transactions
            .txid
            .collect_range_at(first, first + tx_count))
    }

    fn block_txid_at_index_by_height(&self, height: Height, index: usize) -> Result<Txid> {
        let (first, tx_count) = self.block_tx_range(height)?;
        if index >= tx_count {
            return Err(Error::OutOfRange("Transaction index out of range".into()));
        }
        Ok(self
            .indexer()
            .vecs
            .transactions
            .txid
            .reader()
            .get(first + index))
    }

    /// Batch-read transactions at arbitrary indices.
    /// Reads in ascending index order for I/O locality, returns in caller's order.
    pub fn transactions_by_indices(&self, indices: &[TxIndex]) -> Result<Vec<Transaction>> {
        if indices.is_empty() {
            return Ok(Vec::new());
        }

        let len = indices.len();

        // Sort positions ascending for sequential I/O (O(n) when already sorted)
        let mut order: Vec<usize> = (0..len).collect();
        order.sort_unstable_by_key(|&i| indices[i]);

        let indexer = self.indexer();
        let reader = self.reader();

        let mut txid_cursor = indexer.vecs.transactions.txid.cursor();
        let mut height_cursor = indexer.vecs.transactions.height.cursor();
        let mut locktime_cursor = indexer.vecs.transactions.raw_locktime.cursor();
        let mut total_size_cursor = indexer.vecs.transactions.total_size.cursor();
        let mut first_txin_cursor = indexer.vecs.transactions.first_txin_index.cursor();
        let mut position_cursor = indexer.vecs.transactions.position.cursor();

        let txid_reader = indexer.vecs.transactions.txid.reader();
        let first_txout_index_reader = indexer.vecs.transactions.first_txout_index.reader();
        let value_reader = indexer.vecs.outputs.value.reader();
        let output_type_reader = indexer.vecs.outputs.output_type.reader();
        let type_index_reader = indexer.vecs.outputs.type_index.reader();
        let addr_readers = indexer.vecs.addrs.addr_readers();

        let mut cached_block: Option<(Height, BlockHash, Timestamp)> = None;

        // Read in sorted order, write directly to original position
        let mut txs: Vec<Option<Transaction>> = (0..len).map(|_| None).collect();

        for &pos in &order {
            let tx_index = indices[pos];
            let idx = tx_index.to_usize();

            let txid = txid_cursor.get(idx).unwrap();
            let height = height_cursor.get(idx).unwrap();
            let lock_time = locktime_cursor.get(idx).unwrap();
            let total_size = total_size_cursor.get(idx).unwrap();
            let first_txin_index = first_txin_cursor.get(idx).unwrap();
            let position = position_cursor.get(idx).unwrap();

            let (block_hash, block_time) = if let Some((h, ref bh, bt)) = cached_block
                && h == height
            {
                (bh.clone(), bt)
            } else {
                let bh = indexer.vecs.blocks.blockhash.read_once(height)?;
                let bt = indexer.vecs.blocks.timestamp.collect_one(height).unwrap();
                cached_block = Some((height, bh.clone(), bt));
                (bh, bt)
            };

            let buffer = reader.read_raw_bytes(position, *total_size as usize)?;
            let tx = bitcoin::Transaction::consensus_decode(&mut Cursor::new(buffer))
                .map_err(|_| Error::Parse("Failed to decode transaction".into()))?;

            let outpoints = indexer.vecs.inputs.outpoint.collect_range_at(
                usize::from(first_txin_index),
                usize::from(first_txin_index) + tx.input.len(),
            );

            let input: Vec<TxIn> = tx
                .input
                .iter()
                .enumerate()
                .map(|(j, txin)| {
                    let outpoint = outpoints[j];
                    let is_coinbase = outpoint.is_coinbase();

                    let (prev_txid, prev_vout, prevout) = if is_coinbase {
                        (Txid::COINBASE, Vout::MAX, None)
                    } else {
                        let prev_tx_index = outpoint.tx_index();
                        let prev_vout = outpoint.vout();
                        let prev_txid = txid_reader.get(prev_tx_index.to_usize());
                        let prev_first_txout_index =
                            first_txout_index_reader.get(prev_tx_index.to_usize());
                        let prev_txout_index = prev_first_txout_index + prev_vout;
                        let prev_value = value_reader.get(usize::from(prev_txout_index));
                        let prev_output_type: OutputType =
                            output_type_reader.get(usize::from(prev_txout_index));
                        let prev_type_index = type_index_reader.get(usize::from(prev_txout_index));
                        let script_pubkey =
                            addr_readers.script_pubkey(prev_output_type, prev_type_index);
                        (
                            prev_txid,
                            prev_vout,
                            Some(TxOut::from((script_pubkey, prev_value))),
                        )
                    };

                    let witness = txin
                        .witness
                        .iter()
                        .map(|w| w.to_lower_hex_string())
                        .collect();

                    TxIn {
                        txid: prev_txid,
                        vout: prev_vout,
                        prevout,
                        script_sig: txin.script_sig.clone(),
                        script_sig_asm: (),
                        witness,
                        is_coinbase,
                        sequence: txin.sequence.0,
                        inner_redeem_script_asm: (),
                        inner_witness_script_asm: (),
                    }
                })
                .collect();

            let weight = Weight::from(tx.weight());
            let total_sigop_cost = tx.total_sigop_cost(|outpoint| {
                tx.input
                    .iter()
                    .position(|i| i.previous_output == *outpoint)
                    .and_then(|j| input[j].prevout.as_ref())
                    .map(|p| bitcoin::TxOut {
                        value: bitcoin::Amount::from_sat(u64::from(p.value)),
                        script_pubkey: p.script_pubkey.clone(),
                    })
            });
            let output: Vec<TxOut> = tx.output.into_iter().map(TxOut::from).collect();

            let mut transaction = Transaction {
                index: Some(tx_index),
                txid,
                version: tx.version.into(),
                lock_time,
                total_size: *total_size as usize,
                weight,
                total_sigop_cost,
                fee: Sats::ZERO,
                input,
                output,
                status: TxStatus {
                    confirmed: true,
                    block_height: Some(height),
                    block_hash: Some(block_hash),
                    block_time: Some(block_time),
                },
            };

            transaction.compute_fee();
            txs[pos] = Some(transaction);
        }

        Ok(txs.into_iter().map(Option::unwrap).collect())
    }

    /// Returns (first_tx_raw_index, tx_count) for a block at `height`.
    fn block_tx_range(&self, height: Height) -> Result<(usize, usize)> {
        let indexer = self.indexer();
        if height > self.indexed_height() {
            return Err(Error::OutOfRange("Block height out of range".into()));
        }
        let first: usize = indexer
            .vecs
            .transactions
            .first_tx_index
            .collect_one(height)
            .unwrap()
            .into();
        let next: usize = indexer
            .vecs
            .transactions
            .first_tx_index
            .collect_one(height.incremented())
            .unwrap_or_else(|| TxIndex::from(indexer.vecs.transactions.txid.len()))
            .into();
        Ok((first, next - first))
    }
}
