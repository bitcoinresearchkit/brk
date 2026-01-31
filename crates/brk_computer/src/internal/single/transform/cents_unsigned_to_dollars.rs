use brk_types::{CentsUnsigned, Dollars};
use vecdb::UnaryTransform;

/// CentsUnsigned -> Dollars (convert cents to dollars for display)
pub struct CentsUnsignedToDollars;

impl UnaryTransform<CentsUnsigned, Dollars> for CentsUnsignedToDollars {
    #[inline(always)]
    fn apply(cents: CentsUnsigned) -> Dollars {
        cents.into()
    }
}
