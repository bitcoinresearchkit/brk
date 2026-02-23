use brk_types::StoredF32;
use vecdb::UnaryTransform;

/// StoredF32 × sqrt(365) -> StoredF32 (1-year volatility from daily SD)
pub struct StoredF32TimesSqrt365;

impl UnaryTransform<StoredF32, StoredF32> for StoredF32TimesSqrt365 {
    #[inline(always)]
    fn apply(v: StoredF32) -> StoredF32 {
        // 365.0_f32.sqrt() = 19.104973
        (*v * 19.104973_f32).into()
    }
}
