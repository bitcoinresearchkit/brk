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
        self.indexes_to_tx_weight.derive_from(
            indexer,
            indexes,
            starting_indexes,
            &self.txindex_to_weight,
            exit,
        )?;

        self.indexes_to_tx_vsize.derive_from(
            indexer,
            indexes,
            starting_indexes,
            &self.txindex_to_vsize,
            exit,
        )?;

        Ok(())
    }
}
