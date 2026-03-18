use brk_types::StoredF32;

use super::fenwick::FenwickTree;

/// Fast expanding percentile tracker using a Fenwick tree (Binary Indexed Tree).
///
/// Values are discretized to 10 BPS (0.1%) resolution and tracked in
/// a fixed-size frequency array with Fenwick prefix sums. This gives:
/// - O(log N) insert (N = tree size, ~16 ops for 43k buckets)
/// - O(log N) percentile query via prefix-sum walk
/// - 0.1% value resolution (10 BPS granularity)
#[derive(Clone)]
pub(crate) struct ExpandingPercentiles {
    tree: FenwickTree<u32>,
    count: u32,
}

/// Bucket granularity in BPS. 10 BPS = 0.1% = 0.001 ratio.
const BUCKET_BPS: i32 = 10;
/// Max ratio supported: 43.0 = 430,000 BPS.
const MAX_BPS: i32 = 430_000;
const TREE_SIZE: usize = (MAX_BPS / BUCKET_BPS) as usize + 1;

impl Default for ExpandingPercentiles {
    fn default() -> Self {
        Self {
            tree: FenwickTree::new(TREE_SIZE),
            count: 0,
        }
    }
}

impl ExpandingPercentiles {
    pub fn count(&self) -> u32 {
        self.count
    }

    pub fn reset(&mut self) {
        self.tree.reset();
        self.count = 0;
    }

    /// Convert f32 ratio to 0-indexed bucket.
    #[inline]
    fn to_bucket(value: f32) -> usize {
        let bps = (value as f64 * 10000.0).round() as i32;
        (bps / BUCKET_BPS).clamp(0, TREE_SIZE as i32 - 1) as usize
    }

    /// Bulk-load values in O(n + N) instead of O(n log N).
    /// Builds raw frequency counts, then converts to Fenwick in-place.
    pub fn add_bulk(&mut self, values: &[StoredF32]) {
        for &v in values {
            let v = *v;
            if v.is_nan() {
                continue;
            }
            self.count += 1;
            self.tree.add_raw(Self::to_bucket(v), &1);
        }
        self.tree.build_in_place();
    }

    /// Add a value. O(log N).
    #[inline]
    pub fn add(&mut self, value: f32) {
        if value.is_nan() {
            return;
        }
        self.count += 1;
        self.tree.add(Self::to_bucket(value), &1);
    }

    /// Compute 6 percentiles in one call via kth. O(6 × log N) but with
    /// shared tree traversal across all 6 targets for better cache locality.
    /// Quantiles q must be sorted ascending in (0, 1). Output is in BPS.
    pub fn quantiles(&self, qs: &[f64; 6], out: &mut [u32; 6]) {
        if self.count == 0 {
            out.iter_mut().for_each(|o| *o = 0);
            return;
        }
        let mut targets = [0u32; 6];
        for (i, &q) in qs.iter().enumerate() {
            let k = ((q * self.count as f64).ceil() as u32).clamp(1, self.count);
            targets[i] = k - 1; // 0-indexed
        }
        let mut buckets = [0usize; 6];
        self.tree.kth(&targets, &|n: &u32| *n, &mut buckets);
        for (i, bucket) in buckets.iter().enumerate() {
            out[i] = *bucket as u32 * BUCKET_BPS as u32;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn quantile(ep: &ExpandingPercentiles, q: f64) -> u32 {
        let mut out = [0u32; 6];
        ep.quantiles(&[q, q, q, q, q, q], &mut out);
        out[0]
    }

    #[test]
    fn basic_quantiles() {
        let mut ep = ExpandingPercentiles::default();
        for i in 1..=1000 {
            ep.add(i as f32 / 1000.0);
        }
        assert_eq!(ep.count(), 1000);

        let median = quantile(&ep, 0.5);
        assert!((median as i32 - 5000).abs() < 100, "median was {median}");

        let p99 = quantile(&ep, 0.99);
        assert!((p99 as i32 - 9900).abs() < 100, "p99 was {p99}");

        let p01 = quantile(&ep, 0.01);
        assert!((p01 as i32 - 100).abs() < 100, "p01 was {p01}");
    }

    #[test]
    fn empty() {
        let ep = ExpandingPercentiles::default();
        assert_eq!(ep.count(), 0);
        assert_eq!(quantile(&ep, 0.5), 0);
    }

    #[test]
    fn single_value() {
        let mut ep = ExpandingPercentiles::default();
        ep.add(0.42);
        let v = quantile(&ep, 0.5);
        assert!((v as i32 - 4200).abs() <= BUCKET_BPS, "got {v}");
    }

    #[test]
    fn reset_works() {
        let mut ep = ExpandingPercentiles::default();
        for i in 0..100 {
            ep.add(i as f32 / 100.0);
        }
        assert_eq!(ep.count(), 100);
        ep.reset();
        assert_eq!(ep.count(), 0);
        assert_eq!(quantile(&ep, 0.5), 0);
    }

}
