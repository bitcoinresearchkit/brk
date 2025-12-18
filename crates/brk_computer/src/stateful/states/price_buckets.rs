//! Logarithmic price buckets with Fenwick tree for O(log n) percentile queries.
//!
//! Uses logarithmic buckets to maintain constant relative precision across all price levels.
//! Bucket i represents prices in range [MIN_PRICE * BASE^i, MIN_PRICE * BASE^(i+1)).

use brk_types::{Dollars, Sats};

use super::fenwick::FenwickTree;
use crate::grouped::{PERCENTILES, PERCENTILES_LEN};

/// Minimum price tracked (sub-cent for early Bitcoin days).
const MIN_PRICE: f64 = 0.001;

/// Maximum price tracked ($100M for future-proofing).
#[allow(unused)]
const MAX_PRICE: f64 = 100_000_000.0;

/// Base for logarithmic buckets (0.1% precision).
const BASE: f64 = 1.001;

/// Pre-computed ln(BASE) for efficiency.
const LN_BASE: f64 = 0.0009995003; // ln(1.001)

/// Pre-computed ln(MIN_PRICE) for efficiency.
const LN_MIN_PRICE: f64 = -6.907755279; // ln(0.001)

/// Number of buckets needed: ceil(ln(MAX/MIN) / ln(BASE)).
/// ln(100_000_000 / 0.001) / ln(1.001) ≈ 25,328
const NUM_BUCKETS: usize = 25_400; // Rounded up for safety

/// Logarithmic price buckets with O(log n) percentile queries.
#[derive(Clone, Debug)]
pub struct PriceBuckets {
    /// Fenwick tree for O(log n) prefix sums.
    fenwick: FenwickTree,
    /// Direct bucket access for iteration (needed for unrealized computation).
    buckets: Vec<Sats>,
    /// Total supply tracked.
    total: Sats,
}

impl Default for PriceBuckets {
    fn default() -> Self {
        Self::new()
    }
}

impl PriceBuckets {
    /// Create new empty price buckets.
    pub fn new() -> Self {
        Self {
            fenwick: FenwickTree::new(NUM_BUCKETS),
            buckets: vec![Sats::ZERO; NUM_BUCKETS],
            total: Sats::ZERO,
        }
    }

    /// Convert price to bucket index. O(1).
    #[inline]
    pub fn price_to_bucket(price: Dollars) -> usize {
        let price_f64 = f64::from(price);
        if price_f64 <= MIN_PRICE {
            return 0;
        }
        let bucket = ((price_f64.ln() - LN_MIN_PRICE) / LN_BASE) as usize;
        bucket.min(NUM_BUCKETS - 1)
    }

    /// Convert bucket index to representative price (bucket midpoint). O(1).
    #[inline]
    pub fn bucket_to_price(bucket: usize) -> Dollars {
        // Use geometric mean of bucket range for better accuracy
        let low = MIN_PRICE * BASE.powi(bucket as i32);
        let high = low * BASE;
        Dollars::from((low * high).sqrt())
    }

    /// Add amount at given price. O(log n).
    pub fn increment(&mut self, price: Dollars, amount: Sats) {
        if amount == Sats::ZERO {
            return;
        }
        let bucket = Self::price_to_bucket(price);
        self.fenwick.add(bucket, u64::from(amount));
        self.buckets[bucket] += amount;
        self.total += amount;
    }

    /// Remove amount at given price. O(log n).
    pub fn decrement(&mut self, price: Dollars, amount: Sats) {
        if amount == Sats::ZERO {
            return;
        }
        let bucket = Self::price_to_bucket(price);
        self.fenwick.sub(bucket, u64::from(amount));
        self.buckets[bucket] -= amount;
        self.total -= amount;
    }

    /// Check if empty.
    #[allow(unused)]
    pub fn is_empty(&self) -> bool {
        self.total == Sats::ZERO
    }

    /// Get total supply.
    #[allow(unused)]
    pub fn total(&self) -> Sats {
        self.total
    }

