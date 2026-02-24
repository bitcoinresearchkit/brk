use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::StoredU64;
use vecdb::Exit;

use super::Vecs;
use crate::{blocks, ComputeIndexes};

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        count_vecs: &blocks::CountVecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        let window_starts = count_vecs.window_starts();

        // vbytes = ceil(weight / 4), stored at height level
        self.vbytes.compute(
            starting_indexes.height,
            &window_starts,
            exit,
            |height| {
                Ok(height.compute_transform(
                    starting_indexes.height,
                    &indexer.vecs.blocks.weight,
                    |(h, weight, ..)| (h, StoredU64::from(weight.to_vbytes_floor())),
                    exit,
                )?)
            },
        )?;

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
