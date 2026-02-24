use brk_error::Result;
use brk_indexer::Indexer;
use vecdb::Exit;

use crate::{blocks, indexes, inputs, outputs, prices, ComputeIndexes};

use super::Vecs;

impl Vecs {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        blocks: &blocks::Vecs,
        inputs: &inputs::Vecs,
        outputs: &outputs::Vecs,
        prices: &prices::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        // Count computes first
        self.count
            .compute(indexer, &blocks.count, starting_indexes, exit)?;

        // Versions depends on count
        self.versions
            .compute(indexer, starting_indexes, exit)?;

        // Size computes next
        self.size
            .compute(indexer, indexes, starting_indexes, exit)?;

        // Fees depends on size, blocks (window starts), prices (USD conversion)
        self.fees.compute(
            indexer,
            indexes,
            inputs,
            &self.size,
            blocks,
            prices,
            starting_indexes,
            exit,
        )?;

        // Volume depends on fees, counts, and blocks (lookback vecs, interval)
        self.volume.compute(
            indexer,
            indexes,
            blocks,
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
