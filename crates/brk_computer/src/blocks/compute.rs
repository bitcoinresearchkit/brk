use brk_error::Result;
use brk_indexer::Indexer;
use vecdb::Exit;

use crate::{ComputeIndexes, indexes, transactions};

use super::Vecs;

impl Vecs {
    pub fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        transactions: &transactions::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        // Core block metrics
        self.count
            .compute(indexer, indexes, &self.time, starting_indexes, exit)?;
        self.interval.compute(indexes, starting_indexes, exit)?;
        self.size
            .compute(indexer, indexes, starting_indexes, exit)?;
        self.weight
            .compute(indexer, indexes, starting_indexes, exit)?;

        // Time metrics (timestamps)
        self.time.compute(indexes, starting_indexes, exit)?;

        // Epoch metrics
        self.difficulty
            .compute(indexer, indexes, starting_indexes, exit)?;
        self.halving.compute(indexes, starting_indexes, exit)?;

        // Rewards depends on count and transactions fees
        self.rewards.compute(
            indexer,
            indexes,
            &self.count,
            &transactions.fees,
            starting_indexes,
            exit,
        )?;

        // Mining depends on count, difficulty, and rewards
        self.mining.compute(
            indexes,
            &self.count,
            &self.difficulty,
            &self.rewards,
            starting_indexes,
            exit,
        )?;

        let _lock = exit.lock();
        self.db.compact()?;
        Ok(())
    }
}
