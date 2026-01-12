#![doc = include_str!("../README.md")]

use std::{path::Path, thread::sleep, time::Duration};

use brk_error::{Error, Result};
use brk_types::{Date, Height, OHLCCents, Timestamp};
use tracing::info;

mod binance;
mod brk;
mod kraken;
mod ohlc;
mod retry;
mod source;

pub use binance::*;
pub use brk::*;
pub use kraken::*;
pub use ohlc::compute_ohlc_from_range;
use retry::*;
pub use source::{PriceSource, TrackedSource};

const MAX_RETRIES: usize = 12 * 60; // 12 hours of retrying

/// Check HTTP response status and return bytes or error
pub fn check_response(response: minreq::Response, url: &str) -> Result<Vec<u8>> {
    let status = response.status_code as u16;
    if (200..300).contains(&status) {
        Ok(response.into_bytes())
    } else {
        Err(Error::HttpStatus {
            status,
            url: url.to_string(),
        })
    }
}

#[derive(Clone)]
pub struct Fetcher {
    pub binance: Option<TrackedSource<Binance>>,
    pub kraken: Option<TrackedSource<Kraken>>,
    pub brk: TrackedSource<BRK>,
}

impl Fetcher {
    pub fn import(exchanges: bool, hars_path: Option<&Path>) -> Result<Self> {
        Self::new(exchanges, hars_path)
    }

    pub fn new(exchanges: bool, hars_path: Option<&Path>) -> Result<Self> {
        Ok(Self {
            binance: exchanges.then(|| TrackedSource::new(Binance::init(hars_path))),
            kraken: exchanges.then(|| TrackedSource::new(Kraken::default())),
            brk: TrackedSource::new(BRK::default()),
        })
    }

    /// Iterate over all active sources in priority order
    fn for_each_source<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut dyn PriceSource),
    {
        if let Some(binance) = &mut self.binance {
            f(binance);
        }
        if let Some(kraken) = &mut self.kraken {
            f(kraken);
        }
        f(&mut self.brk);
    }

    /// Try fetching from each source in order, return first success
    fn try_sources<F>(&mut self, mut fetch: F) -> Option<Result<OHLCCents>>
    where
        F: FnMut(&mut dyn PriceSource) -> Option<Result<OHLCCents>>,
    {
        if let Some(binance) = &mut self.binance
            && let Some(Ok(ohlc)) = fetch(binance)
        {
            return Some(Ok(ohlc));
        }
        if let Some(kraken) = &mut self.kraken
            && let Some(Ok(ohlc)) = fetch(kraken)
        {
            return Some(Ok(ohlc));
        }
        if let Some(Ok(ohlc)) = fetch(&mut self.brk) {
            return Some(Ok(ohlc));
        }
        None
    }

    pub fn get_date(&mut self, date: Date) -> Result<OHLCCents> {
        self.fetch_with_retry(
            |source| source.get_date(date),
            || format!("Failed to fetch price for date {date}"),
        )
    }

    pub fn get_height(
        &mut self,
        height: Height,
        timestamp: Timestamp,
        previous_timestamp: Option<Timestamp>,
    ) -> Result<OHLCCents> {
        let timestamp = timestamp.floor_seconds();
        let previous_timestamp = previous_timestamp.map(|t| t.floor_seconds());

        if previous_timestamp.is_none() && height != Height::ZERO {
            panic!("previous_timestamp required for non-genesis blocks");
        }

        self.fetch_with_retry(
            |source| {
                // Try 1mn data first, fall back to height-based
                source
                    .get_1mn(timestamp, previous_timestamp)
                    .or_else(|| source.get_height(height))
            },
            || {
                let date = Date::from(timestamp);
                format!(
                    "
Can't find the price for: height: {height} - date: {date}
1mn APIs are limited to the last 16 hours for Binance's and the last 10 hours for Kraken's
How to fix this:
0. Try rerunning the program first, it usually fixes the problem
1. If it didn't, go to https://www.binance.com/en/trade/BTC_USDT?type=spot
2. Select 1mn interval
3. Open the inspector/dev tools
4. Go to the Network Tab
5. Filter URLs by 'uiKlines'
6. Go back to the chart and scroll until you pass the date mentioned few lines ago
7. Go back to the dev tools
8. Export to a har file (if there is no explicit button, click on the cog button)
9. Move the file to 'parser/imports/binance.har'
"
                )
            },
        )
    }

    /// Try each source in order, with retries on total failure
    fn fetch_with_retry<F, E>(&mut self, mut fetch: F, error_message: E) -> Result<OHLCCents>
    where
        F: FnMut(&mut dyn PriceSource) -> Option<Result<OHLCCents>>,
        E: Fn() -> String,
    {
        for retry in 0..=MAX_RETRIES {
            if let Some(ohlc) = self.try_sources(&mut fetch) {
                return ohlc;
            }

            // All sources failed
            if retry < MAX_RETRIES {
                info!("All sources failed, retrying in 60s...");
                sleep(Duration::from_secs(60));
                self.clear_caches();
            }
        }

        Err(Error::FetchFailed(error_message()))
    }

    fn clear_caches(&mut self) {
        self.for_each_source(|s| s.clear());
    }

    /// Clear caches and reset health state for all sources
    pub fn clear(&mut self) {
        if let Some(binance) = &mut self.binance {
            binance.clear();
            binance.reset_health();
        }
        if let Some(kraken) = &mut self.kraken {
            kraken.clear();
            kraken.reset_health();
        }
        self.brk.clear();
        self.brk.reset_health();
    }

    /// Ping all sources and return results for each
    pub fn ping(&self) -> Vec<(&'static str, Result<()>)> {
        let mut results = Vec::new();

        if let Some(binance) = &self.binance {
            results.push((binance.name(), binance.ping()));
        }
        if let Some(kraken) = &self.kraken {
            results.push((kraken.name(), kraken.ping()));
        }
        results.push((self.brk.name(), self.brk.ping()));

        results
    }
}
