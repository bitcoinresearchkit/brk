use brk_error::Result;
use brk_indexer::Indexer;
use vecdb::Exit;

use super::Vecs;
use crate::{ComputeIndexes, blocks, indexes, transactions};

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        blocks: &blocks::Vecs,
        transactions: &transactions::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        // Block rewards (coinbase, subsidy, fee_dominance, etc.)
        self.rewards.compute(
            indexer,
            indexes,
            &blocks.count,
            &transactions.fees,
            starting_indexes,
            exit,
        )?;

        // Hashrate metrics (uses rewards.coinbase_24h_sum — disjoint field borrow)
        self.hashrate.compute(
            &blocks.count,
            &blocks.difficulty,
            &self.rewards.coinbase_24h_sum,
            starting_indexes,
            exit,
        )?;

        let _lock = exit.lock();
        self.db.compact()?;
        Ok(())
    }
}
