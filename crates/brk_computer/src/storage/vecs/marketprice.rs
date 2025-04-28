use std::{fs, path::Path};

use brk_core::{
    Cents, Close, DateIndex, DecadeIndex, DifficultyEpoch, Dollars, Height, High, Low, MonthIndex,
    OHLCCents, OHLCDollars, OHLCSats, Open, QuarterIndex, Sats, WeekIndex, YearIndex,
};
use brk_exit::Exit;
use brk_fetcher::Fetcher;
use brk_indexer::Indexer;
use brk_vec::{Compressed, Version};

use super::{
    EagerVec, Indexes,
    grouped::{
        ComputedVecsFromDateindex, ComputedVecsFromHeightStrict, StorableVecGeneatorOptions,
    },
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
    pub timeindexes_to_close: ComputedVecsFromDateindex<Close<Dollars>>,
    pub timeindexes_to_high: ComputedVecsFromDateindex<High<Dollars>>,
    pub timeindexes_to_low: ComputedVecsFromDateindex<Low<Dollars>>,
    pub timeindexes_to_open: ComputedVecsFromDateindex<Open<Dollars>>,
    pub timeindexes_to_open_in_sats: ComputedVecsFromDateindex<Open<Sats>>,
    pub timeindexes_to_high_in_sats: ComputedVecsFromDateindex<High<Sats>>,
    pub timeindexes_to_low_in_sats: ComputedVecsFromDateindex<Low<Sats>>,
    pub timeindexes_to_close_in_sats: ComputedVecsFromDateindex<Close<Sats>>,
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
    pub yearindex_to_ohlc: EagerVec<YearIndex, OHLCDollars>,
    pub yearindex_to_ohlc_in_sats: EagerVec<YearIndex, OHLCSats>,
    // pub halvingepoch_to_ohlc: StorableVec<Halvingepoch, OHLCDollars>,
    // pub halvingepoch_to_ohlc_in_sats: StorableVec<Halvingepoch, OHLCSats>,
    pub decadeindex_to_ohlc: EagerVec<DecadeIndex, OHLCDollars>,
    pub decadeindex_to_ohlc_in_sats: EagerVec<DecadeIndex, OHLCSats>,
}

const VERSION: Version = Version::ZERO;
const VERSION_IN_SATS: Version = Version::ONE;

