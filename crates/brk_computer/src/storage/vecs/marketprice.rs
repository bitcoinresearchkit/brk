use std::{fs, path::Path};

use brk_core::{
    Cents, Close, Dateindex, Decadeindex, Difficultyepoch, Dollars, Height, High, Low, Monthindex,
    OHLCCents, OHLCDollars, Open, Quarterindex, Sats, Weekindex, Yearindex,
};
use brk_exit::Exit;
use brk_fetcher::Fetcher;
use brk_indexer::Indexer;
use brk_vec::{AnyStorableVec, Compressed, Version};

use super::{
    ComputedVec, Indexes,
    grouped::{
        ComputedVecsFromDateindex, ComputedVecsFromHeightStrict, StorableVecGeneatorOptions,
    },
    indexes,
};

#[derive(Clone)]
pub struct Vecs {
    // pub dateindex_to_close: ComputedVec<Dateindex, Close<Dollars>>,
    pub dateindex_to_close_in_cents: ComputedVec<Dateindex, Close<Cents>>,
    pub dateindex_to_high_in_cents: ComputedVec<Dateindex, High<Cents>>,
    pub dateindex_to_low_in_cents: ComputedVec<Dateindex, Low<Cents>>,
    pub dateindex_to_ohlc: ComputedVec<Dateindex, OHLCDollars>,
    pub dateindex_to_ohlc_in_cents: ComputedVec<Dateindex, OHLCCents>,
    pub dateindex_to_open_in_cents: ComputedVec<Dateindex, Open<Cents>>,
    pub height_to_close_in_cents: ComputedVec<Height, Close<Cents>>,
    pub height_to_high_in_cents: ComputedVec<Height, High<Cents>>,
    pub height_to_low_in_cents: ComputedVec<Height, Low<Cents>>,
    pub height_to_ohlc: ComputedVec<Height, OHLCDollars>,
    pub height_to_ohlc_in_cents: ComputedVec<Height, OHLCCents>,
    pub height_to_open_in_cents: ComputedVec<Height, Open<Cents>>,
    pub timeindexes_to_close: ComputedVecsFromDateindex<Close<Dollars>>,
    pub timeindexes_to_high: ComputedVecsFromDateindex<High<Dollars>>,
    pub timeindexes_to_low: ComputedVecsFromDateindex<Low<Dollars>>,
    pub timeindexes_to_open: ComputedVecsFromDateindex<Open<Dollars>>,
    pub timeindexes_to_sats_per_dollar: ComputedVecsFromDateindex<Close<Sats>>,
    pub chainindexes_to_close: ComputedVecsFromHeightStrict<Close<Dollars>>,
    pub chainindexes_to_high: ComputedVecsFromHeightStrict<High<Dollars>>,
    pub chainindexes_to_low: ComputedVecsFromHeightStrict<Low<Dollars>>,
    pub chainindexes_to_open: ComputedVecsFromHeightStrict<Open<Dollars>>,
    pub chainindexes_to_sats_per_dollar: ComputedVecsFromHeightStrict<Close<Sats>>,
    pub weekindex_to_ohlc: ComputedVec<Weekindex, OHLCDollars>,
    pub difficultyepoch_to_ohlc: ComputedVec<Difficultyepoch, OHLCDollars>,
    pub monthindex_to_ohlc: ComputedVec<Monthindex, OHLCDollars>,
    pub quarterindex_to_ohlc: ComputedVec<Quarterindex, OHLCDollars>,
    pub yearindex_to_ohlc: ComputedVec<Yearindex, OHLCDollars>,
    // pub halvingepoch_to_ohlc: StorableVec<Halvingepoch, OHLCDollars>,
    pub decadeindex_to_ohlc: ComputedVec<Decadeindex, OHLCDollars>,
}

