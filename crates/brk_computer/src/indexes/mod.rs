mod address;
mod block;
mod time;
mod transaction;

use std::path::Path;

use brk_error::Result;
use brk_indexer::Indexer;
use brk_traversable::Traversable;
use brk_types::{Indexes, Version};

pub use brk_types::ComputeIndexes;
use vecdb::{Database, Exit, PAGE_SIZE};

pub use address::Vecs as AddressVecs;
pub use block::Vecs as BlockVecs;
pub use time::Vecs as TimeVecs;
pub use transaction::Vecs as TransactionVecs;

const VERSION: Version = Version::ZERO;
pub const DB_NAME: &str = "indexes";

#[derive(Clone, Traversable)]
pub struct Vecs {
    db: Database,
    pub address: AddressVecs,
    pub block: BlockVecs,
    pub time: TimeVecs,
    pub transaction: TransactionVecs,
}

impl Vecs {
    pub fn forced_import(
        parent: &Path,
        parent_version: Version,
        indexer: &Indexer,
    ) -> Result<Self> {
        let db = Database::open(&parent.join(DB_NAME))?;
        db.set_min_len(PAGE_SIZE * 10_000_000)?;

        let version = parent_version + VERSION;

        let this = Self {
            address: AddressVecs::forced_import(version, indexer),
            block: BlockVecs::forced_import(&db, version)?,
            time: TimeVecs::forced_import(&db, version)?,
            transaction: TransactionVecs::forced_import(&db, version, indexer)?,
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
        starting_indexes: Indexes,
        exit: &Exit,
    ) -> Result<ComputeIndexes> {
        let indexes = self.compute_(indexer, starting_indexes, exit)?;
        let _lock = exit.lock();
        self.db.compact()?;
        Ok(indexes)
    }

    fn compute_(
        &mut self,
        indexer: &Indexer,
        starting_indexes: Indexes,
        exit: &Exit,
    ) -> Result<ComputeIndexes> {
        // Transaction indexes
        self.transaction.compute(indexer, &starting_indexes, exit)?;

        // Block indexes (height, dateindex, difficultyepoch, halvingepoch)
        let (starting_dateindex, starting_difficultyepoch, starting_halvingepoch) =
            self.block.compute(indexer, &starting_indexes, exit)?;

        // Time indexes (depends on block.height_to_dateindex)
        let time_indexes = self
            .time
            .compute(indexer, &starting_indexes, starting_dateindex, &self.block, exit)?;

        Ok(ComputeIndexes::new(
            starting_indexes,
            time_indexes.dateindex,
            time_indexes.weekindex,
            time_indexes.monthindex,
            time_indexes.quarterindex,
            time_indexes.semesterindex,
            time_indexes.yearindex,
            time_indexes.decadeindex,
            starting_difficultyepoch,
            starting_halvingepoch,
        ))
    }
}
