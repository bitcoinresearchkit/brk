use std::path::Path;

use brk_error::Result;
use brk_indexer::Indexer;
use brk_traversable::Traversable;
use brk_types::{Height, TxInIndex, TxOutIndex, Version};
use log::info;
use vecdb::{
    AnyStoredVec, AnyVec, BytesVec, Database, Exit, GenericStoredVec, ImportableVec, PAGE_SIZE,
    Stamp, TypedVecIterator, VecIndex,
};

use super::{Indexes, txins};

pub const DB_NAME: &str = "txouts";
const HEIGHT_BATCH: u32 = 10_000;

#[derive(Clone, Traversable)]
pub struct Vecs {
    db: Database,
    pub txoutindex_to_txinindex: BytesVec<TxOutIndex, TxInIndex>,
}

impl Vecs {
    pub fn forced_import(parent_path: &Path, parent_version: Version) -> Result<Self> {
        let db = Database::open(&parent_path.join(DB_NAME))?;
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
        let _lock = exit.lock();
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
        let target_height = indexer.vecs.block.height_to_blockhash.len();
        if target_height == 0 {
            return Ok(());
        }
        let target_height = Height::from(target_height - 1);

        // Find min_height from current vec length
        let current_txoutindex = self.txoutindex_to_txinindex.len();
        let min_txoutindex = current_txoutindex.min(starting_indexes.txoutindex.to_usize());

        let starting_stamp = Stamp::from(starting_indexes.height);
        let _ = self.txoutindex_to_txinindex.rollback_before(starting_stamp);

        self.txoutindex_to_txinindex
            .truncate_if_needed(TxOutIndex::from(min_txoutindex))?;

        let mut height_to_first_txoutindex =
            indexer.vecs.txout.height_to_first_txoutindex.iter()?;
        let mut height_to_first_txinindex = indexer.vecs.txin.height_to_first_txinindex.iter()?;
        let mut txinindex_to_txoutindex = txins.txinindex_to_txoutindex.iter()?;

        // Find starting height from min_txoutindex
        let mut min_height = Height::ZERO;
        for h in 0..=target_height.to_usize() {
            let txoutindex = height_to_first_txoutindex.get_unwrap(Height::from(h));
            if txoutindex.to_usize() > min_txoutindex {
                break;
            }
            min_height = Height::from(h);
        }

        // Validate: computed height must not exceed starting height
        assert!(
            min_height <= starting_indexes.height,
            "txouts min_height ({}) exceeds starting_indexes.height ({})",
            min_height,
            starting_indexes.height
        );

        let mut pairs: Vec<(TxOutIndex, TxInIndex)> = Vec::new();

        let mut batch_start_height = min_height;
        while batch_start_height <= target_height {
            let batch_end_height = (batch_start_height + HEIGHT_BATCH).min(target_height);

            // Fill txoutindex up to batch_end_height + 1
            let batch_txoutindex = if batch_end_height >= target_height {
                indexer.vecs.txout.txoutindex_to_value.len()
            } else {
                height_to_first_txoutindex
                    .get_unwrap(batch_end_height + 1_u32)
                    .to_usize()
            };
            self.txoutindex_to_txinindex
                .fill_to(batch_txoutindex, TxInIndex::UNSPENT)?;

            // Get txin range for this height batch
            let txin_start = height_to_first_txinindex
                .get_unwrap(batch_start_height)
                .to_usize();
            let txin_end = if batch_end_height >= target_height {
                txins.txinindex_to_txoutindex.len()
            } else {
                height_to_first_txinindex
                    .get_unwrap(batch_end_height + 1_u32)
                    .to_usize()
            };

            // Collect and process txins
            pairs.clear();
            for i in txin_start..txin_end {
                let txinindex = TxInIndex::from(i);
                let txoutindex = txinindex_to_txoutindex.get_unwrap(txinindex);

                if txoutindex.is_coinbase() {
                    continue;
                }

                pairs.push((txoutindex, txinindex));
            }

            pairs.sort_unstable_by_key(|(txoutindex, _)| *txoutindex);

            for &(txoutindex, txinindex) in &pairs {
                self.txoutindex_to_txinindex.update(txoutindex, txinindex)?;
            }

            if batch_end_height < target_height {
                let _lock = exit.lock();
                self.txoutindex_to_txinindex.write()?;
                info!(
                    "TxOuts: {:.2}%",
                    batch_end_height.to_usize() as f64 / target_height.to_usize() as f64 * 100.0
                );
                self.db.flush()?;
            }

            batch_start_height = batch_end_height + 1_u32;
        }

        let _lock = exit.lock();
        self.txoutindex_to_txinindex
            .stamped_write_with_changes(Stamp::from(target_height))?;
        self.db.flush()?;

        Ok(())
    }
}
