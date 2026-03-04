use brk_types::StoredF32;

/// Fast expanding percentile tracker using a Fenwick tree (Binary Indexed Tree).
///
/// Values are discretized to BasisPoints32 precision (×10000) and tracked in
/// a fixed-size frequency array with Fenwick prefix sums. This gives:
/// - O(log N) insert (N = tree size, ~18 ops for 200k buckets)
/// - O(log N) percentile query via prefix-sum walk
/// - Exact at BasisPoints32 resolution (no approximation)
#[derive(Clone)]
pub(crate) struct ExpandingPercentiles {
    /// Fenwick tree storing cumulative frequency counts.
    /// Index 0 is unused (1-indexed). tree[i] covers bucket (i - 1 + offset).
    tree: Vec<u64>,
    count: u64,
    /// Offset so bucket 0 in the tree corresponds to BPS value `offset`.
    offset: i32,
    size: usize,
}

/// Max BPS value supported. Ratio of 42.0 = 420,000 BPS.
const MAX_BPS: i32 = 430_000;
/// Min BPS value supported (0 = ratio of 0.0).
const MIN_BPS: i32 = 0;
const TREE_SIZE: usize = (MAX_BPS - MIN_BPS) as usize + 1;

impl Default for ExpandingPercentiles {
    fn default() -> Self {
        Self {
            tree: vec![0u64; TREE_SIZE + 1], // 1-indexed
            count: 0,
            offset: MIN_BPS,
            size: TREE_SIZE,
        }
    }
}

impl ExpandingPercentiles {
    pub fn count(&self) -> u64 {
        self.count
    }

    pub fn reset(&mut self) {
        self.tree.iter_mut().for_each(|v| *v = 0);
        self.count = 0;
    }

    /// Convert f32 ratio to bucket index (1-indexed for Fenwick).
    #[inline]
    fn to_bucket(&self, value: f32) -> usize {
        let bps = (value as f64 * 10000.0).round() as i32;
        let clamped = bps.clamp(self.offset, self.offset + self.size as i32 - 1);
        (clamped - self.offset) as usize + 1 // 1-indexed
    }

    /// Bulk-load values in O(n + N) instead of O(n log N).
    /// Builds raw frequency counts, then converts to Fenwick in-place.
    pub fn add_bulk(&mut self, values: &[StoredF32]) {
        // Build raw frequency counts into tree (treated as flat array)
        for &v in values {
            let v = *v;
            if v.is_nan() {
                continue;
            }
            self.count += 1;
            let bucket = self.to_bucket(v);
            self.tree[bucket] += 1;
        }
        // Convert flat frequencies to Fenwick tree in O(N)
        for i in 1..=self.size {
            let parent = i + (i & i.wrapping_neg());
            if parent <= self.size {
                let val = self.tree[i];
                self.tree[parent] += val;
            }
        }
    }

    /// Add a value. O(log N).
    #[inline]
    pub fn add(&mut self, value: f32) {
        if value.is_nan() {
            return;
        }
        self.count += 1;
        let mut i = self.to_bucket(value);
        while i <= self.size {
            self.tree[i] += 1;
            i += i & i.wrapping_neg(); // i += lowbit(i)
        }
    }

    /// Find the bucket containing the k-th element (1-indexed k).
    /// Uses the standard Fenwick tree walk-down in O(log N).
    #[inline]
    fn kth(&self, mut k: u64) -> usize {
        let mut pos = 0;
        let mut bit = 1 << (usize::BITS - 1 - self.size.leading_zeros()); // highest power of 2 <= size
        while bit > 0 {
            let next = pos + bit;
            if next <= self.size && self.tree[next] < k {
                k -= self.tree[next];
                pos = next;
            }
            bit >>= 1;
        }
        pos + 1 // 1-indexed bucket
    }

    /// Convert bucket index back to BPS u32 value.
    #[inline]
    fn bucket_to_bps(&self, bucket: usize) -> u32 {
        (bucket as i32 - 1 + self.offset) as u32
    }

    /// Compute 6 percentiles in one call. O(6 × log N).
    /// Quantiles q must be in (0, 1).
    pub fn quantiles(&self, qs: &[f64; 6], out: &mut [u32; 6]) {
        if self.count == 0 {
            out.iter_mut().for_each(|o| *o = 0);
            return;
        }
        for (i, &q) in qs.iter().enumerate() {
            // k = ceil(q * count), clamped to [1, count]
            let k = ((q * self.count as f64).ceil() as u64).clamp(1, self.count);
            out[i] = self.bucket_to_bps(self.kth(k));
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
        // Add ratios 0.01 to 1.0 (BPS 100 to 10000)
        for i in 1..=1000 {
            ep.add(i as f32 / 1000.0);
        }
        assert_eq!(ep.count(), 1000);

        let median = quantile(&ep, 0.5);
        // 0.5 ratio = 5000 BPS, median of 1..1000 ratios ≈ 500/1000 = 0.5 = 5000 BPS
        assert!(
            (median as i32 - 5000).abs() < 100,
            "median was {median}"
        );

        let p99 = quantile(&ep, 0.99);
        assert!(
            (p99 as i32 - 9900).abs() < 100,
            "p99 was {p99}"
        );

        let p01 = quantile(&ep, 0.01);
        assert!(
            (p01 as i32 - 100).abs() < 100,
            "p01 was {p01}"
        );
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
        ep.add(0.42); // 4200 BPS
        assert_eq!(quantile(&ep, 0.0001), 4200);
        assert_eq!(quantile(&ep, 0.5), 4200);
        assert_eq!(quantile(&ep, 0.9999), 4200);
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
