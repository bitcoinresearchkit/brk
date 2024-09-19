use std::{collections::BTreeMap, str::FromStr};

use chrono::NaiveDate;
use color_eyre::eyre::ContextCompat;
use itertools::Itertools;
use serde_json::Value;

use crate::{
    datasets::OHLC,
    structs::{Date, DateMapChunkId, HeightMapChunkId},
    utils::{log, retry},
    MapChunkId,
};

pub struct Kibo;

const KIBO_OFFICIAL_URL: &str = "https://kibo.money/api";
const KIBO_OFFICIAL_BACKUP_URL: &str = "https://backup.kibo.money/api";

const RETRIES: usize = 10;

impl Kibo {
    fn get_base_url(try_index: usize) -> &'static str {
        if try_index < RETRIES / 2 {
            KIBO_OFFICIAL_URL
        } else {
            KIBO_OFFICIAL_BACKUP_URL
        }
    }

    pub fn fetch_height_prices(chunk_id: HeightMapChunkId) -> color_eyre::Result<Vec<OHLC>> {
        log("kibo: fetch height prices");

        retry(
            |try_index| {
                let base_url = Self::get_base_url(try_index);

                let body: Value = reqwest::blocking::get(format!(
                    "{base_url}/height-to-price?chunk={}",
                    chunk_id.to_usize()
                ))?
                .json()?;

                Ok(body
                    .as_object()
                    .context("Expect to be an object")?
                    .get("dataset")
                    .context("Expect object to have dataset")?
                    .as_object()
                    .context("Expect to be an object")?
                    .get("map")
                    .context("Expect to have map")?
                    .as_array()
                    .context("Expect to be an array")?
                    .iter()
                    .map(Self::value_to_ohlc)
                    .collect_vec())
            },
            10,
            RETRIES,
        )
    }

    pub fn fetch_date_prices(chunk_id: DateMapChunkId) -> color_eyre::Result<BTreeMap<Date, OHLC>> {
        log("kibo: fetch date prices");

        retry(
            |try_index| {
                let base_url = Self::get_base_url(try_index);

                let body: Value = reqwest::blocking::get(format!(
                    "{base_url}/date-to-price?chunk={}",
                    chunk_id.to_usize()
                ))?
                .json()?;

                Ok(body
                    .as_object()
                    .context("Expect to be an object")?
                    .get("dataset")
                    .context("Expect object to have dataset")?
                    .as_object()
                    .context("Expect to be an object")?
                    .get("map")
                    .context("Expect to have map")?
                    .as_object()
                    .context("Expect to be an object")?
                    .iter()
                    .map(|(serialized_date, value)| {
                        let date = Date::wrap(NaiveDate::from_str(serialized_date).unwrap());

                        (date, Self::value_to_ohlc(value))
                    })
                    .collect::<BTreeMap<_, _>>())
            },
            10,
            RETRIES,
        )
    }

    fn value_to_ohlc(value: &Value) -> OHLC {
        let ohlc = value.as_object().unwrap();

        let get_value = |key: &str| ohlc.get(key).unwrap().as_f64().unwrap() as f32;

        OHLC {
            open: get_value("open"),
            high: get_value("high"),
            low: get_value("low"),
            close: get_value("close"),
        }
    }
}
