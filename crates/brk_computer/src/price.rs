use std::{path::Path, sync::Arc};

use brk_error::Result;
use brk_indexer::Indexer;
use brk_structs::{
    Cents, Close, DateIndex, DecadeIndex, DifficultyEpoch, Dollars, Height, High, Low, MonthIndex,
    OHLCDollars, OHLCSats, Open, QuarterIndex, Sats, SemesterIndex, Version, WeekIndex, YearIndex,
};
use brk_vecs::{
    AnyCollectableVec, AnyIterableVec, AnyStoredVec, Computation, EagerVec, Exit, File, Format,
    GenericStoredVec, RawVec,
};

use crate::{fetched, grouped::Source};

use super::{
    Indexes,
    grouped::{ComputedVecsFromDateIndex, ComputedVecsFromHeightStrict, VecBuilderOptions},
    indexes,
};

#[derive(Clone)]
pub struct Vecs {
    file: Arc<File>,

    pub dateindex_to_close_in_cents: EagerVec<DateIndex, Close<Cents>>,
    pub dateindex_to_high_in_cents: EagerVec<DateIndex, High<Cents>>,
    pub dateindex_to_low_in_cents: EagerVec<DateIndex, Low<Cents>>,
    pub dateindex_to_ohlc: RawVec<DateIndex, OHLCDollars>,
    pub dateindex_to_ohlc_in_sats: RawVec<DateIndex, OHLCSats>,
    pub dateindex_to_open_in_cents: EagerVec<DateIndex, Open<Cents>>,
    pub height_to_close_in_cents: EagerVec<Height, Close<Cents>>,
    pub height_to_high_in_cents: EagerVec<Height, High<Cents>>,
    pub height_to_low_in_cents: EagerVec<Height, Low<Cents>>,
    pub height_to_ohlc: RawVec<Height, OHLCDollars>,
    pub height_to_ohlc_in_sats: RawVec<Height, OHLCSats>,
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
    pub weekindex_to_ohlc: RawVec<WeekIndex, OHLCDollars>,
    pub weekindex_to_ohlc_in_sats: RawVec<WeekIndex, OHLCSats>,
    pub difficultyepoch_to_ohlc: RawVec<DifficultyEpoch, OHLCDollars>,
    pub difficultyepoch_to_ohlc_in_sats: RawVec<DifficultyEpoch, OHLCSats>,
    pub monthindex_to_ohlc: RawVec<MonthIndex, OHLCDollars>,
    pub monthindex_to_ohlc_in_sats: RawVec<MonthIndex, OHLCSats>,
    pub quarterindex_to_ohlc: RawVec<QuarterIndex, OHLCDollars>,
    pub quarterindex_to_ohlc_in_sats: RawVec<QuarterIndex, OHLCSats>,
    pub semesterindex_to_ohlc: RawVec<SemesterIndex, OHLCDollars>,
    pub semesterindex_to_ohlc_in_sats: RawVec<SemesterIndex, OHLCSats>,
    pub yearindex_to_ohlc: RawVec<YearIndex, OHLCDollars>,
    pub yearindex_to_ohlc_in_sats: RawVec<YearIndex, OHLCSats>,
    // pub halvingepoch_to_ohlc: StorableVec<Halvingepoch, OHLCDollars>,
    // pub halvingepoch_to_ohlc_in_sats: StorableVec<Halvingepoch, OHLCSats>,
    pub decadeindex_to_ohlc: RawVec<DecadeIndex, OHLCDollars>,
    pub decadeindex_to_ohlc_in_sats: RawVec<DecadeIndex, OHLCSats>,
}

const VERSION: Version = Version::ZERO;
const VERSION_IN_SATS: Version = Version::ZERO;

