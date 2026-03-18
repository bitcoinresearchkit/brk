use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{Indexes, StoredU32};
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
        self.adjustment.bps.height.compute_ratio_change(
            starting_indexes.height,
            &indexer.vecs.blocks.difficulty,
            2016,
            exit,
        )?;

        self.epoch.height.compute_transform(
            starting_indexes.height,
            &indexes.height.epoch,
            |(h, epoch, ..)| (h, epoch),
            exit,
        )?;

        self.blocks_to_retarget.height.compute_transform(
            starting_indexes.height,
            &indexes.height.identity,
            |(h, ..)| (h, StoredU32::from(h.left_before_next_diff_adj())),
            exit,
        )?;

        Ok(())
    }
}
