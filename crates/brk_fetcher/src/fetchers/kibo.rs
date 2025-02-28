use std::{collections::BTreeMap, str::FromStr};

use brk_core::{Date, Height, OHLCCents};
use color_eyre::eyre::ContextCompat;
use log::info;
use serde_json::Value;

use crate::{Cents, Close, Dollars, High, Low, Open, fetchers::retry};

#[derive(Default)]
pub struct Kibo {
    height_to_ohlc_vec: BTreeMap<Height, Vec<OHLCCents>>,
    year_to_date_to_ohlc: BTreeMap<u16, BTreeMap<Date, OHLCCents>>,
}

const KIBO_OFFICIAL_URL: &str = "https://kibo.money/api";
const KIBO_OFFICIAL_BACKUP_URL: &str = "https://backup.kibo.money/api";

const RETRIES: usize = 10;

impl Kibo {
    fn get_base_url(try_index: usize) -> &'static str {
        if try_index < RETRIES / 2 {
            KIBO_OFFICIAL_URL
        } else {
            KIBO_OFFICIAL_BACKUP_URL
        }
    }

    pub fn get_from_height(&mut self, height: Height) -> color_eyre::Result<OHLCCents> {
        #[allow(clippy::map_entry)]
        if !self.height_to_ohlc_vec.contains_key(&height)
            || ((usize::from(height) + self.height_to_ohlc_vec.get(&height).unwrap().len()) <= usize::from(height))
        {
            self.height_to_ohlc_vec
                .insert(height, Self::fetch_height_prices(height)?);
        }

        self.height_to_ohlc_vec
            .get(&height)
            .unwrap()
            .get(usize::from(height))
            .cloned()
            .ok_or(color_eyre::eyre::Error::msg("Couldn't find height in kibo"))
    }

    fn fetch_height_prices(height: Height) -> color_eyre::Result<Vec<OHLCCents>> {
        info!("Fetching Kibo height prices...");

        retry(
            |try_index| {
                let base_url = Self::get_base_url(try_index);

                let body: Value = minreq::get(format!("{base_url}/height-to-price?chunk={}", height))
                    .send()?
                    .json()?;

                let vec = body
                    .as_object()
                    .context("Expect to be an object")?
                    .get("dataset")
                    .context("Expect object to have dataset")?
                    .as_object()
                    .context("Expect to be an object")?
                    .get("map")
                    .context("Expect to have map")?
                    .as_array()
                    .context("Expect to be an array")?
                    .iter()
                    .map(Self::value_to_ohlc)
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(vec)
            },
            30,
            RETRIES,
        )
    }

    pub fn get_from_date(&mut self, date: &Date) -> color_eyre::Result<OHLCCents> {
        let year = date.year();

        #[allow(clippy::map_entry)]
        if !self.year_to_date_to_ohlc.contains_key(&year)
            || self
                .year_to_date_to_ohlc
                .get(&year)
                .unwrap()
                .last_key_value()
                .unwrap()
                .0
                < date
        {
            self.year_to_date_to_ohlc.insert(year, Self::fetch_date_prices(year)?);
        }

        self.year_to_date_to_ohlc
            .get(&year)
            .unwrap()
            .get(date)
            .cloned()
            .ok_or(color_eyre::eyre::Error::msg("Couldn't find date in kibo"))
    }

    fn fetch_date_prices(year: u16) -> color_eyre::Result<BTreeMap<Date, OHLCCents>> {
        info!("Fetching Kibo date prices...");

        retry(
            |try_index| {
                let base_url = Self::get_base_url(try_index);

                let body: Value = minreq::get(format!("{base_url}/date-to-price?chunk={}", year))
                    .send()?
                    .json()?;

                body.as_object()
                    .context("Expect to be an object")?
                    .get("dataset")
                    .context("Expect object to have dataset")?
                    .as_object()
                    .context("Expect to be an object")?
                    .get("map")
                    .context("Expect to have map")?
                    .as_object()
                    .context("Expect to be an object")?
                    .iter()
                    .map(|(serialized_date, value)| -> color_eyre::Result<_> {
                        let date = Date::from(jiff::civil::Date::from_str(serialized_date).unwrap());
                        Ok((date, Self::value_to_ohlc(value)?))
                    })
                    .collect::<Result<BTreeMap<_, _>, _>>()
            },
            30,
            RETRIES,
        )
    }

    fn value_to_ohlc(value: &Value) -> color_eyre::Result<OHLCCents> {
        let ohlc = value.as_object().context("Expect as_object to work")?;

        let get_value = |key: &str| -> color_eyre::Result<_> {
            Ok(Cents::from(Dollars::from(
                ohlc.get(key)
                    .context("Expect get key to work")?
                    .as_f64()
                    .context("Expect as_f64 to work")?,
            )))
        };

        Ok((
            Open::from(get_value("open")?),
            High::from(get_value("high")?),
            Low::from(get_value("low")?),
            Close::from(get_value("close")?),
        ))
    }
}
