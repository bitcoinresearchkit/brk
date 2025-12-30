mod address;
mod block;
mod time;
mod transaction;

use std::{ops::Deref, path::Path};

use brk_error::Result;
use brk_indexer::Indexer;
use brk_traversable::Traversable;
use brk_types::{
    DateIndex, DecadeIndex, DifficultyEpoch, HalvingEpoch, Height, MonthIndex, QuarterIndex,
    SemesterIndex, Version, WeekIndex, YearIndex,
};
use vecdb::{Database, Exit, PAGE_SIZE, TypedVecIterator};

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
        starting_indexes: brk_indexer::Indexes,
        exit: &Exit,
    ) -> Result<Indexes> {
        let indexes = self.compute_(indexer, starting_indexes, exit)?;
        let _lock = exit.lock();
        self.db.compact()?;
        Ok(indexes)
    }

    fn compute_(
        &mut self,
        indexer: &Indexer,
        starting_indexes: brk_indexer::Indexes,
        exit: &Exit,
    ) -> Result<Indexes> {
        // Transaction indexes
        self.transaction.compute(indexer, &starting_indexes, exit)?;

        // Block indexes (height, dateindex, difficultyepoch, halvingepoch)
        let (starting_dateindex, starting_difficultyepoch, starting_halvingepoch) =
            self.block.compute(indexer, &starting_indexes, exit)?;

        // Time indexes (depends on block.height_to_dateindex)
        let time_indexes = self
            .time
            .compute(indexer, &starting_indexes, starting_dateindex, &self.block, exit)?;

        Ok(Indexes {
            indexes: starting_indexes,
            dateindex: time_indexes.dateindex,
            weekindex: time_indexes.weekindex,
            monthindex: time_indexes.monthindex,
            quarterindex: time_indexes.quarterindex,
            semesterindex: time_indexes.semesterindex,
            yearindex: time_indexes.yearindex,
            decadeindex: time_indexes.decadeindex,
            difficultyepoch: starting_difficultyepoch,
            halvingepoch: starting_halvingepoch,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Indexes {
    indexes: brk_indexer::Indexes,
    pub dateindex: DateIndex,
    pub weekindex: WeekIndex,
    pub monthindex: MonthIndex,
    pub quarterindex: QuarterIndex,
    pub semesterindex: SemesterIndex,
    pub yearindex: YearIndex,
    pub decadeindex: DecadeIndex,
    pub difficultyepoch: DifficultyEpoch,
    pub halvingepoch: HalvingEpoch,
}

impl Indexes {
    pub fn update_from_height(&mut self, height: Height, indexes: &Vecs) {
        self.indexes.height = height;
        self.dateindex = DateIndex::try_from(
            indexes
                .block
                .height_to_date_fixed
                .into_iter()
                .get_unwrap(height),
        )
        .unwrap();
        self.weekindex = WeekIndex::from(self.dateindex);
        self.monthindex = MonthIndex::from(self.dateindex);
        self.quarterindex = QuarterIndex::from(self.monthindex);
        self.semesterindex = SemesterIndex::from(self.monthindex);
        self.yearindex = YearIndex::from(self.monthindex);
        self.decadeindex = DecadeIndex::from(self.dateindex);
        self.difficultyepoch = DifficultyEpoch::from(self.height);
        self.halvingepoch = HalvingEpoch::from(self.height);
    }
}

impl Deref for Indexes {
    type Target = brk_indexer::Indexes;
    fn deref(&self) -> &Self::Target {
        &self.indexes
    }
}
