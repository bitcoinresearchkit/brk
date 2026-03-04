use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::Indexes;
use vecdb::Exit;

use super::Vecs;
use crate::{blocks, indexes, inputs, scripts};

impl Vecs {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        inputs: &inputs::Vecs,
        scripts: &scripts::Vecs,
        blocks: &blocks::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.spent
            .compute(&self.db, indexer, inputs, starting_indexes, exit)?;
        self.count.compute(
            indexer,
            indexes,
            &inputs.count,
            &scripts.count,
            blocks,
            starting_indexes,
            exit,
        )?;

        let _lock = exit.lock();
        self.db.compact()?;
        Ok(())
    }
}
