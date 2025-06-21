use std::collections::BTreeMap;

use brk_core::{Cents, CheckedSub, Date, DateIndex, Height, OHLCCents};
use color_eyre::eyre::{ContextCompat, eyre};
use log::info;
use serde_json::Value;

use crate::{Close, Dollars, High, Low, Open, fetchers::retry};

#[derive(Default, Clone)]
pub struct BRK {
    height_to_ohlc: BTreeMap<Height, Vec<OHLCCents>>,
    dateindex_to_ohlc: BTreeMap<DateIndex, Vec<OHLCCents>>,
}

const API_URL: &str = "https://bitcoinresearchkit.org/api/vecs";
const RETRIES: usize = 10;
const CHUNK_SIZE: usize = 10_000;

impl BRK {
    pub fn get_from_height(&mut self, height: Height) -> color_eyre::Result<OHLCCents> {
        let key = height.checked_sub(height % CHUNK_SIZE).unwrap();

        #[allow(clippy::map_entry)]
        if !self.height_to_ohlc.contains_key(&key)
            || ((key + self.height_to_ohlc.get(&key).unwrap().len()) <= height)
        {
            self.height_to_ohlc.insert(
                key,
                Self::fetch_height_prices(key).inspect_err(|e| {
                    dbg!(e);
                })?,
            );
        }

        self.height_to_ohlc
            .get(&key)
            .unwrap()
            .get(usize::from(height.checked_sub(key).unwrap()))
            .cloned()
            .ok_or(eyre!("Couldn't find height in BRK"))
    }

    fn fetch_height_prices(height: Height) -> color_eyre::Result<Vec<OHLCCents>> {
        info!("Fetching BRK height {height} prices...");

        retry(
            |_| {
                let url = format!(
                    "{API_URL}/query?index=height&values=ohlc&from={}&to={}",
                    height,
                    height + CHUNK_SIZE
                );

                let body: Value = minreq::get(url).send()?.json()?;

                body.as_array()
                    .context("Expect to be an array")?
                    .iter()
                    .map(Self::value_to_ohlc)
                    .collect::<Result<Vec<_>, _>>()
            },
            30,
            RETRIES,
        )
    }

    pub fn get_from_date(&mut self, date: Date) -> color_eyre::Result<OHLCCents> {
        let dateindex = DateIndex::try_from(date)?;

        let key = dateindex.checked_sub(dateindex % CHUNK_SIZE).unwrap();

        #[allow(clippy::map_entry)]
        if !self.dateindex_to_ohlc.contains_key(&key)
            || ((key + self.dateindex_to_ohlc.get(&key).unwrap().len()) <= dateindex)
        {
            self.dateindex_to_ohlc.insert(
                key,
                Self::fetch_date_prices(key).inspect_err(|e| {
                    dbg!(e);
                })?,
            );
        }

        self.dateindex_to_ohlc
            .get(&key)
            .unwrap()
            .get(usize::from(dateindex.checked_sub(key).unwrap()))
            .cloned()
            .ok_or(eyre!("Couldn't find date in BRK"))
    }

    fn fetch_date_prices(dateindex: DateIndex) -> color_eyre::Result<Vec<OHLCCents>> {
        info!("Fetching BRK dateindex {dateindex} prices...");

        retry(
            |_| {
                let url = format!(
                    "{API_URL}/query?index=dateindex&values=ohlc&from={}&to={}",
                    dateindex,
                    dateindex + CHUNK_SIZE
                );

                let body: Value = minreq::get(url).send()?.json()?;

                body.as_array()
                    .context("Expect to be an array")?
                    .iter()
                    .map(Self::value_to_ohlc)
                    .collect::<Result<Vec<_>, _>>()
            },
            30,
            RETRIES,
        )
    }

    fn value_to_ohlc(value: &Value) -> color_eyre::Result<OHLCCents> {
        let ohlc = value.as_array().context("Expect as_array to work")?;

        let get_value = |index: usize| -> color_eyre::Result<_> {
            Ok(Cents::from(Dollars::from(
                ohlc.get(index)
                    .context("Expect index key to work")?
                    .as_f64()
                    .context("Expect as_f64 to work")?,
            )))
        };

        Ok(OHLCCents::from((
            Open::new(get_value(0)?),
            High::new(get_value(1)?),
            Low::new(get_value(2)?),
            Close::new(get_value(3)?),
        )))
    }

    pub fn clear(&mut self) {
        self.height_to_ohlc.clear();
        self.dateindex_to_ohlc.clear();
    }
}
