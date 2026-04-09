use std::io::Cursor;

use bitcoin::{consensus::Decodable, hex::DisplayHex};
use brk_error::{Error, OptionData, Result};
use brk_types::{
    BlkPosition, BlockHash, Height, OutPoint, OutputType, RawLockTime, Sats, StoredU32, Timestamp,
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

        let t0 = std::time::Instant::now();
        let len = indices.len();

        // Sort positions ascending for sequential I/O (O(n) when already sorted)
        let mut order: Vec<usize> = (0..len).collect();
        order.sort_unstable_by_key(|&i| indices[i]);

        let indexer = self.indexer();
        let reader = self.reader();

        // ── Phase 1: Decode all transactions, collect outpoints ─────────

        let tx_heights = &self.computer().indexes.tx_heights;
        let mut txid_cursor = indexer.vecs.transactions.txid.cursor();
        let mut locktime_cursor = indexer.vecs.transactions.raw_locktime.cursor();
        let mut total_size_cursor = indexer.vecs.transactions.total_size.cursor();
        let mut first_txin_cursor = indexer.vecs.transactions.first_txin_index.cursor();
        let mut position_cursor = indexer.vecs.transactions.position.cursor();
        struct DecodedTx {
            pos: usize,
            tx_index: TxIndex,
            txid: Txid,
            height: Height,
            lock_time: RawLockTime,
            total_size: StoredU32,
            block_hash: BlockHash,
            block_time: Timestamp,
            decoded: bitcoin::Transaction,
            first_txin_index: TxInIndex,
            outpoints: Vec<OutPoint>,
        }

        let mut decoded_txs: Vec<DecodedTx> = Vec::with_capacity(len);
        let mut total_inputs: usize = 0;

        // Phase 1a: Read metadata + decode transactions (no outpoint reads yet)
        for &pos in &order {
            let tx_index = indices[pos];
            let idx = tx_index.to_usize();

            let txid: Txid = txid_cursor.get(idx).data()?;
            let height: Height = tx_heights.get_shared(tx_index).data()?;
            let lock_time: RawLockTime = locktime_cursor.get(idx).data()?;
            let total_size: StoredU32 = total_size_cursor.get(idx).data()?;
            let first_txin_index: TxInIndex = first_txin_cursor.get(idx).data()?;
            let position: BlkPosition = position_cursor.get(idx).data()?;

            let block_hash = indexer
                .vecs
                .blocks
                .cached_blockhash
                .collect_one(height)
                .data()?;
            let block_time = indexer
                .vecs
                .blocks
                .cached_timestamp
                .collect_one(height)
                .data()?;

            let buffer = reader.read_raw_bytes(position, *total_size as usize)?;
            let decoded = bitcoin::Transaction::consensus_decode(&mut Cursor::new(buffer))
                .map_err(|_| Error::Parse("Failed to decode transaction".into()))?;

            total_inputs += decoded.input.len();

            decoded_txs.push(DecodedTx {
                pos,
                tx_index,
                txid,
                height,
                lock_time,
                total_size,
                block_hash,
                block_time,
                decoded,
                first_txin_index,
                outpoints: Vec::new(),
            });
        }

        let t_phase1a = t0.elapsed();

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

        let t_phase1b = t0.elapsed();

        // ── Phase 2: Batch-read prevout data in sorted order ────────────

        // Collect all non-coinbase outpoints, deduplicate, sort by tx_index
        let mut prevout_keys: Vec<OutPoint> = Vec::with_capacity(total_inputs);
        for dtx in &decoded_txs {
            for &op in &dtx.outpoints {
                if op.is_not_coinbase() {
                    prevout_keys.push(op);
                }
            }
        }
        prevout_keys.sort_unstable();
        prevout_keys.dedup();

        // Batch-read txid sorted by prev_tx_index (only remaining random read)
        let txid_reader = indexer.vecs.transactions.txid.reader();

        let addr_readers = indexer.vecs.addrs.addr_readers();

        let mut prevout_map: FxHashMap<OutPoint, (Txid, TxOut)> =
            FxHashMap::with_capacity_and_hasher(prevout_keys.len(), Default::default());

        for &op in &prevout_keys {
            let txid = txid_reader.get(op.tx_index().to_usize());
            // output_type, type_index, value pre-read from inputs vecs (sequential)
            let &(output_type, type_index, value) =
                prevout_input_data.get(&op).unwrap();
            let script_pubkey = addr_readers.script_pubkey(output_type, type_index);
            prevout_map.insert(op, (txid, TxOut::from((script_pubkey, value))));
        }

        let t_phase2 = t0.elapsed();

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
                        let (prev_txid, prev_txout) =
                            prevout_map.get(&outpoint).data()?.clone();
                        (prev_txid, outpoint.vout(), Some(prev_txout))
                    };

                    let witness = txin
                        .witness
                        .iter()
                        .map(|w| w.to_lower_hex_string())
                        .collect();

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
                lock_time: dtx.lock_time,
                total_size: *dtx.total_size as usize,
                weight,
                total_sigop_cost,
                fee: Sats::ZERO,
                input,
                output,
                status: TxStatus {
                    confirmed: true,
                    block_height: Some(dtx.height),
                    block_hash: Some(dtx.block_hash),
                    block_time: Some(dtx.block_time),
                },
            };

            transaction.compute_fee();
            txs[dtx.pos] = Some(transaction);
        }

        let t_phase3 = t0.elapsed();

        if t_phase3.as_millis() > 50 {
            eprintln!(
                "[perf:txs] n={} vin={} prevouts={} | 1a={:.1?} 1b={:.1?} | 2={:.1?} | 3={:.1?} | total={:.1?}",
                len,
                total_inputs,
                prevout_keys.len(),
                t_phase1a,
                t_phase1b - t_phase1a,
                t_phase2 - t_phase1b,
                t_phase3 - t_phase2,
                t_phase3,
            );
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
