use brk_error::Result;
use brk_types::{Day1, Sats, StoredF32, StoredU64, Version};
use vecdb::{AnyStoredVec, AnyVec, Exit, ReadableVec, VecIndex, WritableVec};

use crate::{ComputeIndexes, distribution, internal::ComputedFromHeightLast};

pub(super) fn compute(
    gini: &mut ComputedFromHeightLast<StoredF32>,
    distribution: &distribution::Vecs,
    h2d: &[Day1],
    total_heights: usize,
    starting_indexes: &ComputeIndexes,
    exit: &Exit,
) -> Result<()> {
    let amount_range = &distribution.utxo_cohorts.amount_range;

    let supply_vecs: Vec<&_> = amount_range
        .iter()
        .map(|c| &c.metrics.supply.total.sats.day1)
        .collect();
    let count_vecs: Vec<&_> = amount_range
        .iter()
        .map(|c| &c.metrics.outputs.utxo_count.day1)
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

    gini.height
        .validate_computed_version_or_reset(source_version)?;
    gini.height
        .truncate_if_needed_at(gini.height.len().min(starting_indexes.height.to_usize()))?;

    let start_height = gini.height.len();
    if start_height >= total_heights {
        return Ok(());
    }

    // Pre-collect all daily data
    let supply_data: Vec<Vec<Sats>> = supply_vecs
        .iter()
        .map(|v| ReadableVec::collect(*v))
        .collect();
    let count_data: Vec<Vec<StoredU64>> = count_vecs
        .iter()
        .map(|v| ReadableVec::collect(*v))
        .collect();
    let num_days = supply_data.first().map_or(0, |v| v.len());

    // Compute gini per day in-memory
    let mut gini_daily = Vec::with_capacity(num_days);
    let mut buckets: Vec<(u64, u64)> = Vec::with_capacity(supply_data.len());
    for di in 0..num_days {
        buckets.clear();
        buckets.extend(supply_data.iter().zip(count_data.iter()).map(|(s, c)| {
            let count: u64 = c[di].into();
            let supply: u64 = s[di].into();
            (count, supply)
        }));
        gini_daily.push(gini_from_lorenz(&buckets));
    }

    // Expand to Height
    (start_height..total_heights).for_each(|h| {
        let di = h2d[h].to_usize();
        let val = if di < gini_daily.len() {
            StoredF32::from(gini_daily[di])
        } else {
            StoredF32::NAN
        };
        gini.height.push(val);
    });

    {
        let _lock = exit.lock();
        gini.height.write()?;
    }

    Ok(())
}

fn gini_from_lorenz(buckets: &[(u64, u64)]) -> f32 {
    let total_count: u64 = buckets.iter().map(|(c, _)| c).sum();
    let total_supply: u64 = buckets.iter().map(|(_, s)| s).sum();

    if total_count == 0 || total_supply == 0 {
        return 0.0;
    }

    let (mut cum_count, mut cum_supply, mut area) = (0u64, 0u64, 0.0f64);
    let (tc, ts) = (total_count as f64, total_supply as f64);

    for &(count, supply) in buckets {
        let (p0, w0) = (cum_count as f64 / tc, cum_supply as f64 / ts);
        cum_count += count;
        cum_supply += supply;
        let (p1, w1) = (cum_count as f64 / tc, cum_supply as f64 / ts);
        area += (p1 - p0) * (w0 + w1) / 2.0;
    }

    (1.0 - 2.0 * area) as f32
}
