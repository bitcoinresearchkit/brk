mod ohlc;

use std::collections::BTreeMap;

use allocative::Allocative;
use chrono::Days;
use color_eyre::eyre::Error;

pub use ohlc::*;

use crate::{
    price::{Binance, Kibo, Kraken},
    structs::{
        Amount, AnyBiMap, AnyDateMap, BiMap, Date, DateMap, DateMapChunkId, Height,
        HeightMapChunkId, MapKey, Timestamp,
    },
    utils::{ONE_MONTH_IN_DAYS, ONE_WEEK_IN_DAYS, ONE_YEAR_IN_DAYS},
};

use super::{AnyDataset, ComputeData, MinInitialStates, RatioDataset};

#[derive(Allocative)]
pub struct PriceDatasets {
    min_initial_states: MinInitialStates,

    kraken_daily: Option<BTreeMap<Date, OHLC>>,
    kraken_1mn: Option<BTreeMap<u32, OHLC>>,
    binance_1mn: Option<BTreeMap<u32, OHLC>>,
    binance_daily: Option<BTreeMap<Date, OHLC>>,
    binance_har: Option<BTreeMap<u32, OHLC>>,
    kibo_by_height: BTreeMap<HeightMapChunkId, Vec<OHLC>>,
    kibo_by_date: BTreeMap<DateMapChunkId, BTreeMap<Date, OHLC>>,

    // Inserted
    pub ohlcs: BiMap<OHLC>,

    // Computed
    pub closes: BiMap<f32>,
    pub market_cap: BiMap<f32>,
    pub price_1w_sma: BiMap<f32>,
    pub price_1w_sma_ratio: RatioDataset,
    pub price_1m_sma: BiMap<f32>,
    pub price_1m_sma_ratio: RatioDataset,
    pub price_1y_sma: BiMap<f32>,
    pub price_1y_sma_ratio: RatioDataset,
    pub price_2y_sma: BiMap<f32>,
    pub price_2y_sma_ratio: RatioDataset,
    pub price_4y_sma: BiMap<f32>,
    pub price_4y_sma_ratio: RatioDataset,
    pub price_8d_sma: BiMap<f32>,
    pub price_8d_sma_ratio: RatioDataset,
    pub price_13d_sma: BiMap<f32>,
    pub price_13d_sma_ratio: RatioDataset,
    pub price_21d_sma: BiMap<f32>,
    pub price_21d_sma_ratio: RatioDataset,
    pub price_34d_sma: BiMap<f32>,
    pub price_34d_sma_ratio: RatioDataset,
    pub price_55d_sma: BiMap<f32>,
    pub price_55d_sma_ratio: RatioDataset,
    pub price_89d_sma: BiMap<f32>,
    pub price_89d_sma_ratio: RatioDataset,
    pub price_144d_sma: BiMap<f32>,
    pub price_144d_sma_ratio: RatioDataset,
    pub price_200w_sma: BiMap<f32>,
    pub price_200w_sma_ratio: RatioDataset,
    pub price_1d_total_return: DateMap<f32>,
    pub price_1m_total_return: DateMap<f32>,
    pub price_6m_total_return: DateMap<f32>,
    pub price_1y_total_return: DateMap<f32>,
    pub price_2y_total_return: DateMap<f32>,
    pub price_3y_total_return: DateMap<f32>,
    pub price_4y_total_return: DateMap<f32>,
    pub price_6y_total_return: DateMap<f32>,
    pub price_8y_total_return: DateMap<f32>,
    pub price_10y_total_return: DateMap<f32>,
    pub price_4y_compound_return: DateMap<f32>,
    // projection via lowest 4y compound value
    pub all_time_high: BiMap<f32>,
    pub market_price_to_all_time_high_ratio: BiMap<f32>,
    pub drawdown: BiMap<f32>,
    pub sats_per_dollar: BiMap<f32>,
    // volatility
}

