use brk_types::{Bitcoin, Dollars, Sats};
use vecdb::BinaryTransform;

/// Sats * Dollars -> Dollars (sats / 1e8 × price)
pub struct SatsTimesPrice;

impl BinaryTransform<Sats, Dollars, Dollars> for SatsTimesPrice {
    #[inline(always)]
    fn apply(sats: Sats, price: Dollars) -> Dollars {
        price * Bitcoin::from(sats)
    }
}
