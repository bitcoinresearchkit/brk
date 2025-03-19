use std::{fs, path::Path};

use brk_core::{
    Cents, Close, Dateindex, Dollars, Height, High, Low, OHLCCents, OHLCDollars, Open, Sats,
};
use brk_exit::Exit;
use brk_fetcher::Fetcher;
use brk_indexer::Indexer;
use brk_vec::{AnyStorableVec, Compressed, Version};

use super::{Indexes, StorableVec, indexes};

#[derive(Clone)]
pub struct Vecs {
    pub dateindex_to_ohlc_in_cents: StorableVec<Dateindex, OHLCCents>,
    pub dateindex_to_ohlc: StorableVec<Dateindex, OHLCDollars>,
    pub dateindex_to_close_in_cents: StorableVec<Dateindex, Close<Cents>>,
    pub dateindex_to_close: StorableVec<Dateindex, Close<Dollars>>,
    pub dateindex_to_high_in_cents: StorableVec<Dateindex, High<Cents>>,
    pub dateindex_to_high: StorableVec<Dateindex, High<Dollars>>,
    pub dateindex_to_low_in_cents: StorableVec<Dateindex, Low<Cents>>,
    pub dateindex_to_low: StorableVec<Dateindex, Low<Dollars>>,
    pub dateindex_to_open_in_cents: StorableVec<Dateindex, Open<Cents>>,
    pub dateindex_to_open: StorableVec<Dateindex, Open<Dollars>>,
    pub dateindex_to_sats_per_dollar: StorableVec<Dateindex, Close<Sats>>,
    pub height_to_ohlc_in_cents: StorableVec<Height, OHLCCents>,
    pub height_to_ohlc: StorableVec<Height, OHLCDollars>,
    pub height_to_close_in_cents: StorableVec<Height, Close<Cents>>,
    pub height_to_close: StorableVec<Height, Close<Dollars>>,
    pub height_to_high_in_cents: StorableVec<Height, High<Cents>>,
    pub height_to_high: StorableVec<Height, High<Dollars>>,
    pub height_to_low_in_cents: StorableVec<Height, Low<Cents>>,
    pub height_to_low: StorableVec<Height, Low<Dollars>>,
    pub height_to_open_in_cents: StorableVec<Height, Open<Cents>>,
    pub height_to_open: StorableVec<Height, Open<Dollars>>,
    pub height_to_sats_per_dollar: StorableVec<Height, Close<Sats>>,
}

