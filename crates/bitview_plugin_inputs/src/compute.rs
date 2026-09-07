use brk_error::Result;

use bitview_plugin::{ComputePlugin, UpdateContext};
use rayon::join;

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
        let Dependencies { indexer, blocks } = dependencies;
        let exit = context.exit();

        self.db.sync_bg_tasks()?;

        let Vecs {
            value,
            count,
            by_type,
            ..
        } = self;
        let (value_result, rest_result) = join(
            || super::value::compute(value, indexer, exit),
            || {
                count.compute(indexer, blocks, exit)?;
                by_type.compute(indexer, exit)
            },
        );
        value_result?;
        rest_result?;

        context.compact_database(&self.db);
        Ok(())
    }
}
