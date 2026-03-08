use brk_cohort::{
    compute_profitability_boundaries, Filter, PROFITABILITY_RANGE_COUNT,
};
use brk_types::{Cents, CentsCompact, Sats};

use crate::{
    distribution::state::PendingDelta,
    internal::{FenwickNode, FenwickTree, PERCENTILES, PERCENTILES_LEN},
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
    let rounded = Cents::from(price).round_to_dollar(COST_BASIS_PRICE_DIGITS);
    dollars_to_bucket(u64::from(rounded) / 100)
}

/// Map a Cents price to a bucket index.
#[inline]
fn cents_to_bucket(price: Cents) -> usize {
    let rounded = price.round_to_dollar(COST_BASIS_PRICE_DIGITS);
    dollars_to_bucket(u64::from(rounded) / 100)
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
        let net_usd = price.as_u128() as i128 * net_sats as i128;
        let delta = CostBasisNode {
            all_sats: net_sats,
            sth_sats: if is_sth { net_sats } else { 0 },
            all_usd: net_usd,
            sth_usd: if is_sth { net_usd } else { 0 },
        };
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
                let usd = price.as_u128() as i128 * s as i128;
                let node = CostBasisNode {
                    all_sats: s,
                    sth_sats: if is_sth { s } else { 0 },
                    all_usd: usd,
                    sth_usd: if is_sth { usd } else { 0 },
                };
                self.tree.add_raw(bucket, &node);
                self.totals.add_assign(&node);
            }
        }
        self.tree.build_in_place();
        self.initialized = true;
    }

    /// Reset to uninitialized empty state.
    pub(super) fn reset(&mut self) {
        self.tree.reset();
        self.totals = CostBasisNode::default();
        self.initialized = false;
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

        // Sat-weighted percentiles: find first bucket where cumulative >= target
        for (i, &p) in PERCENTILES.iter().enumerate() {
            let target = (total_sats * i64::from(p) / 100 - 1).max(0);
            let bucket = self.tree.kth(target, &sat_field);
            result.sat_prices[i] = bucket_to_cents(bucket);
        }

        // USD-weighted percentiles
        if total_usd > 0 {
            for (i, &p) in PERCENTILES.iter().enumerate() {
                let target = (total_usd * i128::from(p) / 100 - 1).max(0);
                let bucket = self.tree.kth(target, &usd_field);
                result.usd_prices[i] = bucket_to_cents(bucket);
            }
        }

        // Min/max via kth(0) and kth(total-1)
        result.min_price = bucket_to_cents(self.tree.kth(0i64, &sat_field));
        result.max_price = bucket_to_cents(self.tree.kth(total_sats - 1, &sat_field));

        result
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn bucket_round_trip() {
        // Low prices: exact dollar precision
        let price = CentsCompact::new(5000_00); // $5000
        let bucket = price_to_bucket(price);
        let back = bucket_to_cents(bucket);
        assert_eq!(u64::from(back), 5000 * 100);

        // High price: $90,000 → rounded to $90,000 (already 5 digits)
        let price = CentsCompact::new(90_000_00);
        let bucket = price_to_bucket(price);
        let back = bucket_to_cents(bucket);
        assert_eq!(u64::from(back), 90_000 * 100);

        // Tier 1: $123,456 → rounded to $123,460
        let price = CentsCompact::new(123_456_00);
        let bucket = price_to_bucket(price);
        let back = bucket_to_cents(bucket);
        assert_eq!(u64::from(back), 123_460 * 100);

        // Overflow: $2,000,000 → clamped to $1,000,000
        let price = CentsCompact::new(2_000_000_00);
        let bucket = price_to_bucket(price);
        assert_eq!(bucket, TREE_SIZE - 1);
        assert_eq!(u64::from(bucket_to_cents(bucket)), 1_000_000 * 100);
    }

    #[test]
    fn bucket_edge_cases() {
        // $0
        assert_eq!(price_to_bucket(CentsCompact::new(0)), 0);
        assert_eq!(u64::from(bucket_to_cents(0)), 0);

        // $1
        let bucket = price_to_bucket(CentsCompact::new(100));
        assert_eq!(bucket, 1);

        // Max CentsCompact
        let bucket = price_to_bucket(CentsCompact::MAX);
        assert!(bucket < TREE_SIZE);
    }

    #[test]
    fn bulk_init_and_percentiles() {
        let mut fenwick = CostBasisFenwick::new();

        // Create a simple BTreeMap: 100 sats at $10,000, 100 sats at $50,000
        let mut map = BTreeMap::new();
        map.insert(CentsCompact::new(10_000_00), Sats::from(100u64));
        map.insert(CentsCompact::new(50_000_00), Sats::from(100u64));

        fenwick.bulk_init(std::iter::once((&map, true)));

        assert!(fenwick.is_initialized());

        let result = fenwick.percentiles_all();
        // Median (50th percentile) should be at $10,000 (first 100 sats)
        // since target = 200 * 50/100 = 100, and first 100 sats are at $10,000
        assert_eq!(u64::from(result.sat_prices[9]), 10_000 * 100); // index 9 = 50th percentile

        // Min should be $10,000, max should be $50,000
        assert_eq!(u64::from(result.min_price), 10_000 * 100);
        assert_eq!(u64::from(result.max_price), 50_000 * 100);
    }

    #[test]
    fn apply_delta_updates_totals() {
        let mut fenwick = CostBasisFenwick::new();
        fenwick.initialized = true;

        let price = CentsCompact::new(10_000_00);
        fenwick.apply_delta(price, &PendingDelta { inc: Sats::from(500u64), dec: Sats::ZERO }, true);
        assert_eq!(fenwick.totals.all_sats, 500);
        assert_eq!(fenwick.totals.sth_sats, 500);

        fenwick.apply_delta(price, &PendingDelta { inc: Sats::ZERO, dec: Sats::from(200u64) }, true);
        assert_eq!(fenwick.totals.all_sats, 300);
        assert_eq!(fenwick.totals.sth_sats, 300);

        // Non-STH delta
        fenwick.apply_delta(price, &PendingDelta { inc: Sats::from(100u64), dec: Sats::ZERO }, false);
        assert_eq!(fenwick.totals.all_sats, 400);
        assert_eq!(fenwick.totals.sth_sats, 300);
    }

    #[test]
    fn profitability_ranges_sum_to_total() {
        let mut fenwick = CostBasisFenwick::new();

        let mut map = BTreeMap::new();
        // Spread sats across different prices
        map.insert(CentsCompact::new(1_000_00), Sats::from(1000u64));
        map.insert(CentsCompact::new(10_000_00), Sats::from(2000u64));
        map.insert(CentsCompact::new(50_000_00), Sats::from(3000u64));
        map.insert(CentsCompact::new(100_000_00), Sats::from(4000u64));

        fenwick.bulk_init(std::iter::once((&map, false)));

        let spot = Cents::from(50_000u64 * 100);
        let prof = fenwick.profitability(spot);

        let total_sats: u64 = prof.iter().map(|(s, _)| s).sum();
        assert_eq!(total_sats, 10_000);
    }
}
