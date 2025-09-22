use std::path::Path;

use brk_error::Result;
use brk_indexer::Indexer;
use brk_parser::Parser;
use brk_structs::{BlkPosition, Height, TxIndex, Version};
use vecdb::{
    AnyCollectableVec, AnyIterableVec, AnyStoredVec, AnyVec, CompressedVec, Database, Exit,
    GenericStoredVec, PAGE_SIZE, VecIterator,
};

use super::{Indexes, indexes};

#[derive(Clone)]
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
            this.iter_any_collectable()
                .flat_map(|v| v.region_names())
                .collect(),
        )?;

        Ok(this)
    }

    pub fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &Indexes,
        parser: &Parser,
        exit: &Exit,
    ) -> Result<()> {
        self.compute_(indexer, indexes, starting_indexes, parser, exit)?;
        self.db.flush_then_punch()?;
        Ok(())
    }

    fn compute_(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &Indexes,
        parser: &Parser,
        exit: &Exit,
    ) -> Result<()> {
        let min_txindex =
            TxIndex::from(self.txindex_to_position.len()).min(starting_indexes.txindex);

        let Some(min_height) = indexes
            .txindex_to_height
            .iter()
            .get_inner(min_txindex)
            .map(|h| h.min(starting_indexes.height))
        else {
            return Ok(());
        };

        let mut height_to_first_txindex_iter = indexer.vecs.height_to_first_txindex.iter();

        parser
            .parse(
                Some(min_height),
                Some((indexer.vecs.height_to_first_txindex.len() - 1).into()),
            )
            .iter()
            .try_for_each(|block| -> Result<()> {
                let height = block.height();

                self.height_to_position.forced_push_at(
                    height,
                    block.metadata().position(),
                    exit,
                )?;

                let txindex = height_to_first_txindex_iter.unwrap_get_inner(height);

                block.tx_metadata().iter().enumerate().try_for_each(
                    |(index, metadata)| -> Result<()> {
                        let txindex = txindex + index;
                        self.txindex_to_position.forced_push_at(
                            txindex,
                            metadata.position(),
                            exit,
                        )?;
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

    pub fn iter_any_collectable(&self) -> impl Iterator<Item = &dyn AnyCollectableVec> {
        Box::new(
            [
                &self.height_to_position as &dyn AnyCollectableVec,
                &self.txindex_to_position,
            ]
            .into_iter(),
        )
    }
}
