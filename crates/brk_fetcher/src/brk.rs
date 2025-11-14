use std::collections::BTreeMap;

use brk_error::{Error, Result};
use brk_types::{Cents, CheckedSub, Date, DateIndex, Height, OHLCCents};
use log::info;
use serde_json::Value;

use crate::{Close, Dollars, High, Low, Open, default_retry};

#[derive(Default, Clone)]
#[allow(clippy::upper_case_acronyms)]
pub struct BRK {
    height_to_ohlc: BTreeMap<Height, Vec<OHLCCents>>,
    dateindex_to_ohlc: BTreeMap<DateIndex, Vec<OHLCCents>>,
}

const API_URL: &str = "https://bitview.space/api/vecs";
const CHUNK_SIZE: usize = 10_000;

impl BRK {
    pub fn get_from_height(&mut self, height: Height) -> Result<OHLCCents> {
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
            .ok_or(Error::Str("Couldn't find height in BRK"))
    }

    fn fetch_height_prices(height: Height) -> Result<Vec<OHLCCents>> {
        info!("Fetching BRK height {height} prices...");

        default_retry(|_| {
            let url = format!(
                "{API_URL}/height-to-price-ohlc?from={}&to={}",
                height,
                height + CHUNK_SIZE
            );

            let body: Value = serde_json::from_slice(minreq::get(url).send()?.as_bytes())?;

            body.as_array()
                .ok_or(Error::Str("Expect to be an array"))?
                .iter()
                .map(Self::value_to_ohlc)
                .collect::<Result<Vec<_>, _>>()
        })
    }

    pub fn get_from_date(&mut self, date: Date) -> Result<OHLCCents> {
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
            .ok_or(Error::Str("Couldn't find date in BRK"))
    }

    fn fetch_date_prices(dateindex: DateIndex) -> Result<Vec<OHLCCents>> {
        info!("Fetching BRK dateindex {dateindex} prices...");

        default_retry(|_| {
            let url = format!(
                "{API_URL}/dateindex-to-price-ohlc?from={}&to={}",
                dateindex,
                dateindex + CHUNK_SIZE
            );

            let body: Value = serde_json::from_slice(minreq::get(url).send()?.as_bytes())?;

            body.as_array()
                .ok_or(Error::Str("Expect to be an array"))?
                .iter()
                .map(Self::value_to_ohlc)
                .collect::<Result<Vec<_>, _>>()
        })
    }

    fn value_to_ohlc(value: &Value) -> Result<OHLCCents> {
        let ohlc = value
            .as_array()
            .ok_or(Error::Str("Expect as_array to work"))?;

        let get_value = |index: usize| -> Result<_> {
            Ok(Cents::from(Dollars::from(
                ohlc.get(index)
                    .ok_or(Error::Str("Expect index key to work"))?
                    .as_f64()
                    .ok_or(Error::Str("Expect as_f64 to work"))?,
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
