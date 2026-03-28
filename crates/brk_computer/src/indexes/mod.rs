mod addr;
mod cached_mappings;
mod day1;
mod day3;
mod epoch;
mod halving;
mod height;
mod hour1;
mod hour12;
mod hour4;
mod minute10;
mod minute30;
mod month1;
mod month3;
mod month6;
pub mod timestamp;
mod tx_index;
mod txin_index;
mod txout_index;
mod week1;
mod year1;
mod year10;

use std::path::Path;

use brk_error::Result;
use brk_indexer::Indexer;
use brk_traversable::Traversable;
use brk_types::{
    Date, Day1, Day3, Height, Hour1, Hour4, Hour12, Indexes, Minute10, Minute30, Month1, Month3,
    Month6, Version, Week1, Year1, Year10,
};
use vecdb::{CachedVec, Database, Exit, ReadableVec, Rw, StorageMode};

use crate::internal::db_utils::{finalize_db, open_db};

pub use addr::Vecs as AddrVecs;
pub use cached_mappings::CachedMappings;
pub use day1::Vecs as Day1Vecs;
pub use day3::Vecs as Day3Vecs;
pub use epoch::Vecs as EpochVecs;
pub use halving::Vecs as HalvingVecs;
pub use height::Vecs as HeightVecs;
pub use hour1::Vecs as Hour1Vecs;
pub use hour4::Vecs as Hour4Vecs;
pub use hour12::Vecs as Hour12Vecs;
pub use minute10::Vecs as Minute10Vecs;
pub use minute30::Vecs as Minute30Vecs;
pub use month1::Vecs as Month1Vecs;
pub use month3::Vecs as Month3Vecs;
pub use month6::Vecs as Month6Vecs;
pub use timestamp::Timestamps;
pub use tx_index::Vecs as TxIndexVecs;
pub use txin_index::Vecs as TxInIndexVecs;
pub use txout_index::Vecs as TxOutIndexVecs;
pub use week1::Vecs as Week1Vecs;
pub use year1::Vecs as Year1Vecs;
pub use year10::Vecs as Year10Vecs;

pub const DB_NAME: &str = "indexes";

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    db: Database,
    #[traversable(skip)]
    pub cached_mappings: CachedMappings,
    pub addr: AddrVecs,
    pub height: HeightVecs<M>,
    pub epoch: EpochVecs<M>,
    pub halving: HalvingVecs<M>,
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
    pub tx_index: TxIndexVecs<M>,
    pub txin_index: TxInIndexVecs,
    pub txout_index: TxOutIndexVecs,
    pub timestamp: Timestamps<M>,
}

