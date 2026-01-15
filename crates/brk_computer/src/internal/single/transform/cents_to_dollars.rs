use brk_types::{Cents, Dollars};
use vecdb::UnaryTransform;

pub struct CentsToDollars;

impl UnaryTransform<Cents, Dollars> for CentsToDollars {
    #[inline(always)]
    fn apply(cents: Cents) -> Dollars {
        Dollars::from(cents)
    }
}
