use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

use brk_core::{Cents, Close, Date, Dateindex, Dollars, Height, High, Low, OHLCCents, Open, Timestamp};
use color_eyre::eyre::Error;

mod fetchers;

// use brk_indexer::Indexer;
pub use fetchers::*;
use storable_vec::{AnyJsonStorableVec, AnyStorableVec, SINGLE_THREAD, StorableVec, Version};

pub struct Pricer<const MODE: u8> {
    binance: Binance,
    kraken: Kraken,
    kibo: Kibo,
}

impl<const MODE: u8> Pricer<MODE> {
    pub fn import(path: &Path) -> color_eyre::Result<Self> {
        fs::create_dir_all(path)?;

        Ok(Self {
            binance: Binance::init(path),
            kraken: Kraken::default(),
            kibo: Kibo::default(),
        })
    }

    pub fn compute_if_needed(&mut self) {
        // TODO: Remove all outdated

        // indexer
        //     .vecs
        //     .height_to_timestamp
        //     .iter_from(Height::default(), |v| Ok(()));

        // self.open
        //     .multi_insert_simple_transform(heights, dates, &mut self.ohlc, &|ohlc| ohlc.open);

        // self.high
        //     .multi_insert_simple_transform(heights, dates, &mut self.ohlc, &|ohlc| ohlc.high);

        // self.low
        //     .multi_insert_simple_transform(heights, dates, &mut self.ohlc, &|ohlc| ohlc.low);

        // self.close
        //     .multi_insert_simple_transform(heights, dates, &mut self.ohlc, &|ohlc| ohlc.close);
    }

    fn get_date_ohlc(&mut self, date: Date) -> color_eyre::Result<OHLCCents> {
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
    ) -> color_eyre::Result<OHLCCents> {
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
        tree: &BTreeMap<Timestamp, OHLCCents>,
        timestamp: Timestamp,
        previous_timestamp: Option<Timestamp>,
        name: &str,
    ) -> color_eyre::Result<OHLCCents> {
        let previous_ohlc = previous_timestamp.map_or(Some(OHLCCents::default()), |previous_timestamp| {
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
}
