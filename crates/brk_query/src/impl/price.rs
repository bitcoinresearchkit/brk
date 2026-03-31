use brk_error::Result;
use brk_types::{Dollars, ExchangeRates, HistoricalPrice, HistoricalPriceEntry, Timestamp};
use vecdb::{ReadableVec, VecIndex};

use crate::Query;

impl Query {
    pub fn live_price(&self) -> Result<Dollars> {
        let mut oracle = self.computer().prices.live_oracle(self.indexer())?;

        if let Some(mempool) = self.mempool() {
            let txs = mempool.get_txs();
            oracle.process_outputs(
                txs.values()
                    .flat_map(|tx| &tx.tx().output)
                    .map(|txout| (txout.value, txout.type_())),
            );
        }

        Ok(oracle.price_dollars())
    }

    pub fn historical_price(&self, timestamp: Option<Timestamp>) -> Result<HistoricalPrice> {
        let indexer = self.indexer();
        let computer = self.computer();
        let max_height = self.height().to_usize();
        let end = max_height + 1;

        let timestamps = indexer.vecs.blocks.timestamp.collect();
        let all_prices = computer.prices.spot.cents.height.collect();

        let prices = if let Some(target_ts) = timestamp {
            let target = usize::from(target_ts);
            let h = timestamps
                .binary_search_by_key(&target, |t| usize::from(*t))
                .unwrap_or_else(|i| i.min(max_height));

            vec![HistoricalPriceEntry {
                time: usize::from(timestamps[h]) as u64,
                usd: Dollars::from(all_prices[h]),
            }]
        } else {
            let step = (max_height / 200).max(1);
            (0..end)
                .step_by(step)
                .map(|h| HistoricalPriceEntry {
                    time: usize::from(timestamps[h]) as u64,
                    usd: Dollars::from(all_prices[h]),
                })
                .collect()
        };

        Ok(HistoricalPrice {
            prices,
            exchange_rates: ExchangeRates {},
        })
    }
}
