use brk_error::Result;
use brk_indexer::Indexer;
use vecdb::Exit;

use super::Vecs;
use crate::{blocks, ComputeIndexes};

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        count_vecs: &blocks::CountVecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        let window_starts = count_vecs.window_starts();
        self.tx_count.compute(
            starting_indexes.height,
            &window_starts,
            exit,
            |height| {
                Ok(height.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.transactions.first_txindex,
                    &indexer.vecs.transactions.txid,
                    exit,
                )?)
            },
        )?;

        Ok(())
    }
}
