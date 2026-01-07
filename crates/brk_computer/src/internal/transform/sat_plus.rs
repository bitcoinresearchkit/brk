use brk_types::Sats;
use vecdb::BinaryTransform;

/// (Sats, Sats) -> Sats addition
/// Used for computing coinbase = subsidy + fee
pub struct SatsPlus;

impl BinaryTransform<Sats, Sats, Sats> for SatsPlus {
    #[inline(always)]
    fn apply(lhs: Sats, rhs: Sats) -> Sats {
        lhs + rhs
    }
}
