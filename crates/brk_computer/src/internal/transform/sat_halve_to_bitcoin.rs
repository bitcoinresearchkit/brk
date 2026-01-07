use brk_types::{Bitcoin, Sats};
use vecdb::UnaryTransform;

/// Sats -> Bitcoin/2 (halve then convert to bitcoin)
/// Avoids lazy-from-lazy by combining both transforms
pub struct HalveSatsToBitcoin;

impl UnaryTransform<Sats, Bitcoin> for HalveSatsToBitcoin {
    #[inline(always)]
    fn apply(sats: Sats) -> Bitcoin {
        Bitcoin::from(sats / 2)
    }
}
