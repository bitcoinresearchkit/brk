use std::{iter::Sum, ops::Div};

use brk_types::{Cents, Dollars, Height, Timestamp};

use super::block_window::round_half_up;

/// One time-bucket of blocks in a `BlockWindow`.
pub struct BlockBucket {
    pub avg_height: Height,
    pub avg_timestamp: Timestamp,
    /// Offsets into the parent `BlockWindow`'s prefetched `[start, end)` slice.
    offsets: Vec<usize>,
}

impl BlockBucket {
    pub fn new(avg_height: Height, avg_timestamp: Timestamp, offsets: Vec<usize>) -> Self {
        Self {
            avg_height,
            avg_timestamp,
            offsets,
        }
    }

    /// Arithmetic mean of `values[offset]` across this bucket's blocks.
    pub fn mean<T>(&self, values: &[T]) -> T
    where
        T: Copy + Sum + Div<usize, Output = T>,
    {
        self.offsets.iter().map(|&i| values[i]).sum::<T>() / self.offsets.len()
    }

    /// Round-half-up arithmetic mean for integer wrapper types convertible
    /// through `u64`.
    pub fn mean_rounded<T>(&self, values: &[T]) -> T
    where
        T: Copy + From<u64>,
        u64: From<T>,
    {
        let n = self.offsets.len() as u128;
        let sum: u128 = self
            .offsets
            .iter()
            .map(|&index| u128::from(u64::from(values[index])))
            .sum();
        T::from(round_half_up(sum, n))
    }

    /// Preserve the price source's unavailable sentinel instead of converting
    /// it to an integer (which panics). Finite averages retain cent rounding.
    pub fn mean_price(&self, prices: &[Cents]) -> Dollars {
        let sum = self.offsets.iter().try_fold(0u128, |sum, &index| {
            prices[index]
                .finite_inner()
                .map(|value| sum + u128::from(value))
        });
        sum.map(|sum| Dollars::from(Cents::from(round_half_up(sum, self.offsets.len() as u128))))
            .unwrap_or(Dollars::NAN)
    }
}
