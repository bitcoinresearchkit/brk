use bitview_plugin::{ComputePlugin, UpdateContext};
use brk_error::Result;
use rayon::join;

use super::{Vecs, by_type, count, spent, unspent, value};
use crate::Dependencies;

impl ComputePlugin for Vecs {
    type Dependencies<'a> = Dependencies<'a>;
    type Output = ();

    fn compute(
        &mut self,
        dependencies: Self::Dependencies<'_>,
        context: UpdateContext<'_>,
    ) -> Result<Self::Output> {
        let Dependencies {
            indexer,
            inputs,
            blocks,
            price: prices,
        } = dependencies;
        let exit = context.exit();

        self.db.sync_bg_tasks()?;

        let starting_lengths = indexer.safe_lengths();

        count::compute(&mut self.count, indexer, blocks, exit)?;
        let (value_result, by_type_result) = join(
            || value::compute(&mut self.value, indexer, prices, exit),
            || by_type::compute(&mut self.by_type, indexer, exit),
        );
        value_result?;
        by_type_result?;
        unspent::compute(
            &mut self.unspent,
            &self.count,
            &inputs.count,
            &self.by_type,
            &starting_lengths,
            exit,
        )?;
        let lock = spent::compute(&mut self.spent, indexer, exit)?;
        self.db.run_bg(move |db| {
            let _lock = lock;
            db.compact_deferred_default()
        });
        Ok(())
    }
}
