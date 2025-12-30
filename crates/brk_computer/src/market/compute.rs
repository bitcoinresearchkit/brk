use brk_error::Result;
use vecdb::Exit;

use crate::{price, Indexes};
use crate::utils::OptionExt;

use super::Vecs;

impl Vecs {
    pub fn compute(
        &mut self,
        price: &price::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        // ATH metrics (independent)
        self.ath.compute(price, starting_indexes, exit)?;

        // History metrics (independent)
        self.history.compute(price, starting_indexes, exit)?;

        // Volatility metrics (depends on history._1d_price_returns)
        self.volatility.compute(
            starting_indexes,
            exit,
            self.history._1d_price_returns.dateindex.u(),
        )?;

        // Range metrics (independent)
        self.range.compute(price, starting_indexes, exit)?;

        // Moving average metrics (independent)
        self.moving_average.compute(price, starting_indexes, exit)?;

        // DCA metrics
        self.dca.compute(price, starting_indexes, exit)?;

        let _lock = exit.lock();
        self.db.compact()?;
        Ok(())
    }
}
