use std::{cmp::Reverse, collections::BinaryHeap, fs, path::Path};

use brk_cohort::{Filtered, PROFITABILITY_RANGE_COUNT, PROFIT_COUNT, TERM_NAMES};
use brk_error::Result;
use brk_types::{BasisPoints16, Cents, CentsCompact, CostBasisDistribution, Date, Dollars, Height, Sats};

use crate::distribution::metrics::{CostBasis, ProfitabilityMetrics};

use super::fenwick::{PercentileResult, ProfitabilityRangeResult};
use super::groups::UTXOCohorts;

use super::COST_BASIS_PRICE_DIGITS;

impl UTXOCohorts {
    /// Compute and push percentiles + profitability for aggregate cohorts.
    ///
    /// Percentiles and profitability are computed per-block from the Fenwick tree.
    /// Disk distributions are written only at day boundaries via K-way merge.
    pub(crate) fn truncate_push_aggregate_percentiles(
        &mut self,
        height: Height,
        spot_price: Cents,
        date_opt: Option<Date>,
        states_path: &Path,
    ) -> Result<()> {
        if self.fenwick.is_initialized() {
            self.push_fenwick_results(height, spot_price)?;
        }

        // Disk distributions only at day boundaries
        if let Some(date) = date_opt {
            self.write_disk_distributions(date, states_path)?;
        }

        Ok(())
    }

    /// Push all Fenwick-derived per-block results: percentiles, density, profitability.
    fn push_fenwick_results(&mut self, height: Height, spot_price: Cents) -> Result<()> {
        let (all_d, sth_d, lth_d) = self.fenwick.density(spot_price);

        let all = self.fenwick.percentiles_all();
        push_cost_basis(height, &all, all_d, &mut self.all.metrics.cost_basis)?;

        let sth = self.fenwick.percentiles_sth();
        push_cost_basis(height, &sth, sth_d, &mut self.sth.metrics.cost_basis)?;

        let lth = self.fenwick.percentiles_lth();
        push_cost_basis(height, &lth, lth_d, &mut self.lth.metrics.cost_basis)?;

        let prof = self.fenwick.profitability(spot_price);
        push_profitability(height, &prof, &mut self.profitability)
    }

    /// K-way merge only for writing daily cost basis distributions to disk.
    fn write_disk_distributions(&mut self, date: Date, states_path: &Path) -> Result<()> {
        let sth_filter = self.sth.metrics.filter.clone();

        let maps: Vec<_> = self
            .age_range
            .iter()
            .filter_map(|sub| {
                let state = sub.state.as_ref()?;
                let map = state.cost_basis_map();
                if map.is_empty() {
                    return None;
                }
                let is_sth = sth_filter.includes(sub.filter());
                Some((map, is_sth))
            })
            .collect();

        if maps.is_empty() {
            return Ok(());
        }

        let cap = maps.iter().map(|(m, _)| m.len()).max().unwrap_or(0);
        let mut targets = AllSthLth {
            all: MergeTarget::new(cap),
            sth: MergeTarget::new(cap),
            lth: MergeTarget::new(cap),
        };

        merge_k_way(&maps, &mut targets);

        write_distribution(states_path, "all", date, targets.all.merged)?;
        write_distribution(states_path, TERM_NAMES.short.id, date, targets.sth.merged)?;
        write_distribution(states_path, TERM_NAMES.long.id, date, targets.lth.merged)?;

        Ok(())
    }
}

/// Push percentiles + density to cost basis vecs.
fn push_cost_basis(
    height: Height,
    percentiles: &PercentileResult,
    density_bps: u16,
    cost_basis: &mut CostBasis,
) -> Result<()> {
    cost_basis.truncate_push_minmax(height, percentiles.min_price, percentiles.max_price)?;
    cost_basis.truncate_push_percentiles(height, &percentiles.sat_prices, &percentiles.usd_prices)?;
    cost_basis.truncate_push_density(height, BasisPoints16::from(density_bps))
}

/// Convert raw (cents × sats) accumulator to Dollars (÷ 100 for cents→dollars, ÷ 1e8 for sats).
#[inline(always)]
fn raw_usd_to_dollars(raw: u128) -> Dollars {
    Dollars::from(raw as f64 / 1e10)
}

