use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

use color_eyre::eyre::Error;

mod fetchers;
mod structs;

use brk_indexer::{Height, Indexer, Timestamp};
pub use fetchers::*;
use storable_vec::{AnyJsonStorableVec, AnyStorableVec, SINGLE_THREAD, StorableVec, Version};
pub use structs::*;

pub struct Pricer<const MODE: u8> {
    path: PathBuf,
    binance: Binance,
    kraken: Kraken,
    kibo: Kibo,

    pub dateindex_to_close_in_cents: StorableVec<Dateindex, Close<Cents>, MODE>,
    pub dateindex_to_close_in_dollars: StorableVec<Dateindex, Close<Dollars>, MODE>,
    pub dateindex_to_high_in_cents: StorableVec<Dateindex, High<Cents>, MODE>,
    pub dateindex_to_high_in_dollars: StorableVec<Dateindex, High<Dollars>, MODE>,
    pub dateindex_to_low_in_cents: StorableVec<Dateindex, Low<Cents>, MODE>,
    pub dateindex_to_low_in_dollars: StorableVec<Dateindex, Low<Dollars>, MODE>,
    pub dateindex_to_open_in_cents: StorableVec<Dateindex, Open<Cents>, MODE>,
    pub dateindex_to_open_in_dollars: StorableVec<Dateindex, Open<Dollars>, MODE>,
    pub height_to_close_in_cents: StorableVec<Height, Close<Cents>, MODE>,
    pub height_to_close_in_dollars: StorableVec<Height, Close<Dollars>, MODE>,
    pub height_to_high_in_cents: StorableVec<Height, High<Cents>, MODE>,
    pub height_to_high_in_dollars: StorableVec<Height, High<Dollars>, MODE>,
    pub height_to_low_in_cents: StorableVec<Height, Low<Cents>, MODE>,
    pub height_to_low_in_dollars: StorableVec<Height, Low<Dollars>, MODE>,
    pub height_to_open_in_cents: StorableVec<Height, Open<Cents>, MODE>,
    pub height_to_open_in_dollars: StorableVec<Height, Open<Dollars>, MODE>,
}

impl<const MODE: u8> Pricer<MODE> {
    pub fn import(path: &Path) -> color_eyre::Result<Self> {
        fs::create_dir_all(path)?;

        Ok(Self {
            path: path.to_owned(),
            binance: Binance::init(path),
            kraken: Kraken::default(),
            kibo: Kibo::default(),

            // binance_1mn: None,
            // binance_daily: None,
            // binance_har: None,
            // kraken_1mn: None,
            // kraken_daily: None,
            // kibo_by_height: BTreeMap::default(),
            // kibo_by_date: BTreeMap::default(),
            dateindex_to_close_in_cents: StorableVec::import(
                &path.join("dateindex_to_close_in_cents"),
                Version::from(1),
            )?,
            dateindex_to_close_in_dollars: StorableVec::import(
                &path.join("dateindex_to_close_in_dollars"),
                Version::from(1),
            )?,
            dateindex_to_high_in_cents: StorableVec::import(
                &path.join("dateindex_to_high_in_cents"),
                Version::from(1),
            )?,
            dateindex_to_high_in_dollars: StorableVec::import(
                &path.join("dateindex_to_high_in_dollars"),
                Version::from(1),
            )?,
            dateindex_to_low_in_cents: StorableVec::import(&path.join("dateindex_to_low_in_cents"), Version::from(1))?,
            dateindex_to_low_in_dollars: StorableVec::import(
                &path.join("dateindex_to_low_in_dollars"),
                Version::from(1),
            )?,
            dateindex_to_open_in_cents: StorableVec::import(
                &path.join("dateindex_to_open_in_cents"),
                Version::from(1),
            )?,
            dateindex_to_open_in_dollars: StorableVec::import(
                &path.join("dateindex_to_open_in_dollars"),
                Version::from(1),
            )?,
            height_to_close_in_cents: StorableVec::import(&path.join("height_to_close_in_cents"), Version::from(1))?,
            height_to_close_in_dollars: StorableVec::import(
                &path.join("height_to_close_in_dollars"),
                Version::from(1),
            )?,
            height_to_high_in_cents: StorableVec::import(&path.join("height_to_high_in_cents"), Version::from(1))?,
            height_to_high_in_dollars: StorableVec::import(&path.join("height_to_high_in_dollars"), Version::from(1))?,
            height_to_low_in_cents: StorableVec::import(&path.join("height_to_low_in_cents"), Version::from(1))?,
            height_to_low_in_dollars: StorableVec::import(&path.join("height_to_low_in_dollars"), Version::from(1))?,
            height_to_open_in_cents: StorableVec::import(&path.join("height_to_open_in_cents"), Version::from(1))?,
            height_to_open_in_dollars: StorableVec::import(&path.join("height_to_open_in_dollars"), Version::from(1))?,
        })
    }

