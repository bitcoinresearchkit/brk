use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::StoredU32;
use vecdb::Exit;

use super::Vecs;

impl Vecs {
    pub(crate) fn compute(&mut self, indexer: &Indexer, exit: &Exit) -> Result<()> {
        let starting_height = indexer.safe_lengths().height;
        self.total.block.compute_range(
            starting_height,
            &indexer.vecs.blocks.weight,
            |h| (h, StoredU32::from(1_u32)),
            exit,
        )?;
        self.total.compute_rest(starting_height, exit)?;

        Ok(())
    }
}
