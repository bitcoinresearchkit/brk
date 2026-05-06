use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::StoredU64;
use vecdb::Exit;

use super::Vecs;
use crate::blocks;

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        lookback: &blocks::LookbackVecs,
        exit: &Exit,
    ) -> Result<()> {
        let starting_height = indexer.safe_lengths().height;
        let window_starts = lookback.window_starts();

        self.vbytes
            .compute(starting_height, &window_starts, exit, |height| {
                Ok(height.compute_transform(
                    starting_height,
                    &indexer.vecs.blocks.weight,
                    |(h, weight, ..)| (h, StoredU64::from(weight.to_vbytes_floor())),
                    exit,
                )?)
            })?;

        self.size.compute(
            starting_height,
            &window_starts,
            &indexer.vecs.blocks.total,
            exit,
        )?;

        Ok(())
    }
}
