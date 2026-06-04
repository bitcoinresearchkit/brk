use brk_error::Result;
use brk_indexer::Indexer;
use vecdb::Exit;

use super::Vecs;
use crate::{blocks, indexes, inputs, price};

impl Vecs {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        blocks: &blocks::Vecs,
        inputs: &inputs::Vecs,
        prices: &price::Vecs,
        exit: &Exit,
    ) -> Result<()> {
        self.db.sync_bg_tasks()?;

        let (r1, (r2, r3)) = rayon::join(
            || self.count.compute(indexer, &blocks.lookback, exit),
            || {
                rayon::join(
                    || self.versions.compute(indexer, exit),
                    || self.size.compute(indexer, indexes, exit),
                )
            },
        );
        r1?;
        r2?;
        r3?;

        self.fees
            .compute(indexer, indexes, &inputs.spent, &self.size, exit)?;

        self.volume
            .compute(indexer, indexes, prices, &self.count, &self.fees, exit)?;

        let exit = exit.clone();
        self.db.run_bg(move |db| {
            let _lock = exit.lock();
            db.compact_deferred_default()
        });
        Ok(())
    }
}
