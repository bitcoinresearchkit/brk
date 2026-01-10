use brk_types::{Bitcoin, Sats};
use vecdb::UnaryTransform;

/// Sats -> Bitcoin (divide by 1e8)
pub struct SatsToBitcoin;

impl UnaryTransform<Sats, Bitcoin> for SatsToBitcoin {
    #[inline(always)]
    fn apply(sats: Sats) -> Bitcoin {
        Bitcoin::from(sats)
    }
}
