use brk_types::StoredF32;
use vecdb::UnaryTransform;

/// StoredF32 × sqrt(30) -> StoredF32 (1-month volatility from daily SD)
pub struct StoredF32TimesSqrt30;

impl UnaryTransform<StoredF32, StoredF32> for StoredF32TimesSqrt30 {
    #[inline(always)]
    fn apply(v: StoredF32) -> StoredF32 {
        // 30.0_f32.sqrt() = 5.477226
        (*v * 5.477226_f32).into()
    }
}
