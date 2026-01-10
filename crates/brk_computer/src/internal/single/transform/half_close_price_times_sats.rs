use brk_types::{Bitcoin, Close, Dollars, Sats};
use vecdb::BinaryTransform;

/// Close<Dollars> * Sats -> Dollars/2 (price × sats / 1e8 / 2)
/// Computes halved dollars directly from sats, avoiding lazy-from-lazy chains.
pub struct HalfClosePriceTimesSats;

impl BinaryTransform<Close<Dollars>, Sats, Dollars> for HalfClosePriceTimesSats {
    #[inline(always)]
    fn apply(price: Close<Dollars>, sats: Sats) -> Dollars {
        (*price * Bitcoin::from(sats)).halved()
    }
}
