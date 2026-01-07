use brk_error::Result;
use vecdb::Exit;

use super::Vecs;
use crate::{indexes, ComputeIndexes};

impl Vecs {
    pub fn compute(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.indexes_to_block_interval.derive_from(
            indexes,
            starting_indexes,
            &self.height_to_interval,
            exit,
        )?;

        Ok(())
    }
}
