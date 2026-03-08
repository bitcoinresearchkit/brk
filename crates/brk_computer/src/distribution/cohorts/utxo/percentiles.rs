use std::{cmp::Reverse, collections::BinaryHeap, fs, path::Path};

use brk_cohort::{
    compute_profitability_boundaries, Filtered, PROFITABILITY_BOUNDARY_COUNT,
    PROFITABILITY_RANGE_COUNT, PROFIT_COUNT, TERM_NAMES,
};
use brk_error::Result;
use brk_types::{Cents, CentsCompact, CostBasisDistribution, Date, Dollars, Height, Sats};

use crate::internal::{PERCENTILES, PERCENTILES_LEN};

use crate::distribution::metrics::{CostBasis, ProfitabilityMetrics};

use super::groups::UTXOCohorts;

const COST_BASIS_PRICE_DIGITS: i32 = 5;

#[derive(Clone, Default)]
pub(super) struct CachedPercentiles {
    sat_result: [Cents; PERCENTILES_LEN],
    usd_result: [Cents; PERCENTILES_LEN],
    min_price: Cents,
    max_price: Cents,
}

impl CachedPercentiles {
    fn push(&self, height: Height, cost_basis: &mut CostBasis) -> Result<()> {
        cost_basis.truncate_push_minmax(height, self.min_price, self.max_price)?;
        cost_basis.truncate_push_percentiles(height, &self.sat_result, &self.usd_result)
    }
}

/// Cached percentile + profitability results for all/sth/lth.
/// Avoids re-merging 21 BTreeMaps on every block.
#[derive(Clone, Default)]
pub(super) struct PercentileCache {
    all: CachedPercentiles,
    sth: CachedPercentiles,
    lth: CachedPercentiles,
    profitability: [(u64, u128); PROFITABILITY_RANGE_COUNT],
    initialized: bool,
}

impl UTXOCohorts {
    /// Compute and push percentiles + profitability for aggregate cohorts.
    ///
    /// Full K-way merge only runs at day boundaries or when the cache is empty.
    /// For intermediate blocks, pushes cached values.
    pub(crate) fn truncate_push_aggregate_percentiles(
        &mut self,
        height: Height,
        spot_price: Cents,
        date_opt: Option<Date>,
        states_path: &Path,
    ) -> Result<()> {
        if date_opt.is_some() || !self.percentile_cache.initialized {
            self.recompute_cache(spot_price, date_opt, states_path)?;
        }
        self.push_cached(height)
    }

    /// Full K-way merge: recompute percentiles + profitability from scratch, update cache.
    fn recompute_cache(
        &mut self,
        spot_price: Cents,
        date_opt: Option<Date>,
        states_path: &Path,
    ) -> Result<()> {
        let collect_merged = date_opt.is_some();
        let boundaries = compute_profitability_boundaries(spot_price);

        let targets = {
            let sth_filter = self.sth.metrics.filter.clone();
            let mut totals = AllSthLth::<(u64, u128)>::default();

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
                    let mut cs = 0u64;
                    let mut cu = 0u128;
                    for (&price, &sats) in map.iter() {
                        let s = u64::from(sats);
                        cs += s;
                        cu += price.as_u128() * s as u128;
                    }
                    totals.all.0 += cs;
                    totals.all.1 += cu;
                    let term = totals.term_mut(is_sth);
                    term.0 += cs;
                    term.1 += cu;
                    Some((map, is_sth))
                })
                .collect();

            let cap = if collect_merged {
                maps.iter().map(|(m, _)| m.len()).max().unwrap_or(0)
            } else {
                0
            };
            let all_has_data = totals.all.0 > 0;
            let mut targets = totals.map(|(sats, usd)| PercTarget::new(sats, usd, cap));
            self.percentile_cache.profitability = Default::default();

            if all_has_data {
                merge_k_way(
                    &maps,
                    &mut targets,
                    &boundaries,
                    &mut self.percentile_cache.profitability,
                    collect_merged,
                );
            }