impl Vecs {
    pub(crate) fn forced_import(
        parent: &Path,
        parent_version: Version,
        indexer: &Indexer,
    ) -> Result<Self> {
        let db = open_db(parent, DB_NAME, 1_000_000)?;

        let version = parent_version;

        let addr = AddrVecs::forced_import(version, indexer);
        let height = HeightVecs::forced_import(&db, version)?;
        let epoch = EpochVecs::forced_import(&db, version)?;
        let halving = HalvingVecs::forced_import(&db, version)?;
        let minute10 = Minute10Vecs::forced_import(&db, version)?;
        let minute30 = Minute30Vecs::forced_import(&db, version)?;
        let hour1 = Hour1Vecs::forced_import(&db, version)?;
        let hour4 = Hour4Vecs::forced_import(&db, version)?;
        let hour12 = Hour12Vecs::forced_import(&db, version)?;
        let day1 = Day1Vecs::forced_import(&db, version)?;
        let day3 = Day3Vecs::forced_import(&db, version)?;
        let week1 = Week1Vecs::forced_import(&db, version)?;
        let month1 = Month1Vecs::forced_import(&db, version)?;
        let month3 = Month3Vecs::forced_import(&db, version)?;
        let month6 = Month6Vecs::forced_import(&db, version)?;
        let year1 = Year1Vecs::forced_import(&db, version)?;
        let year10 = Year10Vecs::forced_import(&db, version)?;
        let tx_index = TxIndexVecs::forced_import(&db, version, indexer)?;
        let txin_index = TxInIndexVecs::forced_import(version, indexer);
        let txout_index = TxOutIndexVecs::forced_import(version, indexer);

        let cached_mappings = CachedMappings {
            minute10_first_height: CachedVec::new(&minute10.first_height),
            minute30_first_height: CachedVec::new(&minute30.first_height),
            hour1_first_height: CachedVec::new(&hour1.first_height),
            hour4_first_height: CachedVec::new(&hour4.first_height),
            hour12_first_height: CachedVec::new(&hour12.first_height),
            day1_first_height: CachedVec::new(&day1.first_height),
            day3_first_height: CachedVec::new(&day3.first_height),
            week1_first_height: CachedVec::new(&week1.first_height),
            month1_first_height: CachedVec::new(&month1.first_height),
            month3_first_height: CachedVec::new(&month3.first_height),
            month6_first_height: CachedVec::new(&month6.first_height),
            year1_first_height: CachedVec::new(&year1.first_height),
            year10_first_height: CachedVec::new(&year10.first_height),
            halving_identity: CachedVec::new(&halving.identity),
            epoch_identity: CachedVec::new(&epoch.identity),
        };

        let timestamp = Timestamps::forced_import_from_locals(
            &db, version, &minute10, &minute30, &hour1, &hour4, &hour12, &day1, &day3, &week1,
            &month1, &month3, &month6, &year1, &year10,
        )?;

        let this = Self {
            cached_mappings,
            addr,
            height,
            epoch,
            halving,
            minute10,
            minute30,
            hour1,
            hour4,
            hour12,
            day1,
            day3,
            week1,
            month1,
            month3,
            month6,
            year1,
            year10,
            tx_index,
            txin_index,
            txout_index,
            timestamp,
            db,
        };

        finalize_db(&this.db, &this)?;
        Ok(this)
    }

    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        starting_indexes: Indexes,
        exit: &Exit,
    ) -> Result<Indexes> {
        self.db.sync_bg_tasks()?;

        // timestamp_monotonic must be computed first — other mappings read it
        self.timestamp
            .compute_monotonic(indexer, starting_indexes.height, exit)?;

        self.compute_tx_indexes(indexer, &starting_indexes, exit)?;
        self.compute_height_indexes(indexer, &starting_indexes, exit)?;

        let prev_height = starting_indexes.height.decremented().unwrap_or_default();

        self.compute_timestamp_mappings(&starting_indexes, exit)?;

        let starting_day1 =
            self.compute_calendar_mappings(indexer, &starting_indexes, prev_height, exit)?;

        self.compute_period_vecs(indexer, &starting_indexes, prev_height, starting_day1, exit)?;

        self.timestamp.compute_per_resolution(
            indexer,
            &self.height,
            &self.halving,
            &self.epoch,
            &starting_indexes,
            exit,
        )?;

        let exit = exit.clone();
        self.db.run_bg(move |db| {
            let _lock = exit.lock();
            db.compact_deferred_default()
        });
        Ok(starting_indexes)
    }

    fn compute_tx_indexes(
        &mut self,
        indexer: &Indexer,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        let (r1, r2) = rayon::join(
            || {
                self.tx_index.input_count.compute_count_from_indexes(
                    starting_indexes.tx_index,
                    &indexer.vecs.transactions.first_txin_index,
                    &indexer.vecs.inputs.outpoint,
                    exit,
                )
            },
            || {
                self.tx_index.output_count.compute_count_from_indexes(
                    starting_indexes.tx_index,
                    &indexer.vecs.transactions.first_txout_index,
                    &indexer.vecs.outputs.value,
                    exit,
                )
            },
        );
        r1?;
        r2?;
        Ok(())
    }

    fn compute_height_indexes(
        &mut self,
        indexer: &Indexer,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.height.tx_index_count.compute_count_from_indexes(
            starting_indexes.height,
            &indexer.vecs.transactions.first_tx_index,
            &indexer.vecs.transactions.txid,
            exit,
        )?;
        self.height.identity.compute_from_index(
            starting_indexes.height,
            &indexer.vecs.blocks.weight,
            exit,
        )?;
        Ok(())
    }

    fn compute_timestamp_mappings(
        &mut self,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        macro_rules! from_timestamp {
            ($field:ident, $period:ty) => {
                self.height.$field.compute_transform(
                    starting_indexes.height,
                    &self.timestamp.monotonic,
                    |(h, ts, _)| (h, <$period>::from_timestamp(ts)),
                    exit,
                )?;
            };
        }

        from_timestamp!(minute10, Minute10);
        from_timestamp!(minute30, Minute30);
        from_timestamp!(hour1, Hour1);
        from_timestamp!(hour4, Hour4);
        from_timestamp!(hour12, Hour12);
        from_timestamp!(day3, Day3);

        Ok(())
    }

    fn compute_calendar_mappings(
        &mut self,
        indexer: &Indexer,
        starting_indexes: &Indexes,
        prev_height: Height,
        exit: &Exit,
    ) -> Result<Day1> {
        let starting_day1 = self
            .height
            .day1
            .collect_one(prev_height)
            .unwrap_or_default();

        self.height.day1.compute_transform(
            starting_indexes.height,
            &self.timestamp.monotonic,
            |(h, ts, ..)| (h, Day1::try_from(Date::from(ts)).unwrap()),
            exit,
        )?;

        let starting_day1 = if let Some(day1) = self.height.day1.collect_one(prev_height) {
            starting_day1.min(day1)
        } else {
            starting_day1
        };

        self.compute_epoch(indexer, starting_indexes, prev_height, exit)?;

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

        Ok(starting_day1)
    }

    fn compute_epoch(
        &mut self,
        indexer: &Indexer,
        starting_indexes: &Indexes,
        prev_height: Height,
        exit: &Exit,
    ) -> Result<()> {
        let starting_difficulty = self
            .height
            .epoch
            .collect_one(prev_height)
            .unwrap_or_default();

        self.height.epoch.compute_from_index(
            starting_indexes.height,
            &indexer.vecs.blocks.weight,
            exit,
        )?;
        self.epoch.first_height.compute_first_per_index(
            starting_indexes.height,
            &self.height.epoch,
            exit,
        )?;
        self.epoch.identity.compute_from_index(
            starting_difficulty,
            &self.epoch.first_height,
            exit,
        )?;
        self.epoch.height_count.compute_count_from_indexes(
            starting_difficulty,
            &self.epoch.first_height,
            &self.timestamp.monotonic,
            exit,
        )?;

        let starting_halving = self
            .height
            .halving
            .collect_one(prev_height)
            .unwrap_or_default();

        self.height.halving.compute_from_index(
            starting_indexes.height,
            &indexer.vecs.blocks.weight,
            exit,
        )?;
        self.halving.first_height.compute_first_per_index(
            starting_indexes.height,
            &self.height.halving,
            exit,
        )?;
        self.halving.identity.compute_from_index(
            starting_halving,
            &self.halving.first_height,
            exit,
        )?;

        Ok(())
    }

    fn compute_period_vecs(
        &mut self,
        indexer: &Indexer,
        starting_indexes: &Indexes,
        prev_height: Height,
        starting_day1: Day1,
        exit: &Exit,
    ) -> Result<()> {
        macro_rules! basic_period {
            ($period:ident) => {
                self.$period.first_height.compute_first_per_index(
                    starting_indexes.height,
                    &self.height.$period,
                    exit,
                )?;
                self.$period.identity.compute_from_index(
                    self.height
                        .$period
                        .collect_one(prev_height)
                        .unwrap_or_default(),
                    &self.$period.first_height,
                    exit,
                )?;
            };
        }

        basic_period!(minute10);
        basic_period!(minute30);
        basic_period!(hour1);
        basic_period!(hour4);
        basic_period!(hour12);
        basic_period!(day3);

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

        let ts = &self.timestamp.monotonic;

        macro_rules! dated_period {
            ($period:ident) => {{
                self.$period.first_height.compute_first_per_index(
                    starting_indexes.height,
                    &self.height.$period,
                    exit,
                )?;
                let start = self
                    .height
                    .$period
                    .collect_one(prev_height)
                    .unwrap_or_default();
                self.$period.identity.compute_from_index(
                    start,
                    &self.$period.first_height,
                    exit,
                )?;
                self.$period.date.compute_transform(
                    start,
                    &self.$period.first_height,
                    |(idx, first_h, _)| (idx, Date::from(ts.collect_one(first_h).unwrap())),
                    exit,
                )?;
            }};
        }

        dated_period!(week1);
        dated_period!(month1);
        dated_period!(month3);
        dated_period!(month6);
        dated_period!(year1);
        dated_period!(year10);

        Ok(())
    }
}