impl Vecs {
    pub fn forced_import(
        parent: &Path,
        version: Version,
        computation: Computation,
        format: Format,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let file = Arc::new(File::open(&parent.join("price"))?);

        Ok(Self {
            dateindex_to_ohlc: RawVec::forced_import(
                &file,
                "ohlc",
                version + VERSION + Version::ZERO,
            )?,
            dateindex_to_ohlc_in_sats: RawVec::forced_import(
                &file,
                "ohlc_in_sats",
                version + VERSION + VERSION_IN_SATS + Version::ZERO,
            )?,
            dateindex_to_close_in_cents: EagerVec::forced_import(
                &file,
                "close_in_cents",
                version + VERSION + Version::ZERO,
                format,
            )?,
            dateindex_to_high_in_cents: EagerVec::forced_import(
                &file,
                "high_in_cents",
                version + VERSION + Version::ZERO,
                format,
            )?,
            dateindex_to_low_in_cents: EagerVec::forced_import(
                &file,
                "low_in_cents",
                version + VERSION + Version::ZERO,
                format,
            )?,
            dateindex_to_open_in_cents: EagerVec::forced_import(
                &file,
                "open_in_cents",
                version + VERSION + Version::ZERO,
                format,
            )?,
            height_to_ohlc: RawVec::forced_import(
                &file,
                "ohlc",
                version + VERSION + Version::ZERO,
            )?,
            height_to_ohlc_in_sats: RawVec::forced_import(
                &file,
                "ohlc_in_sats",
                version + VERSION + VERSION_IN_SATS + Version::ZERO,
            )?,
            height_to_close_in_cents: EagerVec::forced_import(
                &file,
                "close_in_cents",
                version + VERSION + Version::ZERO,
                format,
            )?,
            height_to_high_in_cents: EagerVec::forced_import(
                &file,
                "high_in_cents",
                version + VERSION + Version::ZERO,
                format,
            )?,
            height_to_low_in_cents: EagerVec::forced_import(
                &file,
                "low_in_cents",
                version + VERSION + Version::ZERO,
                format,
            )?,
            height_to_open_in_cents: EagerVec::forced_import(
                &file,
                "open_in_cents",
                version + VERSION + Version::ZERO,
                format,
            )?,
            timeindexes_to_open: ComputedVecsFromDateIndex::forced_import(
                &file,
                "open",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_first(),
            )?,
            timeindexes_to_high: ComputedVecsFromDateIndex::forced_import(
                &file,
                "high",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_max(),
            )?,
            timeindexes_to_low: ComputedVecsFromDateIndex::forced_import(
                &file,
                "low",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_min(),
            )?,
            timeindexes_to_close: ComputedVecsFromDateIndex::forced_import(
                &file,
                "close",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            timeindexes_to_open_in_sats: ComputedVecsFromDateIndex::forced_import(
                &file,
                "open_in_sats",
                Source::Compute,
                version + VERSION + VERSION_IN_SATS + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_first(),
            )?,
            timeindexes_to_high_in_sats: ComputedVecsFromDateIndex::forced_import(
                &file,
                "high_in_sats",
                Source::Compute,
                version + VERSION + VERSION_IN_SATS + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_max(),
            )?,
            timeindexes_to_low_in_sats: ComputedVecsFromDateIndex::forced_import(
                &file,
                "low_in_sats",
                Source::Compute,
                version + VERSION + VERSION_IN_SATS + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_min(),
            )?,
            timeindexes_to_close_in_sats: ComputedVecsFromDateIndex::forced_import(
                &file,
                "close_in_sats",
                Source::Compute,
                version + VERSION + VERSION_IN_SATS + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            chainindexes_to_open: ComputedVecsFromHeightStrict::forced_import(
                &file,
                "open",
                version + VERSION + Version::ZERO,
                format,
                VecBuilderOptions::default().add_first(),
            )?,
            chainindexes_to_high: ComputedVecsFromHeightStrict::forced_import(
                &file,
                "high",
                version + VERSION + Version::ZERO,
                format,
                VecBuilderOptions::default().add_max(),
            )?,
            chainindexes_to_low: ComputedVecsFromHeightStrict::forced_import(
                &file,
                "low",
                version + VERSION + Version::ZERO,
                format,
                VecBuilderOptions::default().add_min(),
            )?,
            chainindexes_to_close: ComputedVecsFromHeightStrict::forced_import(
                &file,
                "close",
                version + VERSION + Version::ZERO,
                format,
                VecBuilderOptions::default().add_last(),
            )?,
            chainindexes_to_open_in_sats: ComputedVecsFromHeightStrict::forced_import(
                &file,
                "open_in_sats",
                version + VERSION + VERSION_IN_SATS + Version::ZERO,
                format,
                VecBuilderOptions::default().add_first(),
            )?,
            chainindexes_to_high_in_sats: ComputedVecsFromHeightStrict::forced_import(
                &file,
                "high_in_sats",
                version + VERSION + VERSION_IN_SATS + Version::ZERO,
                format,
                VecBuilderOptions::default().add_max(),
            )?,
            chainindexes_to_low_in_sats: ComputedVecsFromHeightStrict::forced_import(
                &file,
                "low_in_sats",
                version + VERSION + VERSION_IN_SATS + Version::ZERO,
                format,
                VecBuilderOptions::default().add_min(),
            )?,
            chainindexes_to_close_in_sats: ComputedVecsFromHeightStrict::forced_import(
                &file,
                "close_in_sats",
                version + VERSION + VERSION_IN_SATS + Version::ZERO,
                format,
                VecBuilderOptions::default().add_last(),
            )?,
            weekindex_to_ohlc: RawVec::forced_import(
                &file,
                "ohlc",
                version + VERSION + Version::ZERO,
            )?,
            weekindex_to_ohlc_in_sats: RawVec::forced_import(
                &file,
                "ohlc_in_sats",
                version + VERSION + VERSION_IN_SATS + Version::ZERO,
            )?,
            difficultyepoch_to_ohlc: RawVec::forced_import(
                &file,
                "ohlc",
                version + VERSION + Version::ZERO,
            )?,
            difficultyepoch_to_ohlc_in_sats: RawVec::forced_import(
                &file,
                "ohlc_in_sats",
                version + VERSION + VERSION_IN_SATS + Version::ZERO,
            )?,
            monthindex_to_ohlc: RawVec::forced_import(
                &file,
                "ohlc",
                version + VERSION + Version::ZERO,
            )?,
            monthindex_to_ohlc_in_sats: RawVec::forced_import(
                &file,
                "ohlc_in_sats",
                version + VERSION + VERSION_IN_SATS + Version::ZERO,
            )?,
            quarterindex_to_ohlc: RawVec::forced_import(
                &file,
                "ohlc",
                version + VERSION + Version::ZERO,
            )?,
            quarterindex_to_ohlc_in_sats: RawVec::forced_import(
                &file,
                "ohlc_in_sats",
                version + VERSION + VERSION_IN_SATS + Version::ZERO,
            )?,
            semesterindex_to_ohlc: RawVec::forced_import(
                &file,
                "ohlc",
                version + VERSION + Version::ZERO,
            )?,
            semesterindex_to_ohlc_in_sats: RawVec::forced_import(
                &file,
                "ohlc_in_sats",
                version + VERSION + VERSION_IN_SATS + Version::ZERO,
            )?,
            yearindex_to_ohlc: RawVec::forced_import(
                &file,
                "ohlc",
                version + VERSION + Version::ZERO,
            )?,
            yearindex_to_ohlc_in_sats: RawVec::forced_import(
                &file,
                "ohlc_in_sats",
                version + VERSION + VERSION_IN_SATS + Version::ZERO,
            )?,
            // halvingepoch_to_ohlc: StorableVec::forced_import(file,
            // "halvingepoch_to_ohlc"), version + VERSION + Version::ZERO, format)?,
            decadeindex_to_ohlc: RawVec::forced_import(
                &file,
                "ohlc",
                version + VERSION + Version::ZERO,
            )?,
            decadeindex_to_ohlc_in_sats: RawVec::forced_import(
                &file,
                "ohlc_in_sats",
                version + VERSION + VERSION_IN_SATS + Version::ZERO,
            )?,

            file,
        })
    }

    pub fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &Indexes,
        fetched: &fetched::Vecs,
        exit: &Exit,
    ) -> Result<()> {
        self.height_to_open_in_cents.compute_transform(
            starting_indexes.height,
            &fetched.height_to_ohlc_in_cents,
            |(di, ohlc, ..)| (di, ohlc.open),
            exit,
        )?;

        self.height_to_high_in_cents.compute_transform(
            starting_indexes.height,
            &fetched.height_to_ohlc_in_cents,
            |(di, ohlc, ..)| (di, ohlc.high),
            exit,
        )?;

        self.height_to_low_in_cents.compute_transform(
            starting_indexes.height,
            &fetched.height_to_ohlc_in_cents,
            |(di, ohlc, ..)| (di, ohlc.low),
            exit,
        )?;

        self.height_to_close_in_cents.compute_transform(
            starting_indexes.height,
            &fetched.height_to_ohlc_in_cents,
            |(di, ohlc, ..)| (di, ohlc.close),
            exit,
        )?;

        fetched
            .height_to_ohlc_in_cents
            .iter_at(starting_indexes.height)
            .try_for_each(|(i, v)| -> Result<()> {
                self.height_to_ohlc
                    .forced_push_at(i, OHLCDollars::from(v.into_owned()), exit)?;
                Ok(())
            })?;
        self.height_to_ohlc.safe_flush(exit)?;

        self.dateindex_to_open_in_cents.compute_transform(
            starting_indexes.dateindex,
            &fetched.dateindex_to_ohlc_in_cents,
            |(di, ohlc, ..)| (di, ohlc.open),
            exit,
        )?;

        self.dateindex_to_high_in_cents.compute_transform(
            starting_indexes.dateindex,
            &fetched.dateindex_to_ohlc_in_cents,
            |(di, ohlc, ..)| (di, ohlc.high),
            exit,
        )?;

        self.dateindex_to_low_in_cents.compute_transform(
            starting_indexes.dateindex,
            &fetched.dateindex_to_ohlc_in_cents,
            |(di, ohlc, ..)| (di, ohlc.low),
            exit,
        )?;

        self.dateindex_to_close_in_cents.compute_transform(
            starting_indexes.dateindex,
            &fetched.dateindex_to_ohlc_in_cents,
            |(di, ohlc, ..)| (di, ohlc.close),
            exit,
        )?;

        fetched
            .dateindex_to_ohlc_in_cents
            .iter_at(starting_indexes.dateindex)
            .try_for_each(|(i, v)| -> Result<()> {
                self.dateindex_to_ohlc.forced_push_at(
                    i,
                    OHLCDollars::from(v.into_owned()),
                    exit,
                )?;
                Ok(())
            })?;
        self.dateindex_to_ohlc.safe_flush(exit)?;

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
                )?;
                Ok(())
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
                )?;
                Ok(())
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
                )?;
                Ok(())
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
                )?;
                Ok(())
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
                )?;
                Ok(())
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
                )?;
                Ok(())
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
                )?;
                Ok(())
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
                )?;
                Ok(())
            },
        )?;

        let mut weekindex_first_iter = self.timeindexes_to_open.weekindex.unwrap_first().iter();
        let mut weekindex_max_iter = self.timeindexes_to_high.weekindex.unwrap_max().iter();
        let mut weekindex_min_iter = self.timeindexes_to_low.weekindex.unwrap_min().iter();
        self.timeindexes_to_close
            .weekindex
            .unwrap_last()
            .iter_at(starting_indexes.weekindex)
            .try_for_each(|(i, v)| -> Result<()> {
                let close = v.into_owned();
                let open = weekindex_first_iter.unwrap_get_inner(i);
                let high = weekindex_max_iter.unwrap_get_inner(i);
                let low = weekindex_min_iter.unwrap_get_inner(i);
                self.weekindex_to_ohlc.forced_push_at(
                    i,
                    OHLCDollars {
                        open,
                        high,
                        low,
                        close,
                    },
                    exit,
                )?;
                Ok(())
            })?;
        self.weekindex_to_ohlc.safe_flush(exit)?;

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
        self.chainindexes_to_close
            .difficultyepoch
            .unwrap_last()
            .iter_at(starting_indexes.difficultyepoch)
            .try_for_each(|(i, v)| -> Result<()> {
                let close = v.into_owned();
                let open = difficultyepoch_first_iter.unwrap_get_inner(i);
                let high = difficultyepoch_max_iter.unwrap_get_inner(i);
                let low = difficultyepoch_min_iter.unwrap_get_inner(i);
                self.difficultyepoch_to_ohlc.forced_push_at(
                    i,
                    OHLCDollars {
                        open,
                        high,
                        low,
                        close,
                    },
                    exit,
                )?;
                Ok(())
            })?;
        self.difficultyepoch_to_ohlc.safe_flush(exit)?;

        let mut monthindex_first_iter = self.timeindexes_to_open.monthindex.unwrap_first().iter();
        let mut monthindex_max_iter = self.timeindexes_to_high.monthindex.unwrap_max().iter();
        let mut monthindex_min_iter = self.timeindexes_to_low.monthindex.unwrap_min().iter();
        self.timeindexes_to_close
            .monthindex
            .unwrap_last()
            .iter_at(starting_indexes.monthindex)
            .try_for_each(|(i, v)| -> Result<()> {
                let close = v.into_owned();
                let open = monthindex_first_iter.unwrap_get_inner(i);
                let high = monthindex_max_iter.unwrap_get_inner(i);
                let low = monthindex_min_iter.unwrap_get_inner(i);
                self.monthindex_to_ohlc.forced_push_at(
                    i,
                    OHLCDollars {
                        open,
                        high,
                        low,
                        close,
                    },
                    exit,
                )?;
                Ok(())
            })?;
        self.monthindex_to_ohlc.safe_flush(exit)?;

        let mut quarterindex_first_iter =
            self.timeindexes_to_open.quarterindex.unwrap_first().iter();
        let mut quarterindex_max_iter = self.timeindexes_to_high.quarterindex.unwrap_max().iter();
        let mut quarterindex_min_iter = self.timeindexes_to_low.quarterindex.unwrap_min().iter();
        self.timeindexes_to_close
            .quarterindex
            .unwrap_last()
            .iter_at(starting_indexes.quarterindex)
            .try_for_each(|(i, v)| -> Result<()> {
                let close = v.into_owned();
                let open = quarterindex_first_iter.unwrap_get_inner(i);
                let high = quarterindex_max_iter.unwrap_get_inner(i);
                let low = quarterindex_min_iter.unwrap_get_inner(i);
                self.quarterindex_to_ohlc.forced_push_at(
                    i,
                    OHLCDollars {
                        open,
                        high,
                        low,
                        close,
                    },
                    exit,
                )?;
                Ok(())
            })?;
        self.quarterindex_to_ohlc.safe_flush(exit)?;

        let mut semesterindex_first_iter =
            self.timeindexes_to_open.semesterindex.unwrap_first().iter();
        let mut semesterindex_max_iter = self.timeindexes_to_high.semesterindex.unwrap_max().iter();
        let mut semesterindex_min_iter = self.timeindexes_to_low.semesterindex.unwrap_min().iter();
        self.timeindexes_to_close
            .semesterindex
            .unwrap_last()
            .iter_at(starting_indexes.semesterindex)
            .try_for_each(|(i, v)| -> Result<()> {
                let close = v.into_owned();
                let open = semesterindex_first_iter.unwrap_get_inner(i);
                let high = semesterindex_max_iter.unwrap_get_inner(i);
                let low = semesterindex_min_iter.unwrap_get_inner(i);
                self.semesterindex_to_ohlc.forced_push_at(
                    i,
                    OHLCDollars {
                        open,
                        high,
                        low,
                        close,
                    },
                    exit,
                )?;
                Ok(())
            })?;
        self.semesterindex_to_ohlc.safe_flush(exit)?;

        let mut yearindex_first_iter = self.timeindexes_to_open.yearindex.unwrap_first().iter();
        let mut yearindex_max_iter = self.timeindexes_to_high.yearindex.unwrap_max().iter();
        let mut yearindex_min_iter = self.timeindexes_to_low.yearindex.unwrap_min().iter();
        self.timeindexes_to_close
            .yearindex
            .unwrap_last()
            .iter_at(starting_indexes.yearindex)
            .try_for_each(|(i, v)| -> Result<()> {
                let close = v.into_owned();
                let open = yearindex_first_iter.unwrap_get_inner(i);
                let high = yearindex_max_iter.unwrap_get_inner(i);
                let low = yearindex_min_iter.unwrap_get_inner(i);
                self.yearindex_to_ohlc.forced_push_at(
                    i,
                    OHLCDollars {
                        open,
                        high,
                        low,
                        close,
                    },
                    exit,
                )?;
                Ok(())
            })?;
        self.yearindex_to_ohlc.safe_flush(exit)?;

        // self.halvingepoch_to_ohlc
        //     .compute_transform(starting_indexes.halvingepoch, other, t, exit)?;

        let mut decadeindex_first_iter = self.timeindexes_to_open.decadeindex.unwrap_first().iter();
        let mut decadeindex_max_iter = self.timeindexes_to_high.decadeindex.unwrap_max().iter();
        let mut decadeindex_min_iter = self.timeindexes_to_low.decadeindex.unwrap_min().iter();
        self.timeindexes_to_close
            .decadeindex
            .unwrap_last()
            .iter_at(starting_indexes.decadeindex)
            .try_for_each(|(i, v)| -> Result<()> {
                let close = v.into_owned();
                let open = decadeindex_first_iter.unwrap_get_inner(i);
                let high = decadeindex_max_iter.unwrap_get_inner(i);
                let low = decadeindex_min_iter.unwrap_get_inner(i);
                self.decadeindex_to_ohlc.forced_push_at(
                    i,
                    OHLCDollars {
                        open,
                        high,
                        low,
                        close,
                    },
                    exit,
                )?;
                Ok(())
            })?;
        self.decadeindex_to_ohlc.safe_flush(exit)?;

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
                )?;
                Ok(())
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
                )?;
                Ok(())
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
                )?;
                Ok(())
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
                )?;
                Ok(())
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
                )?;
                Ok(())
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
                )?;
                Ok(())
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
                )?;
                Ok(())
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
                )?;
                Ok(())
            },
        )?;

        let mut height_first_iter = self.chainindexes_to_open_in_sats.height.iter();
        let mut height_max_iter = self.chainindexes_to_high_in_sats.height.iter();
        let mut height_min_iter = self.chainindexes_to_low_in_sats.height.iter();
        self.chainindexes_to_close_in_sats
            .height
            .iter_at(starting_indexes.height)
            .try_for_each(|(i, v)| -> Result<()> {
                let close = v.into_owned();
                self.height_to_ohlc_in_sats.forced_push_at(
                    i,
                    OHLCSats {
                        open: height_first_iter.unwrap_get_inner(i),
                        high: height_max_iter.unwrap_get_inner(i),
                        low: height_min_iter.unwrap_get_inner(i),
                        close,
                    },
                    exit,
                )?;
                Ok(())
            })?;
        self.height_to_ohlc_in_sats.safe_flush(exit)?;

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
        self.timeindexes_to_close_in_sats
            .dateindex
            .as_ref()
            .unwrap()
            .iter_at(starting_indexes.dateindex)
            .try_for_each(|(i, v)| -> Result<()> {
                let close = v.into_owned();
                self.dateindex_to_ohlc_in_sats.forced_push_at(
                    i,
                    OHLCSats {
                        open: dateindex_first_iter.unwrap_get_inner(i),
                        high: dateindex_max_iter.unwrap_get_inner(i),
                        low: dateindex_min_iter.unwrap_get_inner(i),
                        close,
                    },
                    exit,
                )?;
                Ok(())
            })?;
        self.dateindex_to_ohlc_in_sats.safe_flush(exit)?;

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
        self.timeindexes_to_close_in_sats
            .weekindex
            .unwrap_last()
            .iter_at(starting_indexes.weekindex)
            .try_for_each(|(i, v)| -> Result<()> {
                let close = v.into_owned();
                self.weekindex_to_ohlc_in_sats.forced_push_at(
                    i,
                    OHLCSats {
                        open: weekindex_first_iter.unwrap_get_inner(i),
                        high: weekindex_max_iter.unwrap_get_inner(i),
                        low: weekindex_min_iter.unwrap_get_inner(i),
                        close,
                    },
                    exit,
                )?;
                Ok(())
            })?;
        self.weekindex_to_ohlc_in_sats.safe_flush(exit)?;

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
        self.chainindexes_to_close_in_sats
            .difficultyepoch
            .unwrap_last()
            .iter_at(starting_indexes.difficultyepoch)
            .try_for_each(|(i, v)| -> Result<()> {
                let close = v.into_owned();
                self.difficultyepoch_to_ohlc_in_sats.forced_push_at(
                    i,
                    OHLCSats {
                        open: difficultyepoch_first_iter.unwrap_get_inner(i),
                        high: difficultyepoch_max_iter.unwrap_get_inner(i),
                        low: difficultyepoch_min_iter.unwrap_get_inner(i),
                        close,
                    },
                    exit,
                )?;
                Ok(())
            })?;
        self.difficultyepoch_to_ohlc_in_sats.safe_flush(exit)?;

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
        self.timeindexes_to_close_in_sats
            .monthindex
            .unwrap_last()
            .iter_at(starting_indexes.monthindex)
            .try_for_each(|(i, v)| -> Result<()> {
                let close = v.into_owned();
                self.monthindex_to_ohlc_in_sats.forced_push_at(
                    i,
                    OHLCSats {
                        open: monthindex_first_iter.unwrap_get_inner(i),
                        high: monthindex_max_iter.unwrap_get_inner(i),
                        low: monthindex_min_iter.unwrap_get_inner(i),
                        close,
                    },
                    exit,
                )?;
                Ok(())
            })?;
        self.monthindex_to_ohlc_in_sats.safe_flush(exit)?;

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
        self.timeindexes_to_close_in_sats
            .quarterindex
            .unwrap_last()
            .iter_at(starting_indexes.quarterindex)
            .try_for_each(|(i, v)| -> Result<()> {
                let close = v.into_owned();
                self.quarterindex_to_ohlc_in_sats.forced_push_at(
                    i,
                    OHLCSats {
                        open: quarterindex_first_iter.unwrap_get_inner(i),
                        high: quarterindex_max_iter.unwrap_get_inner(i),
                        low: quarterindex_min_iter.unwrap_get_inner(i),
                        close,
                    },
                    exit,
                )?;
                Ok(())
            })?;
        self.quarterindex_to_ohlc_in_sats.safe_flush(exit)?;

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
        self.timeindexes_to_close_in_sats
            .semesterindex
            .unwrap_last()
            .iter_at(starting_indexes.semesterindex)
            .try_for_each(|(i, v)| -> Result<()> {
                let close = v.into_owned();
                self.semesterindex_to_ohlc_in_sats.forced_push_at(
                    i,
                    OHLCSats {
                        open: semesterindex_first_iter.unwrap_get_inner(i),
                        high: semesterindex_max_iter.unwrap_get_inner(i),
                        low: semesterindex_min_iter.unwrap_get_inner(i),
                        close,
                    },
                    exit,
                )?;
                Ok(())
            })?;
        self.semesterindex_to_ohlc_in_sats.safe_flush(exit)?;

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
        self.timeindexes_to_close_in_sats
            .yearindex
            .unwrap_last()
            .iter_at(starting_indexes.yearindex)
            .try_for_each(|(i, v)| -> Result<()> {
                let close = v.into_owned();
                self.yearindex_to_ohlc_in_sats.forced_push_at(
                    i,
                    OHLCSats {
                        open: yearindex_first_iter.unwrap_get_inner(i),
                        high: yearindex_max_iter.unwrap_get_inner(i),
                        low: yearindex_min_iter.unwrap_get_inner(i),
                        close,
                    },
                    exit,
                )?;
                Ok(())
            })?;
        self.yearindex_to_ohlc_in_sats.safe_flush(exit)?;

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
        self.timeindexes_to_close_in_sats
            .decadeindex
            .unwrap_last()
            .iter_at(starting_indexes.decadeindex)
            .try_for_each(|(i, v)| -> Result<()> {
                let close = v.into_owned();
                self.decadeindex_to_ohlc_in_sats.forced_push_at(
                    i,
                    OHLCSats {
                        open: decadeindex_first_iter.unwrap_get_inner(i),
                        high: decadeindex_max_iter.unwrap_get_inner(i),
                        low: decadeindex_min_iter.unwrap_get_inner(i),
                        close,
                    },
                    exit,
                )?;
                Ok(())
            })?;
        self.decadeindex_to_ohlc_in_sats.safe_flush(exit)?;

        self.file.flush()?;
        self.file.punch_holes()?;
        Ok(())
    }

    pub fn vecs(&self) -> Vec<&dyn AnyCollectableVec> {
        vec![
            vec![
                &self.dateindex_to_close_in_cents as &dyn AnyCollectableVec,
                &self.dateindex_to_high_in_cents,
                &self.dateindex_to_low_in_cents,
                &self.dateindex_to_ohlc,
                &self.dateindex_to_open_in_cents,
                &self.height_to_close_in_cents,
                &self.height_to_high_in_cents,
                &self.height_to_low_in_cents,
                &self.height_to_ohlc,
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
