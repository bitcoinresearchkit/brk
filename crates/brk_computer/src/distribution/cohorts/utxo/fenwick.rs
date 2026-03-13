use brk_cohort::{Filter, PROFITABILITY_RANGE_COUNT, compute_profitability_boundaries};
use brk_types::{Cents, CentsCompact, Sats};

use crate::{
    distribution::state::PendingDelta,
    internal::{PERCENTILES, PERCENTILES_LEN, algo::{FenwickNode, FenwickTree}},
};

use super::COST_BASIS_PRICE_DIGITS;

/// Number of age range cohorts (21: 20 boundaries + 1 unbounded).
const AGE_RANGE_COUNT: usize = 21;

// Tier boundaries for 5-significant-digit dollar bucketing.
// Matches the rounding used by `Cents::round_to_dollar(5)`.
const TIER0_COUNT: usize = 100_000; // $0-$99,999 exact dollars
const TIER1_COUNT: usize = 90_000; // $100,000-$999,990 step $10
const OVERFLOW: usize = 1; // $1,000,000+ clamped to last bucket

const TIER1_START: usize = TIER0_COUNT;

/// Total number of buckets.
const TREE_SIZE: usize = TIER0_COUNT + TIER1_COUNT + OVERFLOW; // 190,001

/// 4-field Fenwick tree node for combined cost basis tracking.
#[derive(Clone, Copy, Default)]
pub(super) struct CostBasisNode {
    all_sats: i64,
    sth_sats: i64,
    all_usd: i128,
    sth_usd: i128,
}

impl CostBasisNode {
    #[inline]
    fn new(sats: i64, usd: i128, is_sth: bool) -> Self {
        Self {
            all_sats: sats,
            sth_sats: if is_sth { sats } else { 0 },
            all_usd: usd,
            sth_usd: if is_sth { usd } else { 0 },
        }
    }
}

impl FenwickNode for CostBasisNode {
    #[inline(always)]
    fn add_assign(&mut self, other: &Self) {
        self.all_sats += other.all_sats;
        self.sth_sats += other.sth_sats;
        self.all_usd += other.all_usd;
        self.sth_usd += other.sth_usd;
    }
}

/// Combined Fenwick tree for per-block accurate percentile and profitability queries.
#[derive(Clone)]
pub(super) struct CostBasisFenwick {
    tree: FenwickTree<CostBasisNode>,
    /// Running totals (sum of all underlying frequencies).
    totals: CostBasisNode,
    /// Pre-computed: which age-range cohort index is STH?
    is_sth: [bool; AGE_RANGE_COUNT],
    initialized: bool,
}

// ---------------------------------------------------------------------------
// Bucket mapping: 5-significant-digit dollar precision
// Uses Cents::round_to_dollar(5) for rounding, then maps rounded dollars
// to a flat bucket index across two tiers.
// ---------------------------------------------------------------------------

/// Map rounded dollars to a flat bucket index.
/// Prices >= $1M are clamped to the last bucket.
#[inline]
fn dollars_to_bucket(dollars: u64) -> usize {
    if dollars < 100_000 {
        dollars as usize
    } else if dollars < 1_000_000 {
        TIER1_START + ((dollars - 100_000) / 10) as usize
    } else {
        TREE_SIZE - 1 // overflow bucket for $1M+
    }
}

/// Convert a bucket index back to a price in Cents.
#[inline]
fn bucket_to_cents(bucket: usize) -> Cents {
    let dollars: u64 = if bucket < TIER1_START {
        bucket as u64
    } else if bucket < TREE_SIZE - 1 {
        100_000 + (bucket - TIER1_START) as u64 * 10
    } else {
        1_000_000
    };
    Cents::from(dollars * 100)
}

/// Map a CentsCompact price to a bucket index.
#[inline]
fn price_to_bucket(price: CentsCompact) -> usize {
    cents_to_bucket(price.into())
}

/// Map a Cents price to a bucket index.
#[inline]
fn cents_to_bucket(price: Cents) -> usize {
    dollars_to_bucket(u64::from(price.round_to_dollar(COST_BASIS_PRICE_DIGITS)) / 100)
}

// ---------------------------------------------------------------------------
// CostBasisFenwick implementation
// ---------------------------------------------------------------------------

impl CostBasisFenwick {
    pub(super) fn new() -> Self {
        Self {
            tree: FenwickTree::new(TREE_SIZE),
            totals: CostBasisNode::default(),
            is_sth: [false; AGE_RANGE_COUNT],
            initialized: false,
        }
    }

