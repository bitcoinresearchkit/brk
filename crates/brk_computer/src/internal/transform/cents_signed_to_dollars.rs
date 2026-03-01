use brk_types::{CentsSigned, Dollars};
use vecdb::UnaryTransform;

/// CentsSigned -> Dollars (convert signed cents to dollars for display)
pub struct CentsSignedToDollars;

impl UnaryTransform<CentsSigned, Dollars> for CentsSignedToDollars {
    #[inline(always)]
    fn apply(cents: CentsSigned) -> Dollars {
        cents.into()
    }
}
