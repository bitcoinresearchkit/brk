use brk_types::StoredF32;
use vecdb::BinaryTransform;

/// (StoredF32, StoredF32) -> StoredF32 RSI formula: 100 * a / (a + b)
pub struct RsiFormula;

impl BinaryTransform<StoredF32, StoredF32, StoredF32> for RsiFormula {
    #[inline(always)]
    fn apply(average_gain: StoredF32, average_loss: StoredF32) -> StoredF32 {
        let sum = *average_gain + *average_loss;
        if sum == 0.0 {
            StoredF32::from(50.0)
        } else {
            StoredF32::from(100.0 * *average_gain / sum)
        }
    }
}
