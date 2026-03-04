use std::{fs, path::Path};

use brk_error::Result;
use brk_indexer::Indexer;
use brk_reader::{Reader, XOR_LEN, XORBytes};
use brk_traversable::Traversable;
use brk_types::{BlkPosition, Height, Indexes, TxIndex, Version};
use tracing::info;
use vecdb::{
    AnyStoredVec, AnyVec, Database, Exit, ImportableVec, PcoVec, ReadableVec, Rw, StorageMode,
    VecIndex, WritableVec,
};

use crate::internal::{finalize_db, open_db};

pub const DB_NAME: &str = "positions";

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    db: Database,

    pub block_position: M::Stored<PcoVec<Height, BlkPosition>>,
    pub tx_position: M::Stored<PcoVec<TxIndex, BlkPosition>>,
}

impl Vecs {
    pub(crate) fn forced_import(parent_path: &Path, parent_version: Version) -> Result<Self> {
        let db = open_db(parent_path, DB_NAME, 1_000_000)?;
        let version = parent_version;

        let this = Self {
            block_position: PcoVec::forced_import(&db, "position", version + Version::TWO)?,
            tx_position: PcoVec::forced_import(&db, "position", version + Version::TWO)?,
            db,
        };
        finalize_db(&this.db, &this)?;
        Ok(this)
    }

    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        starting_indexes: &Indexes,
        reader: &Reader,
        exit: &Exit,
    ) -> Result<()> {
        self.compute_(indexer, starting_indexes, reader, exit)?;
        let _lock = exit.lock();
        self.db.compact()?;
        Ok(())
    }

    fn check_xor_bytes(&mut self, reader: &Reader) -> Result<()> {
        let xor_path = self.db.path().join("xor.dat");
        let current = reader.xor_bytes();
        let cached = fs::read(&xor_path)
            .ok()
            .and_then(|b| <[u8; XOR_LEN]>::try_from(b).ok())
            .map(XORBytes::from);

        match cached {
            Some(c) if c == current => return Ok(()),
            Some(_) => {
                info!("XOR bytes changed, resetting positions...");
                self.block_position.reset()?;
                self.tx_position.reset()?;
            }
            None => {}
        }

        fs::write(&xor_path, *current)?;

        Ok(())
    }

    fn compute_(
        &mut self,
        indexer: &Indexer,
        starting_indexes: &Indexes,
        parser: &Reader,
        exit: &Exit,
    ) -> Result<()> {
        self.check_xor_bytes(parser)?;

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
