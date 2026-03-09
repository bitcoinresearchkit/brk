use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{Indexes, StoredU64};
use vecdb::Exit;

use super::Vecs;
use crate::blocks;

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        lookback: &blocks::LookbackVecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        let window_starts = lookback.window_starts();

        // vbytes = floor(weight / 4), stored at height level
        self.vbytes
            .compute(starting_indexes.height, &window_starts, exit, |height| {
                Ok(height.compute_transform(
                    starting_indexes.height,
                    &indexer.vecs.blocks.weight,
                    |(h, weight, ..)| (h, StoredU64::from(weight.to_vbytes_floor())),
                    exit,
                )?)
            })?;

        // size from indexer total_size
        self.size.compute(
            starting_indexes.height,
            &window_starts,
            &indexer.vecs.blocks.total_size,
            exit,
        )?;

        Ok(())
    }
}
