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
        let Dependencies {
            indexer,
            price: prices,
            mappings,
            blocks,
        } = dependencies;
        let exit = context.exit();

        self.db.sync_bg_tasks()?;

        let (ath, (range, moving_average)) = join(
            || super::ath::compute(&mut self.ath, indexer, prices, mappings, exit),
            || {
                join(
                    || super::range::compute(&mut self.range, indexer, prices, blocks, exit),
                    || {
                        super::moving_average::compute(
                            &mut self.moving_average,
                            indexer,
                            blocks,
                            prices,
                            exit,
                        )
                    },
                )
            },
        );
        ath?;
        range?;
        moving_average?;

        let (returns, technical) = join(
            || super::returns::compute(&mut self.returns, indexer, blocks, exit),
            || {
                super::technical::compute(
                    &mut self.technical,
                    indexer,
                    prices,
                    blocks,
                    &self.moving_average,
                    exit,
                )
            },
        );
        returns?;
        technical?;

        context.compact_database(&self.db);
        Ok(())
    }
}
