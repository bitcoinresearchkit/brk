use std::collections::BTreeMap;

use color_eyre::eyre::ContextCompat;
use indexer::Timestamp;
use logger::info;
use serde_json::Value;

use crate::{fetchers::retry, structs::Date, Cents, Close, Dollars, High, Low, Open, OHLC};

pub struct Kraken;

impl Kraken {
    pub fn fetch_1mn_prices() -> color_eyre::Result<BTreeMap<Timestamp, OHLC>> {
        info!("Fetching 1mn prices from Kraken...");

        retry(
            |_| Self::json_to_timestamp_to_ohlc(&reqwest::blocking::get(Self::url(1))?.json()?),
            30,
            10,
        )
    }

    pub fn fetch_daily_prices() -> color_eyre::Result<BTreeMap<Date, OHLC>> {
        info!("Fetching daily prices from Kraken...");

        retry(
            |_| Self::json_to_date_to_ohlc(&reqwest::blocking::get(Self::url(1440))?.json()?),
            30,
            10,
        )
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
        json.as_object()
            .context("Expect to be an object")?
            .get("result")
            .context("Expect object to have result")?
            .as_object()
            .context("Expect to be an object")?
            .get("XXBTZUSD")
            .context("Expect to have XXBTZUSD")?
            .as_array()
            .context("Expect to be an array")?
            .iter()
            .map(fun)
            .collect::<Result<BTreeMap<_, _>, _>>()
    }

    fn array_to_timestamp_and_ohlc(array: &Value) -> color_eyre::Result<(Timestamp, OHLC)> {
        let array = array.as_array().context("Expect to be array")?;

        let timestamp = Timestamp::from(array.first().unwrap().as_u64().unwrap() as u32);

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

    fn url(interval: usize) -> String {
        format!("https://api.kraken.com/0/public/OHLC?pair=XBTUSD&interval={interval}")
    }
}