    /// Compute all percentile prices. O(19 * log n) ≈ O(323 ops).
    pub fn compute_percentiles(&self) -> [Dollars; PERCENTILES_LEN] {
        let mut result = [Dollars::NAN; PERCENTILES_LEN];

        if self.total == Sats::ZERO {
            return result;
        }

        let total = u64::from(self.total);

        for (i, &percentile) in PERCENTILES.iter().enumerate() {
            let target = total * u64::from(percentile) / 100;
            if let Some(bucket) = self.fenwick.lower_bound(target) {
                result[i] = Self::bucket_to_price(bucket);
            }
        }

        result
    }

    /// Get amount in a specific bucket.
    #[allow(unused)]
    pub fn get_bucket(&self, bucket: usize) -> Sats {
        self.buckets.get(bucket).copied().unwrap_or(Sats::ZERO)
    }

    /// Iterate over non-empty buckets in a price range.
    /// Used for unrealized computation flip range.
    #[allow(unused)]
    pub fn iter_range(
        &self,
        from_price: Dollars,
        to_price: Dollars,
    ) -> impl Iterator<Item = (Dollars, Sats)> + '_ {
        let from_bucket = Self::price_to_bucket(from_price);
        let to_bucket = Self::price_to_bucket(to_price);

        let (start, end) = if from_bucket <= to_bucket {
            (from_bucket, to_bucket)
        } else {
            (to_bucket, from_bucket)
        };

        (start..=end).filter_map(move |bucket| {
            let amount = self.buckets[bucket];
            if amount > Sats::ZERO {
                Some((Self::bucket_to_price(bucket), amount))
            } else {
                None
            }
        })
    }

    /// Iterate over all non-empty buckets (for full unrealized computation).
    #[allow(unused)]
    pub fn iter(&self) -> impl Iterator<Item = (Dollars, Sats)> + '_ {
        self.buckets
            .iter()
            .enumerate()
            .filter_map(|(bucket, &amount)| {
                if amount > Sats::ZERO {
                    Some((Self::bucket_to_price(bucket), amount))
                } else {
                    None
                }
            })
    }

    /// Get the lowest price bucket with non-zero amount.
    #[allow(unused)]
    pub fn min_price(&self) -> Option<Dollars> {
        self.buckets
            .iter()
            .position(|&s| s > Sats::ZERO)
            .map(Self::bucket_to_price)
    }

    /// Get the highest price bucket with non-zero amount.
    #[allow(unused)]
    pub fn max_price(&self) -> Option<Dollars> {
        self.buckets
            .iter()
            .rposition(|&s| s > Sats::ZERO)
            .map(Self::bucket_to_price)
    }

    /// Clear all data.
    #[allow(unused)]
    pub fn clear(&mut self) {
        self.fenwick.clear();
        self.buckets.fill(Sats::ZERO);
        self.total = Sats::ZERO;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bucket_conversion() {
        // Test price -> bucket -> price roundtrip
        let prices = [0.01, 1.0, 100.0, 10000.0, 50000.0, 100000.0];

        for &price in &prices {
            let bucket = PriceBuckets::price_to_bucket(Dollars::from(price));
            let recovered = PriceBuckets::bucket_to_price(bucket);
            let ratio = f64::from(recovered) / price;
            // Should be within 0.1% (our bucket precision)
            assert!(
                (0.999..=1.001).contains(&ratio),
                "price={}, recovered={}, ratio={}",
                price,
                f64::from(recovered),
                ratio
            );
        }
    }

    #[test]
    fn test_percentiles() {
        let mut buckets = PriceBuckets::new();

        // Add 100 sats at $10, 200 sats at $20, 300 sats at $30
        buckets.increment(Dollars::from(10.0), Sats::from(100u64));
        buckets.increment(Dollars::from(20.0), Sats::from(200u64));
        buckets.increment(Dollars::from(30.0), Sats::from(300u64));

        // Total = 600 sats
        // 50th percentile = 300 sats = should be around $20-$30
        let percentiles = buckets.compute_percentiles();

        // Median (index 9 in PERCENTILES which is 50%)
        let median = percentiles[9]; // PERCENTILES[9] = 50
        let median_f64 = f64::from(median);
        assert!(
            (15.0..=35.0).contains(&median_f64),
            "median={} should be around $20-$30",
            median_f64
        );
    }
}
