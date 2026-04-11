mod addr;
mod height;
mod resolution;
pub mod timestamp;
mod tx_heights;
mod tx_index;
mod txin_index;
mod txout_index;

use std::path::Path;

use brk_error::Result;
use brk_indexer::Indexer;
use brk_traversable::Traversable;
use brk_types::{
    Date, Day1, Day3, Epoch, Halving, Height, Hour1, Hour4, Hour12, Indexes, Minute10, Minute30,
    Month1, Month3, Month6, Version, Week1, Year1, Year10,
};
use vecdb::{Database, Exit, ReadableVec, Rw, StorageMode};

use crate::internal::db_utils::{finalize_db, open_db};

pub use addr::Vecs as AddrVecs;
pub use height::Vecs as HeightVecs;
pub use resolution::{DatedResolutionVecs, ResolutionVecs};
pub use timestamp::Timestamps;
pub use tx_heights::TxHeights;
pub use tx_index::Vecs as TxIndexVecs;
pub use txin_index::Vecs as TxInIndexVecs;
pub use txout_index::Vecs as TxOutIndexVecs;

pub type Minute10Vecs<M = Rw> = ResolutionVecs<Minute10, M>;
pub type Minute30Vecs<M = Rw> = ResolutionVecs<Minute30, M>;
pub type Hour1Vecs<M = Rw> = ResolutionVecs<Hour1, M>;
pub type Hour4Vecs<M = Rw> = ResolutionVecs<Hour4, M>;
pub type Hour12Vecs<M = Rw> = ResolutionVecs<Hour12, M>;
pub type Day1Vecs<M = Rw> = DatedResolutionVecs<Day1, M>;
pub type Day3Vecs<M = Rw> = DatedResolutionVecs<Day3, M>;
pub type EpochVecs<M = Rw> = ResolutionVecs<Epoch, M>;
pub type HalvingVecs<M = Rw> = ResolutionVecs<Halving, M>;
pub type Week1Vecs<M = Rw> = DatedResolutionVecs<Week1, M>;
pub type Month1Vecs<M = Rw> = DatedResolutionVecs<Month1, M>;
pub type Month3Vecs<M = Rw> = DatedResolutionVecs<Month3, M>;
pub type Month6Vecs<M = Rw> = DatedResolutionVecs<Month6, M>;
pub type Year1Vecs<M = Rw> = DatedResolutionVecs<Year1, M>;
pub type Year10Vecs<M = Rw> = DatedResolutionVecs<Year10, M>;

pub const DB_NAME: &str = "indexes";

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    db: Database,
    #[traversable(skip)]
    pub tx_heights: TxHeights,
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
        let epoch = ResolutionVecs::forced_import(&db, version)?;
        let halving = ResolutionVecs::forced_import(&db, version)?;
        let minute10 = ResolutionVecs::forced_import(&db, version)?;
        let minute30 = ResolutionVecs::forced_import(&db, version)?;
        let hour1 = ResolutionVecs::forced_import(&db, version)?;
        let hour4 = ResolutionVecs::forced_import(&db, version)?;
        let hour12 = ResolutionVecs::forced_import(&db, version)?;
        let day1 = Day1Vecs::forced_import(&db, version)?;
        let day3 = DatedResolutionVecs::forced_import(&db, version)?;
        let week1 = DatedResolutionVecs::forced_import(&db, version)?;
        let month1 = DatedResolutionVecs::forced_import(&db, version)?;
        let month3 = DatedResolutionVecs::forced_import(&db, version)?;
        let month6 = DatedResolutionVecs::forced_import(&db, version)?;
        let year1 = DatedResolutionVecs::forced_import(&db, version)?;
        let year10 = DatedResolutionVecs::forced_import(&db, version)?;
        let tx_index = TxIndexVecs::forced_import(&db, version, indexer)?;
        let txin_index = TxInIndexVecs::forced_import(version, indexer);
        let txout_index = TxOutIndexVecs::forced_import(version, indexer);

        let timestamp = Timestamps::forced_import_from_locals(
            &db, version, &minute10, &minute30, &hour1, &hour4, &hour12, &day1, &day3, &week1,
            &month1, &month3, &month6, &year1, &year10,
        )?;

        let this = Self {
            tx_heights: TxHeights::init(indexer),
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

        self.tx_heights.update(indexer, starting_indexes.height);

        // timestamp_monotonic must be computed first — other mappings read it
        self.timestamp
            .compute_monotonic(indexer, starting_indexes.height, exit)?;

        self.compute_tx_indexes(indexer, &starting_indexes, exit)?;
        self.compute_height_indexes(indexer, &starting_indexes, exit)?;

        let prev_height = starting_indexes.height.decremented().unwrap_or_default();

        self.compute_timestamp_mappings(&starting_indexes, exit)?;

        let starting_day1 =
            self.compute_calendar_mappings(indexer, &starting_indexes, prev_height, exit)?;

        self.compute_period_vecs(&starting_indexes, prev_height, starting_day1, exit)?;

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

        self.compute_epoch(indexer, starting_indexes, exit)?;

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
        exit: &Exit,
    ) -> Result<()> {
        self.height.epoch.compute_from_index(
            starting_indexes.height,
            &indexer.vecs.blocks.weight,
            exit,
        )?;
        self.epoch.first_height.inner.compute_first_per_index(
            starting_indexes.height,
            &self.height.epoch,
            exit,
        )?;

        self.height.halving.compute_from_index(
            starting_indexes.height,
            &indexer.vecs.blocks.weight,
            exit,
        )?;
        self.halving.first_height.inner.compute_first_per_index(
            starting_indexes.height,
            &self.height.halving,
            exit,
        )?;
        Ok(())
    }

    fn compute_period_vecs(
        &mut self,
        starting_indexes: &Indexes,
        prev_height: Height,
        starting_day1: Day1,
        exit: &Exit,
    ) -> Result<()> {
        macro_rules! basic_period {
            ($period:ident) => {
                self.$period.first_height.inner.compute_first_per_index(
                    starting_indexes.height,
                    &self.height.$period,
                    exit,
                )?;
            };
        }

        basic_period!(minute10);
        basic_period!(minute30);
        basic_period!(hour1);
        basic_period!(hour4);
        basic_period!(hour12);

        self.day1.first_height.inner.compute_first_per_index(
            starting_indexes.height,
            &self.height.day1,
            exit,
        )?;
        self.day1.date.compute_transform(
            starting_day1,
            &self.day1.first_height,
            |(di, ..)| (di, Date::from(di)),
            exit,
        )?;
        let ts = &self.timestamp.monotonic;

        macro_rules! dated_period {
            ($period:ident) => {{
                self.$period.first_height.inner.compute_first_per_index(
                    starting_indexes.height,
                    &self.height.$period,
                    exit,
                )?;
                let start = self
                    .height
                    .$period
                    .collect_one(prev_height)
                    .unwrap_or_default();
                self.$period.date.compute_transform(
                    start,
                    &self.$period.first_height,
                    |(idx, first_h, _)| (idx, Date::from(ts.collect_one(first_h).unwrap())),
                    exit,
                )?;
            }};
        }

        dated_period!(day3);
        dated_period!(week1);
        dated_period!(month1);
        dated_period!(month3);
        dated_period!(month6);
        dated_period!(year1);
        dated_period!(year10);

        Ok(())
    }
}
