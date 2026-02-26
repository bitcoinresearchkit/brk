use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Dollars, Height, StoredF32};
use vecdb::{Exit, ReadableVec, Rw, StorageMode};

use crate::internal::{
    ComputedFromHeightLast, PercentageDollarsF32,
};

use crate::distribution::metrics::ImportConfig;

/// Peak regret relative metric.
#[derive(Traversable)]
pub struct RelativePeakRegret<M: StorageMode = Rw> {
    pub unrealized_peak_regret_rel_to_market_cap:
        ComputedFromHeightLast<StoredF32, M>,
}

impl RelativePeakRegret {
    pub(crate) fn forced_import(
        cfg: &ImportConfig,
    ) -> Result<Self> {
        Ok(Self {
            unrealized_peak_regret_rel_to_market_cap:
                ComputedFromHeightLast::forced_import(
                    cfg.db,
                    &cfg.name("unrealized_peak_regret_rel_to_market_cap"),
                    cfg.version,
                    cfg.indexes,
                )?,
        })
    }

    pub(crate) fn compute(
        &mut self,
        max_from: Height,
        peak_regret: &impl ReadableVec<Height, Dollars>,
        market_cap: &impl ReadableVec<Height, Dollars>,
        exit: &Exit,
    ) -> Result<()> {
        self.unrealized_peak_regret_rel_to_market_cap
            .compute_binary::<Dollars, Dollars, PercentageDollarsF32>(
                max_from, peak_regret, market_cap, exit,
            )
    }
}
