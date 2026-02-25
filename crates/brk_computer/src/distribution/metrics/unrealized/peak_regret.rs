use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::Dollars;
use vecdb::{AnyStoredVec, Exit, Rw, StorageMode};

use crate::{ComputeIndexes, internal::ComputedFromHeightLast};

use crate::distribution::metrics::ImportConfig;

/// Unrealized peak regret extension (only for age-based UTXO cohorts).
#[derive(Traversable)]
pub struct UnrealizedPeakRegret<M: StorageMode = Rw> {
    /// Unrealized peak regret: sum of (peak_price - reference_price) x supply
    pub peak_regret: ComputedFromHeightLast<Dollars, M>,
}

impl UnrealizedPeakRegret {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        Ok(Self {
            peak_regret: ComputedFromHeightLast::forced_import(
                cfg.db,
                &cfg.name("unrealized_peak_regret"),
                cfg.version,
                cfg.indexes,
            )?,
        })
    }

    pub(crate) fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        vec![&mut self.peak_regret.height]
    }

    pub(crate) fn compute_from_stateful(
        &mut self,
        starting_indexes: &ComputeIndexes,
        others: &[&Self],
        exit: &Exit,
    ) -> Result<()> {
        self.peak_regret.height.compute_sum_of_others(
            starting_indexes.height,
            &others
                .iter()
                .map(|v| &v.peak_regret.height)
                .collect::<Vec<_>>(),
            exit,
        )?;
        Ok(())
    }
}
