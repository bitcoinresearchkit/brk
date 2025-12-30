use std::ops::{Add, Div};

/// Standard percentile values used throughout BRK.
pub const PERCENTILES: [u8; 19] = [
    5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 55, 60, 65, 70, 75, 80, 85, 90, 95,
];

/// Length of the PERCENTILES array.
pub const PERCENTILES_LEN: usize = PERCENTILES.len();

/// Get a percentile value from a sorted slice.
///
/// # Panics
/// Panics if the slice is empty.
pub fn get_percentile<T>(sorted: &[T], percentile: f64) -> T
where
    T: Clone + Div<usize, Output = T> + Add<T, Output = T>,
{
    let len = sorted.len();

    if len == 0 {
        panic!("Cannot get percentile from empty slice");
    } else if len == 1 {
        sorted[0].clone()
    } else {
        let index = (len - 1) as f64 * percentile;

        let fract = index.fract();

        if fract != 0.0 {
            let left = sorted.get(index as usize).unwrap().clone();
            let right = sorted.get(index.ceil() as usize).unwrap().clone();
            left / 2 + right / 2
        } else {
            sorted.get(index as usize).unwrap().clone()
        }
    }
}
