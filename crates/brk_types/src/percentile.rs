use crate::VSize;

/// Standard percentile values used throughout BRK.
pub const PERCENTILES: [u8; 19] = [
    5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 55, 60, 65, 70, 75, 80, 85, 90, 95,
];

/// Length of the PERCENTILES array.
pub const PERCENTILES_LEN: usize = PERCENTILES.len();

/// Get a percentile value from a sorted slice using nearest-rank method.
///
/// # Panics
/// Panics if the slice is empty.
pub fn get_percentile<T: Clone>(sorted: &[T], percentile: f64) -> T {
    let len = sorted.len();
    assert!(len > 0, "Cannot get percentile from empty slice");
    let index = ((len - 1) as f64 * percentile).round() as usize;
    sorted[index].clone()
}

/// Get a percentile value from a sorted (value, vsize) slice using
/// vsize-weighted interpolation — matches mempool.space's feeRange calculation.
///
/// Walks through the sorted pairs accumulating vsize. When cumulative vsize
/// crosses `total_vsize * percentile`, returns that value.
///
/// # Panics
/// Panics if the slice is empty.
pub fn get_weighted_percentile<T: Clone>(sorted_with_vsizes: &[(T, VSize)], percentile: f64) -> T {
    assert!(
        !sorted_with_vsizes.is_empty(),
        "Cannot get percentile from empty slice"
    );
    let total: u64 = sorted_with_vsizes.iter().map(|(_, v)| u64::from(*v)).sum();
    let target = (total as f64 * percentile).round() as u64;
    let mut cumulative = 0u64;
    for (value, vsize) in sorted_with_vsizes {
        cumulative += u64::from(*vsize);
        if cumulative >= target {
            return value.clone();
        }
    }
    sorted_with_vsizes.last().unwrap().0.clone()
}
