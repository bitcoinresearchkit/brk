use brk_types::Sats;
use vecdb::UnaryTransform;

/// Sats -> Sats (identity transform for lazy references)
pub struct SatsIdentity;

impl UnaryTransform<Sats, Sats> for SatsIdentity {
    #[inline(always)]
    fn apply(sats: Sats) -> Sats {
        sats
    }
}
