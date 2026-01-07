use brk_types::Dollars;
use vecdb::BinaryTransform;

pub struct DollarsMinus;

impl BinaryTransform<Dollars, Dollars, Dollars> for DollarsMinus {
    #[inline(always)]
    fn apply(lhs: Dollars, rhs: Dollars) -> Dollars {
        lhs - rhs
    }
}
