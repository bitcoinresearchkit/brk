use brk_types::{Sats, StoredU32};
use vecdb::BinaryTransform;

/// (StoredU32, Sats) -> Sats mask
/// Returns value if mask == 1, else 0. Used for pool fee/subsidy from chain data.
pub struct MaskSats;

impl BinaryTransform<StoredU32, Sats, Sats> for MaskSats {
    #[inline(always)]
    fn apply(mask: StoredU32, value: Sats) -> Sats {
        if mask == StoredU32::ONE {
            value
        } else {
            Sats::ZERO
        }
    }
}
