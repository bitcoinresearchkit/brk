use std::collections::BTreeMap;

use allocative::Allocative;
use chrono::Days;
use color_eyre::eyre::Error;

use struct_iterable::Iterable;

use crate::{
    price::{Binance, Kibo, Kraken},
    structs::{
        Amount, BiMap, Config, Date, DateMap, DateMapChunkId, Height, HeightMapChunkId, MapKey,
        MapKind, Timestamp, OHLC,
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
    pub all_time_high_date: DateMap<Date>,
    pub days_since_all_time_high: DateMap<u32>,
    pub max_days_between_all_time_highs: DateMap<u32>,
    pub max_years_between_all_time_highs: DateMap<f32>,
    pub market_price_to_all_time_high_ratio: BiMap<f32>,
    pub drawdown: BiMap<f32>,
    pub sats_per_dollar: BiMap<f32>,
    // volatility
}

impl PriceDatasets {
    pub fn import(datasets_path: &str, config: &Config) -> color_eyre::Result<Self> {
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

            // ---
            // Inserted
            // ---
            ohlc: BiMap::new_json(1, MapKind::Inserted, price_path),

            // ---
            // Computed
            // ---
            open: BiMap::new_bin(1, MapKind::Computed, &f("open")),
            high: BiMap::new_bin(1, MapKind::Computed, &f("high")),
            low: BiMap::new_bin(1, MapKind::Computed, &f("low")),
            close: BiMap::new_bin(1, MapKind::Computed, &f("close")),
            market_cap: BiMap::new_bin(1, MapKind::Computed, &f("market_cap")),
            price_1w_sma: BiMap::new_bin(1, MapKind::Computed, &f("price_1w_sma")),
            price_1w_sma_ratio: RatioDataset::import(datasets_path, "price_1w_sma", config)?,
            price_1m_sma: BiMap::new_bin(1, MapKind::Computed, &f("price_1m_sma")),
            price_1m_sma_ratio: RatioDataset::import(datasets_path, "price_1m_sma", config)?,
            price_1y_sma: BiMap::new_bin(1, MapKind::Computed, &f("price_1y_sma")),
            price_1y_sma_ratio: RatioDataset::import(datasets_path, "price_1y_sma", config)?,
            price_2y_sma: BiMap::new_bin(1, MapKind::Computed, &f("price_2y_sma")),
            price_2y_sma_ratio: RatioDataset::import(datasets_path, "price_2y_sma", config)?,
            price_4y_sma: BiMap::new_bin(1, MapKind::Computed, &f("price_4y_sma")),
            price_4y_sma_ratio: RatioDataset::import(datasets_path, "price_4y_sma", config)?,
            price_8d_sma: BiMap::new_bin(1, MapKind::Computed, &f("price_8d_sma")),
            price_8d_sma_ratio: RatioDataset::import(datasets_path, "price_8d_sma", config)?,
            price_13d_sma: BiMap::new_bin(1, MapKind::Computed, &f("price_13d_sma")),
            price_13d_sma_ratio: RatioDataset::import(datasets_path, "price_13d_sma", config)?,
            price_21d_sma: BiMap::new_bin(1, MapKind::Computed, &f("price_21d_sma")),
            price_21d_sma_ratio: RatioDataset::import(datasets_path, "price_21d_sma", config)?,
            price_34d_sma: BiMap::new_bin(1, MapKind::Computed, &f("price_34d_sma")),
            price_34d_sma_ratio: RatioDataset::import(datasets_path, "price_34d_sma", config)?,
            price_55d_sma: BiMap::new_bin(1, MapKind::Computed, &f("price_55d_sma")),
            price_55d_sma_ratio: RatioDataset::import(datasets_path, "price_55d_sma", config)?,
            price_89d_sma: BiMap::new_bin(1, MapKind::Computed, &f("price_89d_sma")),
            price_89d_sma_ratio: RatioDataset::import(datasets_path, "price_89d_sma", config)?,
            price_144d_sma: BiMap::new_bin(1, MapKind::Computed, &f("price_144d_sma")),
            price_144d_sma_ratio: RatioDataset::import(datasets_path, "price_144d_sma", config)?,
            price_200w_sma: BiMap::new_bin(1, MapKind::Computed, &f("price_200w_sma")),
            price_200w_sma_ratio: RatioDataset::import(datasets_path, "price_200w_sma", config)?,
            price_1d_total_return: DateMap::new_bin(
                1,
                MapKind::Computed,
                &f("price_1d_total_return"),
            ),
            price_1m_total_return: DateMap::new_bin(
                1,
                MapKind::Computed,
                &f("price_1m_total_return"),
            ),
            price_6m_total_return: DateMap::new_bin(
                1,
                MapKind::Computed,
                &f("price_6m_total_return"),
            ),
            price_1y_total_return: DateMap::new_bin(
                1,
                MapKind::Computed,
                &f("price_1y_total_return"),
            ),
            price_2y_total_return: DateMap::new_bin(
                1,
                MapKind::Computed,
                &f("price_2y_total_return"),
            ),
            price_3y_total_return: DateMap::new_bin(
                1,
                MapKind::Computed,
                &f("price_3y_total_return"),
            ),
            price_4y_total_return: DateMap::new_bin(
                1,
                MapKind::Computed,
                &f("price_4y_total_return"),
            ),
            price_6y_total_return: DateMap::new_bin(
                1,
                MapKind::Computed,
                &f("price_6y_total_return"),
            ),
            price_8y_total_return: DateMap::new_bin(
                1,
                MapKind::Computed,
                &f("price_8y_total_return"),
            ),
            price_10y_total_return: DateMap::new_bin(
                1,
                MapKind::Computed,
                &f("price_10y_total_return"),
            ),
            price_4y_compound_return: DateMap::new_bin(
                1,
                MapKind::Computed,
                &f("price_4y_compound_return"),
            ),
            all_time_high: BiMap::new_bin(1, MapKind::Computed, &f("all_time_high")),
            all_time_high_date: DateMap::new_bin(1, MapKind::Computed, &f("all_time_high_date")),
            days_since_all_time_high: DateMap::new_bin(
                1,
                MapKind::Computed,
                &f("days_since_all_time_high"),
            ),
            max_days_between_all_time_highs: DateMap::new_bin(
                1,
                MapKind::Computed,
                &f("max_days_between_all_time_highs"),
            ),
            max_years_between_all_time_highs: DateMap::new_bin(
                2,
                MapKind::Computed,
                &f("max_years_between_all_time_highs"),
            ),
            market_price_to_all_time_high_ratio: BiMap::new_bin(
                1,
                MapKind::Computed,
                &f("market_price_to_all_time_high_ratio"),
            ),
            drawdown: BiMap::new_bin(1, MapKind::Computed, &f("drawdown")),
            sats_per_dollar: BiMap::new_bin(1, MapKind::Computed, &f("sats_per_dollar")),
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

        self.market_cap
            .multi_insert_multiply(heights, dates, &mut self.close, circulating_supply);

        self.price_1w_sma.multi_insert_simple_average(
            heights,
            dates,
            &mut self.close,
            ONE_WEEK_IN_DAYS,
        );

        self.price_1m_sma.multi_insert_simple_average(
            heights,
            dates,
            &mut self.close,
            ONE_MONTH_IN_DAYS,
        );

        self.price_1y_sma.multi_insert_simple_average(
            heights,
            dates,
            &mut self.close,
            ONE_YEAR_IN_DAYS,
        );

        self.price_2y_sma.multi_insert_simple_average(
            heights,
            dates,
            &mut self.close,
            2 * ONE_YEAR_IN_DAYS,
        );

        self.price_4y_sma.multi_insert_simple_average(
            heights,
            dates,
            &mut self.close,
            4 * ONE_YEAR_IN_DAYS,
        );

        self.price_8d_sma
            .multi_insert_simple_average(heights, dates, &mut self.close, 8);

        self.price_13d_sma
            .multi_insert_simple_average(heights, dates, &mut self.close, 13);

        self.price_21d_sma
            .multi_insert_simple_average(heights, dates, &mut self.close, 21);

        self.price_34d_sma
            .multi_insert_simple_average(heights, dates, &mut self.close, 34);

        self.price_55d_sma
            .multi_insert_simple_average(heights, dates, &mut self.close, 55);

        self.price_89d_sma
            .multi_insert_simple_average(heights, dates, &mut self.close, 89);

        self.price_144d_sma
            .multi_insert_simple_average(heights, dates, &mut self.close, 144);

        self.price_200w_sma.multi_insert_simple_average(
            heights,
            dates,
            &mut self.close,
            200 * ONE_WEEK_IN_DAYS,
        );

        self.price_1d_total_return
            .multi_insert_percentage_change(dates, &mut self.close.date, 1);
        self.price_1m_total_return.multi_insert_percentage_change(
            dates,
            &mut self.close.date,
            ONE_MONTH_IN_DAYS,
        );
        self.price_6m_total_return.multi_insert_percentage_change(
            dates,
            &mut self.close.date,
            6 * ONE_MONTH_IN_DAYS,
        );
        self.price_1y_total_return.multi_insert_percentage_change(
            dates,
            &mut self.close.date,
            ONE_YEAR_IN_DAYS,
        );
        self.price_2y_total_return.multi_insert_percentage_change(
            dates,
            &mut self.close.date,
            2 * ONE_YEAR_IN_DAYS,
        );
        self.price_3y_total_return.multi_insert_percentage_change(
            dates,
            &mut self.close.date,
            3 * ONE_YEAR_IN_DAYS,
        );
        self.price_4y_total_return.multi_insert_percentage_change(
            dates,
            &mut self.close.date,
            4 * ONE_YEAR_IN_DAYS,
        );
        self.price_6y_total_return.multi_insert_percentage_change(
            dates,
            &mut self.close.date,
            6 * ONE_YEAR_IN_DAYS,
        );
        self.price_8y_total_return.multi_insert_percentage_change(
            dates,
            &mut self.close.date,
            8 * ONE_YEAR_IN_DAYS,
        );
        self.price_10y_total_return.multi_insert_percentage_change(
            dates,
            &mut self.close.date,
            10 * ONE_YEAR_IN_DAYS,
        );

        self.price_4y_compound_return
            .multi_insert_complex_transform(
                dates,
                &mut self.close.date,
                |(last_value, date, closes, _)| {
                    let previous_value = date
                        .checked_sub_days(Days::new(4 * ONE_YEAR_IN_DAYS as u64))
                        .and_then(|date| closes.get_or_import(&Date::wrap(date)))
                        .unwrap_or_default();

                    (((last_value / previous_value).powf(1.0 / 4.0)) - 1.0) * 100.0
                },
            );

        self.price_1w_sma_ratio
            .compute(compute_data, &mut self.close, &mut self.price_1w_sma);
        self.price_1m_sma_ratio
            .compute(compute_data, &mut self.close, &mut self.price_1m_sma);
        self.price_1y_sma_ratio
            .compute(compute_data, &mut self.close, &mut self.price_1y_sma);
        self.price_2y_sma_ratio
            .compute(compute_data, &mut self.close, &mut self.price_2y_sma);
        self.price_4y_sma_ratio
            .compute(compute_data, &mut self.close, &mut self.price_4y_sma);
        self.price_8d_sma_ratio
            .compute(compute_data, &mut self.close, &mut self.price_8d_sma);
        self.price_13d_sma_ratio
            .compute(compute_data, &mut self.close, &mut self.price_13d_sma);
        self.price_21d_sma_ratio
            .compute(compute_data, &mut self.close, &mut self.price_21d_sma);
        self.price_34d_sma_ratio
            .compute(compute_data, &mut self.close, &mut self.price_34d_sma);
        self.price_55d_sma_ratio
            .compute(compute_data, &mut self.close, &mut self.price_55d_sma);
        self.price_89d_sma_ratio
            .compute(compute_data, &mut self.close, &mut self.price_89d_sma);
        self.price_144d_sma_ratio
            .compute(compute_data, &mut self.close, &mut self.price_144d_sma);
        self.price_200w_sma_ratio
            .compute(compute_data, &mut self.close, &mut self.price_200w_sma);

        self.all_time_high
            .multi_insert_max(heights, dates, &mut self.high);

        self.market_price_to_all_time_high_ratio
            .multi_insert_percentage(heights, dates, &mut self.close, &mut self.all_time_high);

        self.all_time_high_date.multi_insert_complex_transform(
            dates,
            &mut self.all_time_high.date,
            |(value, date, _, map)| {
                let high = self.high.date.get_or_import(date).unwrap();
                let is_ath = high == value;

                if is_ath {
                    *date
                } else {
                    let previous_date = date.checked_sub(1).unwrap();
                    *map.get_or_import(&previous_date).as_ref().unwrap_or(date)
                }
            },
        );

        self.days_since_all_time_high.multi_insert_simple_transform(
            dates,
            &mut self.all_time_high_date,
            |value, key| key.difference_in_days_between(value),
        );

        self.max_days_between_all_time_highs
            .multi_insert_max(dates, &mut self.days_since_all_time_high);

        self.max_years_between_all_time_highs
            .multi_insert_simple_transform(
                dates,
                &mut self.max_days_between_all_time_highs,
                |days, _| (days as f64 / ONE_YEAR_IN_DAYS as f64) as f32,
            );

        self.drawdown.multi_insert_simple_transform(
            heights,
            dates,
            &mut self.market_price_to_all_time_high_ratio,
            &|v| -(100.0 - v),
        );

        self.sats_per_dollar.multi_insert_simple_transform(
            heights,
            dates,
            &mut self.close,
            &|price| Amount::ONE_BTC_F32 / price,
        );
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

        self.ohlc.height.insert(height, ohlc);

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
}
