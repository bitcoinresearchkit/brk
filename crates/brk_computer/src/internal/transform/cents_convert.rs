use brk_types::{Cents, CentsSigned, Dollars, Sats};
use vecdb::UnaryTransform;

/// CentsUnsigned -> Dollars (convert cents to dollars for display)
pub struct CentsUnsignedToDollars;

impl UnaryTransform<Cents, Dollars> for CentsUnsignedToDollars {
    #[inline(always)]
    fn apply(cents: Cents) -> Dollars {
        cents.into()
    }
}

/// Cents -> -Dollars (negate after converting to dollars)
/// Avoids lazy-from-lazy by combining both transforms.
pub struct NegCentsUnsignedToDollars;

impl UnaryTransform<Cents, Dollars> for NegCentsUnsignedToDollars {
    #[inline(always)]
    fn apply(cents: Cents) -> Dollars {
        -Dollars::from(cents)
    }
}

/// CentsSigned -> Dollars (convert signed cents to dollars for display)
pub struct CentsSignedToDollars;

impl UnaryTransform<CentsSigned, Dollars> for CentsSignedToDollars {
    #[inline(always)]
    fn apply(cents: CentsSigned) -> Dollars {
        cents.into()
    }
}

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
