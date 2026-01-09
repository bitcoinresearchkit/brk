use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{Sats, TxInIndex, TxIndex, TxOutIndex, Vout};
use tracing::info;
use vecdb::{AnyStoredVec, AnyVec, Database, Exit, GenericStoredVec, TypedVecIterator, VecIndex};

use super::Vecs;
use crate::ComputeIndexes;

const BATCH_SIZE: usize = 2 * 1024 * 1024 * 1024 / size_of::<Entry>();

impl Vecs {
    pub fn compute(
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

        let mut outpoint_iter = indexer.vecs.inputs.outpoint.iter()?;
        let mut first_txoutindex_iter = indexer.vecs.transactions.first_txoutindex.iter()?;
        let mut value_iter = indexer.vecs.outputs.value.iter()?;
        let mut entries: Vec<Entry> = Vec::with_capacity(BATCH_SIZE);

        let mut batch_start = min;
        while batch_start < target {
            let batch_end = (batch_start + BATCH_SIZE).min(target);

            entries.clear();
            for i in batch_start..batch_end {
                let txinindex = TxInIndex::from(i);
                let outpoint = outpoint_iter.get_unwrap(txinindex);
                entries.push(Entry {
                    txinindex,
                    txindex: outpoint.txindex(),
                    vout: outpoint.vout(),
                    txoutindex: TxOutIndex::COINBASE,
                    value: Sats::MAX,
                });
            }

            // Coinbase entries (txindex MAX) sorted to end
            entries.sort_unstable_by_key(|e| e.txindex);
            for entry in &mut entries {
                if entry.txindex.is_coinbase() {
                    break;
                }
                entry.txoutindex = first_txoutindex_iter.get_unwrap(entry.txindex) + entry.vout;
            }

            entries.sort_unstable_by_key(|e| e.txoutindex);
            for entry in &mut entries {
                if entry.txoutindex.is_coinbase() {
                    break;
                }
                entry.value = value_iter.get_unwrap(entry.txoutindex);
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