impl PriceDatasets {
    pub fn import(datasets_path: &str) -> color_eyre::Result<Self> {
        let price_path = "../price";

        let f = |s: &str| format!("{datasets_path}/{s}");

        let mut s = Self {
            min_initial_states: MinInitialStates::default(),

            binance_1mn: None,
            binance_daily: None,
            binance_har: None,
            kraken_1mn: None,
            kraken_daily: None,
            kibo_by_height: BTreeMap::default(),
            kibo_by_date: BTreeMap::default(),

            ohlcs: BiMap::new_json(1, price_path),
            closes: BiMap::new_bin(1, &f("close")),
            market_cap: BiMap::new_bin(1, &f("market_cap")),
            price_1w_sma: BiMap::new_bin(1, &f("price_1w_sma")),
            price_1w_sma_ratio: RatioDataset::import(datasets_path, "price_1w_sma")?,
            price_1m_sma: BiMap::new_bin(1, &f("price_1m_sma")),
            price_1m_sma_ratio: RatioDataset::import(datasets_path, "price_1m_sma")?,
            price_1y_sma: BiMap::new_bin(1, &f("price_1y_sma")),
            price_1y_sma_ratio: RatioDataset::import(datasets_path, "price_1y_sma")?,
            price_2y_sma: BiMap::new_bin(1, &f("price_2y_sma")),
            price_2y_sma_ratio: RatioDataset::import(datasets_path, "price_2y_sma")?,
            price_4y_sma: BiMap::new_bin(1, &f("price_4y_sma")),
            price_4y_sma_ratio: RatioDataset::import(datasets_path, "price_4y_sma")?,
            price_8d_sma: BiMap::new_bin(1, &f("price_8d_sma")),
            price_8d_sma_ratio: RatioDataset::import(datasets_path, "price_8d_sma")?,
            price_13d_sma: BiMap::new_bin(1, &f("price_13d_sma")),
            price_13d_sma_ratio: RatioDataset::import(datasets_path, "price_13d_sma")?,
            price_21d_sma: BiMap::new_bin(1, &f("price_21d_sma")),
            price_21d_sma_ratio: RatioDataset::import(datasets_path, "price_21d_sma")?,
            price_34d_sma: BiMap::new_bin(1, &f("price_34d_sma")),
            price_34d_sma_ratio: RatioDataset::import(datasets_path, "price_34d_sma")?,
            price_55d_sma: BiMap::new_bin(1, &f("price_55d_sma")),
            price_55d_sma_ratio: RatioDataset::import(datasets_path, "price_55d_sma")?,
            price_89d_sma: BiMap::new_bin(1, &f("price_89d_sma")),
            price_89d_sma_ratio: RatioDataset::import(datasets_path, "price_89d_sma")?,
            price_144d_sma: BiMap::new_bin(1, &f("price_144d_sma")),
            price_144d_sma_ratio: RatioDataset::import(datasets_path, "price_144d_sma")?,
            price_200w_sma: BiMap::new_bin(1, &f("price_200w_sma")),
            price_200w_sma_ratio: RatioDataset::import(datasets_path, "price_200w_sma")?,
            price_1d_total_return: DateMap::new_bin(1, &f("price_1d_total_return")),
            price_1m_total_return: DateMap::new_bin(1, &f("price_1m_total_return")),
            price_6m_total_return: DateMap::new_bin(1, &f("price_6m_total_return")),
            price_1y_total_return: DateMap::new_bin(1, &f("price_1y_total_return")),
            price_2y_total_return: DateMap::new_bin(1, &f("price_2y_total_return")),
            price_3y_total_return: DateMap::new_bin(1, &f("price_3y_total_return")),
            price_4y_total_return: DateMap::new_bin(1, &f("price_4y_total_return")),
            price_6y_total_return: DateMap::new_bin(1, &f("price_6y_total_return")),
            price_8y_total_return: DateMap::new_bin(1, &f("price_8y_total_return")),
            price_10y_total_return: DateMap::new_bin(1, &f("price_10y_total_return")),
            price_4y_compound_return: DateMap::new_bin(1, &f("price_4y_compound_return")),
            all_time_high: BiMap::new_bin(1, &f("all_time_high")),
            market_price_to_all_time_high_ratio: BiMap::new_bin(
                1,
                &f("market_price_to_all_time_high_ratio"),
            ),
            drawdown: BiMap::new_bin(1, &f("drawdown")),
            sats_per_dollar: BiMap::new_bin(1, &f("sats_per_dollar")),
        };

        s.min_initial_states
            .consume(MinInitialStates::compute_from_dataset(&s));

        Ok(s)
    }

