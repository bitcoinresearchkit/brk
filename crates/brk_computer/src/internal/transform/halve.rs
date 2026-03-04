use brk_types::{Bitcoin, Cents, Dollars, Sats};
use vecdb::UnaryTransform;

/// Sats -> Sats/2 (for supply_halved)
pub struct HalveSats;

impl UnaryTransform<Sats, Sats> for HalveSats {
    #[inline(always)]
    fn apply(sats: Sats) -> Sats {
        sats / 2
    }
}

/// Sats -> Bitcoin/2 (halve then convert to bitcoin)
/// Avoids lazy-from-lazy by combining both transforms
pub struct HalveSatsToBitcoin;

impl UnaryTransform<Sats, Bitcoin> for HalveSatsToBitcoin {
    #[inline(always)]
    fn apply(sats: Sats) -> Bitcoin {
        Bitcoin::from(sats / 2)
    }
}

/// Cents -> Cents/2 (for supply_halved_cents)
pub struct HalveCents;

impl UnaryTransform<Cents, Cents> for HalveCents {
    #[inline(always)]
    fn apply(cents: Cents) -> Cents {
        cents / 2u64
    }
}

/// Dollars -> Dollars/2 (for supply_halved_usd)
pub struct HalveDollars;

impl UnaryTransform<Dollars, Dollars> for HalveDollars {
    #[inline(always)]
    fn apply(dollars: Dollars) -> Dollars {
        dollars.halved()
    }
}
