use brk_error::Result;
use brk_indexer::Indexer;
use vecdb::Exit;

use super::Vecs;
use crate::{indexes, ComputeIndexes};

impl Vecs {
    #[allow(unused_variables)]
    pub fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.usd.compute(starting_indexes, &self.cents, exit)?;

        self.sats.compute(starting_indexes, &self.usd, exit)?;

        // Oracle price computation is slow and still WIP, only run in dev builds
        // #[cfg(debug_assertions)]
        // {
        //     use std::time::Instant;
        //     use tracing::info;
        //
        //     info!("Computing oracle prices...");
        //     let i = Instant::now();
        //     self.oracle
        //         .compute(indexer, indexes, &self.cents, starting_indexes, exit)?;
        //     info!("Computed oracle prices in {:?}", i.elapsed());
        // }

        let _lock = exit.lock();
        self.db().compact()?;
        Ok(())
    }
}
