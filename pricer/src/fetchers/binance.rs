use std::{
    collections::BTreeMap,
    fs::{self, File},
    io::BufReader,
    path::Path,
    str::FromStr,
};

use color_eyre::eyre::{eyre, ContextCompat};
use indexer::Timestamp;
use logger::info;
use serde_json::Value;

use crate::{
    fetchers::retry,
    structs::{Cents, OHLC},
    Close, Date, Dollars, High, Low, Open,
};

pub struct Binance;

impl Binance {
    pub fn fetch_1mn_prices() -> color_eyre::Result<BTreeMap<Timestamp, OHLC>> {
        info!("Fetching 1mn prices from Binance...");

        retry(
            |_| Self::json_to_timestamp_to_ohlc(&reqwest::blocking::get(Self::url("interval=1m&limit=1000"))?.json()?),
            30,
            10,
        )
    }

    pub fn fetch_daily_prices() -> color_eyre::Result<BTreeMap<Date, OHLC>> {
        info!("Fetching daily prices from Kraken...");

        retry(
            |_| Self::json_to_date_to_ohlc(&reqwest::blocking::get(Self::url("interval=1d"))?.json()?),
            30,
            10,
        )
    }

    pub fn read_har_file(path: &Path) -> color_eyre::Result<BTreeMap<Timestamp, OHLC>> {
        info!("Reading Binance har file...");

        fs::create_dir_all(&path)?;

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

    fn json_to_timestamp_to_ohlc(json: &Value) -> color_eyre::Result<BTreeMap<Timestamp, OHLC>> {
        Self::json_to_btree(json, Self::array_to_timestamp_and_ohlc)
    }

    fn json_to_date_to_ohlc(json: &Value) -> color_eyre::Result<BTreeMap<Date, OHLC>> {
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

    fn array_to_timestamp_and_ohlc(array: &Value) -> color_eyre::Result<(Timestamp, OHLC)> {
        let array = array.as_array().context("Expect to be array")?;

        let timestamp = Timestamp::from((array.first().unwrap().as_u64().unwrap() / 1_000) as u32);

        let get_cents = |index: usize| {
            Cents::from(Dollars::from(
                array.get(index).unwrap().as_str().unwrap().parse::<f64>().unwrap(),
            ))
        };

        Ok((
            timestamp,
            OHLC::from((
                Open::from(get_cents(1)),
                High::from(get_cents(2)),
                Low::from(get_cents(3)),
                Close::from(get_cents(4)),
            )),
        ))
    }

    fn array_to_date_and_ohlc(array: &Value) -> color_eyre::Result<(Date, OHLC)> {
        Self::array_to_timestamp_and_ohlc(array).map(|(t, ohlc)| (Date::from(t), ohlc))
    }

    fn url(query: &str) -> String {
        format!("https://api.binance.com/api/v3/uiKlines?symbol=BTCUSDT&{query}")
    }
}
