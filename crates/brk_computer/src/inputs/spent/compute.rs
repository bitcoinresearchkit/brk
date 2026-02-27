use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{Sats, TxInIndex, TxIndex, TxOutIndex, Vout};
use tracing::info;
use vecdb::{AnyStoredVec, AnyVec, Database, Exit, WritableVec, ReadableVec, VecIndex};

use super::Vecs;
use crate::ComputeIndexes;

const BATCH_SIZE: usize = 2 * 1024 * 1024 * 1024 / size_of::<Entry>();

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        db: &Database,
        indexer: &Indexer,
        starting_indexes: &ComputeIndexes,
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

        let mut batch_start = min;
        while batch_start < target {
            let batch_end = (batch_start + BATCH_SIZE).min(target);

            entries.clear();
            let mut j = 0usize;
            indexer.vecs.inputs.outpoint.for_each_range_at(batch_start, batch_end, |outpoint| {
                entries.push(Entry {
                    txinindex: TxInIndex::from(batch_start + j),
                    txindex: outpoint.txindex(),
                    vout: outpoint.vout(),
                    txoutindex: TxOutIndex::COINBASE,
                    value: Sats::MAX,
                });
                j += 1;
            });

            // Coinbase entries (txindex MAX) sorted to end
            entries.sort_unstable_by_key(|e| e.txindex);
            for entry in &mut entries {
                if entry.txindex.is_coinbase() {
                    break;
                }
                entry.txoutindex = first_txoutindex_reader.get(entry.txindex.to_usize()) + entry.vout;
            }

            entries.sort_unstable_by_key(|e| e.txoutindex);
            for entry in &mut entries {
                if entry.txoutindex.is_coinbase() {
                    break;
                }
                entry.value = value_reader.get(entry.txoutindex.to_usize());
            }

            entries.sort_unstable_by_key(|e| e.txinindex);
            for entry in &entries {
                self.txoutindex
                    .truncate_push(entry.txinindex, entry.txoutindex)?;
                self.value.truncate_push(entry.txinindex, entry.value)?;
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
    txinindex: TxInIndex,
    txindex: TxIndex,
    vout: Vout,
    txoutindex: TxOutIndex,
    value: Sats,
}
