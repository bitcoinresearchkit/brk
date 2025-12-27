use std::path::Path;

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{
    Cents, Close, DateIndex, DecadeIndex, DifficultyEpoch, Dollars, Height, High, Low, MonthIndex,
    OHLCDollars, OHLCSats, Open, QuarterIndex, Sats, SemesterIndex, Version, WeekIndex, YearIndex,
};
use vecdb::{BytesVec, Database, EagerVec, Exit, ImportableVec, PAGE_SIZE, PcoVec};

use crate::{fetched, grouped::Source, utils::OptionExt};

use super::{
    Indexes,
    grouped::{ComputedVecsFromDateIndex, ComputedVecsFromHeightStrict, VecBuilderOptions},
    indexes,
};

pub const DB_NAME: &str = "price";

#[derive(Clone, Traversable)]
pub struct Vecs {
    db: Database,

    pub dateindex_to_price_close_in_cents: EagerVec<PcoVec<DateIndex, Close<Cents>>>,
    pub dateindex_to_price_high_in_cents: EagerVec<PcoVec<DateIndex, High<Cents>>>,
    pub dateindex_to_price_low_in_cents: EagerVec<PcoVec<DateIndex, Low<Cents>>>,
    pub dateindex_to_price_ohlc: EagerVec<BytesVec<DateIndex, OHLCDollars>>,
    pub dateindex_to_price_ohlc_in_sats: EagerVec<BytesVec<DateIndex, OHLCSats>>,
    pub dateindex_to_price_open_in_cents: EagerVec<PcoVec<DateIndex, Open<Cents>>>,
    pub height_to_price_close_in_cents: EagerVec<PcoVec<Height, Close<Cents>>>,
    pub height_to_price_high_in_cents: EagerVec<PcoVec<Height, High<Cents>>>,
    pub height_to_price_low_in_cents: EagerVec<PcoVec<Height, Low<Cents>>>,
    pub height_to_price_ohlc: EagerVec<BytesVec<Height, OHLCDollars>>,
    pub height_to_price_ohlc_in_sats: EagerVec<BytesVec<Height, OHLCSats>>,
    pub height_to_price_open_in_cents: EagerVec<PcoVec<Height, Open<Cents>>>,
    pub timeindexes_to_price_close: ComputedVecsFromDateIndex<Close<Dollars>>,
    pub timeindexes_to_price_high: ComputedVecsFromDateIndex<High<Dollars>>,
    pub timeindexes_to_price_low: ComputedVecsFromDateIndex<Low<Dollars>>,
    pub timeindexes_to_price_open: ComputedVecsFromDateIndex<Open<Dollars>>,
    pub timeindexes_to_price_open_in_sats: ComputedVecsFromDateIndex<Open<Sats>>,
    pub timeindexes_to_price_high_in_sats: ComputedVecsFromDateIndex<High<Sats>>,
    pub timeindexes_to_price_low_in_sats: ComputedVecsFromDateIndex<Low<Sats>>,
    pub timeindexes_to_price_close_in_sats: ComputedVecsFromDateIndex<Close<Sats>>,
    pub chainindexes_to_price_close: ComputedVecsFromHeightStrict<Close<Dollars>>,
    pub chainindexes_to_price_high: ComputedVecsFromHeightStrict<High<Dollars>>,
    pub chainindexes_to_price_low: ComputedVecsFromHeightStrict<Low<Dollars>>,
    pub chainindexes_to_price_open: ComputedVecsFromHeightStrict<Open<Dollars>>,
    pub chainindexes_to_price_open_in_sats: ComputedVecsFromHeightStrict<Open<Sats>>,
    pub chainindexes_to_price_high_in_sats: ComputedVecsFromHeightStrict<High<Sats>>,
    pub chainindexes_to_price_low_in_sats: ComputedVecsFromHeightStrict<Low<Sats>>,
    pub chainindexes_to_price_close_in_sats: ComputedVecsFromHeightStrict<Close<Sats>>,
    pub weekindex_to_price_ohlc: EagerVec<BytesVec<WeekIndex, OHLCDollars>>,
    pub weekindex_to_price_ohlc_in_sats: EagerVec<BytesVec<WeekIndex, OHLCSats>>,
    pub difficultyepoch_to_price_ohlc: EagerVec<BytesVec<DifficultyEpoch, OHLCDollars>>,
    pub difficultyepoch_to_price_ohlc_in_sats: EagerVec<BytesVec<DifficultyEpoch, OHLCSats>>,
    pub monthindex_to_price_ohlc: EagerVec<BytesVec<MonthIndex, OHLCDollars>>,
    pub monthindex_to_price_ohlc_in_sats: EagerVec<BytesVec<MonthIndex, OHLCSats>>,
    pub quarterindex_to_price_ohlc: EagerVec<BytesVec<QuarterIndex, OHLCDollars>>,
    pub quarterindex_to_price_ohlc_in_sats: EagerVec<BytesVec<QuarterIndex, OHLCSats>>,
    pub semesterindex_to_price_ohlc: EagerVec<BytesVec<SemesterIndex, OHLCDollars>>,
    pub semesterindex_to_price_ohlc_in_sats: EagerVec<BytesVec<SemesterIndex, OHLCSats>>,
    pub yearindex_to_price_ohlc: EagerVec<BytesVec<YearIndex, OHLCDollars>>,
    pub yearindex_to_price_ohlc_in_sats: EagerVec<BytesVec<YearIndex, OHLCSats>>,
    // pub halvingepoch_to_price_ohlc: StorableVec<Halvingepoch, OHLCDollars>,
    // pub halvingepoch_to_price_ohlc_in_sats: StorableVec<Halvingepoch, OHLCSats>,
    pub decadeindex_to_price_ohlc: EagerVec<BytesVec<DecadeIndex, OHLCDollars>>,
    pub decadeindex_to_price_ohlc_in_sats: EagerVec<BytesVec<DecadeIndex, OHLCSats>>,
}

