use std::{collections::BTreeMap, str::FromStr};

use chrono::NaiveDate;
use color_eyre::eyre::ContextCompat;
use log::info;
use serde_json::Value;

use crate::{
    structs::{Date, DateMapChunkId, HeightMapChunkId, MapChunkId, OHLC},
    utils::retry,
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
        info!("kibo: fetch height prices");

        retry(
            |try_index| {
                let base_url = Self::get_base_url(try_index);

                let body: Value = reqwest::blocking::get(format!(
                    "{base_url}/height-to-price?chunk={}",
                    chunk_id.to_usize()
                ))?
                .json()?;

                let vec = body
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
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(vec)
            },
            30,
            RETRIES,
        )
    }

    pub fn fetch_date_prices(chunk_id: DateMapChunkId) -> color_eyre::Result<BTreeMap<Date, OHLC>> {
        info!("kibo: fetch date prices");

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
                    .map(|(serialized_date, value)| -> color_eyre::Result<_> {
                        let date = Date::wrap(NaiveDate::from_str(serialized_date)?);
                        Ok((date, Self::value_to_ohlc(value)?))
                    })
                    .collect::<Result<BTreeMap<_, _>, _>>()?)
            },
            30,
            RETRIES,
        )
    }

    fn value_to_ohlc(value: &Value) -> color_eyre::Result<OHLC> {
        let ohlc = value.as_object().context("Expect as_object to work")?;

        let get_value = |key: &str| -> color_eyre::Result<f32> {
            Ok(ohlc
                .get(key)
                .context("Expect get key to work")?
                .as_f64()
                .context("Expect as_f64 to work")? as f32)
        };

        Ok(OHLC {
            open: get_value("open")?,
            high: get_value("high")?,
            low: get_value("low")?,
            close: get_value("close")?,
        })
    }
}
