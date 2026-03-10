use brk_error::Result;
use brk_types::{BasisPoints16, Indexes, Sats, StoredU64, Version};
use vecdb::{AnyStoredVec, AnyVec, Exit, ReadableVec, VecIndex, WritableVec};

use crate::{distribution, internal::PercentPerBlock};

pub(super) fn compute(
    gini: &mut PercentPerBlock<BasisPoints16>,
    distribution: &distribution::Vecs,
    starting_indexes: &Indexes,
    exit: &Exit,
) -> Result<()> {
    let amount_range = &distribution.utxo_cohorts.amount_range;

    let supply_vecs: Vec<&_> = amount_range
        .iter()
        .map(|c| &c.metrics.supply.total.sats.height)
        .collect();
    let count_vecs: Vec<&_> = amount_range
        .iter()
        .map(|c| &c.metrics.outputs.unspent_count.height)
        .collect();

    if supply_vecs.is_empty() || supply_vecs.len() != count_vecs.len() {
        return Ok(());
    }

    let source_version = supply_vecs
        .iter()
        .fold(Version::ZERO, |acc, v| acc + v.version())
        + count_vecs
            .iter()
            .fold(Version::ZERO, |acc, v| acc + v.version());

    gini.bps
        .height
        .validate_computed_version_or_reset(source_version)?;
    gini.bps.height.truncate_if_needed_at(
        gini.bps
            .height
            .len()
            .min(starting_indexes.height.to_usize()),
    )?;

    let total_heights = supply_vecs
        .iter()
        .map(|v| v.len())
        .min()
        .unwrap_or(0)
        .min(count_vecs.iter().map(|v| v.len()).min().unwrap_or(0));

    let start_height = gini.bps.height.len();
    if start_height >= total_heights {
        return Ok(());
    }

    // Batch-collect all cohort data for the range [start_height, total_heights)
    let n_cohorts = supply_vecs.len();
    let supply_data: Vec<Vec<Sats>> = supply_vecs
        .iter()
        .map(|v| v.collect_range_at(start_height, total_heights))
        .collect();
    let count_data: Vec<Vec<StoredU64>> = count_vecs
        .iter()
        .map(|v| v.collect_range_at(start_height, total_heights))
        .collect();

    let mut buckets: Vec<(u64, u64)> = Vec::with_capacity(n_cohorts);
    for offset in 0..total_heights - start_height {
        buckets.clear();
        for c in 0..n_cohorts {
            let supply: u64 = supply_data[c][offset].into();
            let count: u64 = count_data[c][offset].into();
            buckets.push((count, supply));
        }
        gini.bps.height.push(gini_from_lorenz(&buckets));
    }

    {
        let _lock = exit.lock();
        gini.bps.height.write()?;
    }

    Ok(())
}

fn gini_from_lorenz(buckets: &[(u64, u64)]) -> BasisPoints16 {
    let total_count: u64 = buckets.iter().map(|(c, _)| c).sum();
    let total_supply: u64 = buckets.iter().map(|(_, s)| s).sum();

    if total_count == 0 || total_supply == 0 {
        return BasisPoints16::ZERO;
    }

    let (mut cumulative_count, mut cumulative_supply, mut area) = (0u64, 0u64, 0.0f64);
    let (tc, ts) = (total_count as f64, total_supply as f64);

    for &(count, supply) in buckets {
        let (p0, w0) = (cumulative_count as f64 / tc, cumulative_supply as f64 / ts);
        cumulative_count += count;
        cumulative_supply += supply;
        let (p1, w1) = (cumulative_count as f64 / tc, cumulative_supply as f64 / ts);
        area += (p1 - p0) * (w0 + w1) / 2.0;
    }

    BasisPoints16::from(1.0 - 2.0 * area)
}
