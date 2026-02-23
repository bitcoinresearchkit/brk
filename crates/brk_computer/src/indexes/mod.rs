mod address;
mod day1;
mod day3;
mod difficultyepoch;
mod halvingepoch;
mod height;
mod hour1;
mod hour12;
mod hour4;
mod minute1;
mod minute10;
mod minute30;
mod minute5;
mod month1;
mod month3;
mod month6;
mod txindex;
mod txinindex;
mod txoutindex;
mod week1;
mod year1;
mod year10;

use std::path::Path;

use brk_error::Result;
use brk_indexer::Indexer;
use brk_traversable::Traversable;
use brk_types::{
    Date, Day1, Day3, Hour1, Hour4, Hour12, Indexes, Minute1, Minute5, Minute10, Minute30, Month1,
    Month3, Month6, Version, Week1, Year1, Year10,
};
use vecdb::{Database, Exit, PAGE_SIZE, ReadableVec, Rw, StorageMode};

use crate::blocks;

pub use address::Vecs as AddressVecs;
pub use brk_types::ComputeIndexes;
pub use day1::Vecs as Day1Vecs;
pub use day3::Vecs as Day3Vecs;
pub use difficultyepoch::Vecs as DifficultyEpochVecs;
pub use halvingepoch::Vecs as HalvingEpochVecs;
pub use height::Vecs as HeightVecs;
pub use hour1::Vecs as Hour1Vecs;
pub use hour4::Vecs as Hour4Vecs;
pub use hour12::Vecs as Hour12Vecs;
pub use minute1::Vecs as Minute1Vecs;
pub use minute5::Vecs as Minute5Vecs;
pub use minute10::Vecs as Minute10Vecs;
pub use minute30::Vecs as Minute30Vecs;
pub use month1::Vecs as Month1Vecs;
pub use month3::Vecs as Month3Vecs;
pub use month6::Vecs as Month6Vecs;
pub use txindex::Vecs as TxIndexVecs;
pub use txinindex::Vecs as TxInIndexVecs;
pub use txoutindex::Vecs as TxOutIndexVecs;
pub use week1::Vecs as Week1Vecs;
pub use year1::Vecs as Year1Vecs;
pub use year10::Vecs as Year10Vecs;

const VERSION: Version = Version::ZERO;
pub const DB_NAME: &str = "indexes";

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    db: Database,
    pub address: AddressVecs,
    pub height: HeightVecs<M>,
    pub difficultyepoch: DifficultyEpochVecs<M>,
    pub halvingepoch: HalvingEpochVecs<M>,
    pub minute1: Minute1Vecs<M>,
    pub minute5: Minute5Vecs<M>,
    pub minute10: Minute10Vecs<M>,
    pub minute30: Minute30Vecs<M>,
    pub hour1: Hour1Vecs<M>,
    pub hour4: Hour4Vecs<M>,
    pub hour12: Hour12Vecs<M>,
    pub day1: Day1Vecs<M>,
    pub day3: Day3Vecs<M>,
    pub week1: Week1Vecs<M>,
    pub month1: Month1Vecs<M>,
    pub month3: Month3Vecs<M>,
    pub month6: Month6Vecs<M>,
    pub year1: Year1Vecs<M>,
    pub year10: Year10Vecs<M>,
    pub txindex: TxIndexVecs<M>,
    pub txinindex: TxInIndexVecs,
    pub txoutindex: TxOutIndexVecs,
}

