use brk_error::Result;
use vecdb::Exit;

use super::super::{activity, cap, supply};
use super::Vecs;
use crate::{ComputeIndexes, blocks, distribution, prices};

impl Vecs {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn compute(
        &mut self,
        starting_indexes: &ComputeIndexes,
        prices: &prices::Vecs,
        blocks: &blocks::Vecs,
        distribution: &distribution::Vecs,
        activity: &activity::Vecs,
        supply: &supply::Vecs,
        cap: &cap::Vecs,
        exit: &Exit,
    ) -> Result<()> {
        let circulating_supply = &distribution
            .utxo_cohorts
            .all
            .metrics
            .supply
            .total
            .btc
            .height;
        let realized_price = &distribution
            .utxo_cohorts
            .all
            .metrics
            .realized
            .realized_price
            .usd
            .height;

        self.vaulted_price.usd.height.compute_divide(
            starting_indexes.height,
            realized_price,
            &activity.vaultedness.height,
            exit,
        )?;

        self.vaulted_price_ratio.compute_rest(
            blocks,
            prices,
            starting_indexes,
            exit,
            &self.vaulted_price.usd.height,
        )?;

        self.active_price.usd.height.compute_multiply(
            starting_indexes.height,
            realized_price,
            &activity.liveliness.height,
            exit,
        )?;

        self.active_price_ratio.compute_rest(
            blocks,
            prices,
            starting_indexes,
            exit,
            &self.active_price.usd.height,
        )?;

        self.true_market_mean.usd.height.compute_divide(
            starting_indexes.height,
            &cap.investor_cap.height,
            &supply.active_supply.btc.height,
            exit,
        )?;

        self.true_market_mean_ratio.compute_rest(
            blocks,
            prices,
            starting_indexes,
            exit,
            &self.true_market_mean.usd.height,
        )?;

        // cointime_price = cointime_cap / circulating_supply
        self.cointime_price.usd.height.compute_divide(
            starting_indexes.height,
            &cap.cointime_cap.height,
            circulating_supply,
            exit,
        )?;

        self.cointime_price_ratio.compute_rest(
            blocks,
            prices,
            starting_indexes,
            exit,
            &self.cointime_price.usd.height,
        )?;

        Ok(())
    }
}
