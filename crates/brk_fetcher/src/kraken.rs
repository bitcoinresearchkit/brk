use std::collections::BTreeMap;

use brk_error::{Error, Result};
use brk_structs::{Cents, Close, Date, Dollars, High, Low, OHLCCents, Open, Timestamp};
use log::info;
use sonic_rs::{JsonContainerTrait, JsonValueTrait, Value};

use crate::{Fetcher, default_retry};

#[derive(Default, Clone)]
pub struct Kraken {
    _1mn: Option<BTreeMap<Timestamp, OHLCCents>>,
    _1d: Option<BTreeMap<Date, OHLCCents>>,
}

impl Kraken {
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
        Fetcher::find_height_ohlc(
            self._1mn.as_ref().unwrap(),
            timestamp,
            previous_timestamp,
            "kraken 1m",
        )
    }

    pub fn fetch_1mn() -> Result<BTreeMap<Timestamp, OHLCCents>> {
        info!("Fetching 1mn prices from Kraken...");

        default_retry(|_| {
            Self::json_to_timestamp_to_ohlc(&sonic_rs::from_str(
                minreq::get(Self::url(1)).send()?.as_str()?,
            )?)
        })
    }

    pub fn get_from_1d(&mut self, date: &Date) -> Result<OHLCCents> {
        if self._1d.is_none() || self._1d.as_ref().unwrap().last_key_value().unwrap().0 <= date {
            self._1d.replace(Kraken::fetch_1d()?);
        }
        self._1d
            .as_ref()
            .unwrap()
            .get(date)
            .cloned()
            .ok_or(Error::Str("Couldn't find date"))
    }

    pub fn fetch_1d() -> Result<BTreeMap<Date, OHLCCents>> {
        info!("Fetching daily prices from Kraken...");

        default_retry(|_| {
            Self::json_to_date_to_ohlc(&sonic_rs::from_str(
                minreq::get(Self::url(1440)).send()?.as_str()?,
            )?)
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
        json.as_object()
            .ok_or(Error::Str("Expect to be an object"))?
            .get(&"result")
            .ok_or(Error::Str("Expect object to have result"))?
            .as_object()
            .ok_or(Error::Str("Expect to be an object"))?
            .get(&"XXBTZUSD")
            .ok_or(Error::Str("Expect to have XXBTZUSD"))?
            .as_array()
            .ok_or(Error::Str("Expect to be an array"))?
            .iter()
            .map(fun)
            .collect::<Result<BTreeMap<_, _>, _>>()
    }

    fn array_to_timestamp_and_ohlc(array: &Value) -> Result<(Timestamp, OHLCCents)> {
        let array = array.as_array().ok_or(Error::Str("Expect to be array"))?;

        let timestamp = Timestamp::from(array.first().unwrap().as_u64().unwrap() as u32);

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

    fn url(interval: usize) -> String {
        format!("https://api.kraken.com/0/public/OHLC?pair=XBTUSD&interval={interval}")
    }

    pub fn clear(&mut self) {
        self._1d.take();
        self._1mn.take();
    }
}
