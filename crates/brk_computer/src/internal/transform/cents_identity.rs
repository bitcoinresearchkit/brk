use brk_types::Cents;
use vecdb::UnaryTransform;

/// Cents -> Cents (identity transform for lazy references)
pub struct CentsIdentity;

impl UnaryTransform<Cents, Cents> for CentsIdentity {
    #[inline(always)]
    fn apply(cents: Cents) -> Cents {
        cents
    }
}
