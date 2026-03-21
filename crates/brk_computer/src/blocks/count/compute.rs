use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{Indexes, StoredU32};
use vecdb::Exit;

use super::Vecs;

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        // Block count raw + cumulative
        self.total.block.compute_range(
            starting_indexes.height,
            &indexer.vecs.blocks.weight,
            |h| (h, StoredU32::from(1_u32)),
            exit,
        )?;
        self.total.compute_rest(starting_indexes.height, exit)?;

        Ok(())
    }
}
