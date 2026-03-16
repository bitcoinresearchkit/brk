use std::{fs, path::Path};

use brk_error::Result;
use brk_indexer::Indexer;
use brk_reader::{Reader, XOR_LEN, XORBytes};
use brk_traversable::Traversable;
use brk_types::{BlkPosition, Height, Indexes, TxIndex, Version};
use tracing::info;
use vecdb::{
    AnyStoredVec, AnyVec, Database, Exit, ImportableVec, PcoVec, ReadableVec, Rw, StorageMode,
    WritableVec,
};

use crate::internal::db_utils::{finalize_db, open_db};

pub const DB_NAME: &str = "positions";

#[derive(Traversable)]
#[traversable(hidden)]
pub struct Vecs<M: StorageMode = Rw> {
    db: Database,

    pub block: M::Stored<PcoVec<Height, BlkPosition>>,
    pub tx: M::Stored<PcoVec<TxIndex, BlkPosition>>,
}

impl Vecs {
    pub(crate) fn forced_import(parent_path: &Path, parent_version: Version) -> Result<Self> {
        let db = open_db(parent_path, DB_NAME, 1_000_000)?;
        let version = parent_version;

        let this = Self {
            block: PcoVec::forced_import(&db, "position", version + Version::TWO)?,
            tx: PcoVec::forced_import(&db, "position", version + Version::TWO)?,
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
                self.block.reset()?;
                self.tx.reset()?;
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
        let dep_version = indexer.vecs.transactions.first_tx_index.version()
            + indexer.vecs.transactions.height.version();
        self.block
            .validate_computed_version_or_reset(dep_version)?;
        self.tx
            .validate_computed_version_or_reset(dep_version)?;

        let min_tx_index = TxIndex::from(self.tx.len()).min(starting_indexes.tx_index);

        let Some(min_height) = indexer
            .vecs
            .transactions
            .height
            .collect_one(min_tx_index)
            .map(|h: Height| h.min(starting_indexes.height))
        else {
            return Ok(());
        };

        let first_tx_at_min_height = indexer
            .vecs
            .transactions
            .first_tx_index
            .collect_one(min_height)
            .unwrap();

        self.block.truncate_if_needed(min_height)?;
        self.tx.truncate_if_needed(first_tx_at_min_height)?;

        parser
            .read(
                Some(min_height),
                Some((indexer.vecs.transactions.first_tx_index.len() - 1).into()),
            )
            .iter()
            .try_for_each(|block| -> Result<()> {
                self.block.push(block.metadata().position());

                block.tx_metadata().iter().for_each(|metadata| {
                    self.tx.push(metadata.position());
                });

                if *block.height() % 1_000 == 0 {
                    let _lock = exit.lock();
                    self.block.flush()?;
                    self.tx.flush()?;
                }

                Ok(())
            })?;

        let _lock = exit.lock();
        self.block.flush()?;
        self.tx.flush()?;

        Ok(())
    }
}
