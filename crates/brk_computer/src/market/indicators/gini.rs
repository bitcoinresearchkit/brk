use brk_error::Result;
use brk_types::{Day1, StoredF32, Version};
use vecdb::{AnyStoredVec, AnyVec, Exit, ReadableOptionVec, VecIndex, WritableVec};

use crate::{ComputeIndexes, distribution, internal::ComputedFromHeight};

pub(super) fn compute(
    gini: &mut ComputedFromHeight<StoredF32>,
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

    let num_days = supply_vecs
        .iter()
        .map(|v| v.len())
        .min()
        .unwrap_or(0)
        .min(count_vecs.iter().map(|v| v.len()).min().unwrap_or(0));

    // Only compute gini for new days (each day is independent)
    let start_day = if start_height > 0 {
        h2d[start_height].to_usize()
    } else {
        0
    };

    let mut gini_new: Vec<f32> = Vec::with_capacity(num_days.saturating_sub(start_day));
    let mut buckets: Vec<(u64, u64)> = Vec::with_capacity(supply_vecs.len());
    for di in start_day..num_days {
        buckets.clear();
        let day = Day1::from(di);
        for (sv, cv) in supply_vecs.iter().zip(count_vecs.iter()) {
            let supply: u64 = sv.collect_one_flat(day).unwrap_or_default().into();
            let count: u64 = cv.collect_one_flat(day).unwrap_or_default().into();
            buckets.push((count, supply));
        }
        gini_new.push(gini_from_lorenz(&buckets));
    }

    // Expand to Height
    (start_height..total_heights).for_each(|h| {
        let di = h2d[h].to_usize();
        let offset = di.saturating_sub(start_day);
        let val = if offset < gini_new.len() {
            StoredF32::from(gini_new[offset])
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
        return f32::NAN;
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

    (1.0 - 2.0 * area) as f32
}
