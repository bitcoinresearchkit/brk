use std::collections::BTreeMap;

use brk_error::{Error, Result};
use brk_types::{
    Cents, CheckedSub, Close, Date, Day1, Dollars, Height, High, Low, OHLCCents, Open, Timestamp,
};
use serde_json::Value;
use tracing::info;
use ureq::Agent;

use crate::{PriceSource, checked_get, default_retry};

#[derive(Clone)]
#[allow(clippy::upper_case_acronyms)]
pub struct BRK {
    agent: Agent,
    height_to_ohlc: BTreeMap<Height, Vec<OHLCCents>>,
    day1_to_ohlc: BTreeMap<Day1, Vec<OHLCCents>>,
}

impl BRK {
    pub fn new() -> Self {
        Self::new_with_agent(crate::new_agent(30))
    }

    pub fn new_with_agent(agent: Agent) -> Self {
        Self {
            agent,
            height_to_ohlc: BTreeMap::new(),
            day1_to_ohlc: BTreeMap::new(),
        }
    }
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
            self.height_to_ohlc
                .insert(key, self.fetch_height_prices(key)?);
        }

        self.height_to_ohlc
            .get(&key)
            .unwrap()
            .get(usize::from(height.checked_sub(key).unwrap()))
            .cloned()
            .ok_or(Error::NotFound("Couldn't find height in BRK".into()))
    }

    fn fetch_height_prices(&self, height: Height) -> Result<Vec<OHLCCents>> {
        let agent = &self.agent;
        default_retry(|_| {
            let url = format!(
                "{API_URL}/height-to-price-ohlc?from={}&to={}",
                height,
                height + CHUNK_SIZE
            );
            info!("Fetching {url} ...");

            let bytes = checked_get(agent, &url)?;
            let body: Value = serde_json::from_slice(&bytes)?;

            body.as_array()
                .ok_or(Error::Parse("Expected JSON array".into()))?
                .iter()
                .map(Self::value_to_ohlc)
                .collect::<Result<Vec<_>, _>>()
        })
    }

    pub fn get_from_date(&mut self, date: Date) -> Result<OHLCCents> {
        let day1 = Day1::try_from(date)?;

        let key = day1.checked_sub(day1 % CHUNK_SIZE).unwrap();

        #[allow(clippy::map_entry)]
        if !self.day1_to_ohlc.contains_key(&key)
            || ((key + self.day1_to_ohlc.get(&key).unwrap().len()) <= day1)
        {
            self.day1_to_ohlc.insert(key, self.fetch_date_prices(key)?);
        }

        self.day1_to_ohlc
            .get(&key)
            .unwrap()
            .get(usize::from(day1.checked_sub(key).unwrap()))
            .cloned()
            .ok_or(Error::NotFound("Couldn't find date in BRK".into()))
    }

    fn fetch_date_prices(&self, day1: Day1) -> Result<Vec<OHLCCents>> {
        let agent = &self.agent;
        default_retry(|_| {
            let url = format!(
                "{API_URL}/day1-to-price-ohlc?from={}&to={}",
                day1,
                day1 + CHUNK_SIZE
            );
            info!("Fetching {url}...");

            let bytes = checked_get(agent, &url)?;
            let body: Value = serde_json::from_slice(&bytes)?;

            body.as_array()
                .ok_or(Error::Parse("Expected JSON array".into()))?
                .iter()
                .map(Self::value_to_ohlc)
                .collect::<Result<Vec<_>, _>>()
        })
    }

    fn value_to_ohlc(value: &Value) -> Result<OHLCCents> {
        let ohlc = value
            .as_array()
            .ok_or(Error::Parse("Expected OHLC array".into()))?;

        let get_value = |index: usize| -> Result<_> {
            Ok(Cents::from(Dollars::from(
                ohlc.get(index)
                    .ok_or(Error::Parse("Missing OHLC value at index".into()))?
                    .as_f64()
                    .ok_or(Error::Parse("Invalid OHLC value type".into()))?,
            )))
        };

        Ok(OHLCCents::from((
            Open::new(get_value(0)?),
            High::new(get_value(1)?),
            Low::new(get_value(2)?),
            Close::new(get_value(3)?),
        )))
    }

    pub fn ping(&self) -> Result<()> {
        self.agent.get(API_URL).call()?;
        Ok(())
    }
}

impl PriceSource for BRK {
    fn name(&self) -> &'static str {
        "BRK"
    }

    fn get_date(&mut self, date: Date) -> Option<Result<OHLCCents>> {
        Some(self.get_from_date(date))
    }

    fn get_1mn(
        &mut self,
        _timestamp: Timestamp,
        _previous_timestamp: Option<Timestamp>,
    ) -> Option<Result<OHLCCents>> {
        None // BRK doesn't support timestamp-based queries
    }

    fn get_height(&mut self, height: Height) -> Option<Result<OHLCCents>> {
        Some(self.get_from_height(height))
    }

    fn ping(&self) -> Result<()> {
        self.ping()
    }

    fn clear(&mut self) {
        self.height_to_ohlc.clear();
        self.day1_to_ohlc.clear();
    }
}
