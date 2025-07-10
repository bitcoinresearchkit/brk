use std::path::Path;

use brk_core::{
    Cents, Close, DateIndex, DecadeIndex, DifficultyEpoch, Dollars, Height, High, Low, MonthIndex,
    OHLCCents, OHLCDollars, OHLCSats, Open, QuarterIndex, Sats, SemesterIndex, Version, WeekIndex,
    YearIndex,
};
use brk_exit::Exit;
use brk_fetcher::Fetcher;
use brk_indexer::Indexer;
use brk_vec::{
    AnyCollectableVec, AnyIterableVec, AnyVec, Computation, EagerVec, Format, StoredIndex,
};

use crate::vecs::grouped::Source;

use super::{
    Indexes,
    grouped::{ComputedVecsFromDateIndex, ComputedVecsFromHeightStrict, EagerVecBuilderOptions},
    indexes,
};

#[derive(Clone)]
pub struct Vecs {
    pub dateindex_to_close_in_cents: EagerVec<DateIndex, Close<Cents>>,
    pub dateindex_to_high_in_cents: EagerVec<DateIndex, High<Cents>>,
    pub dateindex_to_low_in_cents: EagerVec<DateIndex, Low<Cents>>,
    pub dateindex_to_ohlc: EagerVec<DateIndex, OHLCDollars>,
    pub dateindex_to_ohlc_in_sats: EagerVec<DateIndex, OHLCSats>,
    pub dateindex_to_ohlc_in_cents: EagerVec<DateIndex, OHLCCents>,
    pub dateindex_to_open_in_cents: EagerVec<DateIndex, Open<Cents>>,
    pub height_to_close_in_cents: EagerVec<Height, Close<Cents>>,
    pub height_to_high_in_cents: EagerVec<Height, High<Cents>>,
    pub height_to_low_in_cents: EagerVec<Height, Low<Cents>>,
    pub height_to_ohlc: EagerVec<Height, OHLCDollars>,
    pub height_to_ohlc_in_sats: EagerVec<Height, OHLCSats>,
    pub height_to_ohlc_in_cents: EagerVec<Height, OHLCCents>,
    pub height_to_open_in_cents: EagerVec<Height, Open<Cents>>,
    pub timeindexes_to_close: ComputedVecsFromDateIndex<Close<Dollars>>,
    pub timeindexes_to_high: ComputedVecsFromDateIndex<High<Dollars>>,
    pub timeindexes_to_low: ComputedVecsFromDateIndex<Low<Dollars>>,
    pub timeindexes_to_open: ComputedVecsFromDateIndex<Open<Dollars>>,
    pub timeindexes_to_open_in_sats: ComputedVecsFromDateIndex<Open<Sats>>,
    pub timeindexes_to_high_in_sats: ComputedVecsFromDateIndex<High<Sats>>,
    pub timeindexes_to_low_in_sats: ComputedVecsFromDateIndex<Low<Sats>>,
    pub timeindexes_to_close_in_sats: ComputedVecsFromDateIndex<Close<Sats>>,
    pub chainindexes_to_close: ComputedVecsFromHeightStrict<Close<Dollars>>,
    pub chainindexes_to_high: ComputedVecsFromHeightStrict<High<Dollars>>,
    pub chainindexes_to_low: ComputedVecsFromHeightStrict<Low<Dollars>>,
    pub chainindexes_to_open: ComputedVecsFromHeightStrict<Open<Dollars>>,
    pub chainindexes_to_open_in_sats: ComputedVecsFromHeightStrict<Open<Sats>>,
    pub chainindexes_to_high_in_sats: ComputedVecsFromHeightStrict<High<Sats>>,
    pub chainindexes_to_low_in_sats: ComputedVecsFromHeightStrict<Low<Sats>>,
    pub chainindexes_to_close_in_sats: ComputedVecsFromHeightStrict<Close<Sats>>,
    pub weekindex_to_ohlc: EagerVec<WeekIndex, OHLCDollars>,
    pub weekindex_to_ohlc_in_sats: EagerVec<WeekIndex, OHLCSats>,
    pub difficultyepoch_to_ohlc: EagerVec<DifficultyEpoch, OHLCDollars>,
    pub difficultyepoch_to_ohlc_in_sats: EagerVec<DifficultyEpoch, OHLCSats>,
    pub monthindex_to_ohlc: EagerVec<MonthIndex, OHLCDollars>,
    pub monthindex_to_ohlc_in_sats: EagerVec<MonthIndex, OHLCSats>,
    pub quarterindex_to_ohlc: EagerVec<QuarterIndex, OHLCDollars>,
    pub quarterindex_to_ohlc_in_sats: EagerVec<QuarterIndex, OHLCSats>,
    pub semesterindex_to_ohlc: EagerVec<SemesterIndex, OHLCDollars>,
    pub semesterindex_to_ohlc_in_sats: EagerVec<SemesterIndex, OHLCSats>,
    pub yearindex_to_ohlc: EagerVec<YearIndex, OHLCDollars>,
    pub yearindex_to_ohlc_in_sats: EagerVec<YearIndex, OHLCSats>,
    // pub halvingepoch_to_ohlc: StorableVec<Halvingepoch, OHLCDollars>,
    // pub halvingepoch_to_ohlc_in_sats: StorableVec<Halvingepoch, OHLCSats>,
    pub decadeindex_to_ohlc: EagerVec<DecadeIndex, OHLCDollars>,
    pub decadeindex_to_ohlc_in_sats: EagerVec<DecadeIndex, OHLCSats>,
}

