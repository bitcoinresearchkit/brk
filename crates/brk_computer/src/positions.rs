use std::path::Path;

use brk_error::Result;
use brk_indexer::Indexer;
use brk_reader::Reader;
use brk_traversable::Traversable;
use brk_types::{BlkPosition, Height, TxIndex, Version};
use vecdb::{
    AnyStoredVec, AnyVec, Database, Exit, WritableVec, ImportableVec, PAGE_SIZE, PcoVec,
    ReadableVec, Rw, StorageMode, VecIndex,
};

use super::ComputeIndexes;

pub const DB_NAME: &str = "positions";

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    db: Database,

    pub block_position: M::Stored<PcoVec<Height, BlkPosition>>,
    pub tx_position: M::Stored<PcoVec<TxIndex, BlkPosition>>,
}

impl Vecs {
    pub(crate) fn forced_import(parent_path: &Path, parent_version: Version) -> Result<Self> {
        let db = Database::open(&parent_path.join(DB_NAME))?;
        db.set_min_len(PAGE_SIZE * 1_000_000)?;

        let version = parent_version;

        let this = Self {
            block_position: PcoVec::forced_import(&db, "position", version + Version::TWO)?,
            tx_position: PcoVec::forced_import(&db, "position", version + Version::TWO)?,

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

    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        starting_indexes: &ComputeIndexes,
        reader: &Reader,
        exit: &Exit,
    ) -> Result<()> {
        self.compute_(indexer, starting_indexes, reader, exit)?;
        let _lock = exit.lock();
        self.db.compact()?;
        Ok(())
    }

    fn compute_(
        &mut self,
        indexer: &Indexer,
        starting_indexes: &ComputeIndexes,
        parser: &Reader,
        exit: &Exit,
    ) -> Result<()> {
        // Validate computed versions against dependencies
        let dep_version = indexer.vecs.transactions.first_txindex.version()
            + indexer.vecs.transactions.height.version();
        self.block_position
            .validate_computed_version_or_reset(dep_version)?;
        self.tx_position
            .validate_computed_version_or_reset(dep_version)?;

        let min_txindex = TxIndex::from(self.tx_position.len()).min(starting_indexes.txindex);

        let Some(min_height) = indexer
            .vecs
            .transactions
            .height
            .collect_one(min_txindex)
            .map(|h: Height| h.min(starting_indexes.height))
        else {
            return Ok(());
        };

        // Cursor avoids per-height PcoVec page decompression.
        // Heights are sequential, so the cursor only advances forward.
        let mut first_txindex_cursor = indexer.vecs.transactions.first_txindex.cursor();
        first_txindex_cursor.advance(min_height.to_usize());

        parser
            .read(
                Some(min_height),
                Some((indexer.vecs.transactions.first_txindex.len() - 1).into()),
            )
            .iter()
            .try_for_each(|block| -> Result<()> {
                let height = block.height();

                self.block_position
                    .truncate_push(height, block.metadata().position())?;

                let txindex = first_txindex_cursor.next().unwrap();

                block.tx_metadata().iter().enumerate().try_for_each(
                    |(index, metadata)| -> Result<()> {
                        let txindex = txindex + index;
                        self.tx_position
                            .truncate_push(txindex, metadata.position())?;
                        Ok(())
                    },
                )?;

                if *height % 1_000 == 0 {
                    let _lock = exit.lock();
                    self.block_position.flush()?;
                    self.tx_position.flush()?;
                }

                Ok(())
            })?;

        let _lock = exit.lock();
        self.block_position.flush()?;
        self.tx_position.flush()?;

        Ok(())
    }
}
