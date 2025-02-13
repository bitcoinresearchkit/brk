use std::collections::BTreeMap;

use allocative::Allocative;
use chrono::Days;
use color_eyre::eyre::Error;

use struct_iterable::Iterable;

use crate::{
    parser::price::{Binance, Kibo, Kraken},
    structs::{
        Amount, BiMap, Config, Date, DateMap, DateMapChunkId, Height, HeightMapChunkId, MapKey, MapKind, Timestamp,
        OHLC,
    },
    utils::{ONE_MONTH_IN_DAYS, ONE_WEEK_IN_DAYS, ONE_YEAR_IN_DAYS},
};

use super::{AnyDataset, ComputeData, MinInitialStates, RatioDataset};

#[derive(Allocative, Iterable)]
pub struct PriceDatasets {
    min_initial_states: MinInitialStates,

    kraken_daily: Option<BTreeMap<Date, OHLC>>,
    kraken_1mn: Option<BTreeMap<u32, OHLC>>,
    binance_1mn: Option<BTreeMap<u32, OHLC>>,
    binance_daily: Option<BTreeMap<Date, OHLC>>,
    binance_har: Option<BTreeMap<u32, OHLC>>,
    kibo_by_height: BTreeMap<HeightMapChunkId, Vec<OHLC>>,
    kibo_by_date: BTreeMap<DateMapChunkId, BTreeMap<Date, OHLC>>,

    pub ohlc: BiMap<OHLC>,
    pub open: BiMap<f32>,
    pub high: BiMap<f32>,
    pub low: BiMap<f32>,
    pub close: BiMap<f32>,
}

impl PriceDatasets {
    pub fn import(config: &Config) -> color_eyre::Result<Self> {
        let path_dataset = config.path_datasets();
        let f = |s: &str| path_dataset.join(s);

        let mut s = Self {
            min_initial_states: MinInitialStates::default(),

            binance_1mn: None,
            binance_daily: None,
            binance_har: None,
            kraken_1mn: None,
            kraken_daily: None,
            kibo_by_height: BTreeMap::default(),
            kibo_by_date: BTreeMap::default(),

            // ---
            // Inserted
            // ---
            ohlc: BiMap::new_json(1, MapKind::Inserted, &config.path_price()),

            // ---
            // Computed
            // ---
            open_cents: BiMap::new_bin(1, MapKind::Computed, &f("open")),
            high_cents: BiMap::new_bin(1, MapKind::Computed, &f("high")),
            low_cents: BiMap::new_bin(1, MapKind::Computed, &f("low")),
            close: BiMap::new_bin(1, MapKind::Computed, &f("close")),
        };

        s.min_initial_states
            .consume(MinInitialStates::compute_from_dataset(&s, config));

        Ok(s)
    }

    pub fn compute(&mut self, compute_data: &ComputeData, circulating_supply: &mut BiMap<f64>) {
        let &ComputeData { dates, heights, .. } = compute_data;

        self.open
            .multi_insert_simple_transform(heights, dates, &mut self.ohlc, &|ohlc| ohlc.open);

        self.high
            .multi_insert_simple_transform(heights, dates, &mut self.ohlc, &|ohlc| ohlc.high);

        self.low
            .multi_insert_simple_transform(heights, dates, &mut self.ohlc, &|ohlc| ohlc.low);

        self.close
            .multi_insert_simple_transform(heights, dates, &mut self.ohlc, &|ohlc| ohlc.close);
    }

    pub fn get_date_ohlc(&mut self, date: Date) -> color_eyre::Result<OHLC> {
        if self.ohlc.date.is_key_safe(date) {
            Ok(self.ohlc.date.get_or_import(&date).unwrap().to_owned())
        } else {
            let ohlc = self
                .get_from_daily_kraken(&date)
                .or_else(|_| self.get_from_daily_binance(&date))
                .or_else(|_| self.get_from_date_kibo(&date))?;

            self.ohlc.date.insert(date, ohlc);

            Ok(ohlc)
        }
    }

    fn get_from_date_kibo(&mut self, date: &Date) -> color_eyre::Result<OHLC> {
        let chunk_id = date.to_chunk_id();

        #[allow(clippy::map_entry)]
        if !self.kibo_by_date.contains_key(&chunk_id)
            || self.kibo_by_date.get(&chunk_id).unwrap().last_key_value().unwrap().0 < date
        {
            self.kibo_by_date.insert(chunk_id, Kibo::fetch_date_prices(chunk_id)?);
        }

        self.kibo_by_date
            .get(&chunk_id)
            .unwrap()
            .get(date)
            .cloned()
            .ok_or(Error::msg("Couldn't find date in satonomics"))
    }

    fn get_from_daily_kraken(&mut self, date: &Date) -> color_eyre::Result<OHLC> {
        if self.kraken_daily.is_none() || self.kraken_daily.as_ref().unwrap().last_key_value().unwrap().0 < date {
            self.kraken_daily.replace(Kraken::fetch_daily_prices()?);
        }

        self.kraken_daily
            .as_ref()
            .unwrap()
            .get(date)
            .cloned()
            .ok_or(Error::msg("Couldn't find date"))
    }

