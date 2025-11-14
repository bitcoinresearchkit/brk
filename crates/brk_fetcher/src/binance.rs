use std::{
    collections::BTreeMap,
    fs::{self, File},
    io::BufReader,
    path::{Path, PathBuf},
};

use brk_error::{Error, Result};
use brk_types::{Cents, OHLCCents, Timestamp};
use log::info;
use serde_json::Value;

use crate::{Close, Date, Dollars, Fetcher, High, Low, Open, default_retry};

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
        if self._1mn.is_none()
            || self._1mn.as_ref().unwrap().last_key_value().unwrap().0 <= &timestamp
        {
            self._1mn.replace(Self::fetch_1mn()?);
        }

        let res = Fetcher::find_height_ohlc(
            self._1mn.as_ref().unwrap(),
            timestamp,
            previous_timestamp,
            "binance 1mn",
        );

        if res.is_ok() {
            return res;
        }

        if self.har.is_none() {
            self.har.replace(self.read_har().unwrap_or_default());
        }

        Fetcher::find_height_ohlc(
            self.har.as_ref().unwrap(),
            timestamp,
            previous_timestamp,
            "binance har",
        )
    }

    pub fn fetch_1mn() -> Result<BTreeMap<Timestamp, OHLCCents>> {
        info!("Fetching 1mn prices from Binance...");

        default_retry(|_| {
            Self::json_to_timestamp_to_ohlc(&serde_json::from_slice(
                minreq::get(Self::url("interval=1m&limit=1000"))
                    .send()?
                    .as_bytes(),
            )?)
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
            .ok_or(Error::Str("Couldn't find date"))
    }

    pub fn fetch_1d() -> Result<BTreeMap<Date, OHLCCents>> {
        info!("Fetching daily prices from Binance...");

        default_retry(|_| {
            Self::json_to_date_to_ohlc(&serde_json::from_slice(
                minreq::get(Self::url("interval=1d")).send()?.as_bytes(),
            )?)
        })
    }

    fn read_har(&self) -> Result<BTreeMap<Timestamp, OHLCCents>> {
        if self.path.is_none() {
            return Err(Error::Str("Path missing"));
        }

        info!("Reading Binance har file...");

        let path = self.path.as_ref().unwrap();

        fs::create_dir_all(path)?;

        let path_binance_har = path.join("binance.har");

        let file = if let Ok(file) = File::open(path_binance_har) {
            file
        } else {
            return Err(Error::Str("Missing binance file"));
        };

        let reader = BufReader::new(file);

        let json: BTreeMap<String, Value> = if let Ok(json) = serde_json::from_reader(reader) {
            json
        } else {
            return Ok(Default::default());
        };

        json.get("log")
            .ok_or(Error::Str("Expect object to have log attribute"))?
            .as_object()
            .ok_or(Error::Str("Expect to be an object"))?
            .get("entries")
            .ok_or(Error::Str("Expect object to have entries"))?
            .as_array()
            .ok_or(Error::Str("Expect to be an array"))?
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

                Self::json_to_timestamp_to_ohlc(&serde_json::from_str(text).unwrap())
            })
            .try_fold(BTreeMap::default(), |mut all, res| {
                all.append(&mut res?);
                Ok(all)
            })
    }

    fn json_to_timestamp_to_ohlc(json: &Value) -> Result<BTreeMap<Timestamp, OHLCCents>> {
        Self::json_to_btree(json, Self::array_to_timestamp_and_ohlc)
    }

    fn json_to_date_to_ohlc(json: &Value) -> Result<BTreeMap<Date, OHLCCents>> {
        Self::json_to_btree(json, Self::array_to_date_and_ohlc)
    }

    fn json_to_btree<F, K, V>(json: &Value, fun: F) -> Result<BTreeMap<K, V>>
    where
        F: Fn(&Value) -> Result<(K, V)>,
        K: Ord,
    {
        json.as_array()
            .ok_or(Error::Str("Expect to be an array"))?
            .iter()
            .map(fun)
            .collect::<Result<BTreeMap<_, _>, _>>()
    }

    fn array_to_timestamp_and_ohlc(array: &Value) -> Result<(Timestamp, OHLCCents)> {
        let array = array.as_array().ok_or(Error::Str("Expect to be array"))?;

        let timestamp = Timestamp::from((array.first().unwrap().as_u64().unwrap() / 1_000) as u32);

        let get_cents = |index: usize| {
            Cents::from(Dollars::from(
                array
                    .get(index)
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .parse::<f64>()
                    .unwrap(),
            ))
        };

        Ok((
            timestamp,
            OHLCCents::from((
                Open::new(get_cents(1)),
                High::new(get_cents(2)),
                Low::new(get_cents(3)),
                Close::new(get_cents(4)),
            )),
        ))
    }

    fn array_to_date_and_ohlc(array: &Value) -> Result<(Date, OHLCCents)> {
        Self::array_to_timestamp_and_ohlc(array).map(|(t, ohlc)| (Date::from(t), ohlc))
    }

    fn url(query: &str) -> String {
        format!("https://api.binance.com/api/v3/uiKlines?symbol=BTCUSDT&{query}")
    }

    pub fn clear(&mut self) {
        self._1d.take();
        self._1mn.take();
    }
}
