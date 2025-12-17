//! Fenwick Tree (Binary Indexed Tree) for O(log n) prefix sums.
//!
//! Used for efficient percentile computation over price distributions.

/// Fenwick tree for O(log n) prefix sum queries and updates.
///
/// Supports:
/// - `add(idx, delta)`: O(log n) - add delta to position idx
/// - `prefix_sum(idx)`: O(log n) - sum of elements 0..=idx
/// - `lower_bound(target)`: O(log n) - find smallest idx where prefix_sum >= target
#[derive(Clone, Debug)]
pub struct FenwickTree {
    tree: Vec<u64>,
    len: usize,
}

impl FenwickTree {
    /// Create a new Fenwick tree with given capacity.
    pub fn new(len: usize) -> Self {
        Self {
            tree: vec![0; len + 1], // 1-indexed
            len,
        }
    }

    /// Add delta to position idx. O(log n).
    pub fn add(&mut self, idx: usize, delta: u64) {
        let mut i = idx + 1; // Convert to 1-indexed
        while i <= self.len {
            self.tree[i] += delta;
            i += i & i.wrapping_neg(); // Add LSB
        }
    }

    /// Subtract delta from position idx. O(log n).
    pub fn sub(&mut self, idx: usize, delta: u64) {
        let mut i = idx + 1;
        while i <= self.len {
            self.tree[i] -= delta;
            i += i & i.wrapping_neg();
        }
    }

    /// Get prefix sum of elements 0..=idx. O(log n).
    pub fn prefix_sum(&self, idx: usize) -> u64 {
        let mut sum = 0u64;
        let mut i = idx + 1; // Convert to 1-indexed
        while i > 0 {
            sum += self.tree[i];
            i -= i & i.wrapping_neg(); // Remove LSB
        }
        sum
    }

    /// Find smallest index where prefix_sum >= target. O(log n).
    /// Returns None if no such index exists (target > total sum).
    pub fn lower_bound(&self, target: u64) -> Option<usize> {
        if target == 0 {
            return Some(0);
        }

        let mut sum = 0u64;
        let mut pos = 0usize;

        // Find highest bit position
        let mut bit = 1usize << (usize::BITS - 1 - self.len.leading_zeros());

        while bit > 0 {
            let next_pos = pos + bit;
            if next_pos <= self.len && sum + self.tree[next_pos] < target {
                sum += self.tree[next_pos];
                pos = next_pos;
            }
            bit >>= 1;
        }

        // pos is now the largest index where prefix_sum < target
        // So pos + 1 is the smallest where prefix_sum >= target
        if pos < self.len {
            Some(pos) // Convert back to 0-indexed
        } else {
            None
        }
    }

    /// Get total sum of all elements. O(log n).
    pub fn total(&self) -> u64 {
        self.prefix_sum(self.len.saturating_sub(1))
    }

    /// Reset all values to zero. O(n).
    pub fn clear(&mut self) {
        self.tree.fill(0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_operations() {
        let mut ft = FenwickTree::new(10);

        ft.add(0, 5);
        ft.add(2, 3);
        ft.add(5, 7);

        assert_eq!(ft.prefix_sum(0), 5);
        assert_eq!(ft.prefix_sum(1), 5);
        assert_eq!(ft.prefix_sum(2), 8);
        assert_eq!(ft.prefix_sum(5), 15);
        assert_eq!(ft.total(), 15);
    }

    #[test]
    fn test_lower_bound() {
        let mut ft = FenwickTree::new(10);

        ft.add(0, 10);
        ft.add(2, 20);
        ft.add(5, 30);

        assert_eq!(ft.lower_bound(5), Some(0));
        assert_eq!(ft.lower_bound(10), Some(0));
        assert_eq!(ft.lower_bound(11), Some(2));
        assert_eq!(ft.lower_bound(30), Some(2));
        assert_eq!(ft.lower_bound(31), Some(5));
        assert_eq!(ft.lower_bound(60), Some(5));
        assert_eq!(ft.lower_bound(61), None);
    }
}
