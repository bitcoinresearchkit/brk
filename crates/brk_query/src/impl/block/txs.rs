use std::io::Cursor;

use bitcoin::consensus::Decodable;
use brk_error::{Error, OptionData, Result};
use brk_types::{
    BlkPosition, BlockHash, Height, OutPoint, OutputType, RawLockTime, Sats, StoredU32,
    Transaction, TxIn, TxInIndex, TxIndex, TxOut, TxStatus, Txid, TypeIndex, Vout, Weight,
};
use rustc_hash::FxHashMap;
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
    ///
    /// Three-phase approach for optimal I/O:
    ///   Phase 1 — Decode transactions & collect outpoints (sorted by tx_index)
    ///   Phase 2 — Batch-read all prevout data (sorted by prev_tx_index, then txout_index)
    ///   Phase 3 — Assemble Transaction objects from pre-fetched data
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

        // ── Phase 1: Decode all transactions, collect outpoints ─────────

        let mut txid_cursor = indexer.vecs.transactions.txid.cursor();
        let mut total_size_cursor = indexer.vecs.transactions.total_size.cursor();
        let mut first_txin_cursor = indexer.vecs.transactions.first_txin_index.cursor();
        let mut position_cursor = indexer.vecs.transactions.position.cursor();

        struct DecodedTx {
            pos: usize,
            tx_index: TxIndex,
            txid: Txid,
            total_size: StoredU32,
            status: TxStatus,
            decoded: bitcoin::Transaction,
            first_txin_index: TxInIndex,
            outpoints: Vec<OutPoint>,
        }

        let mut decoded_txs: Vec<DecodedTx> = Vec::with_capacity(len);
        let mut total_inputs: usize = 0;
        let mut cached_status: Option<(Height, TxStatus)> = None;

        // Phase 1a: Read metadata + decode transactions (no outpoint reads yet)
        for &pos in &order {
            let tx_index = indices[pos];
            let idx = tx_index.to_usize();

            let txid: Txid = txid_cursor.get(idx).data()?;
            let total_size: StoredU32 = total_size_cursor.get(idx).data()?;
            let first_txin_index: TxInIndex = first_txin_cursor.get(idx).data()?;
            let position: BlkPosition = position_cursor.get(idx).data()?;

            let height = self.confirmed_status_height(tx_index)?;
            let status = if let Some((h, ref s)) = cached_status
                && h == height
            {
                s.clone()
            } else {
                let s = self.confirmed_status_at(height)?;
                cached_status = Some((height, s.clone()));
                s
            };

            let buffer = reader.read_raw_bytes(position, *total_size as usize)?;
            let decoded = bitcoin::Transaction::consensus_decode(&mut Cursor::new(buffer))
                .map_err(|_| Error::Parse("Failed to decode transaction".into()))?;

            total_inputs += decoded.input.len();

            decoded_txs.push(DecodedTx {
                pos,
                tx_index,
                txid,
                total_size,
                status,
                decoded,
                first_txin_index,
                outpoints: Vec::new(),
            });
        }

        // Phase 1b: Batch-read outpoints + prevout data via cursors (PcoVec —
        // sequential cursor avoids re-decompressing the same pages).
        // Reading output_type/type_index/value HERE from inputs vecs (sequential)
        // avoids random-reading them from outputs vecs in Phase 2.
        let mut outpoint_cursor = indexer.vecs.inputs.outpoint.cursor();
        let mut input_output_type_cursor = indexer.vecs.inputs.output_type.cursor();
        let mut input_type_index_cursor = indexer.vecs.inputs.type_index.cursor();
        let mut input_value_cursor = self.computer().inputs.spent.value.cursor();

        let mut prevout_input_data: FxHashMap<OutPoint, (OutputType, TypeIndex, Sats)> =
            FxHashMap::with_capacity_and_hasher(total_inputs, Default::default());

        for dtx in &mut decoded_txs {
            let start = usize::from(dtx.first_txin_index);
            let count = dtx.decoded.input.len();
            let mut outpoints = Vec::with_capacity(count);
            for i in 0..count {
                let op: OutPoint = outpoint_cursor.get(start + i).data()?;
                if op.is_not_coinbase() {
                    let ot: OutputType = input_output_type_cursor.get(start + i).data()?;
                    let ti: TypeIndex = input_type_index_cursor.get(start + i).data()?;
                    let val: Sats = input_value_cursor.get(start + i).data()?;
                    prevout_input_data.insert(op, (ot, ti, val));
                }
                outpoints.push(op);
            }
            dtx.outpoints = outpoints;
        }

        // ── Phase 2: Build prevout TxOut map (script_pubkey from addr vecs) ──
        // Sort by (output_type, type_index) for sequential BytesVec access
        // within each address type's file.

        let addr_readers = indexer.vecs.addrs.addr_readers();

        let mut sorted_prevouts: Vec<(OutPoint, OutputType, TypeIndex, Sats)> =
            Vec::with_capacity(prevout_input_data.len());
        for (&op, &(ot, ti, val)) in &prevout_input_data {
            sorted_prevouts.push((op, ot, ti, val));
        }
        sorted_prevouts.sort_unstable_by_key(|&(_, ot, ti, _)| (ot, ti));

        let mut prevout_map: FxHashMap<OutPoint, TxOut> =
            FxHashMap::with_capacity_and_hasher(sorted_prevouts.len(), Default::default());

        for &(op, output_type, type_index, value) in &sorted_prevouts {
            let script_pubkey = addr_readers.script_pubkey(output_type, type_index);
            prevout_map.insert(op, TxOut::from((script_pubkey, value)));
        }

        // ── Phase 3: Assemble Transaction objects ───────────────────────

        let mut txs: Vec<Option<Transaction>> = (0..len).map(|_| None).collect();

        for dtx in decoded_txs {
            let input: Vec<TxIn> = dtx
                .decoded
                .input
                .iter()
                .enumerate()
                .map(|(j, txin)| {
                    let outpoint = dtx.outpoints[j];
                    let is_coinbase = outpoint.is_coinbase();

                    let (prev_txid, prev_vout, prevout) = if is_coinbase {
                        (Txid::COINBASE, Vout::MAX, None)
                    } else {
                        let prev_txid = Txid::from(txin.previous_output.txid);
                        let prev_txout = prevout_map.get(&outpoint).data()?.clone();
                        (prev_txid, outpoint.vout(), Some(prev_txout))
                    };

                    let witness = txin.witness.clone().into();

                    Ok(TxIn {
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
                    })
                })
                .collect::<Result<_>>()?;

            let weight = Weight::from(dtx.decoded.weight());

            // O(n) sigop cost via FxHashMap instead of O(n²) linear scan
            let outpoint_to_idx: FxHashMap<bitcoin::OutPoint, usize> = dtx
                .decoded
                .input
                .iter()
                .enumerate()
                .map(|(j, txin)| (txin.previous_output, j))
                .collect();

            let total_sigop_cost = dtx.decoded.total_sigop_cost(|outpoint| {
                outpoint_to_idx
                    .get(outpoint)
                    .and_then(|&j| input[j].prevout.as_ref())
                    .map(|p| bitcoin::TxOut {
                        value: bitcoin::Amount::from_sat(u64::from(p.value)),
                        script_pubkey: p.script_pubkey.clone(),
                    })
            });

            let output: Vec<TxOut> = dtx.decoded.output.into_iter().map(TxOut::from).collect();

            let mut transaction = Transaction {
                index: Some(dtx.tx_index),
                txid: dtx.txid,
                version: dtx.decoded.version.into(),
                lock_time: RawLockTime::from(dtx.decoded.lock_time),
                total_size: *dtx.total_size as usize,
                weight,
                total_sigop_cost,
                fee: Sats::ZERO,
                input,
                output,
                status: dtx.status,
            };

            transaction.compute_fee();
            txs[dtx.pos] = Some(transaction);
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
            .data()?
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
