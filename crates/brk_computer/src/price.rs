use std::path::Path;

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{
    Cents, Close, DateIndex, DecadeIndex, DifficultyEpoch, Dollars, Height, High, Low, MonthIndex,
    OHLCDollars, OHLCSats, Open, QuarterIndex, Sats, SemesterIndex, Version, WeekIndex, YearIndex,
};
use vecdb::{Database, EagerVec, Exit, PAGE_SIZE};

use crate::{fetched, grouped::Source};

use super::{
    Indexes,
    grouped::{ComputedVecsFromDateIndex, ComputedVecsFromHeightStrict, VecBuilderOptions},
    indexes,
};

#[derive(Clone, Traversable)]
pub struct Vecs {
    db: Database,

    pub dateindex_to_price_close_in_cents: EagerVec<DateIndex, Close<Cents>>,
    pub dateindex_to_price_high_in_cents: EagerVec<DateIndex, High<Cents>>,
    pub dateindex_to_price_low_in_cents: EagerVec<DateIndex, Low<Cents>>,
    pub dateindex_to_price_ohlc: EagerVec<DateIndex, OHLCDollars>,
    pub dateindex_to_price_ohlc_in_sats: EagerVec<DateIndex, OHLCSats>,
    pub dateindex_to_price_open_in_cents: EagerVec<DateIndex, Open<Cents>>,
    pub height_to_price_close_in_cents: EagerVec<Height, Close<Cents>>,
    pub height_to_price_high_in_cents: EagerVec<Height, High<Cents>>,
    pub height_to_price_low_in_cents: EagerVec<Height, Low<Cents>>,
    pub height_to_price_ohlc: EagerVec<Height, OHLCDollars>,
    pub height_to_price_ohlc_in_sats: EagerVec<Height, OHLCSats>,
    pub height_to_price_open_in_cents: EagerVec<Height, Open<Cents>>,
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
    pub weekindex_to_price_ohlc: EagerVec<WeekIndex, OHLCDollars>,
    pub weekindex_to_price_ohlc_in_sats: EagerVec<WeekIndex, OHLCSats>,
    pub difficultyepoch_to_price_ohlc: EagerVec<DifficultyEpoch, OHLCDollars>,
    pub difficultyepoch_to_price_ohlc_in_sats: EagerVec<DifficultyEpoch, OHLCSats>,
    pub monthindex_to_price_ohlc: EagerVec<MonthIndex, OHLCDollars>,
    pub monthindex_to_price_ohlc_in_sats: EagerVec<MonthIndex, OHLCSats>,
    pub quarterindex_to_price_ohlc: EagerVec<QuarterIndex, OHLCDollars>,
    pub quarterindex_to_price_ohlc_in_sats: EagerVec<QuarterIndex, OHLCSats>,
    pub semesterindex_to_price_ohlc: EagerVec<SemesterIndex, OHLCDollars>,
    pub semesterindex_to_price_ohlc_in_sats: EagerVec<SemesterIndex, OHLCSats>,
    pub yearindex_to_price_ohlc: EagerVec<YearIndex, OHLCDollars>,
    pub yearindex_to_price_ohlc_in_sats: EagerVec<YearIndex, OHLCSats>,
    // pub halvingepoch_to_price_ohlc: StorableVec<Halvingepoch, OHLCDollars>,
    // pub halvingepoch_to_price_ohlc_in_sats: StorableVec<Halvingepoch, OHLCSats>,
    pub decadeindex_to_price_ohlc: EagerVec<DecadeIndex, OHLCDollars>,
    pub decadeindex_to_price_ohlc_in_sats: EagerVec<DecadeIndex, OHLCSats>,
}

const VERSION: Version = Version::ZERO;
const VERSION_IN_SATS: Version = Version::ZERO;

