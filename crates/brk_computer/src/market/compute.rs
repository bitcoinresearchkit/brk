use brk_error::Result;
use vecdb::Exit;

use crate::{blocks, distribution, price, ComputeIndexes};

use super::Vecs;

impl Vecs {
    pub fn compute(
        &mut self,
        price: &price::Vecs,
        blocks: &blocks::Vecs,
        distribution: &distribution::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        // ATH metrics (independent)
        self.ath.compute(price, starting_indexes, exit)?;

        // Lookback metrics (independent)
        self.lookback.compute(price, starting_indexes, exit)?;

        // Returns metrics (depends on lookback)
        self.returns.compute(starting_indexes, exit)?;

        // Volatility: all fields are lazy (derived from returns SD)

        // Range metrics (independent)
        self.range.compute(price, starting_indexes, exit)?;

        // Moving average metrics (independent)
        self.moving_average.compute(price, starting_indexes, exit)?;

        // DCA metrics (depends on lookback for lump sum comparison)
        self.dca
            .compute(price, &self.lookback, starting_indexes, exit)?;

        self.indicators.compute(
            &blocks.rewards,
            &self.returns,
            &self.moving_average,
            &self.range,
            price,
            distribution,
            starting_indexes,
            exit,
        )?;

        let _lock = exit.lock();
        self.db.compact()?;
        Ok(())
    }
}
