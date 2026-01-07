use brk_error::Result;
use brk_indexer::Indexer;
use vecdb::Exit;

use super::Vecs;
use crate::{indexes, ComputeIndexes};

impl Vecs {
    pub fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.indexes_to_block_size.derive_from(
            indexes,
            starting_indexes,
            &indexer.vecs.block.height_to_total_size,
            exit,
        )?;

        self.indexes_to_block_vbytes.derive_from(
            indexes,
            starting_indexes,
            &self.height_to_vbytes,
            exit,
        )?;

        Ok(())
    }
}
