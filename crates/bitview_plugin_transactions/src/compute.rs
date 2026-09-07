use brk_error::Result;

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
        let Dependencies {
            indexer,
            inputs,
            mappings,
            blocks,
            price: prices,
        } = dependencies;
        let exit = context.exit();

        self.db.sync_bg_tasks()?;

        let ((r1, r2), (r3, r4)) = rayon::join(
            || {
                rayon::join(
                    || super::count::compute(&mut self.count, indexer, &blocks.lookback, exit),
                    || super::features::compute(&mut self.features, indexer, exit),
                )
            },
            || {
                rayon::join(
                    || super::versions::compute(&mut self.versions, indexer, exit),
                    || super::size::compute(&mut self.size, indexer, mappings, exit),
                )
            },
        );
        r1?;
        r2?;
        r3?;
        r4?;

        super::sigops::compute(&mut self.sigops, indexer, mappings, exit)?;

        super::fees::compute(
            &mut self.fees,
            indexer,
            &inputs.value,
            mappings,
            &self.size,
            exit,
        )?;

        super::patterns::compute(&mut self.patterns, indexer, &inputs.value, mappings, exit)?;

        super::policy::compute(&mut self.policy, indexer, mappings, &self.fees, exit)?;

        super::volume::compute(
            &mut self.volume,
            indexer,
            mappings,
            prices,
            &self.fees,
            exit,
        )?;

        context.compact_database(&self.db);
        Ok(())
    }
}
