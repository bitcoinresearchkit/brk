use brk_types::Sats;
use vecdb::UnaryTransform;

/// Sats -> Sats/2 (for supply_halved)
pub struct HalveSats;

impl UnaryTransform<Sats, Sats> for HalveSats {
    #[inline(always)]
    fn apply(sats: Sats) -> Sats {
        sats / 2
    }
}
