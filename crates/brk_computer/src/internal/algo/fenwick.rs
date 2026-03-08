/// Trait for types that can be stored in a Fenwick tree.
pub(crate) trait FenwickNode: Clone + Copy + Default {
    fn add_assign(&mut self, other: &Self);
}

impl FenwickNode for u32 {
    #[inline(always)]
    fn add_assign(&mut self, other: &Self) {
        *self += other;
    }
}

/// Generic Fenwick tree (Binary Indexed Tree) over arbitrary node types.
///
/// Uses 0-indexed buckets externally; 1-indexed internally.
/// Provides O(log N) point-update, prefix-sum, and kth walk-down.
#[derive(Clone)]
pub(crate) struct FenwickTree<N: FenwickNode> {
    /// 1-indexed tree array. Position 0 is unused.
    tree: Vec<N>,
    size: usize,
}

impl<N: FenwickNode> FenwickTree<N> {
    pub fn new(size: usize) -> Self {
        Self {
            tree: vec![N::default(); size + 1],
            size,
        }
    }

    pub fn reset(&mut self) {
        self.tree.fill(N::default());
    }

    /// Point-update: add `delta` to the node at `bucket` (0-indexed).
    #[inline]
    pub fn add(&mut self, bucket: usize, delta: &N) {
        let mut i = bucket + 1;
        while i <= self.size {
            self.tree[i].add_assign(delta);
            i += i & i.wrapping_neg();
        }
    }

    /// Prefix sum of buckets [0, bucket] inclusive (0-indexed).
    pub fn prefix_sum(&self, bucket: usize) -> N {
        let mut result = N::default();
        let mut i = bucket + 1;
        while i > 0 {
            result.add_assign(&self.tree[i]);
            i -= i & i.wrapping_neg();
        }
        result
    }

    /// Find the 0-indexed bucket containing the k-th element (0-indexed k).
    ///
    /// `field_fn` extracts the relevant count field from a node.
    /// The value type `V` must support comparison and subtraction
    /// (works with `u32`, `i64`, `i128`).
    #[inline]
    pub fn kth<V, F>(&self, k: V, field_fn: F) -> usize
    where
        V: Copy + PartialOrd + std::ops::SubAssign,
        F: Fn(&N) -> V,
    {
        debug_assert!(self.size > 0);
        let mut pos = 0usize;
        let mut remaining = k;
        let mut bit = 1usize << (usize::BITS - 1 - self.size.leading_zeros());
        while bit > 0 {
            let next = pos + bit;
            if next <= self.size {
                let val = field_fn(&self.tree[next]);
                if remaining >= val {
                    remaining -= val;
                    pos = next;
                }
            }
            bit >>= 1;
        }
        pos // 0-indexed bucket
    }

    /// Batch kth for sorted targets. Processes all targets at each tree level
    /// for better cache locality vs individual kth() calls.
    ///
    /// `sorted_targets` must be sorted ascending. `out` receives the 0-indexed bucket
    /// for each target. Both slices must have the same length.
    #[inline]
    pub fn batch_kth<V, F>(&self, sorted_targets: &[V], field_fn: &F, out: &mut [usize])
    where
        V: Copy + PartialOrd + std::ops::SubAssign,
        F: Fn(&N) -> V,
    {
        let k = sorted_targets.len();
        debug_assert_eq!(out.len(), k);
        debug_assert!(self.size > 0);
        out.fill(0);
        // Copy targets so we can subtract in-place
        let mut remaining: smallvec::SmallVec<[V; 24]> = sorted_targets.into();
        let mut bit = 1usize << (usize::BITS - 1 - self.size.leading_zeros());
        while bit > 0 {
            for i in 0..k {
                let next = out[i] + bit;
                if next <= self.size {
                    let val = field_fn(&self.tree[next]);
                    if remaining[i] >= val {
                        remaining[i] -= val;
                        out[i] = next;
                    }
                }
            }
            bit >>= 1;
        }
    }

    /// Write a raw frequency delta at a bucket. Does NOT maintain the Fenwick invariant.
    /// Call [`build_in_place`] after all raw writes.
    #[inline]
    pub fn add_raw(&mut self, bucket: usize, delta: &N) {
        self.tree[bucket + 1].add_assign(delta);
    }

    /// Convert raw frequencies (written via [`add_raw`]) into a valid Fenwick tree. O(size).
    pub fn build_in_place(&mut self) {
        for i in 1..=self.size {
            let parent = i + (i & i.wrapping_neg());
            if parent <= self.size {
                let child = self.tree[i];
                self.tree[parent].add_assign(&child);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_add_and_prefix_sum() {
        let mut tree = FenwickTree::<u32>::new(10);
        tree.add(0, &3);
        tree.add(1, &2);
        tree.add(5, &7);

        assert_eq!(tree.prefix_sum(0), 3);
        assert_eq!(tree.prefix_sum(1), 5);
        assert_eq!(tree.prefix_sum(4), 5);
        assert_eq!(tree.prefix_sum(5), 12);
        assert_eq!(tree.prefix_sum(9), 12);
    }

    #[test]
    fn kth_walk_down() {
        let mut tree = FenwickTree::<u32>::new(5);
        // freq: [3, 2, 0, 5, 1]
        tree.add(0, &3);
        tree.add(1, &2);
        tree.add(3, &5);
        tree.add(4, &1);

        // kth(0) = first element → bucket 0
        assert_eq!(tree.kth(0u32, |n| *n), 0);
        // kth(2) = 3rd element → bucket 0 (last of bucket 0)
        assert_eq!(tree.kth(2u32, |n| *n), 0);
        // kth(3) = 4th element → bucket 1
        assert_eq!(tree.kth(3u32, |n| *n), 1);
        // kth(4) = 5th element → bucket 1
        assert_eq!(tree.kth(4u32, |n| *n), 1);
        // kth(5) = 6th element → bucket 3 (bucket 2 is empty)
        assert_eq!(tree.kth(5u32, |n| *n), 3);
        // kth(10) = 11th element → bucket 4
        assert_eq!(tree.kth(10u32, |n| *n), 4);
    }

    #[test]
    fn build_in_place_matches_add() {
        let mut tree_add = FenwickTree::<u32>::new(8);
        tree_add.add(0, &5);
        tree_add.add(2, &3);
        tree_add.add(5, &7);
        tree_add.add(7, &1);

        let mut tree_bulk = FenwickTree::<u32>::new(8);
        tree_bulk.add_raw(0, &5);
        tree_bulk.add_raw(2, &3);
        tree_bulk.add_raw(5, &7);
        tree_bulk.add_raw(7, &1);
        tree_bulk.build_in_place();

        for i in 0..8 {
            assert_eq!(
                tree_add.prefix_sum(i),
                tree_bulk.prefix_sum(i),
                "mismatch at bucket {i}"
            );
        }
    }

    #[test]
    fn reset_clears_all() {
        let mut tree = FenwickTree::<u32>::new(10);
        tree.add(3, &42);
        tree.reset();
        assert_eq!(tree.prefix_sum(9), 0);
    }
}
