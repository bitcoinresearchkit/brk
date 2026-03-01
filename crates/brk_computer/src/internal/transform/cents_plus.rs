use brk_types::Cents;
use vecdb::BinaryTransform;

/// (Cents, Cents) -> Cents addition
/// Used for computing total = profit + loss
pub struct CentsPlus;

impl BinaryTransform<Cents, Cents, Cents> for CentsPlus {
    #[inline(always)]
    fn apply(lhs: Cents, rhs: Cents) -> Cents {
        lhs + rhs
    }
}
