use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{Indexes, Sats, TxInIndex, TxIndex, TxOutIndex, Vout};
use tracing::info;
use vecdb::{AnyStoredVec, AnyVec, Database, Exit, ReadableVec, VecIndex, WritableVec};

use super::Vecs;

const BATCH_SIZE: usize = 2 * 1024 * 1024 * 1024 / size_of::<Entry>();

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        db: &Database,
        indexer: &Indexer,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        // Validate computed versions against dependencies
        let dep_version = indexer.vecs.inputs.outpoint.version()
            + indexer.vecs.transactions.first_txout_index.version()
            + indexer.vecs.outputs.value.version();
        self.txout_index
            .validate_computed_version_or_reset(dep_version)?;
        self.value.validate_computed_version_or_reset(dep_version)?;

        let target = indexer.vecs.inputs.outpoint.len();
        if target == 0 {
            return Ok(());
        }

        let len1 = self.txout_index.len();
        let len2 = self.value.len();
        let starting = starting_indexes.txin_index.to_usize();
        let min = len1.min(len2).min(starting);

        if min >= target {
            return Ok(());
        }

        let first_txout_index_reader = indexer.vecs.transactions.first_txout_index.reader();
        let value_reader = indexer.vecs.outputs.value.reader();
        let actual_total = target - min;
        let mut entries: Vec<Entry> = Vec::with_capacity(actual_total.min(BATCH_SIZE));
        // Pre-allocate output buffers for scatter-write pattern
        let mut out_txout_index: Vec<TxOutIndex> = Vec::new();
        let mut out_value: Vec<Sats> = Vec::new();

        let mut batch_start = min;
        while batch_start < target {
            let batch_end = (batch_start + BATCH_SIZE).min(target);
            let batch_len = batch_end - batch_start;

            entries.clear();
            let mut j = 0usize;
            indexer
                .vecs
                .inputs
                .outpoint
                .for_each_range_at(batch_start, batch_end, |outpoint| {
                    entries.push(Entry {
                        original_idx: j,
                        tx_index: outpoint.tx_index(),
                        vout: outpoint.vout(),
                        txout_index: TxOutIndex::COINBASE,
                        value: Sats::MAX,
                    });
                    j += 1;
                });

            // Sort 1: by tx_index (group by transaction for sequential first_txout_index reads)
            entries.sort_unstable_by_key(|e| e.tx_index);
            for entry in &mut entries {
                if entry.tx_index.is_coinbase() {
                    break;
                }
                entry.txout_index =
                    first_txout_index_reader.get(entry.tx_index.to_usize()) + entry.vout;
            }

            // Sort 2: by txout_index (sequential value reads)
            entries.sort_unstable_by_key(|e| e.txout_index);
            for entry in &mut entries {
                if entry.txout_index.is_coinbase() {
                    break;
                }
                entry.value = value_reader.get(entry.txout_index.to_usize());
            }

            // Scatter-write to output buffers using original_idx (avoids Sort 3)
            out_txout_index.clear();
            out_txout_index.resize(batch_len, TxOutIndex::COINBASE);
            out_value.clear();
            out_value.resize(batch_len, Sats::MAX);

            for entry in &entries {
                out_txout_index[entry.original_idx] = entry.txout_index;
                out_value[entry.original_idx] = entry.value;
            }

            for i in 0..batch_len {
                let txin_index = TxInIndex::from(batch_start + i);
                self.txout_index
                    .truncate_push(txin_index, out_txout_index[i])?;
                self.value.truncate_push(txin_index, out_value[i])?;
            }

            if batch_end < target {
                info!("TxIns: {:.2}%", batch_end as f64 / target as f64 * 100.0);
            }

            let _lock = exit.lock();
            self.txout_index.write()?;
            self.value.write()?;
            db.flush()?;

            batch_start = batch_end;
        }

        Ok(())
    }
}

struct Entry {
    original_idx: usize,
    tx_index: TxIndex,
    vout: Vout,
    txout_index: TxOutIndex,
    value: Sats,
}
