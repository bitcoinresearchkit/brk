use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::Indexes;
use vecdb::Exit;

use super::Vecs;
use crate::{blocks, indexes};

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        blocks: &blocks::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        let window_starts = blocks.lookback.window_starts();
        self.total.compute(
            starting_indexes.height,
            &indexes.tx_index.output_count,
            &indexer.vecs.transactions.first_tx_index,
            &indexes.height.tx_index_count,
            &window_starts,
            exit,
            0,
        )?;
        Ok(())
    }
}