impl Vecs {
    pub fn forced_import(parent: &Path, version: Version, indexes: &indexes::Vecs) -> Result<Self> {
        let db = Database::open(&parent.join("price"))?;
        db.set_min_len(PAGE_SIZE * 1_000_000)?;

        let this = Self {
            dateindex_to_price_ohlc: EagerVec::forced_import_raw(
                &db,
                "price_ohlc",
                version + VERSION + Version::ZERO,
            )?,
            dateindex_to_price_ohlc_in_sats: EagerVec::forced_import_raw(
                &db,
                "price_ohlc_in_sats",
                version + VERSION + VERSION_IN_SATS + Version::ZERO,
            )?,
            dateindex_to_price_close_in_cents: EagerVec::forced_import_compressed(
                &db,
                "price_close_in_cents",
                version + VERSION + Version::ZERO,
            )?,
            dateindex_to_price_high_in_cents: EagerVec::forced_import_compressed(
                &db,
                "price_high_in_cents",
                version + VERSION + Version::ZERO,
            )?,
            dateindex_to_price_low_in_cents: EagerVec::forced_import_compressed(
                &db,
                "price_low_in_cents",
                version + VERSION + Version::ZERO,
            )?,
            dateindex_to_price_open_in_cents: EagerVec::forced_import_compressed(
                &db,
                "price_open_in_cents",
                version + VERSION + Version::ZERO,
            )?,
            height_to_price_ohlc: EagerVec::forced_import_raw(
                &db,
                "price_ohlc",
                version + VERSION + Version::ZERO,
            )?,
            height_to_price_ohlc_in_sats: EagerVec::forced_import_raw(
                &db,
                "price_ohlc_in_sats",
                version + VERSION + VERSION_IN_SATS + Version::ZERO,
            )?,
            height_to_price_close_in_cents: EagerVec::forced_import_compressed(
                &db,
                "price_close_in_cents",
                version + VERSION + Version::ZERO,
            )?,
            height_to_price_high_in_cents: EagerVec::forced_import_compressed(
                &db,
                "price_high_in_cents",
                version + VERSION + Version::ZERO,
            )?,
            height_to_price_low_in_cents: EagerVec::forced_import_compressed(
                &db,
                "price_low_in_cents",
                version + VERSION + Version::ZERO,
            )?,
            height_to_price_open_in_cents: EagerVec::forced_import_compressed(
                &db,
                "price_open_in_cents",
                version + VERSION + Version::ZERO,
            )?,
            timeindexes_to_price_open: ComputedVecsFromDateIndex::forced_import(
                &db,
                "price_open",
                Source::Compute,
                version + VERSION + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_first(),
            )?,
            timeindexes_to_price_high: ComputedVecsFromDateIndex::forced_import(
                &db,
                "price_high",
                Source::Compute,
                version + VERSION + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_max(),
            )?,
            timeindexes_to_price_low: ComputedVecsFromDateIndex::forced_import(
                &db,
                "price_low",
                Source::Compute,
                version + VERSION + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_min(),
            )?,
            timeindexes_to_price_close: ComputedVecsFromDateIndex::forced_import(
                &db,
                "price_close",
                Source::Compute,
                version + VERSION + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            timeindexes_to_price_open_in_sats: ComputedVecsFromDateIndex::forced_import(
                &db,
                "price_open_in_sats",
                Source::Compute,
                version + VERSION + VERSION_IN_SATS + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_first(),
            )?,
            timeindexes_to_price_high_in_sats: ComputedVecsFromDateIndex::forced_import(
                &db,
                "price_high_in_sats",
                Source::Compute,
                version + VERSION + VERSION_IN_SATS + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_max(),
            )?,
            timeindexes_to_price_low_in_sats: ComputedVecsFromDateIndex::forced_import(
                &db,
                "price_low_in_sats",
                Source::Compute,
                version + VERSION + VERSION_IN_SATS + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_min(),
            )?,
            timeindexes_to_price_close_in_sats: ComputedVecsFromDateIndex::forced_import(
                &db,
                "price_close_in_sats",
                Source::Compute,
                version + VERSION + VERSION_IN_SATS + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            chainindexes_to_price_open: ComputedVecsFromHeightStrict::forced_import(
                &db,
                "price_open",
                version + VERSION + Version::ZERO,
                VecBuilderOptions::default().add_first(),
            )?,
            chainindexes_to_price_high: ComputedVecsFromHeightStrict::forced_import(
                &db,
                "price_high",
                version + VERSION + Version::ZERO,
                VecBuilderOptions::default().add_max(),
            )?,
            chainindexes_to_price_low: ComputedVecsFromHeightStrict::forced_import(
                &db,
                "price_low",
                version + VERSION + Version::ZERO,
                VecBuilderOptions::default().add_min(),
            )?,
            chainindexes_to_price_close: ComputedVecsFromHeightStrict::forced_import(
                &db,
                "price_close",
                version + VERSION + Version::ZERO,
                VecBuilderOptions::default().add_last(),
            )?,
            chainindexes_to_price_open_in_sats: ComputedVecsFromHeightStrict::forced_import(
                &db,
                "price_open_in_sats",
                version + VERSION + VERSION_IN_SATS + Version::ZERO,
                VecBuilderOptions::default().add_first(),
            )?,
            chainindexes_to_price_high_in_sats: ComputedVecsFromHeightStrict::forced_import(
                &db,
                "price_high_in_sats",
                version + VERSION + VERSION_IN_SATS + Version::ZERO,
                VecBuilderOptions::default().add_max(),
            )?,
            chainindexes_to_price_low_in_sats: ComputedVecsFromHeightStrict::forced_import(
                &db,
                "price_low_in_sats",
                version + VERSION + VERSION_IN_SATS + Version::ZERO,
                VecBuilderOptions::default().add_min(),
            )?,
            chainindexes_to_price_close_in_sats: ComputedVecsFromHeightStrict::forced_import(
                &db,
                "price_close_in_sats",
                version + VERSION + VERSION_IN_SATS + Version::ZERO,
                VecBuilderOptions::default().add_last(),
            )?,
            weekindex_to_price_ohlc: EagerVec::forced_import_raw(
                &db,
                "price_ohlc",
                version + VERSION + Version::ZERO,
            )?,
            weekindex_to_price_ohlc_in_sats: EagerVec::forced_import_raw(
                &db,
                "price_ohlc_in_sats",
                version + VERSION + VERSION_IN_SATS + Version::ZERO,
            )?,
            difficultyepoch_to_price_ohlc: EagerVec::forced_import_raw(
                &db,
                "price_ohlc",
                version + VERSION + Version::ZERO,
            )?,
            difficultyepoch_to_price_ohlc_in_sats: EagerVec::forced_import_raw(
                &db,
                "price_ohlc_in_sats",
                version + VERSION + VERSION_IN_SATS + Version::ZERO,
            )?,
            monthindex_to_price_ohlc: EagerVec::forced_import_raw(
                &db,
                "price_ohlc",
                version + VERSION + Version::ZERO,
            )?,
            monthindex_to_price_ohlc_in_sats: EagerVec::forced_import_raw(
                &db,
                "price_ohlc_in_sats",
                version + VERSION + VERSION_IN_SATS + Version::ZERO,
            )?,
            quarterindex_to_price_ohlc: EagerVec::forced_import_raw(
                &db,
                "price_ohlc",
                version + VERSION + Version::ZERO,
            )?,
            quarterindex_to_price_ohlc_in_sats: EagerVec::forced_import_raw(
                &db,
                "price_ohlc_in_sats",
                version + VERSION + VERSION_IN_SATS + Version::ZERO,
            )?,
            semesterindex_to_price_ohlc: EagerVec::forced_import_raw(
                &db,
                "price_ohlc",
                version + VERSION + Version::ZERO,
            )?,
            semesterindex_to_price_ohlc_in_sats: EagerVec::forced_import_raw(
                &db,
                "price_ohlc_in_sats",
                version + VERSION + VERSION_IN_SATS + Version::ZERO,
            )?,
            yearindex_to_price_ohlc: EagerVec::forced_import_raw(
                &db,
                "price_ohlc",
                version + VERSION + Version::ZERO,
            )?,
            yearindex_to_price_ohlc_in_sats: EagerVec::forced_import_raw(
                &db,
                "price_ohlc_in_sats",
                version + VERSION + VERSION_IN_SATS + Version::ZERO,
            )?,
            // halvingepoch_to_price_ohlc: StorableVec::forced_import(db,
            // "halvingepoch_to_price_ohlc"), version + VERSION + Version::ZERO, format)?,
            decadeindex_to_price_ohlc: EagerVec::forced_import_raw(
                &db,
                "price_ohlc",
                version + VERSION + Version::ZERO,
            )?,
            decadeindex_to_price_ohlc_in_sats: EagerVec::forced_import_raw(
                &db,
                "price_ohlc_in_sats",
                version + VERSION + VERSION_IN_SATS + Version::ZERO,
            )?,

            db,
        };

        this.db.retain_regions(
            this.iter_any_writable()
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

        // self.halvingepoch_to_price_ohlc
        //     .compute_transform(starting_indexes.halvingepoch, other, t, exit)?;

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
                    self.timeindexes_to_price_open.dateindex.as_ref().unwrap(),
                    |(i, open, ..)| (i, Open::new(Sats::ONE_BTC / *open)),
                    exit,
                )?;
                Ok(())
            })?;

        self.timeindexes_to_price_high_in_sats
            .compute_all(starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.dateindex,
                    self.timeindexes_to_price_low.dateindex.as_ref().unwrap(),
                    |(i, low, ..)| (i, High::new(Sats::ONE_BTC / *low)),
                    exit,
                )?;
                Ok(())
            })?;

        self.timeindexes_to_price_low_in_sats
            .compute_all(starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.dateindex,
                    self.timeindexes_to_price_high.dateindex.as_ref().unwrap(),
                    |(i, high, ..)| (i, Low::new(Sats::ONE_BTC / *high)),
                    exit,
                )?;
                Ok(())
            })?;

        self.timeindexes_to_price_close_in_sats
            .compute_all(starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.dateindex,
                    self.timeindexes_to_price_close.dateindex.as_ref().unwrap(),
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

        // self.halvingepoch_to_price_ohlc
        //     _in_sats.compute_transform(starting_indexes.halvingepoch, other, t, exit)?;

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
