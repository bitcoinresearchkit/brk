use std::path::Path;

use brk_error::Result;
use brk_indexer::Indexer;
use brk_reader::Reader;
use brk_traversable::Traversable;
use brk_types::{BlkPosition, Height, TxIndex, Version};
use vecdb::{
    AnyStoredVec, AnyVec, CompressedVec, Database, Exit, GenericStoredVec, PAGE_SIZE,
    TypedVecIterator,
};

use super::Indexes;

#[derive(Clone, Traversable)]
pub struct Vecs {
    db: Database,

    pub height_to_position: CompressedVec<Height, BlkPosition>,
    pub txindex_to_position: CompressedVec<TxIndex, BlkPosition>,
}

impl Vecs {
    pub fn forced_import(parent_path: &Path, parent_version: Version) -> Result<Self> {
        let db = Database::open(&parent_path.join("blks"))?;
        db.set_min_len(PAGE_SIZE * 1_000_000)?;

        let version = parent_version + Version::ZERO;

        let this = Self {
            height_to_position: CompressedVec::forced_import(
                &db,
                "position",
                version + Version::TWO,
            )?,
            txindex_to_position: CompressedVec::forced_import(
                &db,
                "position",
                version + Version::TWO,
            )?,

            db,
        };

        this.db.retain_regions(
            this.iter_any_writable()
                .flat_map(|v| v.region_names())
                .collect(),
        )?;

        Ok(this)
    }

    pub fn compute(
        &mut self,
        indexer: &Indexer,
        starting_indexes: &Indexes,
        reader: &Reader,
        exit: &Exit,
    ) -> Result<()> {
        self.compute_(indexer, starting_indexes, reader, exit)?;
        self.db.compact()?;
        Ok(())
    }

    fn compute_(
        &mut self,
        indexer: &Indexer,
        starting_indexes: &Indexes,
        parser: &Reader,
        exit: &Exit,
    ) -> Result<()> {
        let min_txindex =
            TxIndex::from(self.txindex_to_position.len()).min(starting_indexes.txindex);

        let Some(min_height) = indexer
            .vecs
            .txindex_to_height
            .iter()?
            .get(min_txindex)
            .map(|h| h.min(starting_indexes.height))
        else {
            return Ok(());
        };

        let mut height_to_first_txindex_iter = indexer.vecs.height_to_first_txindex.iter()?;

        parser
            .read(
                Some(min_height),
                Some((indexer.vecs.height_to_first_txindex.len() - 1).into()),
            )
            .iter()
            .try_for_each(|block| -> Result<()> {
                let height = block.height();

                self.height_to_position
                    .forced_push(height, block.metadata().position(), exit)?;

                let txindex = height_to_first_txindex_iter.get_unwrap(height);

                block.tx_metadata().iter().enumerate().try_for_each(
                    |(index, metadata)| -> Result<()> {
                        let txindex = txindex + index;
                        self.txindex_to_position
                            .forced_push(txindex, metadata.position(), exit)?;
                        Ok(())
                    },
                )?;

                if *height % 1_000 == 0 {
                    let _lock = exit.lock();
                    self.height_to_position.flush()?;
                    self.txindex_to_position.flush()?;
                }
                Ok(())
            })?;

        let _lock = exit.lock();
        self.height_to_position.flush()?;
        self.txindex_to_position.flush()?;

        Ok(())
    }
}
