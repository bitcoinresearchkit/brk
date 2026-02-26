use brk_error::Result;
use brk_indexer::Indexer;
use vecdb::Exit;

use super::Vecs;
use crate::{ComputeIndexes, blocks, indexes, prices, transactions};

impl Vecs {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        blocks: &blocks::Vecs,
        transactions: &transactions::Vecs,
        prices: &prices::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        // Block rewards (coinbase, subsidy, fee_dominance, etc.)
        self.rewards.compute(
            indexer,
            indexes,
            &blocks.count,
            &transactions.fees,
            prices,
            starting_indexes,
            exit,
        )?;

        // Hashrate metrics (disjoint field borrow via coinbase_sum)
        self.hashrate.compute(
            &blocks.count,
            &blocks.difficulty,
            &self.rewards.coinbase_sum._24h.sats.height,
            &self.rewards.coinbase_sum._24h.usd.height,
            starting_indexes,
            exit,
        )?;

        let _lock = exit.lock();
        self.db.compact()?;
        Ok(())
    }
}
