use brk_types::{Close, High, Low, OHLCCents, OHLCSats, Open};
use vecdb::UnaryTransform;

use super::CentsUnsignedToSats;

/// OHLCCents -> OHLCSats with high/low swapped (inverse price relationship).
pub struct OhlcCentsToSats;

impl UnaryTransform<OHLCCents, OHLCSats> for OhlcCentsToSats {
    #[inline(always)]
    fn apply(cents: OHLCCents) -> OHLCSats {
        OHLCSats {
            open: Open::new(CentsUnsignedToSats::apply(*cents.open)),
            high: High::new(CentsUnsignedToSats::apply(*cents.low)),
            low: Low::new(CentsUnsignedToSats::apply(*cents.high)),
            close: Close::new(CentsUnsignedToSats::apply(*cents.close)),
        }
    }
}
