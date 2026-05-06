use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::StoredU32;
use vecdb::Exit;

use super::Vecs;
use crate::indexes;

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        exit: &Exit,
    ) -> Result<()> {
        let starting_height = indexer.safe_lengths().height;
        self.epoch.height.compute_transform(
            starting_height,
            &indexes.height.halving,
            |(h, epoch, ..)| (h, epoch),
            exit,
        )?;

        self.blocks_to_halving.height.compute_transform(
            starting_height,
            &indexes.height.halving,
            |(h, ..)| (h, StoredU32::from(h.left_before_next_halving())),
            exit,
        )?;

        Ok(())
    }
}