    pub fn compute(&mut self, compute_data: &ComputeData, circulating_supply: &mut BiMap<f64>) {
        let &ComputeData { dates, heights, .. } = compute_data;

        self.closes
            .multi_insert_simple_transform(heights, dates, &mut self.ohlcs, &|ohlc| ohlc.close);

        self.market_cap
            .multi_insert_multiply(heights, dates, &mut self.closes, circulating_supply);

        self.price_1w_sma.multi_insert_simple_average(
            heights,
            dates,
            &mut self.closes,
            ONE_WEEK_IN_DAYS,
        );

        self.price_1m_sma.multi_insert_simple_average(
            heights,
            dates,
            &mut self.closes,
            ONE_MONTH_IN_DAYS,
        );

        self.price_1y_sma.multi_insert_simple_average(
            heights,
            dates,
            &mut self.closes,
            ONE_YEAR_IN_DAYS,
        );

        self.price_2y_sma.multi_insert_simple_average(
            heights,
            dates,
            &mut self.closes,
            2 * ONE_YEAR_IN_DAYS,
        );

        self.price_4y_sma.multi_insert_simple_average(
            heights,
            dates,
            &mut self.closes,
            4 * ONE_YEAR_IN_DAYS,
        );

        self.price_8d_sma
            .multi_insert_simple_average(heights, dates, &mut self.closes, 8);

        self.price_13d_sma
            .multi_insert_simple_average(heights, dates, &mut self.closes, 13);

        self.price_21d_sma
            .multi_insert_simple_average(heights, dates, &mut self.closes, 21);

        self.price_34d_sma
            .multi_insert_simple_average(heights, dates, &mut self.closes, 34);

        self.price_55d_sma
            .multi_insert_simple_average(heights, dates, &mut self.closes, 55);

        self.price_89d_sma
            .multi_insert_simple_average(heights, dates, &mut self.closes, 89);

        self.price_144d_sma
            .multi_insert_simple_average(heights, dates, &mut self.closes, 144);

        self.price_200w_sma.multi_insert_simple_average(
            heights,
            dates,
            &mut self.closes,
            200 * ONE_WEEK_IN_DAYS,
        );

        self.price_1d_total_return
            .multi_insert_percentage_change(dates, &mut self.closes.date, 1);
        self.price_1m_total_return.multi_insert_percentage_change(
            dates,
            &mut self.closes.date,
            ONE_MONTH_IN_DAYS,
        );
        self.price_6m_total_return.multi_insert_percentage_change(
            dates,
            &mut self.closes.date,
            6 * ONE_MONTH_IN_DAYS,
        );
        self.price_1y_total_return.multi_insert_percentage_change(
            dates,
            &mut self.closes.date,
            ONE_YEAR_IN_DAYS,
        );
        self.price_2y_total_return.multi_insert_percentage_change(
            dates,
            &mut self.closes.date,
            2 * ONE_YEAR_IN_DAYS,
        );
        self.price_3y_total_return.multi_insert_percentage_change(
            dates,
            &mut self.closes.date,
            3 * ONE_YEAR_IN_DAYS,
        );
        self.price_4y_total_return.multi_insert_percentage_change(
            dates,
            &mut self.closes.date,
            4 * ONE_YEAR_IN_DAYS,
        );
        self.price_6y_total_return.multi_insert_percentage_change(
            dates,
            &mut self.closes.date,
            6 * ONE_YEAR_IN_DAYS,
        );
        self.price_8y_total_return.multi_insert_percentage_change(
            dates,
            &mut self.closes.date,
            8 * ONE_YEAR_IN_DAYS,
        );
        self.price_10y_total_return.multi_insert_percentage_change(
            dates,
            &mut self.closes.date,
            10 * ONE_YEAR_IN_DAYS,
        );

        self.price_4y_compound_return
            .multi_insert_complex_transform(
                dates,
                &mut self.closes.date,
                |(last_value, date, closes)| {
                    let previous_value = date
                        .checked_sub_days(Days::new(4 * ONE_YEAR_IN_DAYS as u64))
                        .and_then(|date| closes.get_or_import(&Date::wrap(date)))
                        .unwrap_or_default();

                    (((last_value / previous_value).powf(1.0 / 4.0)) - 1.0) * 100.0
                },
            );

        self.price_1w_sma_ratio
            .compute(compute_data, &mut self.closes, &mut self.price_1w_sma);
        self.price_1m_sma_ratio
            .compute(compute_data, &mut self.closes, &mut self.price_1m_sma);
        self.price_1y_sma_ratio
            .compute(compute_data, &mut self.closes, &mut self.price_1y_sma);
        self.price_2y_sma_ratio
            .compute(compute_data, &mut self.closes, &mut self.price_2y_sma);
        self.price_4y_sma_ratio
            .compute(compute_data, &mut self.closes, &mut self.price_4y_sma);
        self.price_8d_sma_ratio
            .compute(compute_data, &mut self.closes, &mut self.price_8d_sma);
        self.price_13d_sma_ratio
            .compute(compute_data, &mut self.closes, &mut self.price_13d_sma);
        self.price_21d_sma_ratio
            .compute(compute_data, &mut self.closes, &mut self.price_21d_sma);
        self.price_34d_sma_ratio
            .compute(compute_data, &mut self.closes, &mut self.price_34d_sma);
        self.price_55d_sma_ratio
            .compute(compute_data, &mut self.closes, &mut self.price_55d_sma);
        self.price_89d_sma_ratio
            .compute(compute_data, &mut self.closes, &mut self.price_89d_sma);
        self.price_144d_sma_ratio
            .compute(compute_data, &mut self.closes, &mut self.price_144d_sma);
        self.price_200w_sma_ratio
            .compute(compute_data, &mut self.closes, &mut self.price_200w_sma);

        self.all_time_high
            .multi_insert_max(heights, dates, &mut self.closes);

        self.market_price_to_all_time_high_ratio
            .multi_insert_percentage(heights, dates, &mut self.closes, &mut self.all_time_high);

        self.drawdown.multi_insert_simple_transform(
            heights,
            dates,
            &mut self.market_price_to_all_time_high_ratio,
            &|v| -(100.0 - v),
        );

        self.sats_per_dollar.multi_insert_simple_transform(
            heights,
            dates,
            &mut self.closes,
            &|price| Amount::ONE_BTC_F32 / price,
        );
    }

