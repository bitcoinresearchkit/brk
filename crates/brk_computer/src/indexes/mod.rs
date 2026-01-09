mod address;
mod dateindex;
mod decadeindex;
mod difficultyepoch;
mod halvingepoch;
mod height;
mod monthindex;
mod quarterindex;
mod semesterindex;
mod txindex;
mod txinindex;
mod txoutindex;
mod weekindex;
mod yearindex;

use std::path::Path;

use brk_error::Result;
use brk_indexer::Indexer;
use brk_traversable::Traversable;
use brk_types::{DateIndex, Indexes, MonthIndex, Version, WeekIndex};
use vecdb::{Database, Exit, TypedVecIterator, PAGE_SIZE};

use crate::blocks;

pub use address::Vecs as AddressVecs;
pub use brk_types::ComputeIndexes;
pub use dateindex::Vecs as DateIndexVecs;
pub use decadeindex::Vecs as DecadeIndexVecs;
pub use difficultyepoch::Vecs as DifficultyEpochVecs;
pub use halvingepoch::Vecs as HalvingEpochVecs;
pub use height::Vecs as HeightVecs;
pub use monthindex::Vecs as MonthIndexVecs;
pub use quarterindex::Vecs as QuarterIndexVecs;
pub use semesterindex::Vecs as SemesterIndexVecs;
pub use txindex::Vecs as TxIndexVecs;
pub use txinindex::Vecs as TxInIndexVecs;
pub use txoutindex::Vecs as TxOutIndexVecs;
pub use weekindex::Vecs as WeekIndexVecs;
pub use yearindex::Vecs as YearIndexVecs;

const VERSION: Version = Version::ZERO;
pub const DB_NAME: &str = "indexes";