const VERSION: Version = Version::ZERO;
const VERSION_IN_SATS: Version = Version::ZERO;

impl Vecs {
    pub fn forced_import(parent: &Path, version: Version, indexes: &indexes::Vecs) -> Result<Self> {
        let db = Database::open(&parent.join(DB_NAME))?;
        db.set_min_len(PAGE_SIZE * 1_000_000)?;

        let v = version + VERSION;
        let v_sats = version + VERSION + VERSION_IN_SATS;

        macro_rules! eager {
            ($name:expr) => {
                EagerVec::forced_import(&db, $name, v)?
            };
        }
        macro_rules! eager_sats {
            ($name:expr) => {
                EagerVec::forced_import(&db, $name, v_sats)?
            };
        }
        macro_rules! computed_di {
            ($name:expr, $opts:expr) => {
                ComputedVecsFromDateIndex::forced_import(
                    &db,
                    $name,
                    Source::Compute,
                    v,
                    indexes,
                    $opts,
                )?
            };
        }
        macro_rules! computed_di_sats {
            ($name:expr, $opts:expr) => {
                ComputedVecsFromDateIndex::forced_import(
                    &db,
                    $name,
                    Source::Compute,
                    v_sats,
                    indexes,
                    $opts,
                )?
            };
        }
        macro_rules! computed_h {
            ($name:expr, $opts:expr) => {
                ComputedVecsFromHeightStrict::forced_import(&db, $name, v, $opts)?
            };
        }
        macro_rules! computed_h_sats {
            ($name:expr, $opts:expr) => {
                ComputedVecsFromHeightStrict::forced_import(&db, $name, v_sats, $opts)?
            };
        }
        let first = || VecBuilderOptions::default().add_first();
        let last = || VecBuilderOptions::default().add_last();
        let min = || VecBuilderOptions::default().add_min();
        let max = || VecBuilderOptions::default().add_max();

        let this = Self {
            dateindex_to_price_ohlc: eager!("price_ohlc"),
            dateindex_to_price_ohlc_in_sats: eager_sats!("price_ohlc_in_sats"),
            dateindex_to_price_close_in_cents: eager!("price_close_in_cents"),
            dateindex_to_price_high_in_cents: eager!("price_high_in_cents"),
            dateindex_to_price_low_in_cents: eager!("price_low_in_cents"),
            dateindex_to_price_open_in_cents: eager!("price_open_in_cents"),
            height_to_price_ohlc: eager!("price_ohlc"),
            height_to_price_ohlc_in_sats: eager_sats!("price_ohlc_in_sats"),
            height_to_price_close_in_cents: eager!("price_close_in_cents"),
            height_to_price_high_in_cents: eager!("price_high_in_cents"),
            height_to_price_low_in_cents: eager!("price_low_in_cents"),
            height_to_price_open_in_cents: eager!("price_open_in_cents"),
            timeindexes_to_price_open: computed_di!("price_open", first()),
            timeindexes_to_price_high: computed_di!("price_high", max()),
            timeindexes_to_price_low: computed_di!("price_low", min()),
            timeindexes_to_price_close: computed_di!("price_close", last()),
            timeindexes_to_price_open_in_sats: computed_di_sats!("price_open_in_sats", first()),
            timeindexes_to_price_high_in_sats: computed_di_sats!("price_high_in_sats", max()),
            timeindexes_to_price_low_in_sats: computed_di_sats!("price_low_in_sats", min()),
            timeindexes_to_price_close_in_sats: computed_di_sats!("price_close_in_sats", last()),
            chainindexes_to_price_open: computed_h!("price_open", first()),
            chainindexes_to_price_high: computed_h!("price_high", max()),
            chainindexes_to_price_low: computed_h!("price_low", min()),
            chainindexes_to_price_close: computed_h!("price_close", last()),
            chainindexes_to_price_open_in_sats: computed_h_sats!("price_open_in_sats", first()),
            chainindexes_to_price_high_in_sats: computed_h_sats!("price_high_in_sats", max()),
            chainindexes_to_price_low_in_sats: computed_h_sats!("price_low_in_sats", min()),
            chainindexes_to_price_close_in_sats: computed_h_sats!("price_close_in_sats", last()),
            weekindex_to_price_ohlc: eager!("price_ohlc"),
            weekindex_to_price_ohlc_in_sats: eager_sats!("price_ohlc_in_sats"),
            difficultyepoch_to_price_ohlc: eager!("price_ohlc"),
            difficultyepoch_to_price_ohlc_in_sats: eager_sats!("price_ohlc_in_sats"),
            monthindex_to_price_ohlc: eager!("price_ohlc"),
            monthindex_to_price_ohlc_in_sats: eager_sats!("price_ohlc_in_sats"),
            quarterindex_to_price_ohlc: eager!("price_ohlc"),
            quarterindex_to_price_ohlc_in_sats: eager_sats!("price_ohlc_in_sats"),
            semesterindex_to_price_ohlc: eager!("price_ohlc"),
            semesterindex_to_price_ohlc_in_sats: eager_sats!("price_ohlc_in_sats"),
            yearindex_to_price_ohlc: eager!("price_ohlc"),
            yearindex_to_price_ohlc_in_sats: eager_sats!("price_ohlc_in_sats"),
            // halvingepoch_to_price_ohlc: StorableVec::forced_import(db,
            // "halvingepoch_to_price_ohlc"), version + VERSION + Version::ZERO, format)?,
            decadeindex_to_price_ohlc: eager!("price_ohlc"),
            decadeindex_to_price_ohlc_in_sats: eager_sats!("price_ohlc_in_sats"),

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
        indexes: &indexes::Vecs,
        starting_indexes: &Indexes,
        fetched: &fetched::Vecs,
        exit: &Exit,
    ) -> Result<()> {
        self.compute_(indexes, starting_indexes, fetched, exit)?;
        let _lock = exit.lock();
        self.db.compact()?;
        Ok(())
    }

    fn compute_(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &Indexes,
        fetched: &fetched::Vecs,
        exit: &Exit,
    ) -> Result<()> {
        self.height_to_price_open_in_cents.compute_transform(
            starting_indexes.height,
            &fetched.height_to_price_ohlc_in_cents,
            |(di, ohlc, ..)| (di, ohlc.open),
            exit,
        )?;

        self.height_to_price_high_in_cents.compute_transform(
            starting_indexes.height,
            &fetched.height_to_price_ohlc_in_cents,
            |(di, ohlc, ..)| (di, ohlc.high),
            exit,
        )?;

        self.height_to_price_low_in_cents.compute_transform(
            starting_indexes.height,
            &fetched.height_to_price_ohlc_in_cents,
            |(di, ohlc, ..)| (di, ohlc.low),
            exit,
        )?;

        self.height_to_price_close_in_cents.compute_transform(
            starting_indexes.height,
            &fetched.height_to_price_ohlc_in_cents,
            |(di, ohlc, ..)| (di, ohlc.close),
            exit,
        )?;

        self.height_to_price_ohlc.compute_transform(
            starting_indexes.height,
            &fetched.height_to_price_ohlc_in_cents,
            |(h, cents, ..)| (h, OHLCDollars::from(cents)),
            exit,
        )?;

        self.dateindex_to_price_open_in_cents.compute_transform(
            starting_indexes.dateindex,
            &fetched.dateindex_to_price_ohlc_in_cents,
            |(di, ohlc, ..)| (di, ohlc.open),
            exit,
        )?;

        self.dateindex_to_price_high_in_cents.compute_transform(
            starting_indexes.dateindex,
            &fetched.dateindex_to_price_ohlc_in_cents,
            |(di, ohlc, ..)| (di, ohlc.high),
            exit,
        )?;

        self.dateindex_to_price_low_in_cents.compute_transform(
            starting_indexes.dateindex,
            &fetched.dateindex_to_price_ohlc_in_cents,
            |(di, ohlc, ..)| (di, ohlc.low),
            exit,
        )?;

        self.dateindex_to_price_close_in_cents.compute_transform(
            starting_indexes.dateindex,
            &fetched.dateindex_to_price_ohlc_in_cents,
            |(di, ohlc, ..)| (di, ohlc.close),
            exit,
        )?;

        self.dateindex_to_price_ohlc.compute_transform(
            starting_indexes.dateindex,
            &fetched.dateindex_to_price_ohlc_in_cents,
            |(di, cents, ..)| (di, OHLCDollars::from(cents)),
            exit,
        )?;

        self.timeindexes_to_price_close
            .compute_all(starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.dateindex,
                    &self.dateindex_to_price_ohlc,
                    |(di, ohlc, ..)| (di, ohlc.close),
                    exit,
                )?;
                Ok(())
            })?;

