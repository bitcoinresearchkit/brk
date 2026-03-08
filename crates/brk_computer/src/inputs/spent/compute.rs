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
            + indexer.vecs.transactions.first_txoutindex.version()
            + indexer.vecs.outputs.value.version();
        self.txoutindex
            .validate_computed_version_or_reset(dep_version)?;
        self.value.validate_computed_version_or_reset(dep_version)?;

        let target = indexer.vecs.inputs.outpoint.len();
        if target == 0 {
            return Ok(());
        }

        let len1 = self.txoutindex.len();
        let len2 = self.value.len();
        let starting = starting_indexes.txinindex.to_usize();
        let min = len1.min(len2).min(starting);

        if min >= target {
            return Ok(());
        }

        let first_txoutindex_reader = indexer.vecs.transactions.first_txoutindex.reader();
        let value_reader = indexer.vecs.outputs.value.reader();
        let actual_total = target - min;
        let mut entries: Vec<Entry> = Vec::with_capacity(actual_total.min(BATCH_SIZE));
        // Pre-allocate output buffers for scatter-write pattern
        let mut out_txoutindex: Vec<TxOutIndex> = Vec::new();
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
                        txindex: outpoint.txindex(),
                        vout: outpoint.vout(),
                        txoutindex: TxOutIndex::COINBASE,
                        value: Sats::MAX,
                    });
                    j += 1;
                });

            // Sort 1: by txindex (group by transaction for sequential first_txoutindex reads)
            entries.sort_unstable_by_key(|e| e.txindex);
            for entry in &mut entries {
                if entry.txindex.is_coinbase() {
                    break;
                }
                entry.txoutindex =
                    first_txoutindex_reader.get(entry.txindex.to_usize()) + entry.vout;
            }

            // Sort 2: by txoutindex (sequential value reads)
            entries.sort_unstable_by_key(|e| e.txoutindex);
            for entry in &mut entries {
                if entry.txoutindex.is_coinbase() {
                    break;
                }
                entry.value = value_reader.get(entry.txoutindex.to_usize());
            }

            // Scatter-write to output buffers using original_idx (avoids Sort 3)
            out_txoutindex.clear();
            out_txoutindex.resize(batch_len, TxOutIndex::COINBASE);
            out_value.clear();
            out_value.resize(batch_len, Sats::MAX);

            for entry in &entries {
                out_txoutindex[entry.original_idx] = entry.txoutindex;
                out_value[entry.original_idx] = entry.value;
            }

            for i in 0..batch_len {
                let txinindex = TxInIndex::from(batch_start + i);
                self.txoutindex
                    .truncate_push(txinindex, out_txoutindex[i])?;
                self.value.truncate_push(txinindex, out_value[i])?;
            }

            if batch_end < target {
                info!("TxIns: {:.2}%", batch_end as f64 / target as f64 * 100.0);
            }

            let _lock = exit.lock();
            self.txoutindex.write()?;
            self.value.write()?;
            db.flush()?;

            batch_start = batch_end;
        }

        Ok(())
    }
}

struct Entry {
    original_idx: usize,
    txindex: TxIndex,
    vout: Vout,
    txoutindex: TxOutIndex,
    value: Sats,
}