/// Push profitability range + profit/loss aggregate values to vecs.
fn push_profitability(
    height: Height,
    buckets: &[ProfitabilityRangeResult; PROFITABILITY_RANGE_COUNT],
    metrics: &mut ProfitabilityMetrics,
) -> Result<()> {
    // Truncate all buckets once upfront to avoid per-push checks
    metrics.truncate(height)?;

    // Push 25 range buckets
    for (i, bucket) in metrics.range.as_array_mut().into_iter().enumerate() {
        let r = &buckets[i];
        bucket.push(
            Sats::from(r.all_sats),
            Sats::from(r.sth_sats),
            raw_usd_to_dollars(r.all_usd),
            raw_usd_to_dollars(r.sth_usd),
        );
    }

    // Profit: forward cumulative sum over ranges[0..15], pushed in reverse.
    // profit[0] (breakeven) = sum(0..=13), ..., profit[13] (_500pct) = ranges[0]
    let profit_arr = metrics.profit.as_array_mut();
    let mut cum_sats = 0u64;
    let mut cum_sth_sats = 0u64;
    let mut cum_usd = 0u128;
    let mut cum_sth_usd = 0u128;
    for i in 0..PROFIT_COUNT {
        cum_sats += buckets[i].all_sats;
        cum_sth_sats += buckets[i].sth_sats;
        cum_usd += buckets[i].all_usd;
        cum_sth_usd += buckets[i].sth_usd;
        profit_arr[PROFIT_COUNT - 1 - i].push(
            Sats::from(cum_sats),
            Sats::from(cum_sth_sats),
            raw_usd_to_dollars(cum_usd),
            raw_usd_to_dollars(cum_sth_usd),
        );
    }

    // Loss: backward cumulative sum over ranges[15..25], pushed in reverse.
    // loss[0] (breakeven) = sum(15..=24), ..., loss[8] (_80pct) = ranges[24]
    let loss_arr = metrics.loss.as_array_mut();
    let loss_count = loss_arr.len();
    cum_sats = 0;
    cum_sth_sats = 0;
    cum_usd = 0;
    cum_sth_usd = 0;
    for i in 0..loss_count {
        let r = &buckets[PROFITABILITY_RANGE_COUNT - 1 - i];
        cum_sats += r.all_sats;
        cum_sth_sats += r.sth_sats;
        cum_usd += r.all_usd;
        cum_sth_usd += r.sth_usd;
        loss_arr[loss_count - 1 - i].push(
            Sats::from(cum_sats),
            Sats::from(cum_sth_sats),
            raw_usd_to_dollars(cum_usd),
            raw_usd_to_dollars(cum_sth_usd),
        );
    }

    Ok(())
}

fn write_distribution(
    states_path: &Path,
    name: &str,
    date: Date,
    merged: Vec<(CentsCompact, Sats)>,
) -> Result<()> {
    let dir = states_path.join(format!("utxo_{name}_cost_basis/by_date"));
    fs::create_dir_all(&dir)?;
    fs::write(
        dir.join(date.to_string()),
        CostBasisDistribution::serialize_iter(merged.into_iter())?,
    )?;
    Ok(())
}

// ---------------------------------------------------------------------------
// K-way merge (retained only for disk distribution writes)
// ---------------------------------------------------------------------------

struct AllSthLth<T> {
    all: T,
    sth: T,
    lth: T,
}

impl<T> AllSthLth<T> {
    fn term_mut(&mut self, is_sth: bool) -> &mut T {
        if is_sth { &mut self.sth } else { &mut self.lth }
    }

    fn for_each_mut(&mut self, mut f: impl FnMut(&mut T)) {
        f(&mut self.all);
        f(&mut self.sth);
        f(&mut self.lth);
    }
}

/// Merge target that only collects rounded (price, sats) pairs for disk distribution.
struct MergeTarget {
    price_sats: u64,
    merged: Vec<(CentsCompact, Sats)>,
}

impl MergeTarget {
    fn new(cap: usize) -> Self {
        Self {
            price_sats: 0,
            merged: Vec::with_capacity(cap),
        }
    }

    #[inline]
    fn accumulate(&mut self, amount: u64) {
        self.price_sats += amount;
    }

    fn finalize_price(&mut self, price: Cents) {
        if self.price_sats > 0 {
            let rounded: CentsCompact = price.round_to_dollar(COST_BASIS_PRICE_DIGITS).into();
            if let Some((lp, ls)) = self.merged.last_mut()
                && *lp == rounded
            {
                *ls += Sats::from(self.price_sats);
            } else {
                self.merged.push((rounded, Sats::from(self.price_sats)));
            }
        }
        self.price_sats = 0;
    }
}

/// K-way merge via BinaryHeap over BTreeMap iterators.
/// Only builds merged distribution for disk writes.
fn merge_k_way(
    maps: &[(&std::collections::BTreeMap<CentsCompact, Sats>, bool)],
    targets: &mut AllSthLth<MergeTarget>,
) {
    let mut iters: Vec<_> = maps
        .iter()
        .map(|(map, is_sth)| (map.iter().peekable(), *is_sth))
        .collect();

    let mut heap: BinaryHeap<Reverse<(CentsCompact, usize)>> =
        BinaryHeap::with_capacity(iters.len());
    for (i, (iter, _)) in iters.iter_mut().enumerate() {
        if let Some(&(&price, _)) = iter.peek() {
            heap.push(Reverse((price, i)));
        }
    }

    let mut current_price: Option<CentsCompact> = None;

    while let Some(Reverse((price, ci))) = heap.pop() {
        let (ref mut iter, is_sth) = iters[ci];
        let (_, &sats) = iter.next().unwrap();
        let amount = u64::from(sats);

        if let Some(prev) = current_price
            && prev != price
        {
            targets.for_each_mut(|t| t.finalize_price(prev.into()));
        }

        current_price = Some(price);
        targets.all.accumulate(amount);
        targets.term_mut(is_sth).accumulate(amount);

        if let Some(&(&next_price, _)) = iter.peek() {
            heap.push(Reverse((next_price, ci)));
        }
    }

    if let Some(price) = current_price {
        targets.for_each_mut(|t| t.finalize_price(price.into()));
    }
}