        self.timeindexes_to_price_high
            .compute_all(starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.dateindex,
                    &self.dateindex_to_price_ohlc,
                    |(di, ohlc, ..)| (di, ohlc.high),
                    exit,
                )?;
                Ok(())
            })?;

        self.timeindexes_to_price_low
            .compute_all(starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.dateindex,
                    &self.dateindex_to_price_ohlc,
                    |(di, ohlc, ..)| (di, ohlc.low),
                    exit,
                )?;
                Ok(())
            })?;

        self.timeindexes_to_price_open
            .compute_all(starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.dateindex,
                    &self.dateindex_to_price_ohlc,
                    |(di, ohlc, ..)| (di, ohlc.open),
                    exit,
                )?;
                Ok(())
            })?;

        self.chainindexes_to_price_close
            .compute(indexes, starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.height,
                    &self.height_to_price_ohlc,
                    |(h, ohlc, ..)| (h, ohlc.close),
                    exit,
                )?;
                Ok(())
            })?;

        self.chainindexes_to_price_high
            .compute(indexes, starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.height,
                    &self.height_to_price_ohlc,
                    |(h, ohlc, ..)| (h, ohlc.high),
                    exit,
                )?;
                Ok(())
            })?;

        self.chainindexes_to_price_low
            .compute(indexes, starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.height,
                    &self.height_to_price_ohlc,
                    |(h, ohlc, ..)| (h, ohlc.low),
                    exit,
                )?;
                Ok(())
            })?;

        self.chainindexes_to_price_open
            .compute(indexes, starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.height,
                    &self.height_to_price_ohlc,
                    |(h, ohlc, ..)| (h, ohlc.open),
                    exit,
                )?;
                Ok(())
            })?;

        self.weekindex_to_price_ohlc.compute_transform4(
            starting_indexes.weekindex,
            self.timeindexes_to_price_open.weekindex.unwrap_first(),
            self.timeindexes_to_price_high.weekindex.unwrap_max(),
            self.timeindexes_to_price_low.weekindex.unwrap_min(),
            self.timeindexes_to_price_close.weekindex.unwrap_last(),
            |(i, open, high, low, close, _)| {
                (
                    i,
                    OHLCDollars {
                        open,
                        high,
                        low,
                        close,
                    },
                )
            },
            exit,
        )?;

        self.difficultyepoch_to_price_ohlc.compute_transform4(
            starting_indexes.difficultyepoch,
            self.chainindexes_to_price_open
                .difficultyepoch
                .unwrap_first(),
            self.chainindexes_to_price_high.difficultyepoch.unwrap_max(),
            self.chainindexes_to_price_low.difficultyepoch.unwrap_min(),
            self.chainindexes_to_price_close
                .difficultyepoch
                .unwrap_last(),
            |(i, open, high, low, close, _)| {
                (
                    i,
                    OHLCDollars {
                        open,
                        high,
                        low,
                        close,
                    },
                )
            },
            exit,
        )?;

        self.monthindex_to_price_ohlc.compute_transform4(
            starting_indexes.monthindex,
            self.timeindexes_to_price_open.monthindex.unwrap_first(),
            self.timeindexes_to_price_high.monthindex.unwrap_max(),
            self.timeindexes_to_price_low.monthindex.unwrap_min(),
            self.timeindexes_to_price_close.monthindex.unwrap_last(),
            |(i, open, high, low, close, _)| {
                (
                    i,
                    OHLCDollars {
                        open,
                        high,
                        low,
                        close,
                    },
                )
            },
            exit,
        )?;

        self.quarterindex_to_price_ohlc.compute_transform4(
            starting_indexes.quarterindex,
            self.timeindexes_to_price_open.quarterindex.unwrap_first(),
            self.timeindexes_to_price_high.quarterindex.unwrap_max(),
            self.timeindexes_to_price_low.quarterindex.unwrap_min(),
            self.timeindexes_to_price_close.quarterindex.unwrap_last(),
            |(i, open, high, low, close, _)| {
                (
                    i,
                    OHLCDollars {
                        open,
                        high,
                        low,
                        close,
                    },
                )
            },
            exit,
        )?;

        self.semesterindex_to_price_ohlc.compute_transform4(
            starting_indexes.semesterindex,
            self.timeindexes_to_price_open.semesterindex.unwrap_first(),
            self.timeindexes_to_price_high.semesterindex.unwrap_max(),
            self.timeindexes_to_price_low.semesterindex.unwrap_min(),
            self.timeindexes_to_price_close.semesterindex.unwrap_last(),
            |(i, open, high, low, close, _)| {
                (
                    i,
                    OHLCDollars {
                        open,
                        high,
                        low,
                        close,
                    },
                )
            },
            exit,
        )?;

        self.yearindex_to_price_ohlc.compute_transform4(
            starting_indexes.yearindex,
            self.timeindexes_to_price_open.yearindex.unwrap_first(),
            self.timeindexes_to_price_high.yearindex.unwrap_max(),
            self.timeindexes_to_price_low.yearindex.unwrap_min(),
            self.timeindexes_to_price_close.yearindex.unwrap_last(),
            |(i, open, high, low, close, _)| {
                (
                    i,
                    OHLCDollars {
                        open,
                        high,
                        low,
                        close,
                    },
                )
            },
            exit,
        )?;

        self.decadeindex_to_price_ohlc.compute_transform4(
            starting_indexes.decadeindex,
            self.timeindexes_to_price_open.decadeindex.unwrap_first(),
            self.timeindexes_to_price_high.decadeindex.unwrap_max(),
            self.timeindexes_to_price_low.decadeindex.unwrap_min(),
            self.timeindexes_to_price_close.decadeindex.unwrap_last(),
            |(i, open, high, low, close, _)| {
                (
                    i,
                    OHLCDollars {
                        open,
                        high,
                        low,
                        close,
                    },
                )
            },
            exit,
        )?;

        self.chainindexes_to_price_open_in_sats
            .compute(indexes, starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.height,
                    &self.chainindexes_to_price_open.height,
                    |(i, open, ..)| (i, Open::new(Sats::ONE_BTC / *open)),
                    exit,
                )?;
                Ok(())
            })?;

        self.chainindexes_to_price_high_in_sats
            .compute(indexes, starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.height,
                    &self.chainindexes_to_price_low.height,
                    |(i, low, ..)| (i, High::new(Sats::ONE_BTC / *low)),
                    exit,
                )?;
                Ok(())
            })?;

        self.chainindexes_to_price_low_in_sats
            .compute(indexes, starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.height,
                    &self.chainindexes_to_price_high.height,
                    |(i, high, ..)| (i, Low::new(Sats::ONE_BTC / *high)),
                    exit,
                )?;
                Ok(())
            })?;

        self.chainindexes_to_price_close_in_sats
            .compute(indexes, starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.height,
                    &self.chainindexes_to_price_close.height,
                    |(i, close, ..)| (i, Close::new(Sats::ONE_BTC / *close)),
                    exit,
                )?;
                Ok(())
            })?;

        self.timeindexes_to_price_open_in_sats
            .compute_all(starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.dateindex,
                    self.timeindexes_to_price_open.dateindex.u(),
                    |(i, open, ..)| (i, Open::new(Sats::ONE_BTC / *open)),
                    exit,
                )?;
                Ok(())
            })?;

        self.timeindexes_to_price_high_in_sats
            .compute_all(starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.dateindex,
                    self.timeindexes_to_price_low.dateindex.u(),
                    |(i, low, ..)| (i, High::new(Sats::ONE_BTC / *low)),
                    exit,
                )?;
                Ok(())
            })?;

        self.timeindexes_to_price_low_in_sats
            .compute_all(starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.dateindex,
                    self.timeindexes_to_price_high.dateindex.u(),
                    |(i, high, ..)| (i, Low::new(Sats::ONE_BTC / *high)),
                    exit,
                )?;
                Ok(())
            })?;

        self.timeindexes_to_price_close_in_sats
            .compute_all(starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.dateindex,
                    self.timeindexes_to_price_close.dateindex.u(),
                    |(i, close, ..)| (i, Close::new(Sats::ONE_BTC / *close)),
                    exit,
                )?;
                Ok(())
            })?;

        self.height_to_price_ohlc_in_sats.compute_transform4(
            starting_indexes.height,
            &self.chainindexes_to_price_open_in_sats.height,
            &self.chainindexes_to_price_high_in_sats.height,
            &self.chainindexes_to_price_low_in_sats.height,
            &self.chainindexes_to_price_close_in_sats.height,
            |(i, open, high, low, close, _)| {
                (
                    i,
                    OHLCSats {
                        open,
                        high,
                        low,
                        close,
                    },
                )
            },
            exit,
        )?;

        self.dateindex_to_price_ohlc_in_sats.compute_transform4(
            starting_indexes.dateindex,
            self.timeindexes_to_price_open_in_sats
                .dateindex
                .as_ref()
                .unwrap(),
            self.timeindexes_to_price_high_in_sats
                .dateindex
                .as_ref()
                .unwrap(),
            self.timeindexes_to_price_low_in_sats
                .dateindex
                .as_ref()
                .unwrap(),
            self.timeindexes_to_price_close_in_sats
                .dateindex
                .as_ref()
                .unwrap(),
            |(i, open, high, low, close, _)| {
                (
                    i,
                    OHLCSats {
                        open,
                        high,
                        low,
                        close,
                    },
                )
            },
            exit,
        )?;

        self.weekindex_to_price_ohlc_in_sats.compute_transform4(
            starting_indexes.weekindex,
            self.timeindexes_to_price_open_in_sats
                .weekindex
                .unwrap_first(),
            self.timeindexes_to_price_high_in_sats
                .weekindex
                .unwrap_max(),
            self.timeindexes_to_price_low_in_sats.weekindex.unwrap_min(),
            self.timeindexes_to_price_close_in_sats
                .weekindex
                .unwrap_last(),
            |(i, open, high, low, close, _)| {
                (
                    i,
                    OHLCSats {
                        open,
                        high,
                        low,
                        close,
                    },
                )
            },
            exit,
        )?;

        self.difficultyepoch_to_price_ohlc_in_sats
            .compute_transform4(
                starting_indexes.difficultyepoch,
                self.chainindexes_to_price_open_in_sats
                    .difficultyepoch
                    .unwrap_first(),
                self.chainindexes_to_price_high_in_sats
                    .difficultyepoch
                    .unwrap_max(),
                self.chainindexes_to_price_low_in_sats
                    .difficultyepoch
                    .unwrap_min(),
                self.chainindexes_to_price_close_in_sats
                    .difficultyepoch
                    .unwrap_last(),
                |(i, open, high, low, close, _)| {
                    (
                        i,
                        OHLCSats {
                            open,
                            high,
                            low,
                            close,
                        },
                    )
                },
                exit,
            )?;

        self.monthindex_to_price_ohlc_in_sats.compute_transform4(
            starting_indexes.monthindex,
            self.timeindexes_to_price_open_in_sats
                .monthindex
                .unwrap_first(),
            self.timeindexes_to_price_high_in_sats
                .monthindex
                .unwrap_max(),
            self.timeindexes_to_price_low_in_sats
                .monthindex
                .unwrap_min(),
            self.timeindexes_to_price_close_in_sats
                .monthindex
                .unwrap_last(),
            |(i, open, high, low, close, _)| {
                (
                    i,
                    OHLCSats {
                        open,
                        high,
                        low,
                        close,
                    },
                )
            },
            exit,
        )?;

        self.quarterindex_to_price_ohlc_in_sats.compute_transform4(
            starting_indexes.quarterindex,
            self.timeindexes_to_price_open_in_sats
                .quarterindex
                .unwrap_first(),
            self.timeindexes_to_price_high_in_sats
                .quarterindex
                .unwrap_max(),
            self.timeindexes_to_price_low_in_sats
                .quarterindex
                .unwrap_min(),
            self.timeindexes_to_price_close_in_sats
                .quarterindex
                .unwrap_last(),
            |(i, open, high, low, close, _)| {
                (
                    i,
                    OHLCSats {
                        open,
                        high,
                        low,
                        close,
                    },
                )
            },
            exit,
        )?;

        self.semesterindex_to_price_ohlc_in_sats
            .compute_transform4(
                starting_indexes.semesterindex,
                self.timeindexes_to_price_open_in_sats
                    .semesterindex
                    .unwrap_first(),
                self.timeindexes_to_price_high_in_sats
                    .semesterindex
                    .unwrap_max(),
                self.timeindexes_to_price_low_in_sats
                    .semesterindex
                    .unwrap_min(),
                self.timeindexes_to_price_close_in_sats
                    .semesterindex
                    .unwrap_last(),
                |(i, open, high, low, close, _)| {
                    (
                        i,
                        OHLCSats {
                            open,
                            high,
                            low,
                            close,
                        },
                    )
                },
                exit,
            )?;

        self.yearindex_to_price_ohlc_in_sats.compute_transform4(
            starting_indexes.yearindex,
            self.timeindexes_to_price_open_in_sats
                .yearindex
                .unwrap_first(),
            self.timeindexes_to_price_high_in_sats
                .yearindex
                .unwrap_max(),
            self.timeindexes_to_price_low_in_sats.yearindex.unwrap_min(),
            self.timeindexes_to_price_close_in_sats
                .yearindex
                .unwrap_last(),
            |(i, open, high, low, close, _)| {
                (
                    i,
                    OHLCSats {
                        open,
                        high,
                        low,
                        close,
                    },
                )
            },
            exit,
        )?;

        self.decadeindex_to_price_ohlc_in_sats.compute_transform4(
            starting_indexes.decadeindex,
            self.timeindexes_to_price_open_in_sats
                .decadeindex
                .unwrap_first(),
            self.timeindexes_to_price_high_in_sats
                .decadeindex
                .unwrap_max(),
            self.timeindexes_to_price_low_in_sats
                .decadeindex
                .unwrap_min(),
            self.timeindexes_to_price_close_in_sats
                .decadeindex
                .unwrap_last(),
            |(i, open, high, low, close, _)| {
                (
                    i,
                    OHLCSats {
                        open,
                        high,
                        low,
                        close,
                    },
                )
            },
            exit,
        )?;

        Ok(())
    }
}
