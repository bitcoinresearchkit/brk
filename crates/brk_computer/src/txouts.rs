use std::path::Path;

use brk_error::Result;
use brk_indexer::Indexer;
use brk_traversable::Traversable;
use brk_types::{Height, TxInIndex, TxOutIndex, Version};
use log::info;
use vecdb::{
    AnyVec, BytesVec, Database, Exit, GenericStoredVec, ImportableVec, PAGE_SIZE, Stamp,
    TypedVecIterator,
};

use super::{txins, Indexes};

const ONE_GB: usize = 1024 * 1024 * 1024;
const BATCH_SIZE: usize = ONE_GB / size_of::<(TxOutIndex, TxInIndex)>();

#[derive(Clone, Traversable)]
pub struct Vecs {
    db: Database,
    pub txoutindex_to_txinindex: BytesVec<TxOutIndex, TxInIndex>,
}

impl Vecs {
    pub fn forced_import(parent_path: &Path, parent_version: Version) -> Result<Self> {
        let db = Database::open(&parent_path.join("txouts"))?;
        db.set_min_len(PAGE_SIZE * 10_000_000)?;

        let version = parent_version + Version::ZERO;

        let this = Self {
            txoutindex_to_txinindex: BytesVec::forced_import(&db, "txinindex", version)?,
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
        txins: &txins::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.compute_(indexer, txins, starting_indexes, exit)?;
        self.db.compact()?;
        Ok(())
    }

    fn compute_(
        &mut self,
        indexer: &Indexer,
        txins: &txins::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        let target_txoutindex = indexer.vecs.txout.txoutindex_to_value.len();
        let target_txinindex = txins.txinindex_to_txoutindex.len();

        if target_txoutindex == 0 {
            return Ok(());
        }

        let target_height = Height::from(indexer.vecs.block.height_to_blockhash.len() - 1);

        let min_txoutindex =
            TxOutIndex::from(self.txoutindex_to_txinindex.len()).min(starting_indexes.txoutindex);
        let min_txinindex = usize::from(starting_indexes.txinindex);

        let starting_stamp = Stamp::from(starting_indexes.height);
        let _ = self.txoutindex_to_txinindex.rollback_before(starting_stamp);

        self.txoutindex_to_txinindex
            .truncate_if_needed(min_txoutindex)?;

        self.txoutindex_to_txinindex
            .fill_to(target_txoutindex, TxInIndex::UNSPENT)?;

        if min_txinindex < target_txinindex {
            info!(
                "TxOuts: computing spend mappings ({} to {})",
                min_txinindex, target_txinindex
            );

            let mut txoutindex_iter = txins.txinindex_to_txoutindex.iter()?;
            let mut pairs: Vec<(TxOutIndex, TxInIndex)> = Vec::with_capacity(BATCH_SIZE);

            let mut batch_start = min_txinindex;
            while batch_start < target_txinindex {
                let batch_end = (batch_start + BATCH_SIZE).min(target_txinindex);

                pairs.clear();
                for i in batch_start..batch_end {
                    let txinindex = TxInIndex::from(i);
                    let txoutindex = txoutindex_iter.get_unwrap(txinindex);

                    if txoutindex.is_coinbase() {
                        continue;
                    }

                    pairs.push((txoutindex, txinindex));
                }

                pairs.sort_unstable_by_key(|(txoutindex, _)| *txoutindex);

                for &(txoutindex, txinindex) in &pairs {
                    self.txoutindex_to_txinindex.update(txoutindex, txinindex)?;
                }

                batch_start = batch_end;
            }
        }

        let _lock = exit.lock();
        self.txoutindex_to_txinindex
            .stamped_write_with_changes(Stamp::from(target_height))?;

        Ok(())
    }
}