impl Vecs {
    pub fn forced_import(path: &Path, compressed: Compressed) -> color_eyre::Result<Self> {
        fs::create_dir_all(path)?;

        Ok(Self {
            dateindex_to_ohlc_in_cents: ComputedVec::forced_import(
                &path.join("dateindex_to_ohlc_in_cents"),
                Version::ONE,
                compressed,
            )?,
            dateindex_to_ohlc: ComputedVec::forced_import(
                &path.join("dateindex_to_ohlc"),
                Version::ONE,
                compressed,
            )?,
            dateindex_to_close_in_cents: ComputedVec::forced_import(
                &path.join("dateindex_to_close_in_cents"),
                Version::ONE,
                compressed,
            )?,
            dateindex_to_high_in_cents: ComputedVec::forced_import(
                &path.join("dateindex_to_high_in_cents"),
                Version::ONE,
                compressed,
            )?,
            dateindex_to_low_in_cents: ComputedVec::forced_import(
                &path.join("dateindex_to_low_in_cents"),
                Version::ONE,
                compressed,
            )?,
            dateindex_to_open_in_cents: ComputedVec::forced_import(
                &path.join("dateindex_to_open_in_cents"),
                Version::ONE,
                compressed,
            )?,
            height_to_ohlc_in_cents: ComputedVec::forced_import(
                &path.join("height_to_ohlc_in_cents"),
                Version::ONE,
                compressed,
            )?,
            height_to_ohlc: ComputedVec::forced_import(
                &path.join("height_to_ohlc"),
                Version::ONE,
                compressed,
            )?,
            height_to_close_in_cents: ComputedVec::forced_import(
                &path.join("height_to_close_in_cents"),
                Version::ONE,
                compressed,
            )?,
            height_to_high_in_cents: ComputedVec::forced_import(
                &path.join("height_to_high_in_cents"),
                Version::ONE,
                compressed,
            )?,
            height_to_low_in_cents: ComputedVec::forced_import(
                &path.join("height_to_low_in_cents"),
                Version::ONE,
                compressed,
            )?,
            height_to_open_in_cents: ComputedVec::forced_import(
                &path.join("height_to_open_in_cents"),
                Version::ONE,
                compressed,
            )?,
            timeindexes_to_open: ComputedVecsFromDateindex::forced_import(
                path,
                "open",
                Version::ONE,
                compressed,
                StorableVecGeneatorOptions::default().add_first(),
            )?,
            timeindexes_to_high: ComputedVecsFromDateindex::forced_import(
                path,
                "high",
                Version::ONE,
                compressed,
                StorableVecGeneatorOptions::default().add_max(),
            )?,
            timeindexes_to_low: ComputedVecsFromDateindex::forced_import(
                path,
                "low",
                Version::ONE,
                compressed,
                StorableVecGeneatorOptions::default().add_min(),
            )?,
            timeindexes_to_close: ComputedVecsFromDateindex::forced_import(
                path,
                "close",
                Version::ONE,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            timeindexes_to_sats_per_dollar: ComputedVecsFromDateindex::forced_import(
                path,
                "sats_per_dollar",
                Version::ONE,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            chainindexes_to_open: ComputedVecsFromHeightStrict::forced_import(
                path,
                "open",
                Version::ONE,
                compressed,
                StorableVecGeneatorOptions::default().add_first(),
            )?,
            chainindexes_to_high: ComputedVecsFromHeightStrict::forced_import(
                path,
                "high",
                Version::ONE,
                compressed,
                StorableVecGeneatorOptions::default().add_max(),
            )?,
            chainindexes_to_low: ComputedVecsFromHeightStrict::forced_import(
                path,
                "low",
                Version::ONE,
                compressed,
                StorableVecGeneatorOptions::default().add_min(),
            )?,
            chainindexes_to_close: ComputedVecsFromHeightStrict::forced_import(
                path,
                "close",
                Version::ONE,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            chainindexes_to_sats_per_dollar: ComputedVecsFromHeightStrict::forced_import(
                path,
                "sats_per_dollar",
                Version::ONE,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            weekindex_to_ohlc: ComputedVec::forced_import(
                &path.join("weekindex_to_ohlc"),
                Version::ONE,
                compressed,
            )?,
            difficultyepoch_to_ohlc: ComputedVec::forced_import(
                &path.join("difficultyepoch_to_ohlc"),
                Version::ONE,
                compressed,
            )?,
            monthindex_to_ohlc: ComputedVec::forced_import(
                &path.join("monthindex_to_ohlc"),
                Version::ONE,
                compressed,
            )?,
            quarterindex_to_ohlc: ComputedVec::forced_import(
                &path.join("quarterindex_to_ohlc"),
                Version::ONE,
                compressed,
            )?,
            yearindex_to_ohlc: ComputedVec::forced_import(
                &path.join("yearindex_to_ohlc"),
                Version::ONE,
                compressed,
            )?,
            // halvingepoch_to_ohlc: StorableVec::forced_import(&path.join("halvingepoch_to_ohlc"), Version::ONE, compressed)?,
            decadeindex_to_ohlc: ComputedVec::forced_import(
                &path.join("decadeindex_to_ohlc"),
                Version::ONE,
                compressed,
            )?,
        })
    }

    pub fn compute(
        &mut self,
        indexer: &mut Indexer,
        indexes: &mut indexes::Vecs,
        starting_indexes: &Indexes,
        fetcher: &mut Fetcher,
        exit: &Exit,
    ) -> color_eyre::Result<()> {
        let indexer_vecs = indexer.mut_vecs();

        self.height_to_ohlc_in_cents.compute_transform(
            starting_indexes.height,
            indexer_vecs.height_to_timestamp.mut_vec(),
            |(h, t, _, height_to_timestamp)| {
                let ohlc = fetcher
                    .get_height(
                        h,
                        t,
                        h.decremented()
                            .map(|prev_h| *height_to_timestamp.get(prev_h).unwrap().unwrap()),
                    )
                    .unwrap();
                (h, ohlc)
            },
            exit,
        )?;

        self.height_to_open_in_cents.compute_transform(
            starting_indexes.height,
            self.height_to_ohlc_in_cents.mut_vec(),
            |(di, ohlc, ..)| (di, ohlc.open),
            exit,
        )?;

        self.height_to_high_in_cents.compute_transform(
            starting_indexes.height,
            self.height_to_ohlc_in_cents.mut_vec(),
            |(di, ohlc, ..)| (di, ohlc.high),
            exit,
        )?;

        self.height_to_low_in_cents.compute_transform(
            starting_indexes.height,
            self.height_to_ohlc_in_cents.mut_vec(),
            |(di, ohlc, ..)| (di, ohlc.low),
            exit,
        )?;

        self.height_to_close_in_cents.compute_transform(
            starting_indexes.height,
            self.height_to_ohlc_in_cents.mut_vec(),
            |(di, ohlc, ..)| (di, ohlc.close),
            exit,
        )?;

        self.height_to_ohlc.compute_transform(
            starting_indexes.height,
            self.height_to_ohlc_in_cents.mut_vec(),
            |(di, ohlc, ..)| (di, OHLCDollars::from(ohlc)),
            exit,
        )?;

        self.dateindex_to_ohlc_in_cents.compute_transform(
            starting_indexes.dateindex,
            indexes.dateindex_to_date.mut_vec(),
            |(di, d, ..)| {
                let ohlc = fetcher.get_date(d).unwrap();
                (di, ohlc)
            },
            exit,
        )?;

        self.dateindex_to_open_in_cents.compute_transform(
            starting_indexes.dateindex,
            self.dateindex_to_ohlc_in_cents.mut_vec(),
            |(di, ohlc, ..)| (di, ohlc.open),
            exit,
        )?;

        self.dateindex_to_high_in_cents.compute_transform(
            starting_indexes.dateindex,
            self.dateindex_to_ohlc_in_cents.mut_vec(),
            |(di, ohlc, ..)| (di, ohlc.high),
            exit,
        )?;

        self.dateindex_to_low_in_cents.compute_transform(
            starting_indexes.dateindex,
            self.dateindex_to_ohlc_in_cents.mut_vec(),
            |(di, ohlc, ..)| (di, ohlc.low),
            exit,
        )?;

        self.dateindex_to_close_in_cents.compute_transform(
            starting_indexes.dateindex,
            self.dateindex_to_ohlc_in_cents.mut_vec(),
            |(di, ohlc, ..)| (di, ohlc.close),
            exit,
        )?;

        self.dateindex_to_ohlc.compute_transform(
            starting_indexes.dateindex,
            self.dateindex_to_ohlc_in_cents.mut_vec(),
            |(di, ohlc, ..)| (di, OHLCDollars::from(ohlc)),
            exit,
        )?;

        self.timeindexes_to_close.compute(
            |v| {
                v.compute_transform(
                    starting_indexes.dateindex,
                    self.dateindex_to_ohlc.mut_vec(),
                    |(di, ohlc, ..)| (di, ohlc.close),
                    exit,
                )
            },
            indexes,
            starting_indexes,
            exit,
        )?;

        self.timeindexes_to_high.compute(
            |v| {
                v.compute_transform(
                    starting_indexes.dateindex,
                    self.dateindex_to_ohlc.mut_vec(),
                    |(di, ohlc, ..)| (di, ohlc.high),
                    exit,
                )
            },
            indexes,
            starting_indexes,
            exit,
        )?;

        self.timeindexes_to_low.compute(
            |v| {
                v.compute_transform(
                    starting_indexes.dateindex,
                    self.dateindex_to_ohlc.mut_vec(),
                    |(di, ohlc, ..)| (di, ohlc.low),
                    exit,
                )
            },
            indexes,
            starting_indexes,
            exit,
        )?;

        self.timeindexes_to_open.compute(
            |v| {
                v.compute_transform(
                    starting_indexes.dateindex,
                    self.dateindex_to_ohlc.mut_vec(),
                    |(di, ohlc, ..)| (di, ohlc.open),
                    exit,
                )
            },
            indexes,
            starting_indexes,
            exit,
        )?;

        self.chainindexes_to_close.compute(
            |v| {
                v.compute_transform(
                    starting_indexes.height,
                    self.height_to_ohlc.mut_vec(),
                    |(di, ohlc, ..)| (di, ohlc.close),
                    exit,
                )
            },
            indexes,
            starting_indexes,
            exit,
        )?;

        self.chainindexes_to_high.compute(
            |v| {
                v.compute_transform(
                    starting_indexes.height,
                    self.height_to_ohlc.mut_vec(),
                    |(di, ohlc, ..)| (di, ohlc.high),
                    exit,
                )
            },
            indexes,
            starting_indexes,
            exit,
        )?;

        self.chainindexes_to_low.compute(
            |v| {
                v.compute_transform(
                    starting_indexes.height,
                    self.height_to_ohlc.mut_vec(),
                    |(di, ohlc, ..)| (di, ohlc.low),
                    exit,
                )
            },
            indexes,
            starting_indexes,
            exit,
        )?;

        self.chainindexes_to_open.compute(
            |v| {
                v.compute_transform(
                    starting_indexes.height,
                    self.height_to_ohlc.mut_vec(),
                    |(di, ohlc, ..)| (di, ohlc.open),
                    exit,
                )
            },
            indexes,
            starting_indexes,
            exit,
        )?;

        self.weekindex_to_ohlc.compute_transform(
            starting_indexes.weekindex,
            self.timeindexes_to_close
                .weekindex
                .last
                .as_mut()
                .unwrap()
                .mut_vec(),
            |(i, close, ..)| {
                (
                    i,
                    OHLCDollars {
                        open: *self
                            .timeindexes_to_open
                            .weekindex
                            .first
                            .as_mut()
                            .unwrap()
                            .get(i)
                            .unwrap()
                            .unwrap(),
                        high: *self
                            .timeindexes_to_high
                            .weekindex
                            .max
                            .as_mut()
                            .unwrap()
                            .get(i)
                            .unwrap()
                            .unwrap(),
                        low: *self
                            .timeindexes_to_low
                            .weekindex
                            .min
                            .as_mut()
                            .unwrap()
                            .get(i)
                            .unwrap()
                            .unwrap(),
                        close,
                    },
                )
            },
            exit,
        )?;

        self.difficultyepoch_to_ohlc.compute_transform(
            starting_indexes.difficultyepoch,
            self.chainindexes_to_close
                .difficultyepoch
                .last
                .as_mut()
                .unwrap()
                .mut_vec(),
            |(i, close, ..)| {
                (
                    i,
                    OHLCDollars {
                        open: *self
                            .chainindexes_to_open
                            .difficultyepoch
                            .first
                            .as_mut()
                            .unwrap()
                            .get(i)
                            .unwrap()
                            .unwrap(),
                        high: *self
                            .chainindexes_to_high
                            .difficultyepoch
                            .max
                            .as_mut()
                            .unwrap()
                            .get(i)
                            .unwrap()
                            .unwrap(),
                        low: *self
                            .chainindexes_to_low
                            .difficultyepoch
                            .min
                            .as_mut()
                            .unwrap()
                            .get(i)
                            .unwrap()
                            .unwrap(),
                        close,
                    },
                )
            },
            exit,
        )?;

        self.monthindex_to_ohlc.compute_transform(
            starting_indexes.monthindex,
            self.timeindexes_to_close
                .monthindex
                .last
                .as_mut()
                .unwrap()
                .mut_vec(),
            |(i, close, ..)| {
                (
                    i,
                    OHLCDollars {
                        open: *self
                            .timeindexes_to_open
                            .monthindex
                            .first
                            .as_mut()
                            .unwrap()
                            .get(i)
                            .unwrap()
                            .unwrap(),
                        high: *self
                            .timeindexes_to_high
                            .monthindex
                            .max
                            .as_mut()
                            .unwrap()
                            .get(i)
                            .unwrap()
                            .unwrap(),
                        low: *self
                            .timeindexes_to_low
                            .monthindex
                            .min
                            .as_mut()
                            .unwrap()
                            .get(i)
                            .unwrap()
                            .unwrap(),
                        close,
                    },
                )
            },
            exit,
        )?;

        self.quarterindex_to_ohlc.compute_transform(
            starting_indexes.quarterindex,
            self.timeindexes_to_close
                .quarterindex
                .last
                .as_mut()
                .unwrap()
                .mut_vec(),
            |(i, close, ..)| {
                (
                    i,
                    OHLCDollars {
                        open: *self
                            .timeindexes_to_open
                            .quarterindex
                            .first
                            .as_mut()
                            .unwrap()
                            .get(i)
                            .unwrap()
                            .unwrap(),
                        high: *self
                            .timeindexes_to_high
                            .quarterindex
                            .max
                            .as_mut()
                            .unwrap()
                            .get(i)
                            .unwrap()
                            .unwrap(),
                        low: *self
                            .timeindexes_to_low
                            .quarterindex
                            .min
                            .as_mut()
                            .unwrap()
                            .get(i)
                            .unwrap()
                            .unwrap(),
                        close,
                    },
                )
            },
            exit,
        )?;

        self.yearindex_to_ohlc.compute_transform(
            starting_indexes.yearindex,
            self.timeindexes_to_close
                .yearindex
                .last
                .as_mut()
                .unwrap()
                .mut_vec(),
            |(i, close, ..)| {
                (
                    i,
                    OHLCDollars {
                        open: *self
                            .timeindexes_to_open
                            .yearindex
                            .first
                            .as_mut()
                            .unwrap()
                            .get(i)
                            .unwrap()
                            .unwrap(),
                        high: *self
                            .timeindexes_to_high
                            .yearindex
                            .max
                            .as_mut()
                            .unwrap()
                            .get(i)
                            .unwrap()
                            .unwrap(),
                        low: *self
                            .timeindexes_to_low
                            .yearindex
                            .min
                            .as_mut()
                            .unwrap()
                            .get(i)
                            .unwrap()
                            .unwrap(),
                        close,
                    },
                )
            },
            exit,
        )?;

        // self.halvingepoch_to_ohlc
        //     .compute_transform(starting_indexes.halvingepoch, other, t, exit)?;

        self.decadeindex_to_ohlc.compute_transform(
            starting_indexes.decadeindex,
            self.timeindexes_to_close
                .decadeindex
                .last
                .as_mut()
                .as_mut()
                .unwrap()
                .mut_vec(),
            |(i, close, ..)| {
                (
                    i,
                    OHLCDollars {
                        open: *self
                            .timeindexes_to_open
                            .decadeindex
                            .first
                            .as_mut()
                            .unwrap()
                            .get(i)
                            .unwrap()
                            .unwrap(),
                        high: *self
                            .timeindexes_to_high
                            .decadeindex
                            .max
                            .as_mut()
                            .unwrap()
                            .get(i)
                            .unwrap()
                            .unwrap(),
                        low: *self
                            .timeindexes_to_low
                            .decadeindex
                            .min
                            .as_mut()
                            .unwrap()
                            .get(i)
                            .unwrap()
                            .unwrap(),
                        close,
                    },
                )
            },
            exit,
        )?;

        self.chainindexes_to_sats_per_dollar.compute(
            |v| {
                v.compute_transform(
                    starting_indexes.height,
                    self.chainindexes_to_close.height.mut_vec(),
                    |(i, close, ..)| (i, Close::from(Sats::ONE_BTC / *close)),
                    exit,
                )
            },
            indexes,
            starting_indexes,
            exit,
        )?;

        self.timeindexes_to_sats_per_dollar.compute(
            |v| {
                v.compute_transform(
                    starting_indexes.dateindex,
                    self.timeindexes_to_close.dateindex.mut_vec(),
                    |(i, close, ..)| (i, Close::from(Sats::ONE_BTC / *close)),
                    exit,
                )
            },
            indexes,
            starting_indexes,
            exit,
        )?;

        Ok(())
    }

    pub fn as_any_vecs(&self) -> Vec<&dyn AnyStorableVec> {
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
            ],
            self.chainindexes_to_sats_per_dollar.any_vecs(),
            self.timeindexes_to_sats_per_dollar.any_vecs(),
            self.timeindexes_to_close.any_vecs(),
            self.timeindexes_to_high.any_vecs(),
            self.timeindexes_to_low.any_vecs(),
            self.timeindexes_to_open.any_vecs(),
            self.chainindexes_to_close.any_vecs(),
            self.chainindexes_to_high.any_vecs(),
            self.chainindexes_to_low.any_vecs(),
            self.chainindexes_to_open.any_vecs(),
        ]
        .concat()
    }
}