    pub fn get_date_ohlc(&mut self, date: Date) -> color_eyre::Result<OHLC> {
        if self.ohlcs.date.is_key_safe(date) {
            Ok(self.ohlcs.date.get(&date).unwrap().to_owned())
        } else {
            let ohlc = self
                .get_from_daily_kraken(&date)
                .or_else(|_| self.get_from_daily_binance(&date))
                .or_else(|_| self.get_from_date_kibo(&date))?;

            self.ohlcs.date.insert(date, ohlc);

            Ok(ohlc)
        }
    }

    fn get_from_date_kibo(&mut self, date: &Date) -> color_eyre::Result<OHLC> {
        let chunk_id = date.to_chunk_id();

        #[allow(clippy::map_entry)]
        if !self.kibo_by_date.contains_key(&chunk_id)
            || self
                .kibo_by_date
                .get(&chunk_id)
                .unwrap()
                .last_key_value()
                .unwrap()
                .0
                < date
        {
            self.kibo_by_date
                .insert(chunk_id, Kibo::fetch_date_prices(chunk_id)?);
        }

        self.kibo_by_date
            .get(&chunk_id)
            .unwrap()
            .get(date)
            .cloned()
            .ok_or(Error::msg("Couldn't find date in satonomics"))
    }

    fn get_from_daily_kraken(&mut self, date: &Date) -> color_eyre::Result<OHLC> {
        if self.kraken_daily.is_none()
            || self
                .kraken_daily
                .as_ref()
                .unwrap()
                .last_key_value()
                .unwrap()
                .0
                < date
        {
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
        if self.binance_daily.is_none()
            || self
                .binance_daily
                .as_ref()
                .unwrap()
                .last_key_value()
                .unwrap()
                .0
                < date
        {
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
    ) -> color_eyre::Result<OHLC> {
        if let Some(ohlc) = self.ohlcs.height.get(&height) {
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
                        self.get_from_har_binance(timestamp, previous_timestamp)
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

        self.ohlcs.height.insert(height, ohlc);

        Ok(ohlc)
    }

    fn get_from_height_kibo(&mut self, height: &Height) -> color_eyre::Result<OHLC> {
        let chunk_id = height.to_chunk_id();

        #[allow(clippy::map_entry)]
        if !self.kibo_by_height.contains_key(&chunk_id)
            || ((chunk_id.to_usize() + self.kibo_by_height.get(&chunk_id).unwrap().len())
                <= height.to_usize())
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
        if self.kraken_1mn.is_none()
            || self
                .kraken_1mn
                .as_ref()
                .unwrap()
                .last_key_value()
                .unwrap()
                .0
                <= &timestamp
        {
            self.kraken_1mn.replace(Kraken::fetch_1mn_prices()?);
        }

        Self::find_height_ohlc(&self.kraken_1mn, timestamp, previous_timestamp, "kraken 1m")
    }

    fn get_from_1mn_binance(
        &mut self,
        timestamp: Timestamp,
        previous_timestamp: Option<Timestamp>,
    ) -> color_eyre::Result<OHLC> {
        if self.binance_1mn.is_none()
            || self
                .binance_1mn
                .as_ref()
                .unwrap()
                .last_key_value()
                .unwrap()
                .0
                <= &timestamp
        {
            self.binance_1mn.replace(Binance::fetch_1mn_prices()?);
        }

        Self::find_height_ohlc(
            &self.binance_1mn,
            timestamp,
            previous_timestamp,
            "binance 1m",
        )
    }

    fn get_from_har_binance(
        &mut self,
        timestamp: Timestamp,
        previous_timestamp: Option<Timestamp>,
    ) -> color_eyre::Result<OHLC> {
        if self.binance_har.is_none() {
            self.binance_har
                .replace(Binance::read_har_file().unwrap_or_default());
        }

        Self::find_height_ohlc(
            &self.binance_har,
            timestamp,
            previous_timestamp,
            "binance har",
        )
    }

    fn find_height_ohlc(
        tree: &Option<BTreeMap<u32, OHLC>>,
        timestamp: Timestamp,
        previous_timestamp: Option<Timestamp>,
        name: &str,
    ) -> color_eyre::Result<OHLC> {
        let tree = tree.as_ref().unwrap();

        let err = Error::msg(format!("Couldn't find timestamp in {name}"));

        let previous_ohlc = previous_timestamp
            .map_or(Some(OHLC::default()), |previous_timestamp| {
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

    fn to_inserted_bi_map_vec(&self) -> Vec<&(dyn AnyBiMap + Send + Sync)> {
        vec![&self.ohlcs]
    }

    fn to_inserted_mut_bi_map_vec(&mut self) -> Vec<&mut dyn AnyBiMap> {
        vec![&mut self.ohlcs]
    }

    fn to_computed_bi_map_vec(&self) -> Vec<&(dyn AnyBiMap + Send + Sync)> {
        let mut v = vec![
            &self.closes as &(dyn AnyBiMap + Send + Sync),
            &self.market_cap,
            &self.price_1w_sma,
            &self.price_1m_sma,
            &self.price_1y_sma,
            &self.price_2y_sma,
            &self.price_4y_sma,
            &self.price_8d_sma,
            &self.price_13d_sma,
            &self.price_21d_sma,
            &self.price_34d_sma,
            &self.price_55d_sma,
            &self.price_89d_sma,
            &self.price_144d_sma,
            &self.price_200w_sma,
            &self.all_time_high,
            &self.market_price_to_all_time_high_ratio,
            &self.drawdown,
            &self.sats_per_dollar,
        ];

        v.append(&mut self.price_1w_sma_ratio.to_computed_bi_map_vec());
        v.append(&mut self.price_1m_sma_ratio.to_computed_bi_map_vec());
        v.append(&mut self.price_1y_sma_ratio.to_computed_bi_map_vec());
        v.append(&mut self.price_2y_sma_ratio.to_computed_bi_map_vec());
        v.append(&mut self.price_4y_sma_ratio.to_computed_bi_map_vec());
        v.append(&mut self.price_8d_sma_ratio.to_computed_bi_map_vec());
        v.append(&mut self.price_13d_sma_ratio.to_computed_bi_map_vec());
        v.append(&mut self.price_21d_sma_ratio.to_computed_bi_map_vec());
        v.append(&mut self.price_34d_sma_ratio.to_computed_bi_map_vec());
        v.append(&mut self.price_55d_sma_ratio.to_computed_bi_map_vec());
        v.append(&mut self.price_89d_sma_ratio.to_computed_bi_map_vec());
        v.append(&mut self.price_144d_sma_ratio.to_computed_bi_map_vec());
        v.append(&mut self.price_200w_sma_ratio.to_computed_bi_map_vec());

        v
    }

    fn to_computed_mut_bi_map_vec(&mut self) -> Vec<&mut dyn AnyBiMap> {
        let mut v = vec![
            &mut self.closes as &mut dyn AnyBiMap,
            &mut self.market_cap,
            &mut self.price_1w_sma,
            &mut self.price_1m_sma,
            &mut self.price_1y_sma,
            &mut self.price_2y_sma,
            &mut self.price_4y_sma,
            &mut self.price_8d_sma,
            &mut self.price_13d_sma,
            &mut self.price_21d_sma,
            &mut self.price_34d_sma,
            &mut self.price_55d_sma,
            &mut self.price_89d_sma,
            &mut self.price_144d_sma,
            &mut self.price_200w_sma,
            &mut self.all_time_high,
            &mut self.market_price_to_all_time_high_ratio,
            &mut self.drawdown,
            &mut self.sats_per_dollar,
        ];

        v.append(&mut self.price_1w_sma_ratio.to_computed_mut_bi_map_vec());
        v.append(&mut self.price_1m_sma_ratio.to_computed_mut_bi_map_vec());
        v.append(&mut self.price_1y_sma_ratio.to_computed_mut_bi_map_vec());
        v.append(&mut self.price_2y_sma_ratio.to_computed_mut_bi_map_vec());
        v.append(&mut self.price_4y_sma_ratio.to_computed_mut_bi_map_vec());
        v.append(&mut self.price_8d_sma_ratio.to_computed_mut_bi_map_vec());
        v.append(&mut self.price_13d_sma_ratio.to_computed_mut_bi_map_vec());
        v.append(&mut self.price_21d_sma_ratio.to_computed_mut_bi_map_vec());
        v.append(&mut self.price_34d_sma_ratio.to_computed_mut_bi_map_vec());
        v.append(&mut self.price_55d_sma_ratio.to_computed_mut_bi_map_vec());
        v.append(&mut self.price_89d_sma_ratio.to_computed_mut_bi_map_vec());
        v.append(&mut self.price_144d_sma_ratio.to_computed_mut_bi_map_vec());
        v.append(&mut self.price_200w_sma_ratio.to_computed_mut_bi_map_vec());

        v
    }

    fn to_computed_date_map_vec(&self) -> Vec<&(dyn AnyDateMap + Send + Sync)> {
        vec![
            &self.price_1d_total_return,
            &self.price_1m_total_return,
            &self.price_6m_total_return,
            &self.price_1y_total_return,
            &self.price_2y_total_return,
            &self.price_3y_total_return,
            &self.price_4y_total_return,
            &self.price_6y_total_return,
            &self.price_8y_total_return,
            &self.price_10y_total_return,
            &self.price_4y_compound_return,
        ]
    }

    fn to_computed_mut_date_map_vec(&mut self) -> Vec<&mut dyn AnyDateMap> {
        vec![
            &mut self.price_1d_total_return,
            &mut self.price_1m_total_return,
            &mut self.price_6m_total_return,
            &mut self.price_1y_total_return,
            &mut self.price_2y_total_return,
            &mut self.price_3y_total_return,
            &mut self.price_4y_total_return,
            &mut self.price_6y_total_return,
            &mut self.price_8y_total_return,
            &mut self.price_10y_total_return,
            &mut self.price_4y_compound_return,
        ]
    }
}
