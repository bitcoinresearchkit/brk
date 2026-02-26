use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::StoredF32;
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

        self.weight.compute(
            starting_indexes.height,
            &window_starts,
            &indexer.vecs.blocks.weight,
            exit,
        )?;

        self.fullness.compute(
            starting_indexes.height,
            &window_starts,
            exit,
            |vec| {
                vec.compute_transform(
                    starting_indexes.height,
                    &indexer.vecs.blocks.weight,
                    |(h, weight, ..)| (h, StoredF32::from(weight.fullness())),
                    exit,
                )?;
                Ok(())
            },
        )?;

        Ok(())
    }
}
