#![allow(dead_code)]

use std::{collections::BTreeMap, fs};

use color_eyre::eyre::ContextCompat;
use itertools::Itertools;
use log::info;
use serde_json::Value;

use crate::{
    io::Json,
    structs::{Config, Date, Timestamp, OHLC},
    utils::retry,
};

pub struct Binance;

impl Binance {
    pub fn read_har_file(config: &Config) -> color_eyre::Result<BTreeMap<u32, OHLC>> {
        info!("binance: read har file");

        let path = config.path_inputs();

        fs::create_dir_all(&path)?;

        let path_binance_har = path.join("binance.har");

        let json: BTreeMap<String, Value> = Json::import(&path_binance_har).unwrap_or_default();

        Ok(json
            .get("log")
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
            .flat_map(|entry| {
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
                    return vec![];
                }

                let text = text.unwrap().as_str().unwrap();

                let arrays: Value = serde_json::from_str(text).unwrap();

                arrays
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|array| {
                        let array = array.as_array().unwrap();

                        let timestamp = (array.first().unwrap().as_u64().unwrap() / 1000) as u32;

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
                    .collect_vec()
            })
            .collect::<BTreeMap<_, _>>())
    }

    pub fn fetch_1mn_prices() -> color_eyre::Result<BTreeMap<u32, OHLC>> {
        info!("binance: fetch 1mn");

        retry(
            |_| {
                let body: Value = reqwest::blocking::get(
                    "https://api.binance.com/api/v3/uiKlines?symbol=BTCUSDT&interval=1m&limit=1000",
                )?
                .json()?;

                Ok(body
                    .as_array()
                    .context("Expect to be an array")?
                    .iter()
                    .map(|value| {
                        // [timestamp, open, high, low, close, volume, ...]
                        let array = value.as_array().unwrap();

                        let timestamp = (array.first().unwrap().as_u64().unwrap() / 1_000) as u32;

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
            30,
            10,
        )
    }

    pub fn fetch_daily_prices() -> color_eyre::Result<BTreeMap<Date, OHLC>> {
        info!("binance: fetch 1d");

        retry(
            |_| {
                let body: Value = reqwest::blocking::get(
                    "https://api.binance.com/api/v3/uiKlines?symbol=BTCUSDT&interval=1d",
                )?
                .json()?;

                Ok(body
                    .as_array()
                    .context("Expect to be an array")?
                    .iter()
                    .map(|value| {
                        // [timestamp, open, high, low, close, volume, ...]
                        let array = value.as_array().unwrap();

                        let date = Timestamp::from(
                            (array.first().unwrap().as_u64().unwrap() / 1_000) as u32,
                        )
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
            30,
            10,
        )
    }
}
