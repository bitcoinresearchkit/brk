use brk_types::{Bitcoin, Dollars, Sats};
use vecdb::BinaryTransform;

/// Dollars * Sats -> Dollars/2 (price × sats / 1e8 / 2)
pub struct HalfPriceTimesSats;

impl BinaryTransform<Dollars, Sats, Dollars> for HalfPriceTimesSats {
    #[inline(always)]
    fn apply(price: Dollars, sats: Sats) -> Dollars {
        (price * Bitcoin::from(sats)).halved()
    }
}
