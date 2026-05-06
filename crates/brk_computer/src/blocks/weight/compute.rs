use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::BasisPoints16;
use vecdb::Exit;

use super::Vecs;

impl Vecs {
    pub(crate) fn compute(&mut self, indexer: &Indexer, exit: &Exit) -> Result<()> {
        let starting_height = indexer.safe_lengths().height;
        self.fullness.bps.compute_transform(
            starting_height,
            &indexer.vecs.blocks.weight,
            |(h, weight, ..)| (h, BasisPoints16::from(weight.fullness())),
            exit,
        )?;

        Ok(())
    }
}