    pub fn compute_if_needed(&mut self, indexer: &mut Indexer<SINGLE_THREAD>) {
        // TODO: Remove all outdated

        indexer
            .vecs
            .height_to_timestamp
            .iter_from(Height::default(), |v| Ok(()));

        // self.open
        //     .multi_insert_simple_transform(heights, dates, &mut self.ohlc, &|ohlc| ohlc.open);

        // self.high
        //     .multi_insert_simple_transform(heights, dates, &mut self.ohlc, &|ohlc| ohlc.high);

        // self.low
        //     .multi_insert_simple_transform(heights, dates, &mut self.ohlc, &|ohlc| ohlc.low);

        // self.close
        //     .multi_insert_simple_transform(heights, dates, &mut self.ohlc, &|ohlc| ohlc.close);
    }

    fn get_date_ohlc(&mut self, date: Date) -> color_eyre::Result<OHLC> {
        todo!();
        // if self.ohlc.date.is_key_safe(date) {
        //     Ok(self.ohlc.date.get_or_import(&date).unwrap().to_owned())
        // } else {
        //     let ohlc = self
        //         .get_from_daily_kraken(&date)
        //         .or_else(|_| self.get_from_daily_binance(&date))
        //         .or_else(|_| self.get_from_date_kibo(&date))?;

        //     self.ohlc.date.insert(date, ohlc);

        //     Ok(ohlc)
        // }
    }

    fn get_height_ohlc(
        &mut self,
        height: Height,
        timestamp: Timestamp,
        previous_timestamp: Option<Timestamp>,
    ) -> color_eyre::Result<OHLC> {
        todo!();

        //         if let Some(ohlc) = self.ohlc.height.get_or_import(&height) {
        //             return Ok(ohlc);
        //         }

        //         let timestamp = timestamp.to_floored_seconds();

        //         if previous_timestamp.is_none() && !height.is_first() {
        //             panic!("Shouldn't be possible");
        //         }

        //         let previous_timestamp = previous_timestamp.map(|t| t.to_floored_seconds());

        //         let ohlc = self
        //             .get_from_1mn_kraken(timestamp, previous_timestamp)
        //             .unwrap_or_else(|_| {
        //                 self.get_from_1mn_binance(timestamp, previous_timestamp)
        //                     .unwrap_or_else(|_| {
        //                         self.get_from_har_binance(timestamp, previous_timestamp, config)
        //                             .unwrap_or_else(|_| {
        //                                 self.get_from_height_kibo(&height).unwrap_or_else(|_| {
        //                                     let date = timestamp.to_date();

        //                                     panic!(
        //                                         "Can't find the price for: height: {height} - date: {date}
        // 1mn APIs are limited to the last 16 hours for Binance's and the last 10 hours for Kraken's
        // How to fix this:
        // 1. Go to https://www.binance.com/en/trade/BTC_USDT?type=spot
        // 2. Select 1mn interval
        // 3. Open the inspector/dev tools
        // 4. Go to the Network Tab
        // 5. Filter URLs by 'uiKlines'
        // 6. Go back to the chart and scroll until you pass the date mentioned few lines ago
        // 7. Go back to the dev tools
        // 8. Export to a har file (if there is no explicit button, click on the cog button)
        // 9. Move the file to 'parser/imports/binance.har'
        // "
        //                                     )
        //                                 })
        //                             })
        //                     })
        //             });

        //         // self.ohlc.height.insert(height, ohlc);

        //         Ok(ohlc)
    }

