use brk_types::{Cents, Dollars};
use vecdb::UnaryTransform;

/// Cents -> -Dollars (negate after converting to dollars)
/// Avoids lazy-from-lazy by combining both transforms.
pub struct NegCentsUnsignedToDollars;

impl UnaryTransform<Cents, Dollars> for NegCentsUnsignedToDollars {
    #[inline(always)]
    fn apply(cents: Cents) -> Dollars {
        -Dollars::from(cents)
    }
}
