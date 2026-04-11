use brk_error::Result;
use brk_types::{Indexes, StoredU32};
use vecdb::Exit;

use super::Vecs;
use crate::indexes;

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.epoch.height.compute_transform(
            starting_indexes.height,
            &indexes.height.halving,
            |(h, epoch, ..)| (h, epoch),
            exit,
        )?;

        self.blocks_to_halving.height.compute_transform(
            starting_indexes.height,
            &indexes.height.halving,
            |(h, ..)| (h, StoredU32::from(h.left_before_next_halving())),
            exit,
        )?;

        Ok(())
    }
}
