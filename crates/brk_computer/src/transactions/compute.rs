use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::Indexes;
use vecdb::Exit;

use crate::{blocks, indexes, inputs, outputs, prices};

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
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        // count, versions, size are independent — parallelize
        let (r1, (r2, r3)) = rayon::join(
            || self.count.compute(indexer, &blocks.lookback, starting_indexes, exit),
            || {
                rayon::join(
                    || self.versions.compute(indexer, starting_indexes, exit),
                    || self.size.compute(indexer, indexes, starting_indexes, exit),
                )
            },
        );
        r1?;
        r2?;
        r3?;

        // Fees depends on size
        self.fees
            .compute(indexer, indexes, inputs, &self.size, starting_indexes, exit)?;

        // Volume depends on fees, counts, and blocks (lookback vecs, interval)
        self.volume.compute(
            indexer,
            indexes,
            blocks,
            prices,
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
