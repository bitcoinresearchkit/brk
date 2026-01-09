use brk_error::Result;
use vecdb::Exit;

use super::Vecs;
use crate::{ComputeIndexes, indexes};

impl Vecs {
    pub fn compute(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.interval.derive_from(indexes, starting_indexes, exit)?;

        Ok(())
    }
}
