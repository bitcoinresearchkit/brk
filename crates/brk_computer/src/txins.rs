use std::path::Path;

use brk_error::Result;
use brk_indexer::Indexer;
use brk_traversable::Traversable;
use brk_types::{Sats, TxInIndex, TxIndex, TxOutIndex, Version, Vout};
use log::info;
use vecdb::{
    AnyStoredVec, AnyVec, Database, Exit, GenericStoredVec, ImportableVec, PAGE_SIZE, PcoVec,
    TypedVecIterator, VecIndex,
};

use super::Indexes;

const BATCH_SIZE: usize = 2 * 1024 * 1024 * 1024 / size_of::<Entry>();

#[derive(Clone, Traversable)]
pub struct Vecs {
    db: Database,
    pub txinindex_to_txoutindex: PcoVec<TxInIndex, TxOutIndex>,
    pub txinindex_to_value: PcoVec<TxInIndex, Sats>,
}

impl Vecs {
    pub fn forced_import(parent_path: &Path, parent_version: Version) -> Result<Self> {
        let db = Database::open(&parent_path.join("txins"))?;
        db.set_min_len(PAGE_SIZE * 10_000_000)?;

        let version = parent_version + Version::ZERO;

        let this = Self {
            txinindex_to_txoutindex: PcoVec::forced_import(&db, "txoutindex", version)?,
            txinindex_to_value: PcoVec::forced_import(&db, "value", version)?,
            db,
        };

        this.db.retain_regions(
            this.iter_any_exportable()
                .flat_map(|v| v.region_names())
                .collect(),
        )?;
        this.db.compact()?;

        Ok(this)
    }

    pub fn compute(
        &mut self,
        indexer: &Indexer,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.compute_(indexer, starting_indexes, exit)?;
        let _lock = exit.lock();
        self.db.compact()?;
        Ok(())
    }

    fn compute_(
        &mut self,
        indexer: &Indexer,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        let target = indexer.vecs.txin.txinindex_to_outpoint.len();
        if target == 0 {
            return Ok(());
        }

        let len1 = self.txinindex_to_txoutindex.len();
        let len2 = self.txinindex_to_value.len();
        let starting = starting_indexes.txinindex.to_usize();
        let min = len1.min(len2).min(starting);

        if min >= target {
            return Ok(());
        }

        let mut outpoint_iter = indexer.vecs.txin.txinindex_to_outpoint.iter()?;
        let mut first_txoutindex_iter = indexer.vecs.tx.txindex_to_first_txoutindex.iter()?;
        let mut value_iter = indexer.vecs.txout.txoutindex_to_value.iter()?;
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
                self.txinindex_to_txoutindex
                    .truncate_push(entry.txinindex, entry.txoutindex)?;
                self.txinindex_to_value
                    .truncate_push(entry.txinindex, entry.value)?;
            }

            if batch_end < target {
                info!("TxIns: {:.2}%", batch_end as f64 / target as f64 * 100.0);
            }

            let _lock = exit.lock();
            self.txinindex_to_txoutindex.write()?;
            self.txinindex_to_value.write()?;
            self.db.flush()?;

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