const VERSION: Version = Version::ZERO;
const VERSION_IN_SATS: Version = Version::ZERO;

impl Vecs {
    pub fn forced_import(
        path: &Path,
        version: Version,
        computation: Computation,
        format: Format,
    ) -> color_eyre::Result<Self> {
        let mut fetched_path = path.to_owned();
        fetched_path.pop();
        fetched_path = fetched_path.join("fetched");

        Ok(Self {
            dateindex_to_ohlc_in_cents: EagerVec::forced_import(
                &fetched_path,
                "ohlc_in_cents",
                version + VERSION + Version::ZERO,
                format,
            )?,
            dateindex_to_ohlc: EagerVec::forced_import(
                path,
                "ohlc",
                version + VERSION + Version::ZERO,
                format,
            )?,
            dateindex_to_ohlc_in_sats: EagerVec::forced_import(
                path,
                "ohlc_in_sats",
                version + VERSION + VERSION_IN_SATS + Version::ZERO,
                format,
            )?,
            dateindex_to_close_in_cents: EagerVec::forced_import(
                path,
                "close_in_cents",
                version + VERSION + Version::ZERO,
                format,
            )?,
            dateindex_to_high_in_cents: EagerVec::forced_import(
                path,
                "high_in_cents",
                version + VERSION + Version::ZERO,
                format,
            )?,
            dateindex_to_low_in_cents: EagerVec::forced_import(
                path,
                "low_in_cents",
                version + VERSION + Version::ZERO,
                format,
            )?,
            dateindex_to_open_in_cents: EagerVec::forced_import(
                path,
                "open_in_cents",
                version + VERSION + Version::ZERO,
                format,
            )?,
            height_to_ohlc_in_cents: EagerVec::forced_import(
                &fetched_path,
                "ohlc_in_cents",
                version + VERSION + Version::ZERO,
                format,
            )?,
            height_to_ohlc: EagerVec::forced_import(
                path,
                "ohlc",
                version + VERSION + Version::ZERO,
                format,
            )?,
            height_to_ohlc_in_sats: EagerVec::forced_import(
                path,
                "ohlc_in_sats",
                version + VERSION + VERSION_IN_SATS + Version::ZERO,
                format,
            )?,
            height_to_close_in_cents: EagerVec::forced_import(
                path,
                "close_in_cents",
                version + VERSION + Version::ZERO,
                format,
            )?,
            height_to_high_in_cents: EagerVec::forced_import(
                path,
                "high_in_cents",
                version + VERSION + Version::ZERO,
                format,
            )?,
            height_to_low_in_cents: EagerVec::forced_import(
                path,
                "low_in_cents",
                version + VERSION + Version::ZERO,
                format,
            )?,
            height_to_open_in_cents: EagerVec::forced_import(
                path,
                "open_in_cents",
                version + VERSION + Version::ZERO,
                format,
            )?,
            timeindexes_to_open: ComputedVecsFromDateIndex::forced_import(
                path,
                "open",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                EagerVecBuilderOptions::default().add_first(),
            )?,
            timeindexes_to_high: ComputedVecsFromDateIndex::forced_import(
                path,
                "high",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                EagerVecBuilderOptions::default().add_max(),
            )?,
            timeindexes_to_low: ComputedVecsFromDateIndex::forced_import(
                path,
                "low",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                EagerVecBuilderOptions::default().add_min(),
            )?,
            timeindexes_to_close: ComputedVecsFromDateIndex::forced_import(
                path,
                "close",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                EagerVecBuilderOptions::default().add_last(),
            )?,
            timeindexes_to_open_in_sats: ComputedVecsFromDateIndex::forced_import(
                path,
                "open_in_sats",
                Source::Compute,
                version + VERSION + VERSION_IN_SATS + Version::ZERO,
                format,
                computation,
                EagerVecBuilderOptions::default().add_first(),
            )?,
            timeindexes_to_high_in_sats: ComputedVecsFromDateIndex::forced_import(
                path,
                "high_in_sats",
                Source::Compute,
                version + VERSION + VERSION_IN_SATS + Version::ZERO,
                format,
                computation,
                EagerVecBuilderOptions::default().add_max(),
            )?,
            timeindexes_to_low_in_sats: ComputedVecsFromDateIndex::forced_import(
                path,
                "low_in_sats",
                Source::Compute,
                version + VERSION + VERSION_IN_SATS + Version::ZERO,
                format,
                computation,
                EagerVecBuilderOptions::default().add_min(),
            )?,
            timeindexes_to_close_in_sats: ComputedVecsFromDateIndex::forced_import(
                path,
                "close_in_sats",
                Source::Compute,
                version + VERSION + VERSION_IN_SATS + Version::ZERO,
                format,
                computation,
                EagerVecBuilderOptions::default().add_last(),
            )?,
            chainindexes_to_open: ComputedVecsFromHeightStrict::forced_import(
                path,
                "open",
                version + VERSION + Version::ZERO,
                format,
                EagerVecBuilderOptions::default().add_first(),
            )?,
            chainindexes_to_high: ComputedVecsFromHeightStrict::forced_import(
                path,
                "high",
                version + VERSION + Version::ZERO,
                format,
                EagerVecBuilderOptions::default().add_max(),
            )?,
            chainindexes_to_low: ComputedVecsFromHeightStrict::forced_import(
                path,
                "low",
                version + VERSION + Version::ZERO,
                format,
                EagerVecBuilderOptions::default().add_min(),
            )?,
            chainindexes_to_close: ComputedVecsFromHeightStrict::forced_import(
                path,
                "close",
                version + VERSION + Version::ZERO,
                format,
                EagerVecBuilderOptions::default().add_last(),
            )?,
            chainindexes_to_open_in_sats: ComputedVecsFromHeightStrict::forced_import(
                path,
                "open_in_sats",
                version + VERSION + VERSION_IN_SATS + Version::ZERO,
                format,
                EagerVecBuilderOptions::default().add_first(),
            )?,
            chainindexes_to_high_in_sats: ComputedVecsFromHeightStrict::forced_import(
                path,
                "high_in_sats",
                version + VERSION + VERSION_IN_SATS + Version::ZERO,
                format,
                EagerVecBuilderOptions::default().add_max(),
            )?,
            chainindexes_to_low_in_sats: ComputedVecsFromHeightStrict::forced_import(
                path,
                "low_in_sats",
                version + VERSION + VERSION_IN_SATS + Version::ZERO,
                format,
                EagerVecBuilderOptions::default().add_min(),
            )?,
            chainindexes_to_close_in_sats: ComputedVecsFromHeightStrict::forced_import(
                path,
                "close_in_sats",
                version + VERSION + VERSION_IN_SATS + Version::ZERO,
                format,
                EagerVecBuilderOptions::default().add_last(),
            )?,
            weekindex_to_ohlc: EagerVec::forced_import(
                path,
                "ohlc",
                version + VERSION + Version::ZERO,
                format,
            )?,
            weekindex_to_ohlc_in_sats: EagerVec::forced_import(
                path,
                "ohlc_in_sats",
                version + VERSION + VERSION_IN_SATS + Version::ZERO,
                format,
            )?,
            difficultyepoch_to_ohlc: EagerVec::forced_import(
                path,
                "ohlc",
                version + VERSION + Version::ZERO,
                format,
            )?,
            difficultyepoch_to_ohlc_in_sats: EagerVec::forced_import(
                path,
                "ohlc_in_sats",
                version + VERSION + VERSION_IN_SATS + Version::ZERO,
                format,
            )?,
            monthindex_to_ohlc: EagerVec::forced_import(
                path,
                "ohlc",
                version + VERSION + Version::ZERO,
                format,
            )?,
            monthindex_to_ohlc_in_sats: EagerVec::forced_import(
                path,
                "ohlc_in_sats",
                version + VERSION + VERSION_IN_SATS + Version::ZERO,
                format,
            )?,
            quarterindex_to_ohlc: EagerVec::forced_import(
                path,
                "ohlc",
                version + VERSION + Version::ZERO,
                format,
            )?,
            quarterindex_to_ohlc_in_sats: EagerVec::forced_import(
                path,
                "ohlc_in_sats",
                version + VERSION + VERSION_IN_SATS + Version::ZERO,
                format,
            )?,
            semesterindex_to_ohlc: EagerVec::forced_import(
                path,
                "ohlc",
                version + VERSION + Version::ZERO,
                format,
            )?,
            semesterindex_to_ohlc_in_sats: EagerVec::forced_import(
                path,
                "ohlc_in_sats",
                version + VERSION + VERSION_IN_SATS + Version::ZERO,
                format,
            )?,
            yearindex_to_ohlc: EagerVec::forced_import(
                path,
                "ohlc",
                version + VERSION + Version::ZERO,
                format,
            )?,
            yearindex_to_ohlc_in_sats: EagerVec::forced_import(
                path,
                "ohlc_in_sats",
                version + VERSION + VERSION_IN_SATS + Version::ZERO,
                format,
            )?,
            // halvingepoch_to_ohlc: StorableVec::forced_import(path,
            // "halvingepoch_to_ohlc"), version + VERSION + Version::ZERO, format)?,
            decadeindex_to_ohlc: EagerVec::forced_import(
                path,
                "ohlc",
                version + VERSION + Version::ZERO,
                format,
            )?,
            decadeindex_to_ohlc_in_sats: EagerVec::forced_import(
                path,
                "ohlc_in_sats",
                version + VERSION + VERSION_IN_SATS + Version::ZERO,
                format,
            )?,
        })
    }

