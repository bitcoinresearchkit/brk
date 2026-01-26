use brk_types::{Close, Dollars, SatsFract};
use vecdb::UnaryTransform;

/// Dollars -> SatsFract (exchange rate: sats per dollar at this price level)
/// Formula: sats = 100_000_000 / usd_price
pub struct DollarsToSatsFract;

impl UnaryTransform<Dollars, SatsFract> for DollarsToSatsFract {
    #[inline(always)]
    fn apply(usd: Dollars) -> SatsFract {
        let usd_f64 = f64::from(usd);
        if usd_f64 == 0.0 {
            SatsFract::NAN
        } else {
            SatsFract::from(SatsFract::SATS_PER_BTC / usd_f64)
        }
    }
}

/// Close<Dollars> -> SatsFract
pub struct CloseDollarsToSatsFract;

impl UnaryTransform<Close<Dollars>, SatsFract> for CloseDollarsToSatsFract {
    #[inline(always)]
    fn apply(usd: Close<Dollars>) -> SatsFract {
        DollarsToSatsFract::apply(*usd)
    }
}