impl Vecs {
    pub fn forced_import(path: &Path, compressed: Compressed) -> color_eyre::Result<Self> {
        fs::create_dir_all(path)?;

        Ok(Self {
            dateindex_to_ohlc_in_cents: StorableVec::forced_import(
                &path.join("dateindex_to_ohlc_in_cents"),
                Version::from(1),
                compressed,
            )?,
            dateindex_to_ohlc: StorableVec::forced_import(
                &path.join("dateindex_to_ohlc"),
                Version::from(1),
                compressed,
            )?,
            dateindex_to_close_in_cents: StorableVec::forced_import(
                &path.join("dateindex_to_close_in_cents"),
                Version::from(1),
                compressed,
            )?,
            dateindex_to_close: StorableVec::forced_import(
                &path.join("dateindex_to_close"),
                Version::from(1),
                compressed,
            )?,
            dateindex_to_high_in_cents: StorableVec::forced_import(
                &path.join("dateindex_to_high_in_cents"),
                Version::from(1),
                compressed,
            )?,
            dateindex_to_high: StorableVec::forced_import(
                &path.join("dateindex_to_high"),
                Version::from(1),
                compressed,
            )?,
            dateindex_to_low_in_cents: StorableVec::forced_import(
                &path.join("dateindex_to_low_in_cents"),
                Version::from(1),
                compressed,
            )?,
            dateindex_to_low: StorableVec::forced_import(
                &path.join("dateindex_to_low"),
                Version::from(1),
                compressed,
            )?,
            dateindex_to_open_in_cents: StorableVec::forced_import(
                &path.join("dateindex_to_open_in_cents"),
                Version::from(1),
                compressed,
            )?,
            dateindex_to_open: StorableVec::forced_import(
                &path.join("dateindex_to_open"),
                Version::from(1),
                compressed,
            )?,
            dateindex_to_sats_per_dollar: StorableVec::forced_import(
                &path.join("dateindex_to_sats_per_dollar"),
                Version::from(1),
                compressed,
            )?,
            height_to_ohlc_in_cents: StorableVec::forced_import(
                &path.join("height_to_ohlc_in_cents"),
                Version::from(1),
                compressed,
            )?,
            height_to_ohlc: StorableVec::forced_import(
                &path.join("height_to_ohlc"),
                Version::from(1),
                compressed,
            )?,
            height_to_close_in_cents: StorableVec::forced_import(
                &path.join("height_to_close_in_cents"),
                Version::from(1),
                compressed,
            )?,
            height_to_close: StorableVec::forced_import(
                &path.join("height_to_close"),
                Version::from(1),
                compressed,
            )?,
            height_to_high_in_cents: StorableVec::forced_import(
                &path.join("height_to_high_in_cents"),
                Version::from(1),
                compressed,
            )?,
            height_to_high: StorableVec::forced_import(
                &path.join("height_to_high"),
                Version::from(1),
                compressed,
            )?,
            height_to_low_in_cents: StorableVec::forced_import(
                &path.join("height_to_low_in_cents"),
                Version::from(1),
                compressed,
            )?,
            height_to_low: StorableVec::forced_import(
                &path.join("height_to_low"),
                Version::from(1),
                compressed,
            )?,
            height_to_open_in_cents: StorableVec::forced_import(
                &path.join("height_to_open_in_cents"),
                Version::from(1),
                compressed,
            )?,
            height_to_open: StorableVec::forced_import(
                &path.join("height_to_open"),
                Version::from(1),
                compressed,
            )?,
            height_to_sats_per_dollar: StorableVec::forced_import(
                &path.join("height_to_sats_per_dollar"),
                Version::from(1),
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

        self.height_to_open.compute_transform(
            starting_indexes.height,
            self.height_to_ohlc.mut_vec(),
            |(di, ohlc, ..)| (di, ohlc.open),
            exit,
        )?;

        self.height_to_high.compute_transform(
            starting_indexes.height,
            self.height_to_ohlc.mut_vec(),
            |(di, ohlc, ..)| (di, ohlc.high),
            exit,
        )?;

        self.height_to_low.compute_transform(
            starting_indexes.height,
            self.height_to_ohlc.mut_vec(),
            |(di, ohlc, ..)| (di, ohlc.low),
            exit,
        )?;

        self.height_to_close.compute_transform(
            starting_indexes.height,
            self.height_to_ohlc.mut_vec(),
            |(di, ohlc, ..)| (di, ohlc.close),
            exit,
        )?;

        self.height_to_sats_per_dollar.compute_transform(
            starting_indexes.height,
            self.height_to_close.mut_vec(),
            |(di, close, ..)| (di, Close::from(Sats::ONE_BTC / *close)),
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

        self.dateindex_to_open.compute_transform(
            starting_indexes.dateindex,
            self.dateindex_to_ohlc.mut_vec(),
            |(di, ohlc, ..)| (di, ohlc.open),
            exit,
        )?;

        self.dateindex_to_high.compute_transform(
            starting_indexes.dateindex,
            self.dateindex_to_ohlc.mut_vec(),
            |(di, ohlc, ..)| (di, ohlc.high),
            exit,
        )?;

        self.dateindex_to_low.compute_transform(
            starting_indexes.dateindex,
            self.dateindex_to_ohlc.mut_vec(),
            |(di, ohlc, ..)| (di, ohlc.low),
            exit,
        )?;

        self.dateindex_to_close.compute_transform(
            starting_indexes.dateindex,
            self.dateindex_to_ohlc.mut_vec(),
            |(di, ohlc, ..)| (di, ohlc.close),
            exit,
        )?;

        self.dateindex_to_sats_per_dollar.compute_transform(
            starting_indexes.dateindex,
            self.dateindex_to_close.mut_vec(),
            |(di, close, ..)| (di, Close::from(Sats::ONE_BTC / *close)),
            exit,
        )?;

        Ok(())
    }

    pub fn as_any_vecs(&self) -> Vec<&dyn AnyStorableVec> {
        vec![
            self.dateindex_to_close.any_vec(),
            self.dateindex_to_close_in_cents.any_vec(),
            self.dateindex_to_high.any_vec(),
            self.dateindex_to_high_in_cents.any_vec(),
            self.dateindex_to_low.any_vec(),
            self.dateindex_to_low_in_cents.any_vec(),
            self.dateindex_to_ohlc.any_vec(),
            self.dateindex_to_ohlc_in_cents.any_vec(),
            self.dateindex_to_open.any_vec(),
            self.dateindex_to_open_in_cents.any_vec(),
            self.dateindex_to_sats_per_dollar.any_vec(),
            self.height_to_close.any_vec(),
            self.height_to_close_in_cents.any_vec(),
            self.height_to_high.any_vec(),
            self.height_to_high_in_cents.any_vec(),
            self.height_to_low.any_vec(),
            self.height_to_low_in_cents.any_vec(),
            self.height_to_ohlc.any_vec(),
            self.height_to_ohlc_in_cents.any_vec(),
            self.height_to_open.any_vec(),
            self.height_to_open_in_cents.any_vec(),
            self.height_to_sats_per_dollar.any_vec(),
        ]
    }
}
