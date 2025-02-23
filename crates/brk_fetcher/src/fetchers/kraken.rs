use std::collections::BTreeMap;

use brk_indexer::Timestamp;
use color_eyre::eyre::ContextCompat;
use log::info;
use serde_json::Value;
use storable_vec::STATELESS;

use crate::{Cents, Close, Dollars, High, Low, OHLC, Open, Pricer, fetchers::retry, structs::Date};

#[derive(Default)]
pub struct Kraken {
    _1mn: Option<BTreeMap<Timestamp, OHLC>>,
    _1d: Option<BTreeMap<Date, OHLC>>,
}

impl Kraken {
    pub fn get_from_1mn(
        &mut self,
        timestamp: Timestamp,
        previous_timestamp: Option<Timestamp>,
    ) -> color_eyre::Result<OHLC> {
        if self._1mn.is_none() || self._1mn.as_ref().unwrap().last_key_value().unwrap().0 <= &timestamp {
            self._1mn.replace(Self::fetch_1mn()?);
        }
        Pricer::<STATELESS>::find_height_ohlc(self._1mn.as_ref().unwrap(), timestamp, previous_timestamp, "kraken 1m")
    }

    fn fetch_1mn() -> color_eyre::Result<BTreeMap<Timestamp, OHLC>> {
        info!("Fetching 1mn prices from Kraken...");

        retry(
            |_| Self::json_to_timestamp_to_ohlc(&minreq::get(Self::url(1)).send()?.json()?),
            30,
            10,
        )
    }

    pub fn get_from_1d(&mut self, date: &Date) -> color_eyre::Result<OHLC> {
        if self._1d.is_none() || self._1d.as_ref().unwrap().last_key_value().unwrap().0 < date {
            self._1d.replace(Kraken::fetch_1d()?);
        }
        self._1d
            .as_ref()
            .unwrap()
            .get(date)
            .cloned()
            .ok_or(color_eyre::eyre::Error::msg("Couldn't find date"))
    }

    fn fetch_1d() -> color_eyre::Result<BTreeMap<Date, OHLC>> {
        info!("Fetching daily prices from Kraken...");

        retry(
            |_| Self::json_to_date_to_ohlc(&minreq::get(Self::url(1440)).send()?.json()?),
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