impl Vecs {
    pub fn forced_import(path: &Path, compressed: Compressed) -> color_eyre::Result<Self> {
        fs::create_dir_all(path)?;

        let mut fetched_path = path.to_owned();
        fetched_path.pop();
        fetched_path = fetched_path.join("fetched");

        Ok(Self {
            dateindex_to_ohlc_in_cents: EagerVec::forced_import(
                &fetched_path.join("dateindex_to_ohlc_in_cents"),
                Version::ZERO,
                compressed,
            )?,
            dateindex_to_ohlc: EagerVec::forced_import(
                &path.join("dateindex_to_ohlc"),
                Version::ZERO,
                compressed,
            )?,
            dateindex_to_ohlc_in_sats: EagerVec::forced_import(
                &path.join("dateindex_to_ohlc_in_sats"),
                VERSION + VERSION_IN_SATS + Version::ZERO,
                compressed,
            )?,
            dateindex_to_close_in_cents: EagerVec::forced_import(
                &path.join("dateindex_to_close_in_cents"),
                Version::ZERO,
                compressed,
            )?,
            dateindex_to_high_in_cents: EagerVec::forced_import(
                &path.join("dateindex_to_high_in_cents"),
                Version::ZERO,
                compressed,
            )?,
            dateindex_to_low_in_cents: EagerVec::forced_import(
                &path.join("dateindex_to_low_in_cents"),
                Version::ZERO,
                compressed,
            )?,
            dateindex_to_open_in_cents: EagerVec::forced_import(
                &path.join("dateindex_to_open_in_cents"),
                Version::ZERO,
                compressed,
            )?,
            height_to_ohlc_in_cents: EagerVec::forced_import(
                &fetched_path.join("height_to_ohlc_in_cents"),
                Version::ZERO,
                compressed,
            )?,
            height_to_ohlc: EagerVec::forced_import(
                &path.join("height_to_ohlc"),
                Version::ZERO,
                compressed,
            )?,
            height_to_ohlc_in_sats: EagerVec::forced_import(
                &path.join("height_to_ohlc_in_sats"),
                VERSION + VERSION_IN_SATS + Version::ZERO,
                compressed,
            )?,
            height_to_close_in_cents: EagerVec::forced_import(
                &path.join("height_to_close_in_cents"),
                Version::ZERO,
                compressed,
            )?,
            height_to_high_in_cents: EagerVec::forced_import(
                &path.join("height_to_high_in_cents"),
                Version::ZERO,
                compressed,
            )?,
            height_to_low_in_cents: EagerVec::forced_import(
                &path.join("height_to_low_in_cents"),
                Version::ZERO,
                compressed,
            )?,
            height_to_open_in_cents: EagerVec::forced_import(
                &path.join("height_to_open_in_cents"),
                Version::ZERO,
                compressed,
            )?,
            timeindexes_to_open: ComputedVecsFromDateindex::forced_import(
                path,
                "open",
                Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_first(),
            )?,
            timeindexes_to_high: ComputedVecsFromDateindex::forced_import(
                path,
                "high",
                Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_max(),
            )?,
            timeindexes_to_low: ComputedVecsFromDateindex::forced_import(
                path,
                "low",
                Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_min(),
            )?,
            timeindexes_to_close: ComputedVecsFromDateindex::forced_import(
                path,
                "close",
                Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            timeindexes_to_open_in_sats: ComputedVecsFromDateindex::forced_import(
                path,
                "open_in_sats",
                VERSION + VERSION_IN_SATS + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_first(),
            )?,
            timeindexes_to_high_in_sats: ComputedVecsFromDateindex::forced_import(
                path,
                "high_in_sats",
                VERSION + VERSION_IN_SATS + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_max(),
            )?,
            timeindexes_to_low_in_sats: ComputedVecsFromDateindex::forced_import(
                path,
                "low_in_sats",
                VERSION + VERSION_IN_SATS + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_min(),
            )?,
            timeindexes_to_close_in_sats: ComputedVecsFromDateindex::forced_import(
                path,
                "close_in_sats",
                VERSION + VERSION_IN_SATS + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            chainindexes_to_open: ComputedVecsFromHeightStrict::forced_import(
                path,
                "open",
                Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_first(),
            )?,
            chainindexes_to_high: ComputedVecsFromHeightStrict::forced_import(
                path,
                "high",
                Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_max(),
            )?,
            chainindexes_to_low: ComputedVecsFromHeightStrict::forced_import(
                path,
                "low",
                Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_min(),
            )?,
            chainindexes_to_close: ComputedVecsFromHeightStrict::forced_import(
                path,
                "close",
                Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            chainindexes_to_open_in_sats: ComputedVecsFromHeightStrict::forced_import(
                path,
                "open_in_sats",
                VERSION + VERSION_IN_SATS + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_first(),
            )?,
            chainindexes_to_high_in_sats: ComputedVecsFromHeightStrict::forced_import(
                path,
                "high_in_sats",
                VERSION + VERSION_IN_SATS + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_max(),
            )?,
            chainindexes_to_low_in_sats: ComputedVecsFromHeightStrict::forced_import(
                path,
                "low_in_sats",
                VERSION + VERSION_IN_SATS + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_min(),
            )?,
            chainindexes_to_close_in_sats: ComputedVecsFromHeightStrict::forced_import(
                path,
                "close_in_sats",
                VERSION + VERSION_IN_SATS + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            weekindex_to_ohlc: EagerVec::forced_import(
                &path.join("weekindex_to_ohlc"),
                Version::ZERO,
                compressed,
            )?,
            weekindex_to_ohlc_in_sats: EagerVec::forced_import(
                &path.join("weekindex_to_ohlc_in_sats"),
                VERSION + VERSION_IN_SATS + Version::ZERO,
                compressed,
            )?,
            difficultyepoch_to_ohlc: EagerVec::forced_import(
                &path.join("difficultyepoch_to_ohlc"),
                Version::ZERO,
                compressed,
            )?,
            difficultyepoch_to_ohlc_in_sats: EagerVec::forced_import(
                &path.join("difficultyepoch_to_ohlc_in_sats"),
                VERSION + VERSION_IN_SATS + Version::ZERO,
                compressed,
            )?,
            monthindex_to_ohlc: EagerVec::forced_import(
                &path.join("monthindex_to_ohlc"),
                Version::ZERO,
                compressed,
            )?,
            monthindex_to_ohlc_in_sats: EagerVec::forced_import(
                &path.join("monthindex_to_ohlc_in_sats"),
                VERSION + VERSION_IN_SATS + Version::ZERO,
                compressed,
            )?,
            quarterindex_to_ohlc: EagerVec::forced_import(
                &path.join("quarterindex_to_ohlc"),
                Version::ZERO,
                compressed,
            )?,
            quarterindex_to_ohlc_in_sats: EagerVec::forced_import(
                &path.join("quarterindex_to_ohlc_in_sats"),
                VERSION + VERSION_IN_SATS + Version::ZERO,
                compressed,
            )?,
            yearindex_to_ohlc: EagerVec::forced_import(
                &path.join("yearindex_to_ohlc"),
                Version::ZERO,
                compressed,
            )?,
            yearindex_to_ohlc_in_sats: EagerVec::forced_import(
                &path.join("yearindex_to_ohlc_in_sats"),
                VERSION + VERSION_IN_SATS + Version::ZERO,
                compressed,
            )?,
            // halvingepoch_to_ohlc: StorableVec::forced_import(&path.join("halvingepoch_to_ohlc"), Version::ZERO, compressed)?,
            decadeindex_to_ohlc: EagerVec::forced_import(
                &path.join("decadeindex_to_ohlc"),
                Version::ZERO,
                compressed,
            )?,
            decadeindex_to_ohlc_in_sats: EagerVec::forced_import(
                &path.join("decadeindex_to_ohlc_in_sats"),
                VERSION + VERSION_IN_SATS + Version::ZERO,
                compressed,
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
        let indexer_vecs = indexer.vecs();

        let mut height_to_timestamp_iter = indexer_vecs.height_to_timestamp.iter();
        self.height_to_ohlc_in_cents.compute_transform(
            starting_indexes.height,
            indexer_vecs.height_to_timestamp.vec(),
            |(h, t)| {
                let ohlc = fetcher
                    .get_height(
                        h,
                        t,
                        h.decremented().map(|prev_h| {
                            height_to_timestamp_iter.get(prev_h).unwrap().1.into_inner()
                        }),
                    )
                    .unwrap();
                (h, ohlc)
            },
            exit,
        )?;

        self.height_to_open_in_cents.compute_transform(
            starting_indexes.height,
            self.height_to_ohlc_in_cents.vec(),
            |(di, ohlc, ..)| (di, ohlc.open),
            exit,
        )?;

        self.height_to_high_in_cents.compute_transform(
            starting_indexes.height,
            self.height_to_ohlc_in_cents.vec(),
            |(di, ohlc, ..)| (di, ohlc.high),
            exit,
        )?;

        self.height_to_low_in_cents.compute_transform(
            starting_indexes.height,
            self.height_to_ohlc_in_cents.vec(),
            |(di, ohlc, ..)| (di, ohlc.low),
            exit,
        )?;

        self.height_to_close_in_cents.compute_transform(
            starting_indexes.height,
            self.height_to_ohlc_in_cents.vec(),
            |(di, ohlc, ..)| (di, ohlc.close),
            exit,
        )?;

        self.height_to_ohlc.compute_transform(
            starting_indexes.height,
            self.height_to_ohlc_in_cents.vec(),
            |(di, ohlc, ..)| (di, OHLCDollars::from(ohlc)),
            exit,
        )?;

        self.dateindex_to_ohlc_in_cents.compute_transform(
            starting_indexes.dateindex,
            indexes.dateindex_to_date.vec(),
            |(di, d, ..)| {
                let ohlc = fetcher.get_date(d).unwrap();
                (di, ohlc)
            },
            exit,
        )?;

        self.dateindex_to_open_in_cents.compute_transform(
            starting_indexes.dateindex,
            self.dateindex_to_ohlc_in_cents.vec(),
            |(di, ohlc, ..)| (di, ohlc.open),
            exit,
        )?;

        self.dateindex_to_high_in_cents.compute_transform(
            starting_indexes.dateindex,
            self.dateindex_to_ohlc_in_cents.vec(),
            |(di, ohlc, ..)| (di, ohlc.high),
            exit,
        )?;

        self.dateindex_to_low_in_cents.compute_transform(
            starting_indexes.dateindex,
            self.dateindex_to_ohlc_in_cents.vec(),
            |(di, ohlc, ..)| (di, ohlc.low),
            exit,
        )?;

        self.dateindex_to_close_in_cents.compute_transform(
            starting_indexes.dateindex,
            self.dateindex_to_ohlc_in_cents.vec(),
            |(di, ohlc, ..)| (di, ohlc.close),
            exit,
        )?;

        self.dateindex_to_ohlc.compute_transform(
            starting_indexes.dateindex,
            self.dateindex_to_ohlc_in_cents.vec(),
            |(di, ohlc, ..)| (di, OHLCDollars::from(ohlc)),
            exit,
        )?;

        self.timeindexes_to_close.compute(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                v.compute_transform(
                    starting_indexes.dateindex,
                    self.dateindex_to_ohlc.vec(),
                    |(di, ohlc, ..)| (di, ohlc.close),
                    exit,
                )
            },
        )?;

        self.timeindexes_to_high.compute(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                v.compute_transform(
                    starting_indexes.dateindex,
                    self.dateindex_to_ohlc.vec(),
                    |(di, ohlc, ..)| (di, ohlc.high),
                    exit,
                )
            },
        )?;

        self.timeindexes_to_low.compute(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                v.compute_transform(
                    starting_indexes.dateindex,
                    self.dateindex_to_ohlc.vec(),
                    |(di, ohlc, ..)| (di, ohlc.low),
                    exit,
                )
            },
        )?;

        self.timeindexes_to_open.compute(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                v.compute_transform(
                    starting_indexes.dateindex,
                    self.dateindex_to_ohlc.vec(),
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
                    self.height_to_ohlc.vec(),
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
                    self.height_to_ohlc.vec(),
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
                    self.height_to_ohlc.vec(),
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
                    self.height_to_ohlc.vec(),
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
            self.timeindexes_to_close.weekindex.unwrap_last().vec(),
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
            self.chainindexes_to_close
                .difficultyepoch
                .unwrap_last()
                .vec(),
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
            self.timeindexes_to_close.monthindex.unwrap_last().vec(),
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
            self.timeindexes_to_close.quarterindex.unwrap_last().vec(),
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

        let mut yearindex_first_iter = self.timeindexes_to_open.yearindex.unwrap_first().iter();
        let mut yearindex_max_iter = self.timeindexes_to_high.yearindex.unwrap_max().iter();
        let mut yearindex_min_iter = self.timeindexes_to_low.yearindex.unwrap_min().iter();
        self.yearindex_to_ohlc.compute_transform(
            starting_indexes.yearindex,
            self.timeindexes_to_close.yearindex.unwrap_last().vec(),
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
            self.timeindexes_to_close.decadeindex.unwrap_last().vec(),
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
                    self.chainindexes_to_open.height.vec(),
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
                    self.chainindexes_to_low.height.vec(),
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
                    self.chainindexes_to_high.height.vec(),
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
                    self.chainindexes_to_close.height.vec(),
                    |(i, close, ..)| (i, Close::new(Sats::ONE_BTC / *close)),
                    exit,
                )
            },
        )?;

        self.timeindexes_to_open_in_sats.compute(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                v.compute_transform(
                    starting_indexes.dateindex,
                    self.timeindexes_to_open.dateindex.vec(),
                    |(i, open, ..)| (i, Open::new(Sats::ONE_BTC / *open)),
                    exit,
                )
            },
        )?;

        self.timeindexes_to_high_in_sats.compute(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                v.compute_transform(
                    starting_indexes.dateindex,
                    self.timeindexes_to_low.dateindex.vec(),
                    |(i, low, ..)| (i, High::new(Sats::ONE_BTC / *low)),
                    exit,
                )
            },
        )?;

        self.timeindexes_to_low_in_sats.compute(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                v.compute_transform(
                    starting_indexes.dateindex,
                    self.timeindexes_to_high.dateindex.vec(),
                    |(i, high, ..)| (i, Low::new(Sats::ONE_BTC / *high)),
                    exit,
                )
            },
        )?;

        self.timeindexes_to_close_in_sats.compute(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                v.compute_transform(
                    starting_indexes.dateindex,
                    self.timeindexes_to_close.dateindex.vec(),
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
            self.chainindexes_to_close_in_sats.height.vec(),
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

        let mut dateindex_first_iter = self.timeindexes_to_open_in_sats.dateindex.iter();
        let mut dateindex_max_iter = self.timeindexes_to_high_in_sats.dateindex.iter();
        let mut dateindex_min_iter = self.timeindexes_to_low_in_sats.dateindex.iter();
        self.dateindex_to_ohlc_in_sats.compute_transform(
            starting_indexes.dateindex,
            self.timeindexes_to_close_in_sats.dateindex.vec(),
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
            self.timeindexes_to_close_in_sats
                .weekindex
                .unwrap_last()
                .vec(),
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
                .unwrap_last()
                .vec(),
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
            self.timeindexes_to_close_in_sats
                .monthindex
                .unwrap_last()
                .vec(),
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
            self.timeindexes_to_close_in_sats
                .quarterindex
                .unwrap_last()
                .vec(),
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
            self.timeindexes_to_close_in_sats
                .yearindex
                .unwrap_last()
                .vec(),
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
            self.timeindexes_to_close_in_sats
                .decadeindex
                .unwrap_last()
                .vec(),
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

    pub fn as_any_vecs(&self) -> Vec<&dyn brk_vec::AnyStoredVec> {
        vec![
            vec![
                self.dateindex_to_close_in_cents.any_vec(),
                self.dateindex_to_high_in_cents.any_vec(),
                self.dateindex_to_low_in_cents.any_vec(),
                self.dateindex_to_ohlc.any_vec(),
                self.dateindex_to_ohlc_in_cents.any_vec(),
                self.dateindex_to_open_in_cents.any_vec(),
                self.height_to_close_in_cents.any_vec(),
                self.height_to_high_in_cents.any_vec(),
                self.height_to_low_in_cents.any_vec(),
                self.height_to_ohlc.any_vec(),
                self.height_to_ohlc_in_cents.any_vec(),
                self.height_to_open_in_cents.any_vec(),
                self.weekindex_to_ohlc.any_vec(),
                self.difficultyepoch_to_ohlc.any_vec(),
                self.monthindex_to_ohlc.any_vec(),
                self.quarterindex_to_ohlc.any_vec(),
                self.yearindex_to_ohlc.any_vec(),
                // self.halvingepoch_to_ohlc.any_vec(),
                self.decadeindex_to_ohlc.any_vec(),
                self.height_to_ohlc_in_sats.any_vec(),
                self.dateindex_to_ohlc_in_sats.any_vec(),
                self.weekindex_to_ohlc_in_sats.any_vec(),
                self.difficultyepoch_to_ohlc_in_sats.any_vec(),
                self.monthindex_to_ohlc_in_sats.any_vec(),
                self.quarterindex_to_ohlc_in_sats.any_vec(),
                self.yearindex_to_ohlc_in_sats.any_vec(),
                // self.halvingepoch_to_ohlc_in_sats.any_vec(),
                self.decadeindex_to_ohlc_in_sats.any_vec(),
            ],
            self.timeindexes_to_close.any_vecs(),
            self.timeindexes_to_high.any_vecs(),
            self.timeindexes_to_low.any_vecs(),
            self.timeindexes_to_open.any_vecs(),
            self.chainindexes_to_close.any_vecs(),
            self.chainindexes_to_high.any_vecs(),
            self.chainindexes_to_low.any_vecs(),
            self.chainindexes_to_open.any_vecs(),
            self.timeindexes_to_close_in_sats.any_vecs(),
            self.timeindexes_to_high_in_sats.any_vecs(),
            self.timeindexes_to_low_in_sats.any_vecs(),
            self.timeindexes_to_open_in_sats.any_vecs(),
            self.chainindexes_to_close_in_sats.any_vecs(),
            self.chainindexes_to_high_in_sats.any_vecs(),
            self.chainindexes_to_low_in_sats.any_vecs(),
            self.chainindexes_to_open_in_sats.any_vecs(),
        ]
        .concat()
    }
}
