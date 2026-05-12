use std::sync::Arc;

use brk_computer::prices::Vecs as PricesVecs;
use brk_error::{Error, Result};
use brk_oracle::{Config, Oracle, cents_to_bin};
use brk_types::{
    Dollars, ExchangeRates, HistoricalPrice, HistoricalPriceEntry, Hour4, INDEX_EPOCH, Timestamp,
};
use vecdb::{AnyVec, ReadableVec, VecIndex};

use crate::Query;

impl Query {
    pub fn live_price(&self) -> Result<Dollars> {
        let base = self.cached_oracle()?;
        Ok(match self.mempool() {
            Some(mempool) => {
                let mut oracle = (*base).clone();
                oracle.process_histogram(&mempool.live_histogram());
                oracle.price_dollars()
            }
            None => base.price_dollars(),
        })
    }

    /// Oracle warmed by the last `window_size` committed blocks, seeded from
    /// the last committed price. Cached per tip height; rebuilt on advance or
    /// reorg. Reads are capped at `safe_lengths` so concurrent indexer writes
    /// stay invisible.
    fn cached_oracle(&self) -> Result<Arc<Oracle>> {
        let safe_lengths = self.safe_lengths();
        let height = safe_lengths.height;

        if let Some(oracle) = self
            .0
            .live_oracle
            .read()
            .unwrap()
            .as_ref()
            .filter(|(h, _)| *h == height)
            .map(|(_, o)| o.clone())
        {
            return Ok(oracle);
        }

        let cents_height = &self.computer().prices.spot.cents.height;
        let last_cents = cents_height
            .len()
            .checked_sub(1)
            .and_then(|i| cents_height.collect_one_at(i))
            .ok_or_else(|| Error::NotFound("oracle prices not yet computed".to_string()))?;

        let config = Config::default();
        let seed_bin = cents_to_bin(last_cents.inner() as f64);
        let tip = height.to_usize();
        let warmup_range = tip.saturating_sub(config.window_size)..tip;
        let oracle = Arc::new(Oracle::from_checkpoint(seed_bin, config, |o| {
            PricesVecs::feed_blocks(o, self.indexer(), warmup_range, Some(&safe_lengths));
        }));

        let mut cache = self.0.live_oracle.write().unwrap();
        if cache.as_ref().is_none_or(|(h, _)| *h != height) {
            *cache = Some((height, oracle.clone()));
        }
        Ok(oracle)
    }

    pub fn historical_price(&self, timestamp: Option<Timestamp>) -> Result<HistoricalPrice> {
        let prices = match timestamp {
            Some(ts) => self.price_at(ts)?,
            None => self.all_prices()?,
        };
        Ok(HistoricalPrice {
            prices,
            exchange_rates: ExchangeRates {},
        })
    }

    fn price_at(&self, target: Timestamp) -> Result<Vec<HistoricalPriceEntry>> {
        if *target < INDEX_EPOCH {
            return Ok(vec![]);
        }
        let h4 = Hour4::from_timestamp(target);
        let cents = self.computer().prices.spot.cents.hour4.collect_one(h4);
        Ok(vec![HistoricalPriceEntry {
            time: h4.to_timestamp(),
            usd: Dollars::from(cents.flatten().unwrap_or_default()),
        }])
    }

    fn all_prices(&self) -> Result<Vec<HistoricalPriceEntry>> {
        let computer = self.computer();
        Ok(computer
            .prices
            .spot
            .cents
            .hour4
            .collect()
            .into_iter()
            .enumerate()
            .filter_map(|(i, cents)| {
                Some(HistoricalPriceEntry {
                    time: Hour4::from(i).to_timestamp(),
                    usd: Dollars::from(cents?),
                })
            })
            .collect())
    }
}
