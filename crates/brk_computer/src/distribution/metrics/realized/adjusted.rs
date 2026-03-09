use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Cents, Height, Indexes, StoredF64, Version};
use vecdb::{Exit, ReadableVec, Rw, StorageMode};

use crate::{
    blocks,
    internal::{ComputedFromHeight, RatioCents64, RollingWindows},
};

use crate::distribution::metrics::ImportConfig;

#[derive(Traversable)]
pub struct RealizedAdjusted<M: StorageMode = Rw> {
    pub value_created: ComputedFromHeight<Cents, M>,
    pub value_destroyed: ComputedFromHeight<Cents, M>,
    pub value_created_sum: RollingWindows<Cents, M>,
    pub value_destroyed_sum: RollingWindows<Cents, M>,
    pub sopr: RollingWindows<StoredF64, M>,
}

impl RealizedAdjusted {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        Ok(RealizedAdjusted {
            value_created: cfg.import("adjusted_value_created", Version::ZERO)?,
            value_destroyed: cfg.import("adjusted_value_destroyed", Version::ZERO)?,
            value_created_sum: cfg.import("adjusted_value_created", Version::ONE)?,
            value_destroyed_sum: cfg.import("adjusted_value_destroyed", Version::ONE)?,
            sopr: cfg.import("adjusted_sopr", Version::ONE)?,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn compute_rest_part2(
        &mut self,
        blocks: &blocks::Vecs,
        starting_indexes: &Indexes,
        base_value_created: &impl ReadableVec<Height, Cents>,
        base_value_destroyed: &impl ReadableVec<Height, Cents>,
        up_to_1h_value_created: &impl ReadableVec<Height, Cents>,
        up_to_1h_value_destroyed: &impl ReadableVec<Height, Cents>,
        exit: &Exit,
    ) -> Result<()> {
        // Compute value_created = base.value_created - up_to_1h.value_created
        self.value_created.height.compute_subtract(
            starting_indexes.height,
            base_value_created,
            up_to_1h_value_created,
            exit,
        )?;
        self.value_destroyed.height.compute_subtract(
            starting_indexes.height,
            base_value_destroyed,
            up_to_1h_value_destroyed,
            exit,
        )?;

        // Adjusted value created/destroyed rolling sums
        let window_starts = blocks.lookback.window_starts();
        self.value_created_sum.compute_rolling_sum(
            starting_indexes.height,
            &window_starts,
            &self.value_created.height,
            exit,
        )?;
        self.value_destroyed_sum.compute_rolling_sum(
            starting_indexes.height,
            &window_starts,
            &self.value_destroyed.height,
            exit,
        )?;

        // SOPR ratios from rolling sums
        for ((sopr, vc), vd) in self
            .sopr
            .as_mut_array()
            .into_iter()
            .zip(self.value_created_sum.as_array())
            .zip(self.value_destroyed_sum.as_array())
        {
            sopr.compute_binary::<Cents, Cents, RatioCents64>(
                starting_indexes.height,
                &vc.height,
                &vd.height,
                exit,
            )?;
        }

        Ok(())
    }
}