    pub fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &Indexes,
        fetcher: &mut Fetcher,
        exit: &Exit,
    ) -> color_eyre::Result<()> {
        let mut height_to_timestamp_iter = indexer.vecs.height_to_timestamp.iter();
        self.height_to_ohlc_in_cents.compute_transform(
            starting_indexes.height,
            &indexer.vecs.height_to_timestamp,
            |(h, t, ..)| {
                let ohlc = fetcher
                    .get_height(
                        h,
                        t,
                        h.decremented()
                            .map(|prev_h| height_to_timestamp_iter.unwrap_get_inner(prev_h)),
                    )
                    .unwrap();
                (h, ohlc)
            },
            exit,
        )?;

        self.height_to_open_in_cents.compute_transform(
            starting_indexes.height,
            &self.height_to_ohlc_in_cents,
            |(di, ohlc, ..)| (di, ohlc.open),
            exit,
        )?;

        self.height_to_high_in_cents.compute_transform(
            starting_indexes.height,
            &self.height_to_ohlc_in_cents,
            |(di, ohlc, ..)| (di, ohlc.high),
            exit,
        )?;

        self.height_to_low_in_cents.compute_transform(
            starting_indexes.height,
            &self.height_to_ohlc_in_cents,
            |(di, ohlc, ..)| (di, ohlc.low),
            exit,
        )?;

        self.height_to_close_in_cents.compute_transform(
            starting_indexes.height,
            &self.height_to_ohlc_in_cents,
            |(di, ohlc, ..)| (di, ohlc.close),
            exit,
        )?;

        self.height_to_ohlc.compute_transform(
            starting_indexes.height,
            &self.height_to_ohlc_in_cents,
            |(di, ohlc, ..)| (di, OHLCDollars::from(ohlc)),
            exit,
        )?;

        self.dateindex_to_ohlc_in_cents.compute_transform(
            starting_indexes.dateindex,
            &indexes.dateindex_to_date,
            |(di, d, this)| {
                let get_prev = || {
                    this.get_or_read(di, &this.mmap().load())
                        .unwrap()
                        .unwrap()
                        .into_owned()
                };

                let mut ohlc = if di.unwrap_to_usize() + 1 >= this.len() {
                    fetcher.get_date(d).unwrap_or_else(|_| get_prev())
                } else {
                    get_prev()
                };

                if let Some(prev) = di.decremented() {
                    let prev_open = *this
                        .get_or_read(prev, &this.mmap().load())
                        .unwrap()
                        .unwrap()
                        .close;
                    *ohlc.open = prev_open;
                    *ohlc.high = (*ohlc.high).max(prev_open);
                    *ohlc.low = (*ohlc.low).min(prev_open);
                }
                (di, ohlc)
            },
            exit,
        )?;

        self.dateindex_to_open_in_cents.compute_transform(
            starting_indexes.dateindex,
            &self.dateindex_to_ohlc_in_cents,
            |(di, ohlc, ..)| (di, ohlc.open),
            exit,
        )?;

        self.dateindex_to_high_in_cents.compute_transform(
            starting_indexes.dateindex,
            &self.dateindex_to_ohlc_in_cents,
            |(di, ohlc, ..)| (di, ohlc.high),
            exit,
        )?;

        self.dateindex_to_low_in_cents.compute_transform(
            starting_indexes.dateindex,
            &self.dateindex_to_ohlc_in_cents,
            |(di, ohlc, ..)| (di, ohlc.low),
            exit,
        )?;

        self.dateindex_to_close_in_cents.compute_transform(
            starting_indexes.dateindex,
            &self.dateindex_to_ohlc_in_cents,
            |(di, ohlc, ..)| (di, ohlc.close),
            exit,
        )?;

        self.dateindex_to_ohlc.compute_transform(
            starting_indexes.dateindex,
            &self.dateindex_to_ohlc_in_cents,
            |(di, ohlc, ..)| (di, OHLCDollars::from(ohlc)),
            exit,
        )?;

        self.timeindexes_to_close.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                v.compute_transform(
                    starting_indexes.dateindex,
                    &self.dateindex_to_ohlc,
                    |(di, ohlc, ..)| (di, ohlc.close),
                    exit,
                )
            },
        )?;

        self.timeindexes_to_high.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                v.compute_transform(
                    starting_indexes.dateindex,
                    &self.dateindex_to_ohlc,
                    |(di, ohlc, ..)| (di, ohlc.high),
                    exit,
                )
            },
        )?;

        self.timeindexes_to_low.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                v.compute_transform(
                    starting_indexes.dateindex,
                    &self.dateindex_to_ohlc,
                    |(di, ohlc, ..)| (di, ohlc.low),
                    exit,
                )
            },
        )?;

        self.timeindexes_to_open.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                v.compute_transform(
                    starting_indexes.dateindex,
                    &self.dateindex_to_ohlc,
                    |(di, ohlc, ..)| (di, ohlc.open),
                    exit,
                )
            },
        )?;

        self.chainindexes_to_close.compute(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                v.compute_transform(
                    starting_indexes.height,
                    &self.height_to_ohlc,
                    |(di, ohlc, ..)| (di, ohlc.close),
                    exit,
                )
            },
        )?;

        self.chainindexes_to_high.compute(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                v.compute_transform(
                    starting_indexes.height,
                    &self.height_to_ohlc,
                    |(di, ohlc, ..)| (di, ohlc.high),
                    exit,
                )
            },
        )?;

        self.chainindexes_to_low.compute(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                v.compute_transform(
                    starting_indexes.height,
                    &self.height_to_ohlc,
                    |(di, ohlc, ..)| (di, ohlc.low),
                    exit,
                )
            },
        )?;

        self.chainindexes_to_open.compute(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                v.compute_transform(
                    starting_indexes.height,
                    &self.height_to_ohlc,
                    |(di, ohlc, ..)| (di, ohlc.open),
                    exit,
                )
            },
        )?;

        let mut weekindex_first_iter = self.timeindexes_to_open.weekindex.unwrap_first().iter();
        let mut weekindex_max_iter = self.timeindexes_to_high.weekindex.unwrap_max().iter();
        let mut weekindex_min_iter = self.timeindexes_to_low.weekindex.unwrap_min().iter();
        self.weekindex_to_ohlc.compute_transform(
            starting_indexes.weekindex,
            self.timeindexes_to_close.weekindex.unwrap_last(),
            |(i, close, ..)| {
                (
                    i,
                    OHLCDollars {
                        open: weekindex_first_iter.unwrap_get_inner(i),
                        high: weekindex_max_iter.unwrap_get_inner(i),
                        low: weekindex_min_iter.unwrap_get_inner(i),
                        close,
                    },
                )
            },
            exit,
        )?;

        let mut difficultyepoch_first_iter = self
            .chainindexes_to_open
            .difficultyepoch
            .unwrap_first()
            .iter();
        let mut difficultyepoch_max_iter = self
            .chainindexes_to_high
            .difficultyepoch
            .unwrap_max()
            .iter();
        let mut difficultyepoch_min_iter =
            self.chainindexes_to_low.difficultyepoch.unwrap_min().iter();
        self.difficultyepoch_to_ohlc.compute_transform(
            starting_indexes.difficultyepoch,
            self.chainindexes_to_close.difficultyepoch.unwrap_last(),
            |(i, close, ..)| {
                (
                    i,
                    OHLCDollars {
                        open: difficultyepoch_first_iter.unwrap_get_inner(i),
                        high: difficultyepoch_max_iter.unwrap_get_inner(i),
                        low: difficultyepoch_min_iter.unwrap_get_inner(i),
                        close,
                    },
                )
            },
            exit,
        )?;

        let mut monthindex_first_iter = self.timeindexes_to_open.monthindex.unwrap_first().iter();
        let mut monthindex_max_iter = self.timeindexes_to_high.monthindex.unwrap_max().iter();
        let mut monthindex_min_iter = self.timeindexes_to_low.monthindex.unwrap_min().iter();
        self.monthindex_to_ohlc.compute_transform(
            starting_indexes.monthindex,
            self.timeindexes_to_close.monthindex.unwrap_last(),
            |(i, close, ..)| {
                (
                    i,
                    OHLCDollars {
                        open: monthindex_first_iter.unwrap_get_inner(i),
                        high: monthindex_max_iter.unwrap_get_inner(i),
                        low: monthindex_min_iter.unwrap_get_inner(i),
                        close,
                    },
                )
            },
            exit,
        )?;

        let mut quarterindex_first_iter =
            self.timeindexes_to_open.quarterindex.unwrap_first().iter();
        let mut quarterindex_max_iter = self.timeindexes_to_high.quarterindex.unwrap_max().iter();
        let mut quarterindex_min_iter = self.timeindexes_to_low.quarterindex.unwrap_min().iter();
        self.quarterindex_to_ohlc.compute_transform(
            starting_indexes.quarterindex,
            self.timeindexes_to_close.quarterindex.unwrap_last(),
            |(i, close, ..)| {
                (
                    i,
                    OHLCDollars {
                        open: quarterindex_first_iter.unwrap_get_inner(i),
                        high: quarterindex_max_iter.unwrap_get_inner(i),
                        low: quarterindex_min_iter.unwrap_get_inner(i),
                        close,
                    },
                )
            },
            exit,
        )?;

        let mut semesterindex_first_iter =
            self.timeindexes_to_open.semesterindex.unwrap_first().iter();
        let mut semesterindex_max_iter = self.timeindexes_to_high.semesterindex.unwrap_max().iter();
        let mut semesterindex_min_iter = self.timeindexes_to_low.semesterindex.unwrap_min().iter();
        self.semesterindex_to_ohlc.compute_transform(
            starting_indexes.semesterindex,
            self.timeindexes_to_close.semesterindex.unwrap_last(),
            |(i, close, ..)| {
                (
                    i,
                    OHLCDollars {
                        open: semesterindex_first_iter.unwrap_get_inner(i),
                        high: semesterindex_max_iter.unwrap_get_inner(i),
                        low: semesterindex_min_iter.unwrap_get_inner(i),
                        close,
                    },
                )
            },
            exit,
        )?;

        let mut yearindex_first_iter = self.timeindexes_to_open.yearindex.unwrap_first().iter();
        let mut yearindex_max_iter = self.timeindexes_to_high.yearindex.unwrap_max().iter();
        let mut yearindex_min_iter = self.timeindexes_to_low.yearindex.unwrap_min().iter();
        self.yearindex_to_ohlc.compute_transform(
            starting_indexes.yearindex,
            self.timeindexes_to_close.yearindex.unwrap_last(),
            |(i, close, ..)| {
                (
                    i,
                    OHLCDollars {
                        open: yearindex_first_iter.unwrap_get_inner(i),
                        high: yearindex_max_iter.unwrap_get_inner(i),
                        low: yearindex_min_iter.unwrap_get_inner(i),
                        close,
                    },
                )
            },
            exit,
        )?;

        // self.halvingepoch_to_ohlc
        //     .compute_transform(starting_indexes.halvingepoch, other, t, exit)?;

        let mut decadeindex_first_iter = self.timeindexes_to_open.decadeindex.unwrap_first().iter();
        let mut decadeindex_max_iter = self.timeindexes_to_high.decadeindex.unwrap_max().iter();
        let mut decadeindex_min_iter = self.timeindexes_to_low.decadeindex.unwrap_min().iter();
        self.decadeindex_to_ohlc.compute_transform(
            starting_indexes.decadeindex,
            self.timeindexes_to_close.decadeindex.unwrap_last(),
            |(i, close, ..)| {
                (
                    i,
                    OHLCDollars {
                        open: decadeindex_first_iter.unwrap_get_inner(i),
                        high: decadeindex_max_iter.unwrap_get_inner(i),
                        low: decadeindex_min_iter.unwrap_get_inner(i),
                        close,
                    },
                )
            },
            exit,
        )?;

        self.chainindexes_to_open_in_sats.compute(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                v.compute_transform(
                    starting_indexes.height,
                    &self.chainindexes_to_open.height,
                    |(i, open, ..)| (i, Open::new(Sats::ONE_BTC / *open)),
                    exit,
                )
            },
        )?;

        self.chainindexes_to_high_in_sats.compute(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                v.compute_transform(
                    starting_indexes.height,
                    &self.chainindexes_to_low.height,
                    |(i, low, ..)| (i, High::new(Sats::ONE_BTC / *low)),
                    exit,
                )
            },
        )?;

        self.chainindexes_to_low_in_sats.compute(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                v.compute_transform(
                    starting_indexes.height,
                    &self.chainindexes_to_high.height,
                    |(i, high, ..)| (i, Low::new(Sats::ONE_BTC / *high)),
                    exit,
                )
            },
        )?;

        self.chainindexes_to_close_in_sats.compute(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                v.compute_transform(
                    starting_indexes.height,
                    &self.chainindexes_to_close.height,
                    |(i, close, ..)| (i, Close::new(Sats::ONE_BTC / *close)),
                    exit,
                )
            },
        )?;

        self.timeindexes_to_open_in_sats.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                v.compute_transform(
                    starting_indexes.dateindex,
                    self.timeindexes_to_open.dateindex.as_ref().unwrap(),
                    |(i, open, ..)| (i, Open::new(Sats::ONE_BTC / *open)),
                    exit,
                )
            },
        )?;

        self.timeindexes_to_high_in_sats.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                v.compute_transform(
                    starting_indexes.dateindex,
                    self.timeindexes_to_low.dateindex.as_ref().unwrap(),
                    |(i, low, ..)| (i, High::new(Sats::ONE_BTC / *low)),
                    exit,
                )
            },
        )?;

        self.timeindexes_to_low_in_sats.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                v.compute_transform(
                    starting_indexes.dateindex,
                    self.timeindexes_to_high.dateindex.as_ref().unwrap(),
                    |(i, high, ..)| (i, Low::new(Sats::ONE_BTC / *high)),
                    exit,
                )
            },
        )?;

        self.timeindexes_to_close_in_sats.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                v.compute_transform(
                    starting_indexes.dateindex,
                    self.timeindexes_to_close.dateindex.as_ref().unwrap(),
                    |(i, close, ..)| (i, Close::new(Sats::ONE_BTC / *close)),
                    exit,
                )
            },
        )?;

        let mut height_first_iter = self.chainindexes_to_open_in_sats.height.iter();
        let mut height_max_iter = self.chainindexes_to_high_in_sats.height.iter();
        let mut height_min_iter = self.chainindexes_to_low_in_sats.height.iter();
        self.height_to_ohlc_in_sats.compute_transform(
            starting_indexes.height,
            &self.chainindexes_to_close_in_sats.height,
            |(i, close, ..)| {
                (
                    i,
                    OHLCSats {
                        open: height_first_iter.unwrap_get_inner(i),
                        high: height_max_iter.unwrap_get_inner(i),
                        low: height_min_iter.unwrap_get_inner(i),
                        close,
                    },
                )
            },
            exit,
        )?;

        let mut dateindex_first_iter = self
            .timeindexes_to_open_in_sats
            .dateindex
            .as_ref()
            .unwrap()
            .iter();
        let mut dateindex_max_iter = self
            .timeindexes_to_high_in_sats
            .dateindex
            .as_ref()
            .unwrap()
            .iter();
        let mut dateindex_min_iter = self
            .timeindexes_to_low_in_sats
            .dateindex
            .as_ref()
            .unwrap()
            .iter();
        self.dateindex_to_ohlc_in_sats.compute_transform(
            starting_indexes.dateindex,
            self.timeindexes_to_close_in_sats
                .dateindex
                .as_ref()
                .unwrap(),
            |(i, close, ..)| {
                (
                    i,
                    OHLCSats {
                        open: dateindex_first_iter.unwrap_get_inner(i),
                        high: dateindex_max_iter.unwrap_get_inner(i),
                        low: dateindex_min_iter.unwrap_get_inner(i),
                        close,
                    },
                )
            },
            exit,
        )?;

        let mut weekindex_first_iter = self
            .timeindexes_to_open_in_sats
            .weekindex
            .unwrap_first()
            .iter();
        let mut weekindex_max_iter = self
            .timeindexes_to_high_in_sats
            .weekindex
            .unwrap_max()
            .iter();
        let mut weekindex_min_iter = self
            .timeindexes_to_low_in_sats
            .weekindex
            .unwrap_min()
            .iter();
        self.weekindex_to_ohlc_in_sats.compute_transform(
            starting_indexes.weekindex,
            self.timeindexes_to_close_in_sats.weekindex.unwrap_last(),
            |(i, close, ..)| {
                (
                    i,
                    OHLCSats {
                        open: weekindex_first_iter.unwrap_get_inner(i),
                        high: weekindex_max_iter.unwrap_get_inner(i),
                        low: weekindex_min_iter.unwrap_get_inner(i),
                        close,
                    },
                )
            },
            exit,
        )?;

        let mut difficultyepoch_first_iter = self
            .chainindexes_to_open_in_sats
            .difficultyepoch
            .unwrap_first()
            .iter();
        let mut difficultyepoch_max_iter = self
            .chainindexes_to_high_in_sats
            .difficultyepoch
            .unwrap_max()
            .iter();
        let mut difficultyepoch_min_iter = self
            .chainindexes_to_low_in_sats
            .difficultyepoch
            .unwrap_min()
            .iter();
        self.difficultyepoch_to_ohlc_in_sats.compute_transform(
            starting_indexes.difficultyepoch,
            self.chainindexes_to_close_in_sats
                .difficultyepoch
                .unwrap_last(),
            |(i, close, ..)| {
                (
                    i,
                    OHLCSats {
                        open: difficultyepoch_first_iter.unwrap_get_inner(i),
                        high: difficultyepoch_max_iter.unwrap_get_inner(i),
                        low: difficultyepoch_min_iter.unwrap_get_inner(i),
                        close,
                    },
                )
            },
            exit,
        )?;

        let mut monthindex_first_iter = self
            .timeindexes_to_open_in_sats
            .monthindex
            .unwrap_first()
            .iter();
        let mut monthindex_max_iter = self
            .timeindexes_to_high_in_sats
            .monthindex
            .unwrap_max()
            .iter();
        let mut monthindex_min_iter = self
            .timeindexes_to_low_in_sats
            .monthindex
            .unwrap_min()
            .iter();
        self.monthindex_to_ohlc_in_sats.compute_transform(
            starting_indexes.monthindex,
            self.timeindexes_to_close_in_sats.monthindex.unwrap_last(),
            |(i, close, ..)| {
                (
                    i,
                    OHLCSats {
                        open: monthindex_first_iter.unwrap_get_inner(i),
                        high: monthindex_max_iter.unwrap_get_inner(i),
                        low: monthindex_min_iter.unwrap_get_inner(i),
                        close,
                    },
                )
            },
            exit,
        )?;

        let mut quarterindex_first_iter = self
            .timeindexes_to_open_in_sats
            .quarterindex
            .unwrap_first()
            .iter();
        let mut quarterindex_max_iter = self
            .timeindexes_to_high_in_sats
            .quarterindex
            .unwrap_max()
            .iter();
        let mut quarterindex_min_iter = self
            .timeindexes_to_low_in_sats
            .quarterindex
            .unwrap_min()
            .iter();
        self.quarterindex_to_ohlc_in_sats.compute_transform(
            starting_indexes.quarterindex,
            self.timeindexes_to_close_in_sats.quarterindex.unwrap_last(),
            |(i, close, ..)| {
                (
                    i,
                    OHLCSats {
                        open: quarterindex_first_iter.unwrap_get_inner(i),
                        high: quarterindex_max_iter.unwrap_get_inner(i),
                        low: quarterindex_min_iter.unwrap_get_inner(i),
                        close,
                    },
                )
            },
            exit,
        )?;

        let mut semesterindex_first_iter = self
            .timeindexes_to_open_in_sats
            .semesterindex
            .unwrap_first()
            .iter();
        let mut semesterindex_max_iter = self
            .timeindexes_to_high_in_sats
            .semesterindex
            .unwrap_max()
            .iter();
        let mut semesterindex_min_iter = self
            .timeindexes_to_low_in_sats
            .semesterindex
            .unwrap_min()
            .iter();
        self.semesterindex_to_ohlc_in_sats.compute_transform(
            starting_indexes.semesterindex,
            self.timeindexes_to_close_in_sats
                .semesterindex
                .unwrap_last(),
            |(i, close, ..)| {
                (
                    i,
                    OHLCSats {
                        open: semesterindex_first_iter.unwrap_get_inner(i),
                        high: semesterindex_max_iter.unwrap_get_inner(i),
                        low: semesterindex_min_iter.unwrap_get_inner(i),
                        close,
                    },
                )
            },
            exit,
        )?;

        let mut yearindex_first_iter = self
            .timeindexes_to_open_in_sats
            .yearindex
            .unwrap_first()
            .iter();
        let mut yearindex_max_iter = self
            .timeindexes_to_high_in_sats
            .yearindex
            .unwrap_max()
            .iter();
        let mut yearindex_min_iter = self
            .timeindexes_to_low_in_sats
            .yearindex
            .unwrap_min()
            .iter();
        self.yearindex_to_ohlc_in_sats.compute_transform(
            starting_indexes.yearindex,
            self.timeindexes_to_close_in_sats.yearindex.unwrap_last(),
            |(i, close, ..)| {
                (
                    i,
                    OHLCSats {
                        open: yearindex_first_iter.unwrap_get_inner(i),
                        high: yearindex_max_iter.unwrap_get_inner(i),
                        low: yearindex_min_iter.unwrap_get_inner(i),
                        close,
                    },
                )
            },
            exit,
        )?;

        // self.halvingepoch_to_ohlc
        //     _in_sats.compute_transform(starting_indexes.halvingepoch, other, t, exit)?;

        let mut decadeindex_first_iter = self
            .timeindexes_to_open_in_sats
            .decadeindex
            .unwrap_first()
            .iter();
        let mut decadeindex_max_iter = self
            .timeindexes_to_high_in_sats
            .decadeindex
            .unwrap_max()
            .iter();
        let mut decadeindex_min_iter = self
            .timeindexes_to_low_in_sats
            .decadeindex
            .unwrap_min()
            .iter();
        self.decadeindex_to_ohlc_in_sats.compute_transform(
            starting_indexes.decadeindex,
            self.timeindexes_to_close_in_sats.decadeindex.unwrap_last(),
            |(i, close, ..)| {
                (
                    i,
                    OHLCSats {
                        open: decadeindex_first_iter.unwrap_get_inner(i),
                        high: decadeindex_max_iter.unwrap_get_inner(i),
                        low: decadeindex_min_iter.unwrap_get_inner(i),
                        close,
                    },
                )
            },
            exit,
        )?;

        Ok(())
    }

    pub fn vecs(&self) -> Vec<&dyn AnyCollectableVec> {
        vec![
            vec![
                &self.dateindex_to_close_in_cents as &dyn AnyCollectableVec,
                &self.dateindex_to_high_in_cents,
                &self.dateindex_to_low_in_cents,
                &self.dateindex_to_ohlc,
                &self.dateindex_to_ohlc_in_cents,
                &self.dateindex_to_open_in_cents,
                &self.height_to_close_in_cents,
                &self.height_to_high_in_cents,
                &self.height_to_low_in_cents,
                &self.height_to_ohlc,
                &self.height_to_ohlc_in_cents,
                &self.height_to_open_in_cents,
                &self.weekindex_to_ohlc,
                &self.difficultyepoch_to_ohlc,
                &self.monthindex_to_ohlc,
                &self.quarterindex_to_ohlc,
                &self.semesterindex_to_ohlc,
                &self.yearindex_to_ohlc,
                // &self.halvingepoch_to_ohlc,
                &self.decadeindex_to_ohlc,
                &self.height_to_ohlc_in_sats,
                &self.dateindex_to_ohlc_in_sats,
                &self.weekindex_to_ohlc_in_sats,
                &self.difficultyepoch_to_ohlc_in_sats,
                &self.monthindex_to_ohlc_in_sats,
                &self.quarterindex_to_ohlc_in_sats,
                &self.semesterindex_to_ohlc_in_sats,
                &self.yearindex_to_ohlc_in_sats,
                // &self.halvingepoch_to_ohlc_in_sats,
                &self.decadeindex_to_ohlc_in_sats,
            ],
            self.timeindexes_to_close.vecs(),
            self.timeindexes_to_high.vecs(),
            self.timeindexes_to_low.vecs(),
            self.timeindexes_to_open.vecs(),
            self.chainindexes_to_close.vecs(),
            self.chainindexes_to_high.vecs(),
            self.chainindexes_to_low.vecs(),
            self.chainindexes_to_open.vecs(),
            self.timeindexes_to_close_in_sats.vecs(),
            self.timeindexes_to_high_in_sats.vecs(),
            self.timeindexes_to_low_in_sats.vecs(),
            self.timeindexes_to_open_in_sats.vecs(),
            self.chainindexes_to_close_in_sats.vecs(),
            self.chainindexes_to_high_in_sats.vecs(),
            self.chainindexes_to_low_in_sats.vecs(),
            self.chainindexes_to_open_in_sats.vecs(),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
    }
}
