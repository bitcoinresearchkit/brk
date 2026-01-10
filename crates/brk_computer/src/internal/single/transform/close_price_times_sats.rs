use brk_types::{Bitcoin, Close, Dollars, Sats};
use vecdb::BinaryTransform;

/// Close<Dollars> * Sats -> Dollars (price × sats / 1e8)
/// Same as PriceTimesSats but accepts Close<Dollars> price source.
pub struct ClosePriceTimesSats;

impl BinaryTransform<Close<Dollars>, Sats, Dollars> for ClosePriceTimesSats {
    #[inline(always)]
    fn apply(price: Close<Dollars>, sats: Sats) -> Dollars {
        *price * Bitcoin::from(sats)
    }
}
