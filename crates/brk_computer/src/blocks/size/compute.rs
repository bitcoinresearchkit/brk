use brk_error::Result;
use brk_indexer::Indexer;
use vecdb::Exit;

use super::Vecs;
use crate::{ComputeIndexes, indexes};

impl Vecs {
    pub fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.size.derive_from(
            indexes,
            starting_indexes,
            &indexer.vecs.blocks.total_size,
            exit,
        )?;

        self.vbytes.derive_from(indexes, starting_indexes, exit)?;

        Ok(())
    }
}
