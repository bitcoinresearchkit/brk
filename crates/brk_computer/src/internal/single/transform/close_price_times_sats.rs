use brk_types::{Bitcoin, Dollars, Sats};
use vecdb::BinaryTransform;

/// Dollars * Sats -> Dollars (price × sats / 1e8)
pub struct PriceTimesSats;

impl BinaryTransform<Dollars, Sats, Dollars> for PriceTimesSats {
    #[inline(always)]
    fn apply(price: Dollars, sats: Sats) -> Dollars {
        price * Bitcoin::from(sats)
    }
}