    pub(super) fn is_initialized(&self) -> bool {
        self.initialized
    }

    /// Pre-compute `is_sth` lookup from the STH filter and age-range filters.
    pub(super) fn compute_is_sth<'a>(
        &mut self,
        sth_filter: &Filter,
        age_range_filters: impl Iterator<Item = &'a Filter>,
    ) {
        for (i, f) in age_range_filters.enumerate() {
            self.is_sth[i] = sth_filter.includes(f);
        }
    }

    pub(super) fn is_sth_at(&self, age_range_idx: usize) -> bool {
        self.is_sth[age_range_idx]
    }

    /// Apply a net delta from a pending map entry.
    pub(super) fn apply_delta(
        &mut self,
        price: CentsCompact,
        pending: &PendingDelta,
        is_sth: bool,
    ) {
        let net_sats = u64::from(pending.inc) as i64 - u64::from(pending.dec) as i64;
        if net_sats == 0 {
            return;
        }
        let bucket = price_to_bucket(price);
        let delta = CostBasisNode::new(net_sats, price.as_u128() as i128 * net_sats as i128, is_sth);
        self.tree.add(bucket, &delta);
        self.totals.add_assign(&delta);
    }

    /// Bulk-initialize from BTreeMaps (one per age-range cohort).
    /// Call after state import when all pending maps have been drained.
    pub(super) fn bulk_init<'a>(
        &mut self,
        maps: impl Iterator<Item = (&'a std::collections::BTreeMap<CentsCompact, Sats>, bool)>,
    ) {
        self.tree.reset();
        self.totals = CostBasisNode::default();

        for (map, is_sth) in maps {
            for (&price, &sats) in map.iter() {
                let bucket = price_to_bucket(price);
                let s = u64::from(sats) as i64;
                let node = CostBasisNode::new(s, price.as_u128() as i128 * s as i128, is_sth);
                self.tree.add_raw(bucket, &node);
                self.totals.add_assign(&node);
            }
        }
        self.tree.build_in_place();
        self.initialized = true;
    }

    // -----------------------------------------------------------------------
    // Percentile queries
    // -----------------------------------------------------------------------

    /// Compute sat-weighted and usd-weighted percentile prices for ALL cohort.
    pub(super) fn percentiles_all(&self) -> PercentileResult {
        self.compute_percentiles(
            self.totals.all_sats,
            self.totals.all_usd,
            |n| n.all_sats,
            |n| n.all_usd,
        )
    }

    /// Compute percentile prices for STH cohort.
    pub(super) fn percentiles_sth(&self) -> PercentileResult {
        self.compute_percentiles(
            self.totals.sth_sats,
            self.totals.sth_usd,
            |n| n.sth_sats,
            |n| n.sth_usd,
        )
    }

    /// Compute percentile prices for LTH cohort (all - sth per node).
    pub(super) fn percentiles_lth(&self) -> PercentileResult {
        self.compute_percentiles(
            self.totals.all_sats - self.totals.sth_sats,
            self.totals.all_usd - self.totals.sth_usd,
            |n| n.all_sats - n.sth_sats,
            |n| n.all_usd - n.sth_usd,
        )
    }

    fn compute_percentiles(
        &self,
        total_sats: i64,
        total_usd: i128,
        sat_field: impl Fn(&CostBasisNode) -> i64,
        usd_field: impl Fn(&CostBasisNode) -> i128,
    ) -> PercentileResult {
        let mut result = PercentileResult::default();

        if total_sats <= 0 {
            return result;
        }

        // Build sorted sat targets: [min=0, percentiles..., max=total-1]
        let mut sat_targets = [0i64; PERCENTILES_LEN + 2];
        sat_targets[0] = 0; // min
        for (i, &p) in PERCENTILES.iter().enumerate() {
            sat_targets[i + 1] = (total_sats * i64::from(p) / 100 - 1).max(0);
        }
        sat_targets[PERCENTILES_LEN + 1] = total_sats - 1; // max

        let mut sat_buckets = [0usize; PERCENTILES_LEN + 2];
        self.tree
            .batch_kth(&sat_targets, &sat_field, &mut sat_buckets);

        result.min_price = bucket_to_cents(sat_buckets[0]);
        (0..PERCENTILES_LEN).for_each(|i| {
            result.sat_prices[i] = bucket_to_cents(sat_buckets[i + 1]);
        });
        result.max_price = bucket_to_cents(sat_buckets[PERCENTILES_LEN + 1]);

        // USD-weighted percentiles (batch)
        if total_usd > 0 {
            let mut usd_targets = [0i128; PERCENTILES_LEN];
            for (i, &p) in PERCENTILES.iter().enumerate() {
                usd_targets[i] = (total_usd * i128::from(p) / 100 - 1).max(0);
            }

            let mut usd_buckets = [0usize; PERCENTILES_LEN];
            self.tree
                .batch_kth(&usd_targets, &usd_field, &mut usd_buckets);

            (0..PERCENTILES_LEN).for_each(|i| {
                result.usd_prices[i] = bucket_to_cents(usd_buckets[i]);
            });
        }

        result
    }

    // -----------------------------------------------------------------------
    // Supply density queries (±5% of spot price)
    // -----------------------------------------------------------------------

    /// Compute supply density: % of supply with cost basis within ±5% of spot.
    /// Returns (all_bps, sth_bps, lth_bps) as basis points (0-10000).
    pub(super) fn density(&self, spot_price: Cents) -> (u16, u16, u16) {
        if self.totals.all_sats <= 0 {
            return (0, 0, 0);
        }

        let spot_f64 = u64::from(spot_price) as f64;
        let low = Cents::from((spot_f64 * 0.95) as u64);
        let high = Cents::from((spot_f64 * 1.05) as u64);

        let low_bucket = cents_to_bucket(low);
        let high_bucket = cents_to_bucket(high);

        let cum_high = self.tree.prefix_sum(high_bucket);
        let cum_low = if low_bucket > 0 {
            self.tree.prefix_sum(low_bucket - 1)
        } else {
            CostBasisNode::default()
        };

        let all_range = (cum_high.all_sats - cum_low.all_sats).max(0);
        let sth_range = (cum_high.sth_sats - cum_low.sth_sats).max(0);
        let lth_range = all_range - sth_range;

        let to_bps = |range: i64, total: i64| -> u16 {
            if total <= 0 {
                0
            } else {
                (range as f64 / total as f64 * 10000.0) as u16
            }
        };

        let lth_total = self.totals.all_sats - self.totals.sth_sats;
        (
            to_bps(all_range, self.totals.all_sats),
            to_bps(sth_range, self.totals.sth_sats),
            to_bps(lth_range, lth_total),
        )
    }

    // -----------------------------------------------------------------------
    // Profitability queries (all cohort only)
    // -----------------------------------------------------------------------

    /// Compute profitability range buckets from current spot price.
    /// Returns 25 ranges: (sats, usd_raw) per range.
    pub(super) fn profitability(
        &self,
        spot_price: Cents,
    ) -> [(u64, u128); PROFITABILITY_RANGE_COUNT] {
        let mut result = [(0u64, 0u128); PROFITABILITY_RANGE_COUNT];

        if self.totals.all_sats <= 0 {
            return result;
        }

        let boundaries = compute_profitability_boundaries(spot_price);

        let mut prev_sats: i64 = 0;
        let mut prev_usd: i128 = 0;

        for (i, &boundary) in boundaries.iter().enumerate() {
            let boundary_bucket = cents_to_bucket(boundary);
            // prefix_sum through the bucket BEFORE the boundary
            let cum = if boundary_bucket > 0 {
                self.tree.prefix_sum(boundary_bucket - 1)
            } else {
                CostBasisNode::default()
            };
            let range_sats = cum.all_sats - prev_sats;
            let range_usd = cum.all_usd - prev_usd;
            result[i] = (range_sats.max(0) as u64, range_usd.max(0) as u128);
            prev_sats = cum.all_sats;
            prev_usd = cum.all_usd;
        }

        // Last range: everything >= last boundary
        let remaining_sats = self.totals.all_sats - prev_sats;
        let remaining_usd = self.totals.all_usd - prev_usd;
        result[PROFITABILITY_RANGE_COUNT - 1] =
            (remaining_sats.max(0) as u64, remaining_usd.max(0) as u128);

        result
    }
}

/// Result of a percentile computation for one cohort.
#[derive(Default)]
pub(super) struct PercentileResult {
    pub sat_prices: [Cents; PERCENTILES_LEN],
    pub usd_prices: [Cents; PERCENTILES_LEN],
    pub min_price: Cents,
    pub max_price: Cents,
}
