use brk_types::Dollars;
use vecdb::UnaryTransform;

/// Dollars -> Dollars (identity transform for lazy references)
pub struct DollarsIdentity;

impl UnaryTransform<Dollars, Dollars> for DollarsIdentity {
    #[inline(always)]
    fn apply(dollars: Dollars) -> Dollars {
        dollars
    }
}
