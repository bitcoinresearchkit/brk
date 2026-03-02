use std::time::Instant;

use brk_types::{Cents, Height, Timestamp};
use tracing::debug;
use vecdb::VecIndex;

/// Sparse table for O(1) range maximum queries on prices.
/// Vec<Vec> per level for incremental O(new_blocks * log n) extension.
#[derive(Debug, Clone, Default)]
pub struct PriceRangeMax {
    levels: Vec<Vec<Cents>>,
    n: usize,
}

impl PriceRangeMax {
    pub(crate) fn extend(&mut self, prices: &[Cents]) {
        let new_n = prices.len();
        if new_n <= self.n || new_n == 0 {
            return;
        }

        let start = Instant::now();
        let old_n = self.n;
        let new_levels_count = (usize::BITS - new_n.leading_zeros()) as usize;

        while self.levels.len() < new_levels_count {
            self.levels.push(Vec::new());
        }

        self.levels[0].extend_from_slice(&prices[old_n..new_n]);

        for k in 1..new_levels_count {
            let half = 1 << (k - 1);
            let new_end = if new_n >= (1 << k) {
                new_n + 1 - (1 << k)
            } else {
                0
            };

            let old_end = self.levels[k].len();
            if new_end > old_end {
                let (prev_levels, curr_levels) = self.levels.split_at_mut(k);
                let prev = &prev_levels[k - 1];
                let curr = &mut curr_levels[0];
                curr.reserve(new_end - old_end);
                for i in old_end..new_end {
                    curr.push(prev[i].max(prev[i + half]));
                }
            }
        }

        self.n = new_n;

        let elapsed = start.elapsed();
        let total_entries: usize = self.levels.iter().map(|l| l.len()).sum();
        debug!(
            "PriceRangeMax extended: {} -> {} heights ({} new), {} levels, {:.2}MB, {:.2}ms",
            old_n,
            new_n,
            new_n - old_n,
            new_levels_count,
            (total_entries * std::mem::size_of::<Cents>()) as f64 / 1_000_000.0,
            elapsed.as_secs_f64() * 1000.0
        );
    }

    pub(crate) fn truncate(&mut self, new_n: usize) {
        if new_n >= self.n {
            return;
        }
        if new_n == 0 {
            self.levels.clear();
            self.n = 0;
            return;
        }
        let new_levels_count = (usize::BITS - new_n.leading_zeros()) as usize;
        self.levels.truncate(new_levels_count);
        for k in 0..new_levels_count {
            let valid = if new_n >= (1 << k) {
                new_n + 1 - (1 << k)
            } else {
                0
            };
            self.levels[k].truncate(valid);
        }
        self.n = new_n;
    }

    #[inline]
    pub(crate) fn range_max(&self, l: usize, r: usize) -> Cents {
        debug_assert!(l <= r && r < self.n);
        let len = r - l + 1;
        let k = (usize::BITS - len.leading_zeros() - 1) as usize;
        let half = 1 << k;
        let level = &self.levels[k];
        unsafe {
            let a = *level.get_unchecked(l);
            let b = *level.get_unchecked(r + 1 - half);
            a.max(b)
        }
    }

    #[inline]
    pub(crate) fn max_between(&self, from: Height, to: Height) -> Cents {
        self.range_max(from.to_usize(), to.to_usize())
    }
}

pub struct ComputeContext<'a> {
    pub starting_height: Height,
    pub last_height: Height,
    pub height_to_timestamp: &'a [Timestamp],
    pub height_to_price: &'a [Cents],
    pub price_range_max: &'a PriceRangeMax,
}

impl<'a> ComputeContext<'a> {
    pub(crate) fn price_at(&self, height: Height) -> Cents {
        self.height_to_price[height.to_usize()]
    }

    pub(crate) fn timestamp_at(&self, height: Height) -> Timestamp {
        self.height_to_timestamp[height.to_usize()]
    }
}
