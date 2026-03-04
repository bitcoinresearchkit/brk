use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::Indexes;
use vecdb::Exit;

use super::Vecs;
use crate::indexes;

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.weight
            .derive_from(indexer, indexes, starting_indexes, exit)?;

        self.vsize
            .derive_from(indexer, indexes, starting_indexes, exit)?;

        Ok(())
    }
}