    fn get_from_daily_binance(&mut self, date: &Date) -> color_eyre::Result<OHLC> {
        if self.binance_daily.is_none() || self.binance_daily.as_ref().unwrap().last_key_value().unwrap().0 < date {
            self.binance_daily.replace(Binance::fetch_daily_prices()?);
        }

        self.binance_daily
            .as_ref()
            .unwrap()
            .get(date)
            .cloned()
            .ok_or(Error::msg("Couldn't find date"))
    }

    pub fn get_height_ohlc(
        &mut self,
        height: Height,
        timestamp: Timestamp,
        previous_timestamp: Option<Timestamp>,
        config: &Config,
    ) -> color_eyre::Result<OHLC> {
        if let Some(ohlc) = self.ohlc.height.get_or_import(&height) {
            return Ok(ohlc);
        }

        let timestamp = timestamp.to_floored_seconds();

        if previous_timestamp.is_none() && !height.is_first() {
            panic!("Shouldn't be possible");
        }

        let previous_timestamp = previous_timestamp.map(|t| t.to_floored_seconds());

        let ohlc = self
            .get_from_1mn_kraken(timestamp, previous_timestamp)
            .unwrap_or_else(|_| {
                self.get_from_1mn_binance(timestamp, previous_timestamp)
                    .unwrap_or_else(|_| {
                        self.get_from_har_binance(timestamp, previous_timestamp, config)
                            .unwrap_or_else(|_| {
                                self.get_from_height_kibo(&height).unwrap_or_else(|_| {
                                    let date = timestamp.to_date();

                                    panic!(
                                        "Can't find the price for: height: {height} - date: {date}
1mn APIs are limited to the last 16 hours for Binance's and the last 10 hours for Kraken's
How to fix this:
1. Go to https://www.binance.com/en/trade/BTC_USDT?type=spot
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
                    })
            });

        self.ohlc.height.insert(height, ohlc);

        Ok(ohlc)
    }

    fn get_from_height_kibo(&mut self, height: &Height) -> color_eyre::Result<OHLC> {
        let chunk_id = height.to_chunk_id();

        #[allow(clippy::map_entry)]
        if !self.kibo_by_height.contains_key(&chunk_id)
            || ((chunk_id.to_usize() + self.kibo_by_height.get(&chunk_id).unwrap().len()) <= height.to_usize())
        {
            self.kibo_by_height
                .insert(chunk_id, Kibo::fetch_height_prices(chunk_id)?);
        }

        self.kibo_by_height
            .get(&chunk_id)
            .unwrap()
            .get(height.to_serialized_key().to_usize())
            .cloned()
            .ok_or(Error::msg("Couldn't find height in kibo"))
    }

    fn get_from_1mn_kraken(
        &mut self,
        timestamp: Timestamp,
        previous_timestamp: Option<Timestamp>,
    ) -> color_eyre::Result<OHLC> {
        if self.kraken_1mn.is_none() || self.kraken_1mn.as_ref().unwrap().last_key_value().unwrap().0 <= &timestamp {
            self.kraken_1mn.replace(Kraken::fetch_1mn_prices()?);
        }

        Self::find_height_ohlc(&self.kraken_1mn, timestamp, previous_timestamp, "kraken 1m")
    }

    fn get_from_1mn_binance(
        &mut self,
        timestamp: Timestamp,
        previous_timestamp: Option<Timestamp>,
    ) -> color_eyre::Result<OHLC> {
        if self.binance_1mn.is_none() || self.binance_1mn.as_ref().unwrap().last_key_value().unwrap().0 <= &timestamp {
            self.binance_1mn.replace(Binance::fetch_1mn_prices()?);
        }

        Self::find_height_ohlc(&self.binance_1mn, timestamp, previous_timestamp, "binance 1m")
    }

    fn get_from_har_binance(
        &mut self,
        timestamp: Timestamp,
        previous_timestamp: Option<Timestamp>,
        config: &Config,
    ) -> color_eyre::Result<OHLC> {
        if self.binance_har.is_none() {
            self.binance_har
                .replace(Binance::read_har_file(config).unwrap_or_default());
        }

        Self::find_height_ohlc(&self.binance_har, timestamp, previous_timestamp, "binance har")
    }

    fn find_height_ohlc(
        tree: &Option<BTreeMap<u32, OHLC>>,
        timestamp: Timestamp,
        previous_timestamp: Option<Timestamp>,
        name: &str,
    ) -> color_eyre::Result<OHLC> {
        let tree = tree.as_ref().unwrap();

        let err = Error::msg(format!("Couldn't find timestamp in {name}"));

        let previous_ohlc = previous_timestamp.map_or(Some(OHLC::default()), |previous_timestamp| {
            tree.get(&previous_timestamp).cloned()
        });

        let last_ohlc = tree.get(&timestamp);

        if previous_ohlc.is_none() || last_ohlc.is_none() {
            return Err(err);
        }

        let previous_ohlc = previous_ohlc.unwrap();

        let mut final_ohlc = OHLC {
            open: previous_ohlc.close,
            high: previous_ohlc.close,
            low: previous_ohlc.close,
            close: previous_ohlc.close,
        };

        let start = previous_timestamp.unwrap_or_default();
        let end = timestamp;

        // Otherwise it's a re-org
        if start < end {
            tree.range(&*start..=&*end).skip(1).for_each(|(_, ohlc)| {
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
}

impl AnyDataset for PriceDatasets {
    fn get_min_initial_states(&self) -> &MinInitialStates {
        &self.min_initial_states
    }
}
