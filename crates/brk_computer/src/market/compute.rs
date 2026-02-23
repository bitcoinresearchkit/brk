use brk_error::Result;
use vecdb::Exit;

use crate::{ComputeIndexes, blocks, distribution, indexes, mining, prices};

use super::Vecs;

impl Vecs {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn compute(
        &mut self,
        indexes: &indexes::Vecs,
        prices: &prices::Vecs,
        blocks: &blocks::Vecs,
        mining: &mining::Vecs,
        distribution: &distribution::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        // ATH metrics (independent)
        self.ath.compute(prices, starting_indexes, exit)?;

        // Lookback metrics (independent)
        self.lookback
            .compute(blocks, prices, starting_indexes, exit)?;

        // Returns metrics (depends on lookback)
        self.returns
            .compute(indexes, blocks, starting_indexes, exit)?;

        // Volatility: all fields are lazy (derived from returns SD)

        // Range metrics (independent)
        self.range
            .compute(prices, blocks, starting_indexes, exit)?;

        // Moving average metrics (independent)
        self.moving_average
            .compute(blocks, prices, indexes, starting_indexes, exit)?;

        // DCA metrics (depends on lookback for lump sum comparison)
        self.dca
            .compute(indexes, prices, blocks, &self.lookback, starting_indexes, exit)?;

        self.indicators.compute(
            indexes,
            &mining.rewards,
            &self.returns,
            &self.range,
            prices,
            blocks,
            distribution,
            starting_indexes,
            exit,
        )?;

        let _lock = exit.lock();
        self.db.compact()?;
        Ok(())
    }
}
