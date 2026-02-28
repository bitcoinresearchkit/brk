use brk_types::{Cents, Dollars, Sats};
use vecdb::UnaryTransform;

/// CentsUnsigned -> Sats (sats per dollar: 1 BTC / price)
pub struct CentsUnsignedToSats;

impl UnaryTransform<Cents, Sats> for CentsUnsignedToSats {
    #[inline(always)]
    fn apply(cents: Cents) -> Sats {
        let dollars = Dollars::from(cents);
        if dollars == Dollars::ZERO {
            Sats::ZERO
        } else {
            Sats::ONE_BTC / dollars
        }
    }
}