    fn find_height_ohlc(
        tree: &BTreeMap<Timestamp, OHLC>,
        timestamp: Timestamp,
        previous_timestamp: Option<Timestamp>,
        name: &str,
    ) -> color_eyre::Result<OHLC> {
        let previous_ohlc = previous_timestamp.map_or(Some(OHLC::default()), |previous_timestamp| {
            tree.get(&previous_timestamp).cloned()
        });

        let last_ohlc = tree.get(&timestamp);

        if previous_ohlc.is_none() || last_ohlc.is_none() {
            return Err(Error::msg(format!("Couldn't find timestamp in {name}")));
        }

        let previous_ohlc = previous_ohlc.unwrap();

        let mut final_ohlc = (
            Open::from(previous_ohlc.3),
            High::from(previous_ohlc.3),
            Low::from(previous_ohlc.3),
            previous_ohlc.3,
        );

        let start = previous_timestamp.unwrap_or(Timestamp::from(0));
        let end = timestamp;

        // Otherwise it's a re-org
        if start < end {
            tree.range(start..=end).skip(1).for_each(|(_, ohlc)| {
                if ohlc.1 > final_ohlc.1 {
                    final_ohlc.1 = ohlc.1
                }

                if ohlc.2 < final_ohlc.2 {
                    final_ohlc.2 = ohlc.2
                }

                final_ohlc.3 = ohlc.3;
            });
        }

        Ok(final_ohlc)
    }

    pub fn as_any_json_vec_slice(&self) -> [&dyn AnyJsonStorableVec; 16] {
        [
            &self.dateindex_to_close_in_cents as &dyn AnyJsonStorableVec,
            &self.dateindex_to_close_in_dollars,
            &self.dateindex_to_high_in_cents,
            &self.dateindex_to_high_in_dollars,
            &self.dateindex_to_low_in_cents,
            &self.dateindex_to_low_in_dollars,
            &self.dateindex_to_open_in_cents,
            &self.dateindex_to_open_in_dollars,
            &self.height_to_close_in_cents,
            &self.height_to_close_in_dollars,
            &self.height_to_high_in_cents,
            &self.height_to_high_in_dollars,
            &self.height_to_low_in_cents,
            &self.height_to_low_in_dollars,
            &self.height_to_open_in_cents,
            &self.height_to_open_in_dollars,
        ]
    }

    pub fn as_mut_any_vec_slice(&mut self) -> [&mut dyn AnyStorableVec; 16] {
        [
            &mut self.dateindex_to_close_in_cents as &mut dyn AnyStorableVec,
            &mut self.dateindex_to_close_in_dollars,
            &mut self.dateindex_to_high_in_cents,
            &mut self.dateindex_to_high_in_dollars,
            &mut self.dateindex_to_low_in_cents,
            &mut self.dateindex_to_low_in_dollars,
            &mut self.dateindex_to_open_in_cents,
            &mut self.dateindex_to_open_in_dollars,
            &mut self.height_to_close_in_cents,
            &mut self.height_to_close_in_dollars,
            &mut self.height_to_high_in_cents,
            &mut self.height_to_high_in_dollars,
            &mut self.height_to_low_in_cents,
            &mut self.height_to_low_in_dollars,
            &mut self.height_to_open_in_cents,
            &mut self.height_to_open_in_dollars,
        ]
    }
}
