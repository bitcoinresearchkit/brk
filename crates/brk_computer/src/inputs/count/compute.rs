use brk_error::Result;
use brk_indexer::Indexer;
use vecdb::Exit;

use super::Vecs;
use crate::{blocks, indexes};

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        blocks: &blocks::Vecs,
        exit: &Exit,
    ) -> Result<()> {
        let starting_height = indexer.safe_lengths().height;
        let window_starts = blocks.lookback.window_starts();
        self.0.compute(
            starting_height,
            &indexes.tx_index.input_count,
            &indexer.vecs.transactions.first_tx_index,
            &indexes.height.tx_index_count,
            &window_starts,
            exit,
            0,
        )?;

        Ok(())
    }
}
