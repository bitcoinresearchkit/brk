use brk_types::{Dollars, Sats};
use vecdb::BinaryTransform;

/// Sats × Dollars → Dollars (price * sats)
pub struct SatsToDollars;

impl BinaryTransform<Sats, Dollars, Dollars> for SatsToDollars {
    #[inline(always)]
    fn apply(sats: Sats, price: Dollars) -> Dollars {
        price * sats
    }
}
