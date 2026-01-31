use brk_types::{CentsUnsigned, SatsFract};
use vecdb::UnaryTransform;

/// CentsUnsigned -> SatsFract (exchange rate: sats per dollar at this price level)
/// Formula: sats = 100_000_000 / dollars = 100_000_000 / (cents / 100) = 10_000_000_000 / cents
pub struct CentsUnsignedToSatsFract;

impl UnaryTransform<CentsUnsigned, SatsFract> for CentsUnsignedToSatsFract {
    #[inline(always)]
    fn apply(cents: CentsUnsigned) -> SatsFract {
        let cents_f64 = cents.inner() as f64;
        if cents_f64 == 0.0 {
            SatsFract::NAN
        } else {
            // sats = 1 BTC * 100 / cents = 10_000_000_000 / cents
            SatsFract::new(SatsFract::SATS_PER_BTC * 100.0 / cents_f64)
        }
    }
}