            targets
        };

        self.percentile_cache.all = targets.all.to_cached();
        self.percentile_cache.sth = targets.sth.to_cached();
        self.percentile_cache.lth = targets.lth.to_cached();
        self.percentile_cache.initialized = true;

        if let Some(date) = date_opt {
            write_distribution(states_path, "all", date, targets.all.merged)?;
            write_distribution(states_path, TERM_NAMES.short.id, date, targets.sth.merged)?;
            write_distribution(states_path, TERM_NAMES.long.id, date, targets.lth.merged)?;
        }

        Ok(())
    }

    /// Push cached percentile + profitability values.
    fn push_cached(&mut self, height: Height) -> Result<()> {
        self.percentile_cache
            .all
            .push(height, &mut self.all.metrics.cost_basis)?;
        self.percentile_cache
            .sth
            .push(height, &mut self.sth.metrics.cost_basis)?;
        self.percentile_cache
            .lth
            .push(height, &mut self.lth.metrics.cost_basis)?;
        push_profitability(
            height,
            &self.percentile_cache.profitability,
            &mut self.profitability,
        )
    }
}

/// Convert raw (cents × sats) accumulator to Dollars (÷ 100 for cents→dollars, ÷ 1e8 for sats).
#[inline]
fn raw_usd_to_dollars(raw: u128) -> Dollars {
    Dollars::from(raw as f64 / 1e10)
}

