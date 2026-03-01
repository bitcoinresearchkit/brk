use brk_types::Cents;
use vecdb::UnaryTransform;

/// Cents -> Cents/2 (for supply_halved_cents)
pub struct HalveCents;

impl UnaryTransform<Cents, Cents> for HalveCents {
    #[inline(always)]
    fn apply(cents: Cents) -> Cents {
        cents / 2u64
    }
}
