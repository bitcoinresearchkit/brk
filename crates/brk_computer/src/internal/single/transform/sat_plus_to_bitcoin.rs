use brk_types::{Bitcoin, Sats};
use vecdb::BinaryTransform;

/// (Sats, Sats) -> Bitcoin addition with conversion
/// Used for computing coinbase_btc = (subsidy + fee) / 1e8
pub struct SatsPlusToBitcoin;

impl BinaryTransform<Sats, Sats, Bitcoin> for SatsPlusToBitcoin {
    #[inline(always)]
    fn apply(lhs: Sats, rhs: Sats) -> Bitcoin {
        Bitcoin::from(lhs + rhs)
    }
}