#[derive(Clone, Traversable)]
pub struct Vecs {
    db: Database,
    pub address: AddressVecs,
    pub height: HeightVecs,
    pub difficultyepoch: DifficultyEpochVecs,
    pub halvingepoch: HalvingEpochVecs,
    pub dateindex: DateIndexVecs,
    pub weekindex: WeekIndexVecs,
    pub monthindex: MonthIndexVecs,
    pub quarterindex: QuarterIndexVecs,
    pub semesterindex: SemesterIndexVecs,
    pub yearindex: YearIndexVecs,
    pub decadeindex: DecadeIndexVecs,
    pub txindex: TxIndexVecs,
    pub txinindex: TxInIndexVecs,
    pub txoutindex: TxOutIndexVecs,
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
            height: HeightVecs::forced_import(&db, version)?,
            difficultyepoch: DifficultyEpochVecs::forced_import(&db, version)?,
            halvingepoch: HalvingEpochVecs::forced_import(&db, version)?,
            dateindex: DateIndexVecs::forced_import(&db, version)?,
            weekindex: WeekIndexVecs::forced_import(&db, version)?,
            monthindex: MonthIndexVecs::forced_import(&db, version)?,
            quarterindex: QuarterIndexVecs::forced_import(&db, version)?,
            semesterindex: SemesterIndexVecs::forced_import(&db, version)?,
            yearindex: YearIndexVecs::forced_import(&db, version)?,
            decadeindex: DecadeIndexVecs::forced_import(&db, version)?,
            txindex: TxIndexVecs::forced_import(&db, version, indexer)?,
            txinindex: TxInIndexVecs::forced_import(version, indexer),
            txoutindex: TxOutIndexVecs::forced_import(version, indexer),
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
        blocks_time: &blocks::time::Vecs,
        starting_indexes: Indexes,
        exit: &Exit,
    ) -> Result<ComputeIndexes> {
        let indexes = self.compute_(indexer, blocks_time, starting_indexes, exit)?;
        let _lock = exit.lock();
        self.db.compact()?;
        Ok(indexes)
    }

    fn compute_(
        &mut self,
        indexer: &Indexer,
        blocks_time: &blocks::time::Vecs,
        starting_indexes: Indexes,
        exit: &Exit,
    ) -> Result<ComputeIndexes> {
        // Transaction indexes - compute input/output counts
        self.txindex.input_count.compute_count_from_indexes(
            starting_indexes.txindex,
            &indexer.vecs.transactions.first_txinindex,
            &indexer.vecs.inputs.outpoint,
            exit,
        )?;
        self.txindex.output_count.compute_count_from_indexes(
            starting_indexes.txindex,
            &indexer.vecs.transactions.first_txoutindex,
            &indexer.vecs.outputs.value,
            exit,
        )?;

        // Height indexes
        self.height.txindex_count.compute_count_from_indexes(
            starting_indexes.height,
            &indexer.vecs.transactions.first_txindex,
            &indexer.vecs.transactions.txid,
            exit,
        )?;

        self.height.identity.compute_from_index(
            starting_indexes.height,
            &indexer.vecs.blocks.weight,
            exit,
        )?;

        let decremented_starting_height = starting_indexes.height.decremented().unwrap_or_default();

        // DateIndex (uses blocks_time.date_fixed computed in blocks::time::compute_early)
        let starting_dateindex = self
            .height
            .dateindex
            .into_iter()
            .get(decremented_starting_height)
            .unwrap_or_default();

        self.height.dateindex.compute_transform(
            starting_indexes.height,
            &blocks_time.date_fixed,
            |(h, d, ..)| (h, DateIndex::try_from(d).unwrap()),
            exit,
        )?;

        let starting_dateindex = if let Some(dateindex) = self
            .height
            .dateindex
            .into_iter()
            .get(decremented_starting_height)
        {
            starting_dateindex.min(dateindex)
        } else {
            starting_dateindex
        };

        // Difficulty epoch
        let starting_difficultyepoch = self
            .height
            .difficultyepoch
            .into_iter()
            .get(decremented_starting_height)
            .unwrap_or_default();

        self.height.difficultyepoch.compute_from_index(
            starting_indexes.height,
            &indexer.vecs.blocks.weight,
            exit,
        )?;

        self.difficultyepoch.first_height.compute_coarser(
            starting_indexes.height,
            &self.height.difficultyepoch,
            exit,
        )?;

        self.difficultyepoch.identity.compute_from_index(
            starting_difficultyepoch,
            &self.difficultyepoch.first_height,
            exit,
        )?;

        self.difficultyepoch.height_count.compute_count_from_indexes(
            starting_difficultyepoch,
            &self.difficultyepoch.first_height,
            &blocks_time.date,
            exit,
        )?;

        // Halving epoch
        let starting_halvingepoch = self
            .height
            .halvingepoch
            .into_iter()
            .get(decremented_starting_height)
            .unwrap_or_default();

        self.height.halvingepoch.compute_from_index(
            starting_indexes.height,
            &indexer.vecs.blocks.weight,
            exit,
        )?;

        self.halvingepoch.first_height.compute_coarser(
            starting_indexes.height,
            &self.height.halvingepoch,
            exit,
        )?;

        self.halvingepoch.identity.compute_from_index(
            starting_halvingepoch,
            &self.halvingepoch.first_height,
            exit,
        )?;

        // Time indexes (depends on height.dateindex)
        self.dateindex.first_height.compute_coarser(
            starting_indexes.height,
            &self.height.dateindex,
            exit,
        )?;

        self.dateindex.identity.compute_from_index(
            starting_dateindex,
            &self.dateindex.first_height,
            exit,
        )?;

        self.dateindex.date.compute_from_index(
            starting_dateindex,
            &self.dateindex.first_height,
            exit,
        )?;

        self.dateindex.height_count.compute_count_from_indexes(
            starting_dateindex,
            &self.dateindex.first_height,
            &indexer.vecs.blocks.weight,
            exit,
        )?;

        // Week
        let starting_weekindex = self
            .dateindex
            .weekindex
            .into_iter()
            .get(starting_dateindex)
            .unwrap_or_default();

        self.dateindex.weekindex.compute_range(
            starting_dateindex,
            &self.dateindex.identity,
            |i| (i, WeekIndex::from(i)),
            exit,
        )?;

        self.weekindex.first_dateindex.compute_coarser(
            starting_dateindex,
            &self.dateindex.weekindex,
            exit,
        )?;

        self.weekindex.identity.compute_from_index(
            starting_weekindex,
            &self.weekindex.first_dateindex,
            exit,
        )?;

        self.weekindex.dateindex_count.compute_count_from_indexes(
            starting_weekindex,
            &self.weekindex.first_dateindex,
            &self.dateindex.date,
            exit,
        )?;

        // Month
        let starting_monthindex = self
            .dateindex
            .monthindex
            .into_iter()
            .get(starting_dateindex)
            .unwrap_or_default();

        self.dateindex.monthindex.compute_range(
            starting_dateindex,
            &self.dateindex.identity,
            |i| (i, MonthIndex::from(i)),
            exit,
        )?;

        self.monthindex.first_dateindex.compute_coarser(
            starting_dateindex,
            &self.dateindex.monthindex,
            exit,
        )?;

        self.monthindex.identity.compute_from_index(
            starting_monthindex,
            &self.monthindex.first_dateindex,
            exit,
        )?;

        self.monthindex.dateindex_count.compute_count_from_indexes(
            starting_monthindex,
            &self.monthindex.first_dateindex,
            &self.dateindex.date,
            exit,
        )?;

        // Quarter
        let starting_quarterindex = self
            .monthindex
            .quarterindex
            .into_iter()
            .get(starting_monthindex)
            .unwrap_or_default();

        self.monthindex.quarterindex.compute_from_index(
            starting_monthindex,
            &self.monthindex.first_dateindex,
            exit,
        )?;

        self.quarterindex.first_monthindex.compute_coarser(
            starting_monthindex,
            &self.monthindex.quarterindex,
            exit,
        )?;

        self.quarterindex.identity.compute_from_index(
            starting_quarterindex,
            &self.quarterindex.first_monthindex,
            exit,
        )?;

        self.quarterindex.monthindex_count.compute_count_from_indexes(
            starting_quarterindex,
            &self.quarterindex.first_monthindex,
            &self.monthindex.identity,
            exit,
        )?;

        // Semester
        let starting_semesterindex = self
            .monthindex
            .semesterindex
            .into_iter()
            .get(starting_monthindex)
            .unwrap_or_default();

        self.monthindex.semesterindex.compute_from_index(
            starting_monthindex,
            &self.monthindex.first_dateindex,
            exit,
        )?;

        self.semesterindex.first_monthindex.compute_coarser(
            starting_monthindex,
            &self.monthindex.semesterindex,
            exit,
        )?;

        self.semesterindex.identity.compute_from_index(
            starting_semesterindex,
            &self.semesterindex.first_monthindex,
            exit,
        )?;

        self.semesterindex.monthindex_count.compute_count_from_indexes(
            starting_semesterindex,
            &self.semesterindex.first_monthindex,
            &self.monthindex.identity,
            exit,
        )?;

        // Year
        let starting_yearindex = self
            .monthindex
            .yearindex
            .into_iter()
            .get(starting_monthindex)
            .unwrap_or_default();

        self.monthindex.yearindex.compute_from_index(
            starting_monthindex,
            &self.monthindex.first_dateindex,
            exit,
        )?;

        self.yearindex.first_monthindex.compute_coarser(
            starting_monthindex,
            &self.monthindex.yearindex,
            exit,
        )?;

        self.yearindex.identity.compute_from_index(
            starting_yearindex,
            &self.yearindex.first_monthindex,
            exit,
        )?;

        self.yearindex.monthindex_count.compute_count_from_indexes(
            starting_yearindex,
            &self.yearindex.first_monthindex,
            &self.monthindex.identity,
            exit,
        )?;

        // Decade
        let starting_decadeindex = self
            .yearindex
            .decadeindex
            .into_iter()
            .get(starting_yearindex)
            .unwrap_or_default();

        self.yearindex.decadeindex.compute_from_index(
            starting_yearindex,
            &self.yearindex.first_monthindex,
            exit,
        )?;

        self.decadeindex.first_yearindex.compute_coarser(
            starting_yearindex,
            &self.yearindex.decadeindex,
            exit,
        )?;

        self.decadeindex.identity.compute_from_index(
            starting_decadeindex,
            &self.decadeindex.first_yearindex,
            exit,
        )?;

        self.decadeindex.yearindex_count.compute_count_from_indexes(
            starting_decadeindex,
            &self.decadeindex.first_yearindex,
            &self.yearindex.identity,
            exit,
        )?;

        Ok(ComputeIndexes::new(
            starting_indexes,
            starting_dateindex,
            starting_weekindex,
            starting_monthindex,
            starting_quarterindex,
            starting_semesterindex,
            starting_yearindex,
            starting_decadeindex,
            starting_difficultyepoch,
            starting_halvingepoch,
        ))
    }
}
