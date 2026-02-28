use brk_types::{Cents, Dollars};
use vecdb::UnaryTransform;

/// CentsUnsigned -> Dollars (convert cents to dollars for display)
pub struct CentsUnsignedToDollars;

impl UnaryTransform<Cents, Dollars> for CentsUnsignedToDollars {
    #[inline(always)]
    fn apply(cents: Cents) -> Dollars {
        cents.into()
    }
}
