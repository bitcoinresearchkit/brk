use brk_types::{StoredF32, StoredU64, Timestamp};
use vecdb::BinaryTransform;

/// (StoredU64, Timestamp) -> StoredF32 rate (count / interval_seconds)
pub struct PerSec;

impl BinaryTransform<StoredU64, Timestamp, StoredF32> for PerSec {
    #[inline(always)]
    fn apply(count: StoredU64, interval: Timestamp) -> StoredF32 {
        let interval_f64 = f64::from(*interval);
        if interval_f64 > 0.0 {
            StoredF32::from(*count as f64 / interval_f64)
        } else {
            StoredF32::NAN
        }
    }
}
