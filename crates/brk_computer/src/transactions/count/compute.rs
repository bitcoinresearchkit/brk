use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::Indexes;
use vecdb::Exit;

use super::Vecs;
use crate::blocks;

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        lookback: &blocks::LookbackVecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        let window_starts = lookback.window_starts();
        self.total
            .compute(starting_indexes.height, &window_starts, exit, |height| {
                Ok(height.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.transactions.first_txindex,
                    &indexer.vecs.transactions.txid,
                    exit,
                )?)
            })?;

        Ok(())
    }
}
