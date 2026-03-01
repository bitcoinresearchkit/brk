use brk_types::{Cents, CentsSigned};
use vecdb::BinaryTransform;

/// (Cents, Cents) -> CentsSigned (a - b)
/// Produces a signed result from two unsigned inputs.
pub struct CentsSubtractToCentsSigned;

impl BinaryTransform<Cents, Cents, CentsSigned> for CentsSubtractToCentsSigned {
    #[inline(always)]
    fn apply(a: Cents, b: Cents) -> CentsSigned {
        CentsSigned::from(a.inner() as i64 - b.inner() as i64)
    }
}
