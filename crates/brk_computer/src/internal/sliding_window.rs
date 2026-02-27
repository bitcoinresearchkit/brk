/// Sqrt-decomposed sorted structure for O(sqrt(n)) insert/remove/kth.
///
/// Maintains `blocks` sorted sub-arrays where each block is sorted and
/// the blocks are ordered (max of block[i] <= min of block[i+1]).
/// Total element count is tracked via `total_len`.
struct SortedBlocks {
    blocks: Vec<Vec<f64>>,
    total_len: usize,
    block_size: usize,
}

impl SortedBlocks {
    fn new(capacity: usize) -> Self {
        let block_size = ((capacity as f64).sqrt() as usize).max(64);
        Self {
            blocks: Vec::new(),
            total_len: 0,
            block_size,
        }
    }

    fn len(&self) -> usize {
        self.total_len
    }

    fn is_empty(&self) -> bool {
        self.total_len == 0
    }

    /// Insert a value in sorted order. O(sqrt(n)).
    fn insert(&mut self, value: f64) {
        self.total_len += 1;

        if self.blocks.is_empty() {
            self.blocks.push(vec![value]);
            return;
        }

        // Find the block where value belongs: first block whose max >= value
        let block_idx = self.blocks.iter().position(|b| {
            *b.last().unwrap() >= value
        }).unwrap_or(self.blocks.len() - 1);

        let block = &mut self.blocks[block_idx];
        let pos = block.partition_point(|a| *a < value);
        block.insert(pos, value);

        // Split if block too large
        if block.len() > 2 * self.block_size {
            let mid = block.len() / 2;
            let right = block[mid..].to_vec();
            block.truncate(mid);
            self.blocks.insert(block_idx + 1, right);
        }
    }

    /// Remove one occurrence of value. O(sqrt(n)).
    fn remove(&mut self, value: f64) -> bool {
        for (bi, block) in self.blocks.iter_mut().enumerate() {
            if block.is_empty() {
                continue;
            }
            // If value > block max, it's not in this block
            if *block.last().unwrap() < value {
                continue;
            }
            let pos = block.partition_point(|a| *a < value);
            if pos < block.len() && block[pos] == value {
                block.remove(pos);
                self.total_len -= 1;
                if block.is_empty() {
                    self.blocks.remove(bi);
                }
                return true;
            }
            // Value not found (would be in this block range but isn't)
            return false;
        }
        false
    }

    /// Get the k-th smallest element (0-indexed). O(sqrt(n)).
    fn kth(&self, mut k: usize) -> f64 {
        for block in &self.blocks {
            if k < block.len() {
                return block[k];
            }
            k -= block.len();
        }
        unreachable!("kth out of bounds")
    }

    fn first(&self) -> f64 {
        self.blocks.first().unwrap().first().copied().unwrap()
    }

    fn last(&self) -> f64 {
        self.blocks.last().unwrap().last().copied().unwrap()
    }
}

/// Sorted sliding window for rolling distribution/median computations.
///
/// Uses sqrt-decomposition for O(sqrt(n)) insert/remove/kth instead of
/// O(n) memmoves with a flat sorted Vec.
pub(crate) struct SlidingWindowSorted {
    sorted: SortedBlocks,
    running_sum: f64,
    prev_start: usize,
}

impl SlidingWindowSorted {
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            sorted: SortedBlocks::new(cap),
            running_sum: 0.0,
            prev_start: 0,
        }
    }

    /// Reconstruct state from historical data (the elements in [range_start..skip]).
    pub fn reconstruct(&mut self, partial_values: &[f64], range_start: usize, skip: usize) {
        self.prev_start = range_start;
        for idx in range_start..skip {
            let v = partial_values[idx - range_start];
            self.running_sum += v;
            self.sorted.insert(v);
        }
    }

    /// Add a new value and remove all expired values up to `new_start`.
    pub fn advance(&mut self, value: f64, new_start: usize, partial_values: &[f64], range_start: usize) {
        self.running_sum += value;
        self.sorted.insert(value);

        while self.prev_start < new_start {
            let old = partial_values[self.prev_start - range_start];
            self.running_sum -= old;
            self.sorted.remove(old);
            self.prev_start += 1;
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.sorted.is_empty()
    }

    #[inline]
    pub fn average(&self) -> f64 {
        if self.sorted.is_empty() {
            0.0
        } else {
            self.running_sum / self.sorted.len() as f64
        }
    }

    #[inline]
    pub fn min(&self) -> f64 {
        if self.sorted.is_empty() { 0.0 } else { self.sorted.first() }
    }

    #[inline]
    pub fn max(&self) -> f64 {
        if self.sorted.is_empty() { 0.0 } else { self.sorted.last() }
    }

    /// Extract a percentile (0.0-1.0) using linear interpolation.
    #[inline]
    pub fn percentile(&self, p: f64) -> f64 {
        let len = self.sorted.len();
        if len == 0 {
            return 0.0;
        }
        if len == 1 {
            return self.sorted.kth(0);
        }
        let rank = p * (len - 1) as f64;
        let lo = rank.floor() as usize;
        let hi = rank.ceil() as usize;
        if lo == hi {
            self.sorted.kth(lo)
        } else {
            let frac = rank - lo as f64;
            self.sorted.kth(lo) * (1.0 - frac) + self.sorted.kth(hi) * frac
        }
    }
}
