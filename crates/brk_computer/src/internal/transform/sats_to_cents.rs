use brk_types::{Cents, Sats};
use vecdb::BinaryTransform;

/// Sats × Cents → Cents (sats × price_cents / 1e8)
/// Uses u128 intermediate to avoid overflow.
pub struct SatsToCents;

impl BinaryTransform<Sats, Cents, Cents> for SatsToCents {
    #[inline(always)]
    fn apply(sats: Sats, price_cents: Cents) -> Cents {
        Cents::from(sats.as_u128() * price_cents.as_u128() / Sats::ONE_BTC_U128)
    }
}
