use brk_error::Result;
use vecdb::Exit;

use super::Vecs;
use crate::{blocks, distribution, indexes, scripts, transactions, ComputeIndexes};

impl Vecs {
    #[allow(clippy::too_many_arguments)]
    pub fn compute(
        &mut self,
        indexes: &indexes::Vecs,
        scripts: &scripts::Vecs,
        blocks: &blocks::Vecs,
        transactions: &transactions::Vecs,
        distribution: &distribution::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        // 1. Compute burned/unspendable supply
        self.burned
            .compute(indexes, scripts, blocks, starting_indexes, exit)?;

        // 2. Compute inflation rate
        self.inflation
            .compute(blocks, distribution, starting_indexes, exit)?;

        // 3. Compute velocity
        self.velocity
            .compute(transactions, distribution, starting_indexes, exit)?;

        // Note: circulating and market_cap are lazy - no compute needed

        let _lock = exit.lock();
        self.db.compact()?;

        Ok(())
    }
}
