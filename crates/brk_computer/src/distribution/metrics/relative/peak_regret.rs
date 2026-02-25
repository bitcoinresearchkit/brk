use brk_traversable::Traversable;
use brk_types::{Dollars, Sats, StoredF32};

use crate::internal::{
    ComputedFromHeightLast, LazyBinaryComputedFromHeightLast, LazyBinaryFromHeightLast,
    PercentageDollarsF32,
};

use crate::distribution::metrics::ImportConfig;

/// Peak regret relative metric.
#[derive(Clone, Traversable)]
pub struct RelativePeakRegret {
    pub unrealized_peak_regret_rel_to_market_cap:
        LazyBinaryFromHeightLast<StoredF32, Dollars, Dollars>,
}

impl RelativePeakRegret {
    pub(crate) fn forced_import(
        cfg: &ImportConfig,
        peak_regret: &ComputedFromHeightLast<Dollars>,
        market_cap: &LazyBinaryComputedFromHeightLast<Dollars, Sats, Dollars>,
    ) -> Self {
        Self {
            unrealized_peak_regret_rel_to_market_cap:
                LazyBinaryFromHeightLast::from_block_last_and_lazy_binary_computed_block_last::<
                    PercentageDollarsF32, _, _,
                >(
                    &cfg.name("unrealized_peak_regret_rel_to_market_cap"),
                    cfg.version,
                    peak_regret,
                    market_cap,
                ),
        }
    }
}
