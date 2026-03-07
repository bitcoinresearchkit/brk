use brk_types::{Bitcoin, Cents, CentsSigned, Dollars, Sats, SatsFract};
use vecdb::{BinaryTransform, UnaryTransform};

pub struct SatsToBitcoin;

impl UnaryTransform<Sats, Bitcoin> for SatsToBitcoin {
    #[inline(always)]
    fn apply(sats: Sats) -> Bitcoin {
        Bitcoin::from(sats)
    }
}

pub struct SatsToCents;

impl BinaryTransform<Sats, Cents, Cents> for SatsToCents {
    #[inline(always)]
    fn apply(sats: Sats, price_cents: Cents) -> Cents {
        Cents::from(sats.as_u128() * price_cents.as_u128() / Sats::ONE_BTC_U128)
    }
}

pub struct CentsUnsignedToDollars;

impl UnaryTransform<Cents, Dollars> for CentsUnsignedToDollars {
    #[inline(always)]
    fn apply(cents: Cents) -> Dollars {
        cents.into()
    }
}

pub struct NegCentsUnsignedToDollars;

impl UnaryTransform<Cents, Dollars> for NegCentsUnsignedToDollars {
    #[inline(always)]
    fn apply(cents: Cents) -> Dollars {
        -Dollars::from(cents)
    }
}

pub struct CentsSignedToDollars;

impl UnaryTransform<CentsSigned, Dollars> for CentsSignedToDollars {
    #[inline(always)]
    fn apply(cents: CentsSigned) -> Dollars {
        cents.into()
    }
}

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

pub struct CentsSubtractToCentsSigned;

impl BinaryTransform<Cents, Cents, CentsSigned> for CentsSubtractToCentsSigned {
    #[inline(always)]
    fn apply(a: Cents, b: Cents) -> CentsSigned {
        CentsSigned::from(a.inner() as i64 - b.inner() as i64)
    }
}

pub struct CentsTimesTenths<const V: u16>;

impl<const V: u16> UnaryTransform<Cents, Cents> for CentsTimesTenths<V> {
    #[inline(always)]
    fn apply(c: Cents) -> Cents {
        Cents::from(c.as_u128() * V as u128 / 10)
    }
}

pub struct DollarsToSatsFract;

impl UnaryTransform<Dollars, SatsFract> for DollarsToSatsFract {
    #[inline(always)]
    fn apply(usd: Dollars) -> SatsFract {
        SatsFract::ONE_BTC / usd
    }
}
