use brk_types::{Bitcoin, StoredF64};
use vecdb::BinaryTransform;

/// (Bitcoin, Bitcoin) -> StoredF64 percentage (a/b × 100)
/// Used for supply ratio calculations like supply_in_profit / total_supply × 100
pub struct PercentageBtcF64;

impl BinaryTransform<Bitcoin, Bitcoin, StoredF64> for PercentageBtcF64 {
    #[inline(always)]
    fn apply(numerator: Bitcoin, denominator: Bitcoin) -> StoredF64 {
        // Bitcoin / Bitcoin returns StoredF64, so dereference and multiply
        StoredF64::from(*(numerator / denominator) * 100.0)
    }
}
