use brk_types::StoredF32;
use vecdb::UnaryTransform;

/// StoredF32 × sqrt(365) -> StoredF32 (1-year volatility from daily SD)
pub struct StoredF32TimesSqrt365;

impl UnaryTransform<StoredF32, StoredF32> for StoredF32TimesSqrt365 {
    #[inline(always)]
    fn apply(v: StoredF32) -> StoredF32 {
        (*v * 365.0_f32.sqrt()).into()
    }
}
