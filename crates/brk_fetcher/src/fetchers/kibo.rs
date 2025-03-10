use std::{collections::BTreeMap, str::FromStr};

use brk_core::{CheckedSub, Date, Height, OHLCCents};
use color_eyre::eyre::{ContextCompat, eyre};
use log::info;
use serde_json::Value;

use crate::{Cents, Close, Dollars, High, Low, Open, fetchers::retry};

#[derive(Default, Clone)]
pub struct Kibo {
    height_to_ohlc_vec: BTreeMap<Height, Vec<OHLCCents>>,
    year_to_date_to_ohlc: BTreeMap<u16, BTreeMap<Date, OHLCCents>>,
}

const KIBO_OFFICIAL_URL: &str = "https://kibo.money/api";

const RETRIES: usize = 10;

impl Kibo {
    pub fn get_from_height(&mut self, height: Height) -> color_eyre::Result<OHLCCents> {
        let key = height.checked_sub(height % 10_000).unwrap_or_default();

        #[allow(clippy::map_entry)]
        if !self.height_to_ohlc_vec.contains_key(&key)
            || ((key + self.height_to_ohlc_vec.get(&key).unwrap().len()) <= height)
        {
            self.height_to_ohlc_vec.insert(
                key,
                Self::fetch_height_prices(key).inspect_err(|e| {
                    dbg!(e);
                })?,
            );
        }

        self.height_to_ohlc_vec
            .get(&key)
            .unwrap()
            .get(usize::from(height.checked_sub(key).unwrap()))
            .cloned()
            .ok_or(color_eyre::eyre::Error::msg("Couldn't find height in kibo"))
    }

    fn fetch_height_prices(height: Height) -> color_eyre::Result<Vec<OHLCCents>> {
        info!("Fetching Kibo height {height} prices...");

        retry(
            |_| {
                let url = format!("{KIBO_OFFICIAL_URL}/height-to-price?chunk={}", height);

                let body: Value = minreq::get(url).send()?.json()?;

                body.as_object()
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
                    .collect::<Result<Vec<_>, _>>()
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
            self.year_to_date_to_ohlc
                .insert(year, Self::fetch_date_prices(year)?);
        }

        self.year_to_date_to_ohlc
            .get(&year)
            .unwrap()
            .get(date)
            .cloned()
            .ok_or(eyre!("Couldn't find date in kibo"))
    }

    fn fetch_date_prices(year: u16) -> color_eyre::Result<BTreeMap<Date, OHLCCents>> {
        info!("Fetching Kibo date {year} prices...");

        retry(
            |_| {
                let body: Value =
                    minreq::get(format!("{KIBO_OFFICIAL_URL}/date-to-price?chunk={}", year))
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
                        let date =
                            Date::from(jiff::civil::Date::from_str(serialized_date).unwrap());
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

        Ok(OHLCCents::from((
            Open::from(get_value("open")?),
            High::from(get_value("high")?),
            Low::from(get_value("low")?),
            Close::from(get_value("close")?),
        )))
    }
}
