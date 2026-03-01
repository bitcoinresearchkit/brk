use brk_types::{Cents, StoredF32};
use vecdb::BinaryTransform;

/// (Cents, Cents) -> StoredF32 percentage difference ((a/b - 1) * 100)
pub struct PercentageDiffCents;

impl BinaryTransform<Cents, Cents, StoredF32> for PercentageDiffCents {
    #[inline(always)]
    fn apply(close: Cents, base: Cents) -> StoredF32 {
        let base_f64 = f64::from(base);
        if base_f64 == 0.0 {
            StoredF32::default()
        } else {
            StoredF32::from((f64::from(close) / base_f64 - 1.0) * 100.0)
        }
    }
}
