#![doc = include_str!("../README.md")]
#![doc = "\n## Example\n\n```rust"]
#![doc = include_str!("../examples/main.rs")]
#![doc = "```"]

use std::{collections::BTreeMap, fs, path::Path, thread::sleep, time::Duration};

use brk_core::{Cents, Close, Date, Dollars, Height, High, Low, OHLCCents, Open, Timestamp};
use color_eyre::eyre::Error;

mod fetchers;

use fetchers::*;

#[derive(Clone)]
pub struct Fetcher {
    binance: Binance,
    kraken: Kraken,
    kibo: Kibo,
}

impl Fetcher {
    pub fn import(hars_path: Option<&Path>) -> color_eyre::Result<Self> {
        if let Some(path) = hars_path {
            fs::create_dir_all(path)?;
        }

        Ok(Self {
            binance: Binance::init(hars_path),
            kraken: Kraken::default(),
            kibo: Kibo::default(),
        })
    }

    pub fn get_date(&mut self, date: Date) -> color_eyre::Result<OHLCCents> {
        self.kraken
            .get_from_1d(&date)
            .or_else(|_| {
                // eprintln!("{e}");
                self.binance.get_from_1d(&date)
            })
            .or_else(|e| {
                eprintln!("{e}");
                self.kibo.get_from_date(&date)
            })
    }

    pub fn get_height(
        &mut self,
        height: Height,
        timestamp: Timestamp,
        previous_timestamp: Option<Timestamp>,
    ) -> color_eyre::Result<OHLCCents> {
        self.get_height_(height, timestamp, previous_timestamp, 0)
    }

    fn get_height_(
        &mut self,
        height: Height,
        timestamp: Timestamp,
        previous_timestamp: Option<Timestamp>,
        tries: usize,
    ) -> color_eyre::Result<OHLCCents> {
        let timestamp = timestamp.floor_seconds();

        if previous_timestamp.is_none() && height != Height::ZERO {
            panic!("Shouldn't be possible");
        }

        let previous_timestamp = previous_timestamp.map(|t| t.floor_seconds());

        let ohlc = self
            .kraken
            .get_from_1mn(timestamp, previous_timestamp)
            .unwrap_or_else(|_| {
                // eprintln!("{e}");
                self.binance
                    .get_from_1mn(timestamp, previous_timestamp)
                    .unwrap_or_else(|_| {
                        // eprintln!("{e}");
                        self.kibo.get_from_height(height).unwrap_or_else(|_| {
                            sleep(Duration::from_secs(30));

                            if tries < 8 * 60 * 2 {
                                self.clear();

                                return self
                                    .get_height_(height, timestamp, previous_timestamp, tries + 1)
                                    .unwrap();
                            }

                            let date = Date::from(timestamp);
                            // eprintln!("{e}");
                            panic!(
                                "
Can't find the price for: height: {height} - date: {date}
1mn APIs are limited to the last 16 hours for Binance's and the last 10 hours for Kraken's
How to fix this:
0. Try rerunning the program first, it usually fixes the problem
1. If it didn't, go to https://www.binance.com/en/trade/BTC_USDT?type=spot
2. Select 1mn interval
3. Open the inspector/dev tools
4. Go to the Network Tab
5. Filter URLs by 'uiKlines'
6. Go back to the chart and scroll until you pass the date mentioned few lines ago
7. Go back to the dev tools
8. Export to a har file (if there is no explicit button, click on the cog button)
9. Move the file to 'parser/imports/binance.har'
        "
                            )
                        })
                    })
            });

        // self.ohlc.height.insert(height, ohlc);

        Ok(ohlc)
    }

    fn find_height_ohlc(
        tree: &BTreeMap<Timestamp, OHLCCents>,
        timestamp: Timestamp,
        previous_timestamp: Option<Timestamp>,
        name: &str,
    ) -> color_eyre::Result<OHLCCents> {
        let previous_ohlc = previous_timestamp
            .map_or(Some(OHLCCents::default()), |previous_timestamp| {
                tree.get(&previous_timestamp).cloned()
            });

        let last_ohlc = tree.get(&timestamp);

        if previous_ohlc.is_none() || last_ohlc.is_none() {
            return Err(Error::msg(format!("Couldn't find timestamp in {name}")));
        }

        let previous_ohlc = previous_ohlc.unwrap();

        let mut final_ohlc = OHLCCents::from(previous_ohlc.close);

        let start = previous_timestamp.unwrap_or(Timestamp::new(0));
        let end = timestamp;

        // Otherwise it's a re-org
        if start < end {
            tree.range(start..=end).skip(1).for_each(|(_, ohlc)| {
                if ohlc.high > final_ohlc.high {
                    final_ohlc.high = ohlc.high
                }

                if ohlc.low < final_ohlc.low {
                    final_ohlc.low = ohlc.low
                }

                final_ohlc.close = ohlc.close;
            });
        }

        Ok(final_ohlc)
    }

    pub fn clear(&mut self) {
        self.binance.clear();
        self.kibo.clear();
        self.kraken.clear();
    }
}
