use std::time::Instant;

use brk_types::{CentsUnsigned, Height, Timestamp};
use tracing::debug;
use vecdb::VecIndex;

use crate::{blocks, price};

/// Sparse table for O(1) range maximum queries on prices.
/// Uses O(n log n) space (~140MB for 880k blocks).
pub struct PriceRangeMax {
    /// Flattened table: table[k * n + i] = max of 2^k elements starting at index i
    /// Using flat layout for better cache locality.
    table: Vec<CentsUnsigned>,
    /// Number of elements
    n: usize,
}

impl PriceRangeMax {
    /// Build sparse table from high prices. O(n log n) time and space.
    pub fn build(prices: &[CentsUnsigned]) -> Self {
        let start = Instant::now();

        let n = prices.len();
        if n == 0 {
            return Self {
                table: vec![],
                n: 0,
            };
        }

        // levels = floor(log2(n)) + 1
        let levels = (usize::BITS - n.leading_zeros()) as usize;

        // Allocate flat table: levels * n elements
        let mut table = vec![CentsUnsigned::ZERO; levels * n];

        // Base case: level 0 = original prices
        table[..n].copy_from_slice(prices);

        // Build each level from the previous
        // table[k][i] = max(table[k-1][i], table[k-1][i + 2^(k-1)])
        for k in 1..levels {
            let prev_offset = (k - 1) * n;
            let curr_offset = k * n;
            let half = 1 << (k - 1);
            let end = n.saturating_sub(1 << k) + 1;

            // Use split_at_mut to avoid bounds checks in the loop
            let (prev_level, rest) = table.split_at_mut(curr_offset);
            let prev = &prev_level[prev_offset..prev_offset + n];
            let curr = &mut rest[..n];

            for i in 0..end {
                curr[i] = prev[i].max(prev[i + half]);
            }
        }

        let elapsed = start.elapsed();
        debug!(
            "PriceRangeMax built: {} heights, {} levels, {:.2}MB, {:.2}ms",
            n,
            levels,
            (levels * n * std::mem::size_of::<CentsUnsigned>()) as f64 / 1_000_000.0,
            elapsed.as_secs_f64() * 1000.0
        );

        Self { table, n }
    }

    /// Query maximum value in range [l, r] (inclusive). O(1) time.
    #[inline]
    pub fn range_max(&self, l: usize, r: usize) -> CentsUnsigned {
        debug_assert!(l <= r && r < self.n);

        let len = r - l + 1;
        // k = floor(log2(len))
        let k = (usize::BITS - len.leading_zeros() - 1) as usize;
        let half = 1 << k;

        // max of [l, l + 2^k) and [r - 2^k + 1, r + 1)
        let offset = k * self.n;
        unsafe {
            let a = *self.table.get_unchecked(offset + l);
            let b = *self.table.get_unchecked(offset + r + 1 - half);
            a.max(b)
        }
    }

    /// Query maximum value in height range. O(1) time.
    #[inline]
    pub fn max_between(&self, from: Height, to: Height) -> CentsUnsigned {
        self.range_max(from.to_usize(), to.to_usize())
    }
}

/// Context shared across block processing.
pub struct ComputeContext {
    /// Starting height for this computation run
    pub starting_height: Height,

    /// Last height to process
    pub last_height: Height,

    /// Pre-computed height -> timestamp mapping
    pub height_to_timestamp: Vec<Timestamp>,

    /// Pre-computed height -> price mapping (if available)
    pub height_to_price: Option<Vec<CentsUnsigned>>,

    /// Sparse table for O(1) range max queries on high prices.
    /// Used for computing max price during UTXO holding periods (ATH regret).
    pub price_range_max: Option<PriceRangeMax>,
}

impl ComputeContext {
    /// Create a new computation context.
    pub fn new(
        starting_height: Height,
        last_height: Height,
        blocks: &blocks::Vecs,
        price: Option<&price::Vecs>,
    ) -> Self {
        let height_to_timestamp: Vec<Timestamp> =
            blocks.time.timestamp_monotonic.into_iter().collect();

        let height_to_price: Option<Vec<CentsUnsigned>> = price
            .map(|p| &p.cents.split.height.close)
            .map(|v| v.into_iter().map(|c| *c).collect());

        // Build sparse table for O(1) range max queries on HIGH prices
        // Used for computing peak price during UTXO holding periods (ATH regret)
        let price_range_max = price
            .map(|p| &p.cents.split.height.high)
            .map(|v| v.into_iter().map(|c| *c).collect::<Vec<_>>())
            .map(|prices| PriceRangeMax::build(&prices));

        Self {
            starting_height,
            last_height,
            height_to_timestamp,
            height_to_price,
            price_range_max,
        }
    }

    /// Get price at height (None if no price data or height out of range).
    pub fn price_at(&self, height: Height) -> Option<CentsUnsigned> {
        self.height_to_price
            .as_ref()?
            .get(height.to_usize())
            .copied()
    }

    /// Get timestamp at height.
    pub fn timestamp_at(&self, height: Height) -> Timestamp {
        self.height_to_timestamp[height.to_usize()]
    }
}
