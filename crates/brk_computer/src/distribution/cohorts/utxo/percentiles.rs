use std::{cmp::Reverse, collections::BinaryHeap, fs, path::Path};

use brk_cohort::{Filtered, TERM_NAMES};
use brk_error::Result;
use brk_types::{
    BasisPoints16, Cents, CentsCompact, CostBasisDistribution, Date, Height, Sats,
};
use vecdb::WritableVec;

use crate::internal::{PERCENTILES, PERCENTILES_LEN, compute_spot_percentile_rank};

use crate::distribution::metrics::{CohortMetricsBase, CostBasisExtended};

use super::groups::UTXOCohorts;

/// Significant digits for cost basis prices (after rounding to dollars).
const COST_BASIS_PRICE_DIGITS: i32 = 5;

impl UTXOCohorts {
    /// Compute and push percentiles for aggregate cohorts (all, sth, lth).
    ///
    /// Single K-way merge pass over all age_range cohorts computes percentiles
    /// for all 3 targets simultaneously, since each cohort belongs to exactly
    /// one of STH/LTH and always contributes to ALL.
    ///
    /// Uses BinaryHeap with direct BTreeMap iterators — O(log K) merge
    /// with zero intermediate Vec allocation.
    pub(crate) fn truncate_push_aggregate_percentiles(
        &mut self,
        height: Height,
        spot: Cents,
        date_opt: Option<Date>,
        states_path: &Path,
    ) -> Result<()> {
        let collect_merged = date_opt.is_some();

        // Phase 1: compute totals + merge.
        // Scoped so age_range borrows release before push_target borrows self.all/sth/lth.
        let targets = {
            let sth_filter = self.sth.metrics.filter().clone();
            let mut totals = AllSthLth::<(u64, u128)>::default();

            // Collect BTreeMap refs from age_range, skip empty, compute totals.
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

            // K-way merge via BinaryHeap + BTreeMap iterators (no Vec copies)
            if all_has_data {
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
                        targets
                            .for_each_mut(|t| t.finalize_price(prev.into(), collect_merged));
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

            targets
        };

        // Phase 2: push results (borrows self.all/sth/lth mutably)
        push_target(
            height, spot, date_opt, states_path, targets.all,
            &mut self.all.metrics.cost_basis.extended, "all",
        )?;
        push_target(
            height, spot, date_opt, states_path, targets.sth,
            &mut self.sth.metrics.cost_basis.extended, TERM_NAMES.short.id,
        )?;
        push_target(
            height, spot, date_opt, states_path, targets.lth,
            &mut self.lth.metrics.cost_basis.extended, TERM_NAMES.long.id,
        )?;

        Ok(())
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

#[allow(clippy::too_many_arguments)]
fn push_target(
    height: Height,
    spot: Cents,
    date_opt: Option<Date>,
    states_path: &Path,
    target: PercTarget,
    ext: &mut CostBasisExtended,
    name: &str,
) -> Result<()> {
    ext.percentiles.truncate_push(height, &target.sat_result)?;
    ext.invested_capital
        .truncate_push(height, &target.usd_result)?;

    let sat_rank = if target.total_sats > 0 {
        compute_spot_percentile_rank(&target.sat_result, spot)
    } else {
        BasisPoints16::ZERO
    };
    ext.spot_cost_basis_percentile
        .bps
        .height
        .truncate_push(height, sat_rank)?;

    let usd_rank = if target.total_usd > 0 {
        compute_spot_percentile_rank(&target.usd_result, spot)
    } else {
        BasisPoints16::ZERO
    };
    ext.spot_invested_capital_percentile
        .bps
        .height
        .truncate_push(height, usd_rank)?;

    if let Some(date) = date_opt {
        let dir = states_path.join(format!("utxo_{name}_cost_basis/by_date"));
        fs::create_dir_all(&dir)?;
        fs::write(
            dir.join(date.to_string()),
            CostBasisDistribution::serialize_iter(target.merged.into_iter())?,
        )?;
    }
    Ok(())
}
