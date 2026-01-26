use brk_types::StoredU64;
use vecdb::BinaryTransform;

/// (StoredU64, StoredU64) -> StoredU64 addition
/// Used for computing total_addr_count = addr_count + empty_addr_count
pub struct U64Plus;

impl BinaryTransform<StoredU64, StoredU64, StoredU64> for U64Plus {
    #[inline(always)]
    fn apply(lhs: StoredU64, rhs: StoredU64) -> StoredU64 {
        StoredU64::from(u64::from(lhs) + u64::from(rhs))
    }
}
