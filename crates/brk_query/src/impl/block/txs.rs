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
        self.transactions_by_range(first + start, count)
    }

    pub fn block_txid_at_index(&self, hash: &BlockHash, index: TxIndex) -> Result<Txid> {
        let height = self.height_by_hash(hash)?;
        self.block_txid_at_index_by_height(height, index.into())
    }

    // === Bulk transaction read ===

    /// Batch-read `count` consecutive transactions starting at raw index `start`.
    /// Block info is cached per unique height — free for same-block batches.
    pub fn transactions_by_range(&self, start: usize, count: usize) -> Result<Vec<Transaction>> {
        if count == 0 {
            return Ok(Vec::new());
        }

        let indexer = self.indexer();
        let reader = self.reader();
        let end = start + count;

        // 7 range reads instead of count * 7 point reads
        let txids: Vec<Txid> = indexer.vecs.transactions.txid.collect_range_at(start, end);
        let heights: Vec<Height> = indexer.vecs.transactions.height.collect_range_at(start, end);
        let versions = indexer.vecs.transactions.tx_version.collect_range_at(start, end);
        let lock_times = indexer.vecs.transactions.raw_locktime.collect_range_at(start, end);
        let total_sizes = indexer.vecs.transactions.total_size.collect_range_at(start, end);
        let first_txin_indices = indexer
            .vecs
            .transactions
            .first_txin_index
            .collect_range_at(start, end);
        let positions = indexer.vecs.transactions.position.collect_range_at(start, end);

        // Readers for prevout lookups (created once)
        let txid_reader = indexer.vecs.transactions.txid.reader();
        let first_txout_index_reader = indexer.vecs.transactions.first_txout_index.reader();
        let value_reader = indexer.vecs.outputs.value.reader();
        let output_type_reader = indexer.vecs.outputs.output_type.reader();
        let type_index_reader = indexer.vecs.outputs.type_index.reader();
        let addr_readers = indexer.vecs.addrs.addr_readers();

        // Block info cache — for same-block batches, read once
        let mut cached_block: Option<(Height, BlockHash, Timestamp)> = None;

        let mut txs = Vec::with_capacity(count);

        for i in 0..count {
            let height = heights[i];

            // Reuse block info if same height as previous tx
            let (block_hash, block_time) =
                if let Some((h, ref bh, bt)) = cached_block && h == height {
                    (bh.clone(), bt)
                } else {
                    let bh = indexer.vecs.blocks.blockhash.read_once(height)?;
                    let bt = indexer.vecs.blocks.timestamp.collect_one(height).unwrap();
                    cached_block = Some((height, bh.clone(), bt));
                    (bh, bt)
                };

            // Decode raw transaction from blk file
            let buffer = reader.read_raw_bytes(positions[i], *total_sizes[i] as usize)?;
            let tx = bitcoin::Transaction::consensus_decode(&mut Cursor::new(buffer))
                .map_err(|_| Error::Parse("Failed to decode transaction".into()))?;

            // Batch-read outpoints for this tx's inputs
            let outpoints = indexer.vecs.inputs.outpoint.collect_range_at(
                usize::from(first_txin_indices[i]),
                usize::from(first_txin_indices[i]) + tx.input.len(),
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
                        let prev_type_index =
                            type_index_reader.get(usize::from(prev_txout_index));
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
            let total_sigop_cost = tx.total_sigop_cost(|_| None);
            let output: Vec<TxOut> = tx.output.into_iter().map(TxOut::from).collect();

            let mut transaction = Transaction {
                index: Some(TxIndex::from(start + i)),
                txid: txids[i].clone(),
                version: versions[i],
                lock_time: lock_times[i],
                total_size: *total_sizes[i] as usize,
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
            txs.push(transaction);
        }

        Ok(txs)
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
