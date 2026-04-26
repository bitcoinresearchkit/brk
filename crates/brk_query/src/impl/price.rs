use brk_error::Result;
use brk_types::{Dollars, ExchangeRates, HistoricalPrice, HistoricalPriceEntry, Hour4, Timestamp};
use vecdb::ReadableVec;

use crate::Query;

impl Query {
    pub fn live_price(&self) -> Result<Dollars> {
        let mut oracle = self.computer().prices.live_oracle(self.indexer())?;

        if let Some(mempool) = self.mempool() {
            let txs = mempool.txs();
            oracle.process_outputs(
                txs.values()
                    .flat_map(|tx| &tx.output)
                    .map(|txout| (txout.value, txout.type_())),
            );
        }

        Ok(oracle.price_dollars())
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
