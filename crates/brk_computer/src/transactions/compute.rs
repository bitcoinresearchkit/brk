use brk_error::Result;
use brk_indexer::Indexer;
use vecdb::Exit;

use crate::{indexes, inputs, outputs, ComputeIndexes};

use super::Vecs;

impl Vecs {
    #[allow(clippy::too_many_arguments)]
    pub fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        inputs: &inputs::Vecs,
        outputs: &outputs::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        // Count computes first
        self.count
            .compute(indexer, indexes, starting_indexes, exit)?;

        // Versions depends on count
        self.versions
            .compute(indexer, indexes, starting_indexes, exit)?;

        // Size computes next
        self.size
            .compute(indexer, indexes, starting_indexes, exit)?;

        // Fees depends on size
        self.fees.compute(
            indexer,
            indexes,
            inputs,
            &self.size,
            starting_indexes,
            exit,
        )?;

        // Volume depends on fees and input/output counts
        self.volume.compute(
            indexer,
            indexes,
            &self.count,
            &self.fees,
            &inputs.count,
            &outputs.count,
            starting_indexes,
            exit,
        )?;

        let _lock = exit.lock();
        self.db.compact()?;
        Ok(())
    }
}