/// Push profitability range + profit/loss aggregate values to vecs.
fn push_profitability(
    height: Height,
    buckets: &[(u64, u128); PROFITABILITY_RANGE_COUNT],
    metrics: &mut ProfitabilityMetrics,
) -> Result<()> {
    // Push 25 range buckets
    for (i, bucket) in metrics.range.as_array_mut().into_iter().enumerate() {
        let (sats, usd_raw) = buckets[i];
        bucket.truncate_push(height, Sats::from(sats), raw_usd_to_dollars(usd_raw))?;
    }

    // ByProfit: forward cumulative sum over ranges[0..15], pushed in reverse.
    // profit[0] (breakeven) = sum(0..=14), ..., profit[14] (_1000pct) = ranges[0]
    let profit_arr = metrics.profit.as_array_mut();
    let mut cum_sats = 0u64;
    let mut cum_usd = 0u128;
    for i in 0..PROFIT_COUNT {
        cum_sats += buckets[i].0;
        cum_usd += buckets[i].1;
        profit_arr[PROFIT_COUNT - 1 - i]
            .truncate_push(height, Sats::from(cum_sats), raw_usd_to_dollars(cum_usd))?;
    }

    // ByLoss: backward cumulative sum over ranges[15..25], pushed in reverse.
    // loss[0] (breakeven) = sum(15..=24), ..., loss[9] (_90pct) = ranges[24]
    let loss_arr = metrics.loss.as_array_mut();
    let loss_count = loss_arr.len();
    cum_sats = 0;
    cum_usd = 0;
    for i in 0..loss_count {
        cum_sats += buckets[PROFITABILITY_RANGE_COUNT - 1 - i].0;
        cum_usd += buckets[PROFITABILITY_RANGE_COUNT - 1 - i].1;
        loss_arr[loss_count - 1 - i]
            .truncate_push(height, Sats::from(cum_sats), raw_usd_to_dollars(cum_usd))?;
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

/// K-way merge via BinaryHeap over BTreeMap iterators.
/// Also accumulates profitability buckets for the "all" target using cursor approach.
fn merge_k_way(
    maps: &[(&std::collections::BTreeMap<CentsCompact, Sats>, bool)],
    targets: &mut AllSthLth<PercTarget>,
    boundaries: &[Cents; PROFITABILITY_BOUNDARY_COUNT],
    prof: &mut [(u64, u128); PROFITABILITY_RANGE_COUNT],
    collect_merged: bool,
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
    let mut boundary_idx = 0usize;

    while let Some(Reverse((price, ci))) = heap.pop() {
        let (ref mut iter, is_sth) = iters[ci];
        let (_, &sats) = iter.next().unwrap();
        let amount = u64::from(sats);
        let price_cents = Cents::from(price);
        let usd = price_cents.as_u128() * amount as u128;

        if let Some(prev) = current_price
            && prev != price
        {
            targets.for_each_mut(|t| t.finalize_price(prev.into(), collect_merged));
        }

        current_price = Some(price);
        targets.all.accumulate(amount, usd);
        targets.term_mut(is_sth).accumulate(amount, usd);

        // Profitability: advance cursor past boundaries (prices are ascending)
        while boundary_idx < PROFITABILITY_BOUNDARY_COUNT
            && price_cents >= boundaries[boundary_idx]
        {
            boundary_idx += 1;
        }
        prof[boundary_idx].0 += amount;
        prof[boundary_idx].1 += usd;

        if let Some(&(&next_price, _)) = iter.peek() {
            heap.push(Reverse((next_price, ci)));
        }
    }

    if let Some(price) = current_price {
        targets.for_each_mut(|t| t.finalize_price(price.into(), collect_merged));
    }
}

struct AllSthLth<T> {
    all: T,
    sth: T,
    lth: T,
}

impl<T: Default> Default for AllSthLth<T> {
    fn default() -> Self {
        Self {
            all: T::default(),
            sth: T::default(),
            lth: T::default(),
        }
    }
}

impl<T> AllSthLth<T> {
    fn term_mut(&mut self, is_sth: bool) -> &mut T {
        if is_sth { &mut self.sth } else { &mut self.lth }
    }

    fn map<U>(self, mut f: impl FnMut(T) -> U) -> AllSthLth<U> {
        AllSthLth {
            all: f(self.all),
            sth: f(self.sth),
            lth: f(self.lth),
        }
    }

    fn for_each_mut(&mut self, mut f: impl FnMut(&mut T)) {
        f(&mut self.all);
        f(&mut self.sth);
        f(&mut self.lth);
    }

}

struct PercTarget {
    total_sats: u64,
    total_usd: u128,
    cum_sats: u64,
    cum_usd: u128,
    sat_idx: usize,
    usd_idx: usize,
    sat_targets: [u64; PERCENTILES_LEN],
    usd_targets: [u128; PERCENTILES_LEN],
    sat_result: [Cents; PERCENTILES_LEN],
    usd_result: [Cents; PERCENTILES_LEN],
    price_sats: u64,
    price_usd: u128,
    min_price: Cents,
    max_price: Cents,
    merged: Vec<(CentsCompact, Sats)>,
}

impl PercTarget {
    fn new(total_sats: u64, total_usd: u128, merged_cap: usize) -> Self {
        Self {
            sat_targets: if total_sats > 0 {
                PERCENTILES.map(|p| total_sats * u64::from(p) / 100)
            } else {
                [0; PERCENTILES_LEN]
            },
            usd_targets: if total_usd > 0 {
                PERCENTILES.map(|p| total_usd * u128::from(p) / 100)
            } else {
                [0; PERCENTILES_LEN]
            },
            total_sats,
            total_usd,
            cum_sats: 0,
            cum_usd: 0,
            sat_idx: 0,
            usd_idx: 0,
            sat_result: [Cents::ZERO; PERCENTILES_LEN],
            usd_result: [Cents::ZERO; PERCENTILES_LEN],
            price_sats: 0,
            price_usd: 0,
            min_price: Cents::ZERO,
            max_price: Cents::ZERO,
            merged: Vec::with_capacity(merged_cap),
        }
    }

    fn to_cached(&self) -> CachedPercentiles {
        CachedPercentiles {
            sat_result: self.sat_result,
            usd_result: self.usd_result,
            min_price: self.min_price,
            max_price: self.max_price,
        }
    }

    #[inline]
    fn accumulate(&mut self, amount: u64, usd: u128) {
        self.price_sats += amount;
        self.price_usd += usd;
    }

    fn finalize_price(&mut self, price: Cents, collect_merged: bool) {
        if self.price_sats > 0 {
            if self.min_price == Cents::ZERO {
                self.min_price = price;
            }
            self.max_price = price;

            if collect_merged {
                let rounded: CentsCompact = price.round_to_dollar(COST_BASIS_PRICE_DIGITS).into();
                if let Some((lp, ls)) = self.merged.last_mut()
                    && *lp == rounded
                {
                    *ls += Sats::from(self.price_sats);
                } else {
                    self.merged.push((rounded, Sats::from(self.price_sats)));
                }
            }
        }

        self.cum_sats += self.price_sats;
        self.cum_usd += self.price_usd;
        if self.total_sats > 0 {
            while self.sat_idx < PERCENTILES_LEN
                && self.cum_sats >= self.sat_targets[self.sat_idx]
            {
                self.sat_result[self.sat_idx] = price;
                self.sat_idx += 1;
            }
        }
        if self.total_usd > 0 {
            while self.usd_idx < PERCENTILES_LEN
                && self.cum_usd >= self.usd_targets[self.usd_idx]
            {
                self.usd_result[self.usd_idx] = price;
                self.usd_idx += 1;
            }
        }
        self.price_sats = 0;
        self.price_usd = 0;
    }

}