impl Vecs {
    pub(crate) fn forced_import(
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
            minute1: Minute1Vecs::forced_import(&db, version)?,
            minute5: Minute5Vecs::forced_import(&db, version)?,
            minute10: Minute10Vecs::forced_import(&db, version)?,
            minute30: Minute30Vecs::forced_import(&db, version)?,
            hour1: Hour1Vecs::forced_import(&db, version)?,
            hour4: Hour4Vecs::forced_import(&db, version)?,
            hour12: Hour12Vecs::forced_import(&db, version)?,
            day1: Day1Vecs::forced_import(&db, version)?,
            day3: Day3Vecs::forced_import(&db, version)?,
            week1: Week1Vecs::forced_import(&db, version)?,
            month1: Month1Vecs::forced_import(&db, version)?,
            month3: Month3Vecs::forced_import(&db, version)?,
            month6: Month6Vecs::forced_import(&db, version)?,
            year1: Year1Vecs::forced_import(&db, version)?,
            year10: Year10Vecs::forced_import(&db, version)?,
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

    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        blocks: &mut blocks::Vecs,
        starting_indexes: Indexes,
        exit: &Exit,
    ) -> Result<ComputeIndexes> {
        blocks
            .time
            .compute(indexer, starting_indexes.height, exit)?;
        let indexes = self.compute_(indexer, &blocks.time, starting_indexes, exit)?;
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

        // --- Timestamp-based height → period mappings ---

        // Minute1
        self.height.minute1.compute_transform(
            starting_indexes.height,
            &blocks_time.timestamp_monotonic,
            |(h, ts, _)| (h, Minute1::from_timestamp(ts)),
            exit,
        )?;

        // Minute5
        self.height.minute5.compute_transform(
            starting_indexes.height,
            &blocks_time.timestamp_monotonic,
            |(h, ts, _)| (h, Minute5::from_timestamp(ts)),
            exit,
        )?;

        // Minute10
        self.height.minute10.compute_transform(
            starting_indexes.height,
            &blocks_time.timestamp_monotonic,
            |(h, ts, _)| (h, Minute10::from_timestamp(ts)),
            exit,
        )?;

        // Minute30
        self.height.minute30.compute_transform(
            starting_indexes.height,
            &blocks_time.timestamp_monotonic,
            |(h, ts, _)| (h, Minute30::from_timestamp(ts)),
            exit,
        )?;

        // Hour1
        self.height.hour1.compute_transform(
            starting_indexes.height,
            &blocks_time.timestamp_monotonic,
            |(h, ts, _)| (h, Hour1::from_timestamp(ts)),
            exit,
        )?;

        // Hour4
        self.height.hour4.compute_transform(
            starting_indexes.height,
            &blocks_time.timestamp_monotonic,
            |(h, ts, _)| (h, Hour4::from_timestamp(ts)),
            exit,
        )?;

        // Hour12
        self.height.hour12.compute_transform(
            starting_indexes.height,
            &blocks_time.timestamp_monotonic,
            |(h, ts, _)| (h, Hour12::from_timestamp(ts)),
            exit,
        )?;

        // Day3
        self.height.day3.compute_transform(
            starting_indexes.height,
            &blocks_time.timestamp_monotonic,
            |(h, ts, _)| (h, Day3::from_timestamp(ts)),
            exit,
        )?;

        // --- Calendar-based height → period mappings ---

        // Day1 (uses blocks_time.date computed in blocks::time::compute_early)
        let starting_day1 = self
            .height
            .day1
            .collect_one(decremented_starting_height)
            .unwrap_or_default();

        self.height.day1.compute_transform(
            starting_indexes.height,
            &blocks_time.date,
            |(h, d, ..)| (h, Day1::try_from(d).unwrap()),
            exit,
        )?;

        let starting_day1 =
            if let Some(day1) = self.height.day1.collect_one(decremented_starting_height) {
                starting_day1.min(day1)
            } else {
                starting_day1
            };

        // Difficulty epoch
        let starting_difficultyepoch = self
            .height
            .difficultyepoch
            .collect_one(decremented_starting_height)
            .unwrap_or_default();

        self.height.difficultyepoch.compute_from_index(
            starting_indexes.height,
            &indexer.vecs.blocks.weight,
            exit,
        )?;

        self.difficultyepoch.first_height.compute_first_per_index(
            starting_indexes.height,
            &self.height.difficultyepoch,
            exit,
        )?;

        self.difficultyepoch.identity.compute_from_index(
            starting_difficultyepoch,
            &self.difficultyepoch.first_height,
            exit,
        )?;

        self.difficultyepoch
            .height_count
            .compute_count_from_indexes(
                starting_difficultyepoch,
                &self.difficultyepoch.first_height,
                &blocks_time.date,
                exit,
            )?;

        // Halving epoch
        let starting_halvingepoch = self
            .height
            .halvingepoch
            .collect_one(decremented_starting_height)
            .unwrap_or_default();

        self.height.halvingepoch.compute_from_index(
            starting_indexes.height,
            &indexer.vecs.blocks.weight,
            exit,
        )?;

        self.halvingepoch.first_height.compute_first_per_index(
            starting_indexes.height,
            &self.height.halvingepoch,
            exit,
        )?;

        self.halvingepoch.identity.compute_from_index(
            starting_halvingepoch,
            &self.halvingepoch.first_height,
            exit,
        )?;

        // Height → period mappings (calendar-based, derived from height.day1)
        self.height.week1.compute_transform(
            starting_indexes.height,
            &self.height.day1,
            |(h, di, _)| (h, Week1::from(di)),
            exit,
        )?;
        self.height.month1.compute_transform(
            starting_indexes.height,
            &self.height.day1,
            |(h, di, _)| (h, Month1::from(di)),
            exit,
        )?;
        self.height.month3.compute_transform(
            starting_indexes.height,
            &self.height.month1,
            |(h, mi, _)| (h, Month3::from(mi)),
            exit,
        )?;
        self.height.month6.compute_transform(
            starting_indexes.height,
            &self.height.month1,
            |(h, mi, _)| (h, Month6::from(mi)),
            exit,
        )?;
        self.height.year1.compute_transform(
            starting_indexes.height,
            &self.height.month1,
            |(h, mi, _)| (h, Year1::from(mi)),
            exit,
        )?;
        self.height.year10.compute_transform(
            starting_indexes.height,
            &self.height.year1,
            |(h, yi, _)| (h, Year10::from(yi)),
            exit,
        )?;

        // --- Starting values from height → period mappings ---

        let starting_minute1 = self
            .height
            .minute1
            .collect_one(decremented_starting_height)
            .unwrap_or_default();
        let starting_minute5 = self
            .height
            .minute5
            .collect_one(decremented_starting_height)
            .unwrap_or_default();
        let starting_minute10 = self
            .height
            .minute10
            .collect_one(decremented_starting_height)
            .unwrap_or_default();
        let starting_minute30 = self
            .height
            .minute30
            .collect_one(decremented_starting_height)
            .unwrap_or_default();
        let starting_hour1 = self
            .height
            .hour1
            .collect_one(decremented_starting_height)
            .unwrap_or_default();
        let starting_hour4 = self
            .height
            .hour4
            .collect_one(decremented_starting_height)
            .unwrap_or_default();
        let starting_hour12 = self
            .height
            .hour12
            .collect_one(decremented_starting_height)
            .unwrap_or_default();
        let starting_day3 = self
            .height
            .day3
            .collect_one(decremented_starting_height)
            .unwrap_or_default();
        let starting_week1 = self
            .height
            .week1
            .collect_one(decremented_starting_height)
            .unwrap_or_default();
        let starting_month1 = self
            .height
            .month1
            .collect_one(decremented_starting_height)
            .unwrap_or_default();
        let starting_month3 = self
            .height
            .month3
            .collect_one(decremented_starting_height)
            .unwrap_or_default();
        let starting_month6 = self
            .height
            .month6
            .collect_one(decremented_starting_height)
            .unwrap_or_default();
        let starting_year1 = self
            .height
            .year1
            .collect_one(decremented_starting_height)
            .unwrap_or_default();
        let starting_year10 = self
            .height
            .year10
            .collect_one(decremented_starting_height)
            .unwrap_or_default();

        // --- Compute period-level vecs (first_height + identity) ---

        // Minute1
        self.minute1.first_height.compute_first_per_index(
            starting_indexes.height,
            &self.height.minute1,
            exit,
        )?;
        self.minute1.identity.compute_from_index(
            starting_minute1,
            &self.minute1.first_height,
            exit,
        )?;

        // Minute5
        self.minute5.first_height.compute_first_per_index(
            starting_indexes.height,
            &self.height.minute5,
            exit,
        )?;
        self.minute5.identity.compute_from_index(
            starting_minute5,
            &self.minute5.first_height,
            exit,
        )?;

        // Minute10
        self.minute10.first_height.compute_first_per_index(
            starting_indexes.height,
            &self.height.minute10,
            exit,
        )?;
        self.minute10.identity.compute_from_index(
            starting_minute10,
            &self.minute10.first_height,
            exit,
        )?;

        // Minute30
        self.minute30.first_height.compute_first_per_index(
            starting_indexes.height,
            &self.height.minute30,
            exit,
        )?;
        self.minute30.identity.compute_from_index(
            starting_minute30,
            &self.minute30.first_height,
            exit,
        )?;

        // Hour1
        self.hour1.first_height.compute_first_per_index(
            starting_indexes.height,
            &self.height.hour1,
            exit,
        )?;
        self.hour1
            .identity
            .compute_from_index(starting_hour1, &self.hour1.first_height, exit)?;

        // Hour4
        self.hour4.first_height.compute_first_per_index(
            starting_indexes.height,
            &self.height.hour4,
            exit,
        )?;
        self.hour4
            .identity
            .compute_from_index(starting_hour4, &self.hour4.first_height, exit)?;

        // Hour12
        self.hour12.first_height.compute_first_per_index(
            starting_indexes.height,
            &self.height.hour12,
            exit,
        )?;
        self.hour12.identity.compute_from_index(
            starting_hour12,
            &self.hour12.first_height,
            exit,
        )?;

        // Day1
        self.day1.first_height.compute_first_per_index(
            starting_indexes.height,
            &self.height.day1,
            exit,
        )?;
        self.day1
            .identity
            .compute_from_index(starting_day1, &self.day1.first_height, exit)?;
        self.day1.date.compute_transform(
            starting_day1,
            &self.day1.identity,
            |(di, ..)| (di, Date::from(di)),
            exit,
        )?;
        self.day1.height_count.compute_count_from_indexes(
            starting_day1,
            &self.day1.first_height,
            &indexer.vecs.blocks.weight,
            exit,
        )?;

        // Day3
        self.day3.first_height.compute_first_per_index(
            starting_indexes.height,
            &self.height.day3,
            exit,
        )?;
        self.day3
            .identity
            .compute_from_index(starting_day3, &self.day3.first_height, exit)?;

        let blocks_time_date = &blocks_time.date;

        // Week
        self.week1.first_height.compute_first_per_index(
            starting_indexes.height,
            &self.height.week1,
            exit,
        )?;
        self.week1
            .identity
            .compute_from_index(starting_week1, &self.week1.first_height, exit)?;
        self.week1.date.compute_transform(
            starting_week1,
            &self.week1.first_height,
            |(wi, first_h, _)| (wi, blocks_time_date.collect_one(first_h).unwrap()),
            exit,
        )?;

        // Month
        self.month1.first_height.compute_first_per_index(
            starting_indexes.height,
            &self.height.month1,
            exit,
        )?;
        self.month1.identity.compute_from_index(
            starting_month1,
            &self.month1.first_height,
            exit,
        )?;
        self.month1.date.compute_transform(
            starting_month1,
            &self.month1.first_height,
            |(mi, first_h, _)| (mi, blocks_time_date.collect_one(first_h).unwrap()),
            exit,
        )?;

        // Quarter
        self.month3.first_height.compute_first_per_index(
            starting_indexes.height,
            &self.height.month3,
            exit,
        )?;
        self.month3.identity.compute_from_index(
            starting_month3,
            &self.month3.first_height,
            exit,
        )?;
        self.month3.date.compute_transform(
            starting_month3,
            &self.month3.first_height,
            |(qi, first_h, _)| (qi, blocks_time_date.collect_one(first_h).unwrap()),
            exit,
        )?;

        // Semester
        self.month6.first_height.compute_first_per_index(
            starting_indexes.height,
            &self.height.month6,
            exit,
        )?;
        self.month6.identity.compute_from_index(
            starting_month6,
            &self.month6.first_height,
            exit,
        )?;
        self.month6.date.compute_transform(
            starting_month6,
            &self.month6.first_height,
            |(si, first_h, _)| (si, blocks_time_date.collect_one(first_h).unwrap()),
            exit,
        )?;

        // Year
        self.year1.first_height.compute_first_per_index(
            starting_indexes.height,
            &self.height.year1,
            exit,
        )?;
        self.year1
            .identity
            .compute_from_index(starting_year1, &self.year1.first_height, exit)?;
        self.year1.date.compute_transform(
            starting_year1,
            &self.year1.first_height,
            |(yi, first_h, _)| (yi, blocks_time_date.collect_one(first_h).unwrap()),
            exit,
        )?;

        // Decade
        self.year10.first_height.compute_first_per_index(
            starting_indexes.height,
            &self.height.year10,
            exit,
        )?;
        self.year10.identity.compute_from_index(
            starting_year10,
            &self.year10.first_height,
            exit,
        )?;
        self.year10.date.compute_transform(
            starting_year10,
            &self.year10.first_height,
            |(di, first_h, _)| (di, blocks_time_date.collect_one(first_h).unwrap()),
            exit,
        )?;

        Ok(ComputeIndexes::new(
            starting_indexes,
            starting_minute1,
            starting_minute5,
            starting_minute10,
            starting_minute30,
            starting_hour1,
            starting_hour4,
            starting_hour12,
            starting_day1,
            starting_day3,
            starting_week1,
            starting_month1,
            starting_month3,
            starting_month6,
            starting_year1,
            starting_year10,
            starting_halvingepoch,
            starting_difficultyepoch,
        ))
    }
}
