use brk_error::Result;

use std::thread;

use bitview_plugin::{ComputePlugin, UpdateContext};

use super::Vecs;
use crate::Dependencies;

impl ComputePlugin for Vecs {
    type Dependencies<'a> = Dependencies<'a>;
    type Output = ();

    fn compute(
        &mut self,
        dependencies: Self::Dependencies<'_>,
        context: UpdateContext<'_>,
    ) -> Result<Self::Output> {
        let Dependencies { indexer } = dependencies;
        let exit = context.exit();

        self.db.sync_bg_tasks()?;

        // Cached lookbacks depend on the monotonic timestamp vec, which may
        // have changed without changing its final length after a reorg.
        self.lookback.invalidate_caches();

        // Interval and size are independent.
        let Vecs {
            lookback,
            interval,
            size,
            ..
        } = self;
        thread::scope(|s| -> Result<()> {
            let r1 = s.spawn(|| interval.compute(indexer, exit));
            size.compute(indexer, &*lookback, exit)?;
            r1.join().unwrap()?;
            Ok(())
        })?;

        context.compact_database(&self.db);
        Ok(())
    }
}
