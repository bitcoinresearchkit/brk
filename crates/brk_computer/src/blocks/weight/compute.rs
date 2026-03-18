use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{BasisPoints16, Indexes};
use vecdb::Exit;

use super::Vecs;

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.fullness.bps.compute_transform(
            starting_indexes.height,
            &indexer.vecs.blocks.weight,
            |(h, weight, ..)| (h, BasisPoints16::from(weight.fullness())),
            exit,
        )?;

        Ok(())
    }
}
