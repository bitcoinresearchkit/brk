use bitview_plugin::{ComputePlugin, UpdateContext};
use bitview_vecs::CachedSeries;
use brk_error::Result;
use brk_exit::Exit;
use brk_types::{Day1, Dollars, Sats};
use vecdb::{ReadableVec, VecIndex};

use super::Vecs;
use crate::{DCA_AMOUNT, Dependencies};

impl ComputePlugin for Vecs {
    type Dependencies<'a> = Dependencies<'a>;
    type Output = ();

    fn compute(
        &mut self,
        dependencies: Self::Dependencies<'_>,
        context: UpdateContext<'_>,
    ) -> Result<()> {
        let Dependencies {
            indexer,
            mappings,
            price,
        } = dependencies;
        self.db.sync_bg_tasks()?;

        // Rebuild from the last retained block's day: a replacement can move
        // the next block across days, changing intervening daily closes too.
        let from = mappings
            .height
            .recompute_day(
                indexer
                    .safe_lengths()
                    .height
                    .decremented()
                    .unwrap_or_default(),
            )
            .unwrap_or_default();
        compute_daily_sats(
            &mut self.sats_cumulative,
            from,
            &price.split.close.usd.day1,
            context.exit(),
        )?;
        context.compact_database(&self.db);
        Ok(())
    }
}

fn compute_daily_sats(
    target: &mut CachedSeries<Day1, Sats>,
    from: Day1,
    prices: &impl ReadableVec<Day1, Option<Dollars>>,
    exit: &Exit,
) -> Result<()> {
    let mut sum = None;
    target.compute_transform(
        from,
        prices,
        |(day, price, target)| {
            let sum = sum.get_or_insert_with(|| {
                day.decremented()
                    .and_then(|day| target.collect_one(day))
                    .unwrap_or_default()
            });
            *sum += Sats::from_dollars_at_price(DCA_AMOUNT, price.unwrap_or_default());
            (day, *sum)
        },
        exit,
    )?;
    Ok(())
}

#[cfg(test)]
#[path = "compute_tests.rs"]
mod tests;
