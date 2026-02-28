use brk_types::{Bitcoin, Sats, SatsSigned};
use vecdb::UnaryTransform;

/// Sats -> Bitcoin (divide by 1e8)
pub struct SatsToBitcoin;

impl UnaryTransform<Sats, Bitcoin> for SatsToBitcoin {
    #[inline(always)]
    fn apply(sats: Sats) -> Bitcoin {
        Bitcoin::from(sats)
    }
}

/// SatsSigned -> Bitcoin (divide by 1e8, preserves sign)
pub struct SatsSignedToBitcoin;

impl UnaryTransform<SatsSigned, Bitcoin> for SatsSignedToBitcoin {
    #[inline(always)]
    fn apply(sats: SatsSigned) -> Bitcoin {
        Bitcoin::from(sats)
    }
}
