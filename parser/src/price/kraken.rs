use std::collections::BTreeMap;

use color_eyre::eyre::ContextCompat;
use serde_json::Value;

use crate::{
    datasets::OHLC,
    structs::{Date, Timestamp},
    utils::{log, retry},
};

pub struct Kraken;

impl Kraken {
    pub fn fetch_1mn_prices() -> color_eyre::Result<BTreeMap<u32, OHLC>> {
        log("kraken: fetch 1mn");

        retry(
            |_| {
                let body: Value = reqwest::blocking::get(
                    "https://api.kraken.com/0/public/OHLC?pair=XBTUSD&interval=1",
                )?
                .json()?;

                Ok(body
                    .as_object()
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
                    .map(|value| {
                        let array = value.as_array().unwrap();

                        let timestamp = array.first().unwrap().as_u64().unwrap() as u32;

                        let get_f32 = |index: usize| {
                            array
                                .get(index)
                                .unwrap()
                                .as_str()
                                .unwrap()
                                .parse::<f32>()
                                .unwrap()
                        };

                        (
                            timestamp,
                            OHLC {
                                open: get_f32(1),
                                high: get_f32(2),
                                low: get_f32(3),
                                close: get_f32(4),
                            },
                        )
                    })
                    .collect::<BTreeMap<_, _>>())
            },
            10,
            10,
        )
    }

    pub fn fetch_daily_prices() -> color_eyre::Result<BTreeMap<Date, OHLC>> {
        log("fetch kraken daily");

        retry(
            |_| {
                let body: Value = reqwest::blocking::get(
                    "https://api.kraken.com/0/public/OHLC?pair=XBTUSD&interval=1440",
                )?
                .json()?;

                Ok(body
                    .as_object()
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
                    .map(|value| {
                        let array = value.as_array().unwrap();

                        let date = Timestamp::wrap(array.first().unwrap().as_u64().unwrap() as u32)
                            .to_date();

                        let get_f32 = |index: usize| {
                            array
                                .get(index)
                                .unwrap()
                                .as_str()
                                .unwrap()
                                .parse::<f32>()
                                .unwrap()
                        };

                        (
                            date,
                            OHLC {
                                open: get_f32(1),
                                high: get_f32(2),
                                low: get_f32(3),
                                close: get_f32(4),
                            },
                        )
                    })
                    .collect::<BTreeMap<_, _>>())
            },
            10,
            10,
        )
    }
}
