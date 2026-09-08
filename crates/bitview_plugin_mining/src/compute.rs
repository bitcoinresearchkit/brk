use brk_error::Result;

use bitview_plugin::{ComputePlugin, UpdateContext};

use super::Vecs;
use crate::Dependencies;

impl ComputePlugin for Vecs {
    type Dependencies<'a> = Dependencies<'a>;
    type Output = ();

    fn compute(
        &mut self,
        dependencies: Self::Dependencies<'_>,
        context: UpdateContext<'_>,
    ) -> Result<Self::Output> {
        let Dependencies {
            indexer,
            mappings,
            blocks,
            transactions,
            price: prices,
        } = dependencies;
        let exit = context.exit();

        self.db.sync_bg_tasks()?;

        // Block rewards (coinbase, subsidy, fee_dominance, etc.)
        super::rewards::compute(
            &mut self.rewards,
            indexer,
            mappings,
            &blocks.lookback,
            transactions,
            prices,
            exit,
        )?;

        super::hashrate::compute(
            &mut self.hashrate,
            indexer,
            &blocks.count,
            &blocks.lookback,
            &blocks.difficulty,
            &self.rewards.coinbase.rolling.sum._24h.sats.height,
            &self.rewards.coinbase.rolling.sum._24h.usd.height,
            exit,
        )?;

        context.compact_database(&self.db);
        Ok(())
    }
}
