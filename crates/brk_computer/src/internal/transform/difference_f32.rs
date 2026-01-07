use brk_types::StoredF32;
use vecdb::BinaryTransform;

/// (StoredF32, StoredF32) -> StoredF32 difference (a - b)
pub struct DifferenceF32;

impl BinaryTransform<StoredF32, StoredF32, StoredF32> for DifferenceF32 {
    #[inline(always)]
    fn apply(a: StoredF32, b: StoredF32) -> StoredF32 {
        StoredF32::from(*a - *b)
    }
}
