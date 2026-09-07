use brk_types::StoredF64;
use vecdb::UnaryTransform;

pub struct OddsF64;

impl UnaryTransform<StoredF64, StoredF64> for OddsF64 {
    #[inline(always)]
    fn apply(value: StoredF64) -> StoredF64 {
        value / StoredF64::from(1.0 - *value)
    }
}
