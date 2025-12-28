use std::collections::BTreeMap;

use brk_error::{Error, Result};
use brk_types::{Date, Height, OHLCCents, Timestamp};
use log::info;
use serde_json::Value;

use crate::{
    PriceSource, default_retry,
    ohlc::{compute_ohlc_from_range, ohlc_from_array, timestamp_from_secs, date_from_timestamp},
};

#[derive(Default, Clone)]
pub struct Kraken {
    _1mn: Option<BTreeMap<Timestamp, OHLCCents>>,
    _1d: Option<BTreeMap<Date, OHLCCents>>,
}

impl Kraken {
    fn get_from_1mn(
        &mut self,
        timestamp: Timestamp,
        previous_timestamp: Option<Timestamp>,
    ) -> Result<OHLCCents> {
        if self._1mn.is_none()
            || self._1mn.as_ref().unwrap().last_key_value().unwrap().0 <= &timestamp
        {
            self._1mn.replace(Self::fetch_1mn()?);
        }
        compute_ohlc_from_range(
            self._1mn.as_ref().unwrap(),
            timestamp,
            previous_timestamp,
            "Kraken 1mn",
        )
    }

    pub fn fetch_1mn() -> Result<BTreeMap<Timestamp, OHLCCents>> {
        default_retry(|_| {
            let url = Self::url(1);
            info!("Fetching {url} ...");
            let json: Value = serde_json::from_slice(minreq::get(url).send()?.as_bytes())?;
            Self::parse_ohlc_response(&json)
        })
    }

    fn get_from_1d(&mut self, date: &Date) -> Result<OHLCCents> {
        if self._1d.is_none() || self._1d.as_ref().unwrap().last_key_value().unwrap().0 <= date {
            self._1d.replace(Self::fetch_1d()?);
        }
        self._1d
            .as_ref()
            .unwrap()
            .get(date)
            .cloned()
            .ok_or(Error::NotFound("Couldn't find date".into()))
    }

    pub fn fetch_1d() -> Result<BTreeMap<Date, OHLCCents>> {
        default_retry(|_| {
            let url = Self::url(1440);
            info!("Fetching {url} ...");
            let json: Value = serde_json::from_slice(minreq::get(url).send()?.as_bytes())?;
            Self::parse_date_ohlc_response(&json)
        })
    }

    /// Parse Kraken's nested JSON response: { result: { XXBTZUSD: [...] } }
    fn parse_ohlc_response(json: &Value) -> Result<BTreeMap<Timestamp, OHLCCents>> {
        let result = json
            .get("result")
            .and_then(|r| r.get("XXBTZUSD"))
            .and_then(|v| v.as_array())
            .ok_or(Error::Parse("Invalid Kraken response format".into()))?
            .iter()
            .filter_map(|v| v.as_array())
            .map(|arr| {
                let ts = arr.first().and_then(|v| v.as_u64()).unwrap_or(0);
                (timestamp_from_secs(ts), ohlc_from_array(arr))
            })
            .collect();
        Ok(result)
    }

    fn parse_date_ohlc_response(json: &Value) -> Result<BTreeMap<Date, OHLCCents>> {
        Self::parse_ohlc_response(json).map(|map| {
            map.into_iter()
                .map(|(ts, ohlc)| (date_from_timestamp(ts), ohlc))
                .collect()
        })
    }

    fn url(interval: usize) -> String {
        format!("https://api.kraken.com/0/public/OHLC?pair=XBTUSD&interval={interval}")
    }

    pub fn ping() -> Result<()> {
        minreq::get("https://api.kraken.com/0/public/Time")
            .send()?;
        Ok(())
    }
}

impl PriceSource for Kraken {
    fn name(&self) -> &'static str {
        "Kraken"
    }

    fn get_date(&mut self, date: Date) -> Option<Result<OHLCCents>> {
        Some(self.get_from_1d(&date))
    }

    fn get_1mn(
        &mut self,
        timestamp: Timestamp,
        previous_timestamp: Option<Timestamp>,
    ) -> Option<Result<OHLCCents>> {
        Some(self.get_from_1mn(timestamp, previous_timestamp))
    }

    fn get_height(&mut self, _height: Height) -> Option<Result<OHLCCents>> {
        None // Kraken doesn't support height-based queries
    }

    fn ping(&self) -> Result<()> {
        Self::ping()
    }

    fn clear(&mut self) {
        self._1d.take();
        self._1mn.take();
    }
}
