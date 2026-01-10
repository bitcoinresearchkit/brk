use brk_types::{Bitcoin, Close, Dollars, Sats};
use vecdb::BinaryTransform;

/// Sats * Close<Dollars> -> Dollars (sats / 1e8 × price)
/// Same as ClosePriceTimesSats but with swapped argument order.
/// Use when sats is the first source (e.g., Full<Sats>) and price is second.
pub struct SatsTimesClosePrice;

impl BinaryTransform<Sats, Close<Dollars>, Dollars> for SatsTimesClosePrice {
    #[inline(always)]
    fn apply(sats: Sats, price: Close<Dollars>) -> Dollars {
        *price * Bitcoin::from(sats)
    }
}
