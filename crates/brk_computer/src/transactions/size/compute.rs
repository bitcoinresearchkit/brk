use brk_error::Result;
use brk_indexer::Indexer;
use vecdb::Exit;

use super::Vecs;
use crate::{indexes, ComputeIndexes};

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.weight
            .derive_from(indexer, indexes, starting_indexes, exit)?;

        self.vsize
            .derive_from(indexer, indexes, starting_indexes, exit)?;

        Ok(())
    }
}
