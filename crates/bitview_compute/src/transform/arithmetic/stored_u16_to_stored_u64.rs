use brk_types::{StoredU16, StoredU64};
use vecdb::UnaryTransform;

pub struct StoredU16ToStoredU64;

impl UnaryTransform<StoredU16, StoredU64> for StoredU16ToStoredU64 {
    #[inline(always)]
    fn apply(value: StoredU16) -> StoredU64 {
        StoredU64::from(u64::from(*value))
    }
}
