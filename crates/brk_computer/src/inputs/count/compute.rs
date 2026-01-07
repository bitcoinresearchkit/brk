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
        self.indexes_to_count.derive_from(
            indexer,
            indexes,
            starting_indexes,
            &indexes.transaction.txindex_to_input_count,
            exit,
        )?;

        Ok(())
    }
}
