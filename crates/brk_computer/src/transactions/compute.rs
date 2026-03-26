use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::Indexes;
use vecdb::Exit;

use crate::{blocks, indexes, inputs, outputs, prices};

use super::Vecs;

impl Vecs {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        blocks: &blocks::Vecs,
        inputs: &inputs::Vecs,
        outputs: &outputs::Vecs,
        prices: &prices::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.db.sync_bg_tasks()?;

        let (r1, (r2, r3)) = rayon::join(
            || {
                self.count
                    .compute(indexer, &blocks.lookback, starting_indexes, exit)
            },
            || {
                rayon::join(
                    || self.versions.compute(indexer, starting_indexes, exit),
                    || self.size.compute(indexer, indexes, starting_indexes, exit),
                )
            },
        );
        r1?;
        r2?;
        r3?;

        self.fees.compute(
            indexer,
            indexes,
            &inputs.spent,
            &self.size,
            starting_indexes,
            exit,
        )?;

        self.volume.compute(
            indexer,
            indexes,
            prices,
            &self.count,
            &self.fees,
            &inputs.count,
            &outputs.count,
            starting_indexes,
            exit,
        )?;

        let exit = exit.clone();
        self.db.run_bg(move |db| {
            let _lock = exit.lock();
            db.compact()
        });
        Ok(())
    }
}
