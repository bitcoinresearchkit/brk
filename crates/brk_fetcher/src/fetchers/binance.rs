use std::{
    collections::BTreeMap,
    fs::{self, File},
    io::BufReader,
    path::{Path, PathBuf},
    str::FromStr,
};

use brk_core::{Cents, OHLCCents, Timestamp};
use color_eyre::eyre::{ContextCompat, eyre};
use log::info;
use serde_json::Value;
use storable_vec::STATELESS;

use crate::{Close, Date, Dollars, High, Low, Open, Pricer, fetchers::retry};

pub struct Binance {
    path: PathBuf,
    _1mn: Option<BTreeMap<Timestamp, OHLCCents>>,
    _1d: Option<BTreeMap<Date, OHLCCents>>,
    har: Option<BTreeMap<Timestamp, OHLCCents>>,
}

impl Binance {
    pub fn init(path: &Path) -> Self {
        Self {
            path: path.to_owned(),
            _1mn: None,
            _1d: None,
            har: None,
        }
    }

    pub fn get_from_1mn(
        &mut self,
        timestamp: Timestamp,
        previous_timestamp: Option<Timestamp>,
    ) -> color_eyre::Result<OHLCCents> {
        if self._1mn.is_none() || self._1mn.as_ref().unwrap().last_key_value().unwrap().0 <= &timestamp {
            self._1mn.replace(Self::fetch_1mn()?);
        }
        Pricer::<STATELESS>::find_height_ohlc(
            self._1mn.as_ref().unwrap(),
            timestamp,
            previous_timestamp,
            "binance 1mn",
        )
    }

    pub fn fetch_1mn() -> color_eyre::Result<BTreeMap<Timestamp, OHLCCents>> {
        info!("Fetching 1mn prices from Binance...");

        retry(
            |_| Self::json_to_timestamp_to_ohlc(&minreq::get(Self::url("interval=1m&limit=1000")).send()?.json()?),
            30,
            10,
        )
    }

    pub fn get_from_1d(&mut self, date: &Date) -> color_eyre::Result<OHLCCents> {
        if self._1d.is_none() || self._1d.as_ref().unwrap().last_key_value().unwrap().0 < date {
            self._1d.replace(Self::fetch_1d()?);
        }

        self._1d
            .as_ref()
            .unwrap()
            .get(date)
            .cloned()
            .ok_or(color_eyre::eyre::Error::msg("Couldn't find date"))
    }

    pub fn fetch_1d() -> color_eyre::Result<BTreeMap<Date, OHLCCents>> {
        info!("Fetching daily prices from Kraken...");

        dbg!(&Self::url("interval=1d"));

        retry(
            |_| Self::json_to_date_to_ohlc(&minreq::get(Self::url("interval=1d")).send()?.json()?),
            30,
            10,
        )
    }

    pub fn get_from_har_binance(
        &mut self,
        timestamp: Timestamp,
        previous_timestamp: Option<Timestamp>,
    ) -> color_eyre::Result<OHLCCents> {
        if self.har.is_none() {
            self.har.replace(self.read_har().unwrap_or_default());
        }
        Pricer::<STATELESS>::find_height_ohlc(self.har.as_ref().unwrap(), timestamp, previous_timestamp, "binance har")
    }

    fn read_har(&self) -> color_eyre::Result<BTreeMap<Timestamp, OHLCCents>> {
        info!("Reading Binance har file...");

        let path = &self.path;

        fs::create_dir_all(path)?;

        let path_binance_har = path.join("binance.har");

        let file = if let Ok(file) = File::open(path_binance_har) {
            file
        } else {
            return Err(eyre!("Missing binance file"));
        };

        let reader = BufReader::new(file);

        let json: BTreeMap<String, Value> = if let Ok(json) = serde_json::from_reader(reader) {
            json
        } else {
            return Ok(Default::default());
        };

        json.get("log")
            .context("Expect object to have log attribute")?
            .as_object()
            .context("Expect to be an object")?
            .get("entries")
            .context("Expect object to have entries")?
            .as_array()
            .context("Expect to be an array")?
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
                let response = entry.as_object().unwrap().get("response").unwrap().as_object().unwrap();

                let content = response.get("content").unwrap().as_object().unwrap();

                let text = content.get("text");

                if text.is_none() {
                    return Ok(BTreeMap::new());
                }

                let text = text.unwrap().as_str().unwrap();

                Self::json_to_timestamp_to_ohlc(&serde_json::Value::from_str(text).unwrap())
            })
            .try_fold(BTreeMap::default(), |mut all, res| {
                all.append(&mut res?);
                Ok(all)
            })
    }

    fn json_to_timestamp_to_ohlc(json: &Value) -> color_eyre::Result<BTreeMap<Timestamp, OHLCCents>> {
        Self::json_to_btree(json, Self::array_to_timestamp_and_ohlc)
    }

    fn json_to_date_to_ohlc(json: &Value) -> color_eyre::Result<BTreeMap<Date, OHLCCents>> {
        Self::json_to_btree(json, Self::array_to_date_and_ohlc)
    }

    fn json_to_btree<F, K, V>(json: &Value, fun: F) -> color_eyre::Result<BTreeMap<K, V>>
    where
        F: Fn(&Value) -> color_eyre::Result<(K, V)>,
        K: Ord,
    {
        json.as_array()
            .context("Expect to be an array")?
            .iter()
            .map(fun)
            .collect::<Result<BTreeMap<_, _>, _>>()
    }

    fn array_to_timestamp_and_ohlc(array: &Value) -> color_eyre::Result<(Timestamp, OHLCCents)> {
        let array = array.as_array().context("Expect to be array")?;

        let timestamp = Timestamp::from((array.first().unwrap().as_u64().unwrap() / 1_000) as u32);

        let get_cents = |index: usize| {
            Cents::from(Dollars::from(
                array.get(index).unwrap().as_str().unwrap().parse::<f64>().unwrap(),
            ))
        };

        Ok((
            timestamp,
            OHLCCents::from((
                Open::from(get_cents(1)),
                High::from(get_cents(2)),
                Low::from(get_cents(3)),
                Close::from(get_cents(4)),
            )),
        ))
    }

    fn array_to_date_and_ohlc(array: &Value) -> color_eyre::Result<(Date, OHLCCents)> {
        Self::array_to_timestamp_and_ohlc(array).map(|(t, ohlc)| (Date::from(t), ohlc))
    }

    fn url(query: &str) -> String {
        format!("https://api.binance.com/api/v3/uiKlines?symbol=BTCUSDT&{query}")
    }
}
