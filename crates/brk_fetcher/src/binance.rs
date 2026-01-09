use std::{
    collections::BTreeMap,
    fs::{self, File},
    io::BufReader,
    path::{Path, PathBuf},
};

use brk_error::{Error, Result};
use brk_types::{Date, Height, OHLCCents, Timestamp};
use serde_json::Value;
use tracing::info;

use crate::{
    PriceSource, default_retry,
    ohlc::{compute_ohlc_from_range, date_from_timestamp, ohlc_from_array, timestamp_from_ms},
};

#[derive(Clone)]
pub struct Binance {
    path: Option<PathBuf>,
    _1mn: Option<BTreeMap<Timestamp, OHLCCents>>,
    _1d: Option<BTreeMap<Date, OHLCCents>>,
    har: Option<BTreeMap<Timestamp, OHLCCents>>,
}

impl Binance {
    pub fn init(path: Option<&Path>) -> Self {
        Self {
            path: path.map(|p| p.to_owned()),
            _1mn: None,
            _1d: None,
            har: None,
        }
    }

    pub fn get_from_1mn(
        &mut self,
        timestamp: Timestamp,
        previous_timestamp: Option<Timestamp>,
    ) -> Result<OHLCCents> {
        // Try live API data first
        if self._1mn.is_none()
            || self._1mn.as_ref().unwrap().last_key_value().unwrap().0 <= &timestamp
        {
            self._1mn.replace(Self::fetch_1mn()?);
        }

        let res = compute_ohlc_from_range(
            self._1mn.as_ref().unwrap(),
            timestamp,
            previous_timestamp,
            "Binance 1mn",
        );

        if res.is_ok() {
            return res;
        }

        // Fall back to HAR file data
        if self.har.is_none() {
            self.har.replace(self.read_har().unwrap_or_default());
        }

        compute_ohlc_from_range(
            self.har.as_ref().unwrap(),
            timestamp,
            previous_timestamp,
            "Binance HAR",
        )
    }

    pub fn fetch_1mn() -> Result<BTreeMap<Timestamp, OHLCCents>> {
        default_retry(|_| {
            let url = Self::url("interval=1m&limit=1000");
            info!("Fetching {url} ...");
            let json: Value =
                serde_json::from_slice(minreq::get(url).with_timeout(30).send()?.as_bytes())?;
            Self::parse_ohlc_array(&json)
        })
    }

    pub fn get_from_1d(&mut self, date: &Date) -> Result<OHLCCents> {
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
            let url = Self::url("interval=1d");
            info!("Fetching {url} ...");
            let json: Value =
                serde_json::from_slice(minreq::get(url).with_timeout(30).send()?.as_bytes())?;
            Self::parse_date_ohlc_array(&json)
        })
    }

    fn read_har(&self) -> Result<BTreeMap<Timestamp, OHLCCents>> {
        if self.path.is_none() {
            return Err(Error::NotFound("HAR path not configured".into()));
        }

        info!("Reading Binance har file...");

        let path = self.path.as_ref().unwrap();

        fs::create_dir_all(path)?;

        let path_binance_har = path.join("binance.har");

        let file = if let Ok(file) = File::open(path_binance_har) {
            file
        } else {
            return Err(Error::NotFound("Binance HAR file not found".into()));
        };

        let reader = BufReader::new(file);

        let json: BTreeMap<String, Value> = if let Ok(json) = serde_json::from_reader(reader) {
            json
        } else {
            return Ok(Default::default());
        };

        json.get("log")
            .ok_or(Error::Parse("HAR missing 'log' field".into()))?
            .as_object()
            .ok_or(Error::Parse("HAR 'log' is not an object".into()))?
            .get("entries")
            .ok_or(Error::Parse("HAR missing 'entries' field".into()))?
            .as_array()
            .ok_or(Error::Parse("HAR 'entries' is not an array".into()))?
            .iter()
            .filter(|entry| {
                entry
                    .as_object()
                    .unwrap()
                    .get("request")
                    .unwrap()
                    .as_object()
                    .unwrap()
                    .get("url")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .contains("/uiKlines")
            })
            .map(|entry| {
                let response = entry
                    .as_object()
                    .unwrap()
                    .get("response")
                    .unwrap()
                    .as_object()
                    .unwrap();

                let content = response.get("content").unwrap().as_object().unwrap();

                let text = content.get("text");

                if text.is_none() {
                    return Ok(BTreeMap::new());
                }

                let text = text.unwrap().as_str().unwrap();
                let json: Value = serde_json::from_str(text).unwrap();
                Self::parse_ohlc_array(&json)
            })
            .try_fold(BTreeMap::default(), |mut all, res| {
                all.append(&mut res?);
                Ok(all)
            })
    }

    fn parse_ohlc_array(json: &Value) -> Result<BTreeMap<Timestamp, OHLCCents>> {
        let result = json
            .as_array()
            .ok_or(Error::Parse("Expected JSON array".into()))?
            .iter()
            .filter_map(|v| v.as_array())
            .map(|arr| {
                let ts = arr.first().and_then(|v| v.as_u64()).unwrap_or(0);
                (timestamp_from_ms(ts), ohlc_from_array(arr))
            })
            .collect();
        Ok(result)
    }

    fn parse_date_ohlc_array(json: &Value) -> Result<BTreeMap<Date, OHLCCents>> {
        Self::parse_ohlc_array(json).map(|map| {
            map.into_iter()
                .map(|(ts, ohlc)| (date_from_timestamp(ts), ohlc))
                .collect()
        })
    }

    fn url(query: &str) -> String {
        format!("https://api.binance.com/api/v3/uiKlines?symbol=BTCUSDT&{query}")
    }

    pub fn ping() -> Result<()> {
        minreq::get("https://api.binance.com/api/v3/ping")
            .with_timeout(10)
            .send()?;
        Ok(())
    }
}

impl PriceSource for Binance {
    fn name(&self) -> &'static str {
        "Binance"
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
        None // Binance doesn't support height-based queries
    }

    fn ping(&self) -> Result<()> {
        Self::ping()
    }

    fn clear(&mut self) {
        self._1d.take();
        self._1mn.take();
    }
}
