use brk_types::{Dollars, SatsFract};
use vecdb::UnaryTransform;

/// Dollars -> SatsFract (exchange rate: sats per dollar at this price level)
/// Formula: sats = 100_000_000 / usd_price
pub struct DollarsToSatsFract;

impl UnaryTransform<Dollars, SatsFract> for DollarsToSatsFract {
    #[inline(always)]
    fn apply(usd: Dollars) -> SatsFract {
        SatsFract::ONE_BTC / usd
    }
}
