use brk_types::StoredF32;
use vecdb::UnaryTransform;

/// StoredF32 × sqrt(7) -> StoredF32 (1-week volatility from daily SD)
pub struct StoredF32TimesSqrt7;

impl UnaryTransform<StoredF32, StoredF32> for StoredF32TimesSqrt7 {
    #[inline(always)]
    fn apply(v: StoredF32) -> StoredF32 {
        (*v * 7.0_f32.sqrt()).into()
    }
}
