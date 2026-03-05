use std::{cmp::Reverse, collections::BinaryHeap, fs, path::Path};

use brk_cohort::{Filtered, TERM_NAMES};
use brk_error::Result;
use brk_types::{Cents, CentsCompact, CostBasisDistribution, Date, Height, Sats};

use crate::internal::{PERCENTILES, PERCENTILES_LEN};

use crate::distribution::metrics::{CohortMetricsBase, CostBasisExtended};

use super::groups::UTXOCohorts;

const COST_BASIS_PRICE_DIGITS: i32 = 5;

#[derive(Clone, Default)]
pub(super) struct CachedPercentiles {
    sat_result: [Cents; PERCENTILES_LEN],
    usd_result: [Cents; PERCENTILES_LEN],
}

impl CachedPercentiles {
    fn push(&self, height: Height, ext: &mut CostBasisExtended) -> Result<()> {
        ext.push_arrays(height, &self.sat_result, &self.usd_result)
    }
}

/// Cached percentile results for all/sth/lth.
/// Avoids re-merging 21 BTreeMaps on every block.
#[derive(Clone, Default)]
pub(super) struct PercentileCache {
    all: CachedPercentiles,
    sth: CachedPercentiles,
    lth: CachedPercentiles,
    initialized: bool,
}

impl UTXOCohorts {
    /// Compute and push percentiles for aggregate cohorts (all, sth, lth).
    ///
    /// Full K-way merge only runs at day boundaries or when the cache is empty.
    /// For intermediate blocks, pushes cached percentile arrays.
    pub(crate) fn truncate_push_aggregate_percentiles(
        &mut self,
        height: Height,
        date_opt: Option<Date>,
        states_path: &Path,
    ) -> Result<()> {
        if date_opt.is_some() || !self.percentile_cache.initialized {
            self.merge_and_push_percentiles(height, date_opt, states_path)
        } else {
            self.push_cached_percentiles(height)
        }
    }

    /// Full K-way merge: compute percentiles from scratch, update cache, push.
    fn merge_and_push_percentiles(
        &mut self,
        height: Height,
        date_opt: Option<Date>,
        states_path: &Path,
    ) -> Result<()> {
        let collect_merged = date_opt.is_some();

        let targets = {
            let sth_filter = self.sth.metrics.filter().clone();
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

            if all_has_data {
                merge_k_way(&maps, &mut targets, collect_merged);
            }

            targets
        };

        // Update cache + push
        self.percentile_cache.all = targets.all.to_cached();
        self.percentile_cache.sth = targets.sth.to_cached();
        self.percentile_cache.lth = targets.lth.to_cached();
        self.percentile_cache.initialized = true;

        self.percentile_cache
            .all
            .push(height, &mut self.all.metrics.cost_basis.extended)?;
        self.percentile_cache
            .sth
            .push(height, &mut self.sth.metrics.cost_basis.extended)?;
        self.percentile_cache
            .lth
            .push(height, &mut self.lth.metrics.cost_basis.extended)?;

        // Serialize full distribution at day boundaries
        if let Some(date) = date_opt {
            write_distribution(states_path, "all", date, targets.all.merged)?;
            write_distribution(states_path, TERM_NAMES.short.id, date, targets.sth.merged)?;
            write_distribution(states_path, TERM_NAMES.long.id, date, targets.lth.merged)?;
        }

        Ok(())
    }

    /// Fast path: push cached percentile arrays.
    fn push_cached_percentiles(&mut self, height: Height) -> Result<()> {
        self.percentile_cache
            .all
            .push(height, &mut self.all.metrics.cost_basis.extended)?;
        self.percentile_cache
            .sth
            .push(height, &mut self.sth.metrics.cost_basis.extended)?;
        self.percentile_cache
            .lth
            .push(height, &mut self.lth.metrics.cost_basis.extended)?;
        Ok(())
    }
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
fn merge_k_way(
    maps: &[(&std::collections::BTreeMap<CentsCompact, Sats>, bool)],
    targets: &mut AllSthLth<PercTarget>,
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
    let mut early_exit = false;

    while let Some(Reverse((price, ci))) = heap.pop() {
        let (ref mut iter, is_sth) = iters[ci];
        let (_, &sats) = iter.next().unwrap();
        let amount = u64::from(sats);
        let usd = Cents::from(price).as_u128() * amount as u128;

        if let Some(prev) = current_price
            && prev != price
        {
            targets.for_each_mut(|t| t.finalize_price(prev.into(), collect_merged));
            if !collect_merged && targets.all_match(|t| t.done()) {
                early_exit = true;
                break;
            }
        }

        current_price = Some(price);
        targets.all.accumulate(amount, usd);
        targets.term_mut(is_sth).accumulate(amount, usd);

        if let Some(&(&next_price, _)) = iter.peek() {
            heap.push(Reverse((next_price, ci)));
        }
    }

    if !early_exit
        && let Some(price) = current_price
    {
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

    fn all_match(&self, mut f: impl FnMut(&T) -> bool) -> bool {
        f(&self.all) && f(&self.sth) && f(&self.lth)
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
            merged: Vec::with_capacity(merged_cap),
        }
    }

    fn to_cached(&self) -> CachedPercentiles {
        CachedPercentiles {
            sat_result: self.sat_result,
            usd_result: self.usd_result,
        }
    }

    #[inline]
    fn accumulate(&mut self, amount: u64, usd: u128) {
        self.price_sats += amount;
        self.price_usd += usd;
    }

    fn finalize_price(&mut self, price: Cents, collect_merged: bool) {
        if collect_merged && self.price_sats > 0 {
            let rounded: CentsCompact = price.round_to_dollar(COST_BASIS_PRICE_DIGITS).into();
            if let Some((lp, ls)) = self.merged.last_mut()
                && *lp == rounded
            {
                *ls += Sats::from(self.price_sats);
            } else {
                self.merged.push((rounded, Sats::from(self.price_sats)));
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

    fn done(&self) -> bool {
        (self.total_sats == 0 || self.sat_idx >= PERCENTILES_LEN)
            && (self.total_usd == 0 || self.usd_idx >= PERCENTILES_LEN)
    }
}
