use brk_types::StoredF32;
use vecdb::UnaryTransform;

/// StoredF32 -> StoredF32 (identity transform for lazy references/proxies)
pub struct StoredF32Identity;

impl UnaryTransform<StoredF32, StoredF32> for StoredF32Identity {
    #[inline(always)]
    fn apply(v: StoredF32) -> StoredF32 {
        v
    }
}
