use bitview_plugin_distribution::Vecs as DistributionVecs;
use bitview_plugin_indexer::Indexer;
use brk_error::Result;
use brk_exit::Exit;
use brk_types::{Bitcoin, BoundedRatio, StoredF64};

use super::Vecs;

pub fn compute(
    vecs: &mut Vecs,
    indexer: &Indexer,
    distribution: &DistributionVecs,
    exit: &Exit,
) -> Result<()> {
    let starting_height = indexer.safe_lengths().height;
    let circulating_supply = &distribution
        .cohorts
        .supply
        .total
        .cohorts
        .utxo
        .all
        .sats
        .height;

    vecs.coinblocks_created.compute_cumulative_transformed(
        starting_height,
        circulating_supply,
        |value| StoredF64::from(Bitcoin::from(value)),
        exit,
    )?;

    vecs.coinblocks_stored.cumulative.height.compute_subtract(
        starting_height,
        &vecs.coinblocks_created.cumulative.height,
        &distribution.coinblocks_destroyed.cumulative.height,
        exit,
    )?;

    vecs.derived.liveliness_source.height.compute_transform2(
        starting_height,
        &distribution.coinblocks_destroyed.cumulative.height,
        &vecs.coinblocks_created.cumulative.height,
        |(h, destroyed, created, ..)| {
            (
                h,
                BoundedRatio::from(f64::from(destroyed) / f64::from(created)),
            )
        },
        exit,
    )?;

    Ok(())
}
