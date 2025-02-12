use std::collections::BTreeMap;

use color_eyre::eyre::ContextCompat;
use log::info;
use serde_json::Value;

use crate::{
    structs::{Date, Timestamp, OHLC},
    utils::retry,
};

pub struct Kraken;

impl Kraken {
    pub fn fetch_1mn_prices() -> color_eyre::Result<BTreeMap<u32, OHLC>> {
        info!("kraken: fetch 1mn");

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
                    .map(|value| -> color_eyre::Result<_> {
                        let array = value.as_array().context("Expect as_array to work")?;

                        let timestamp = array
                            .first()
                            .context("Expect first to work")?
                            .as_u64()
                            .expect("Expect as_u64 to work")
                            as u32;

                        let get_f32 = |index: usize| -> color_eyre::Result<f32> {
                            Ok(array
                                .get(index)
                                .context("Expect get index to work")?
                                .as_str()
                                .context("Expect as_str to work")?
                                .parse::<f32>()?)
                        };

                        Ok((
                            timestamp,
                            OHLC {
                                open: get_f32(1)?,
                                high: get_f32(2)?,
                                low: get_f32(3)?,
                                close: get_f32(4)?,
                            },
                        ))
                    })
                    .collect::<Result<BTreeMap<_, _>, _>>()?)
            },
            30,
            10,
        )
    }

    pub fn fetch_daily_prices() -> color_eyre::Result<BTreeMap<Date, OHLC>> {
        info!("fetch kraken daily");

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
                    .map(|value| -> color_eyre::Result<_> {
                        let array = value.as_array().context("Expect as_array to work")?;

                        let date = Timestamp::from(
                            array
                                .first()
                                .context("Expect first to work")?
                                .as_u64()
                                .context("Expect as_u64 to work")?
                                as u32,
                        )
                        .to_date();

                        let get_f32 = |index: usize| -> color_eyre::Result<f32> {
                            Ok(array
                                .get(index)
                                .context("Expect get index to work")?
                                .as_str()
                                .context("Expect as_str to work")?
                                .parse::<f32>()?)
                        };

                        Ok((
                            date,
                            OHLC {
                                open: get_f32(1)?,
                                high: get_f32(2)?,
                                low: get_f32(3)?,
                                close: get_f32(4)?,
                            },
                        ))
                    })
                    .collect::<Result<BTreeMap<_, _>, _>>()?)
            },
            30,
            10,
        )
    }
}
