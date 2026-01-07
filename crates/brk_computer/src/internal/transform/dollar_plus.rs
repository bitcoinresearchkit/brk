use brk_types::Dollars;
use vecdb::BinaryTransform;

/// (Dollars, Dollars) -> Dollars addition
/// Used for computing total = profit + loss
pub struct DollarsPlus;

impl BinaryTransform<Dollars, Dollars, Dollars> for DollarsPlus {
    #[inline(always)]
    fn apply(lhs: Dollars, rhs: Dollars) -> Dollars {
        lhs + rhs
    }
}
