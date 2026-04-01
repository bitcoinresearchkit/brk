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
