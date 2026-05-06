use brk_error::Result;
use brk_indexer::Indexer;
use vecdb::Exit;

use super::Vecs;
use crate::blocks;

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        lookback: &blocks::LookbackVecs,
        exit: &Exit,
    ) -> Result<()> {
        let starting_height = indexer.safe_lengths().height;

        let window_starts = lookback.window_starts();
        self.total
            .compute(starting_height, &window_starts, exit, |height| {
                Ok(height.compute_count_from_indexes(
                    starting_height,
                    &indexer.vecs.transactions.first_tx_index,
                    &indexer.vecs.transactions.txid,
                    exit,
                )?)
            })?;

        Ok(())
    }
}
