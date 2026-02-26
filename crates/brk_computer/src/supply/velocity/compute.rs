use brk_error::Result;
use vecdb::Exit;

use super::Vecs;
use crate::{blocks, ComputeIndexes, distribution, transactions};

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        blocks: &blocks::Vecs,
        transactions: &transactions::Vecs,
        distribution: &distribution::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        // velocity = rolling_1y_sum(volume) / circulating_supply
        let circulating_supply = &distribution.utxo_cohorts.all.metrics.supply.total;

        // BTC velocity at height level
        self.btc.height.compute_rolling_ratio(
            starting_indexes.height,
            &blocks.count.height_1y_ago,
            &transactions.volume.sent_sum.sats,
            &circulating_supply.sats.height,
            exit,
        )?;

        // USD velocity at height level
        self.usd.height.compute_rolling_ratio(
            starting_indexes.height,
            &blocks.count.height_1y_ago,
            &transactions.volume.sent_sum.usd,
            &circulating_supply.usd.height,
            exit,
        )?;

        Ok(())
    }
}
