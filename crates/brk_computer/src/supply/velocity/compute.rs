use brk_error::Result;
use brk_types::Indexes;
use vecdb::Exit;

use super::Vecs;
use crate::{blocks, distribution, transactions};

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        blocks: &blocks::Vecs,
        transactions: &transactions::Vecs,
        distribution: &distribution::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        // velocity = rolling_1y_sum(volume) / circulating_supply
        let circulating_supply = &distribution.utxo_cohorts.all.metrics.supply.total;

        // BTC velocity at height level
        self.btc.height.compute_rolling_ratio(
            starting_indexes.height,
            &blocks.lookback._1y,
            &transactions.volume.sent_sum.raw.sats.height,
            &circulating_supply.sats.height,
            exit,
        )?;

        // USD velocity at height level
        self.usd.height.compute_rolling_ratio(
            starting_indexes.height,
            &blocks.lookback._1y,
            &transactions.volume.sent_sum.raw.usd.height,
            &circulating_supply.usd.height,
            exit,
        )?;

        Ok(())
    }
}
