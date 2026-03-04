use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Cents, Height, Indexes, StoredF64, Version};
use vecdb::{Exit, ReadableVec, Rw, StorageMode};

use crate::{
    blocks,
    internal::{ComputedFromHeight, RatioCents64, RollingEmas1w1m, RollingWindows},
};

use crate::distribution::metrics::ImportConfig;

#[derive(Traversable)]
pub struct RealizedAdjusted<M: StorageMode = Rw> {
    pub adjusted_value_created: ComputedFromHeight<Cents, M>,
    pub adjusted_value_destroyed: ComputedFromHeight<Cents, M>,

    pub adjusted_value_created_sum: RollingWindows<Cents, M>,
    pub adjusted_value_destroyed_sum: RollingWindows<Cents, M>,

    pub adjusted_sopr: RollingWindows<StoredF64, M>,
    pub adjusted_sopr_ema: RollingEmas1w1m<StoredF64, M>,
}

impl RealizedAdjusted {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        Ok(RealizedAdjusted {
            adjusted_value_created: cfg.import_computed("adjusted_value_created", Version::ZERO)?,
            adjusted_value_destroyed: cfg
                .import_computed("adjusted_value_destroyed", Version::ZERO)?,
            adjusted_value_created_sum: cfg
                .import_rolling("adjusted_value_created", Version::ONE)?,
            adjusted_value_destroyed_sum: cfg
                .import_rolling("adjusted_value_destroyed", Version::ONE)?,
            adjusted_sopr: cfg.import_rolling("adjusted_sopr", Version::ONE)?,
            adjusted_sopr_ema: cfg.import_emas_1w_1m("adjusted_sopr_24h", Version::ONE)?,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn compute_rest_part2_adj(
        &mut self,
        blocks: &blocks::Vecs,
        starting_indexes: &Indexes,
        base_value_created: &impl ReadableVec<Height, Cents>,
        base_value_destroyed: &impl ReadableVec<Height, Cents>,
        up_to_1h_value_created: &impl ReadableVec<Height, Cents>,
        up_to_1h_value_destroyed: &impl ReadableVec<Height, Cents>,
        exit: &Exit,
    ) -> Result<()> {
        // Compute adjusted_value_created = base.value_created - up_to_1h.value_created
        self.adjusted_value_created.height.compute_subtract(
            starting_indexes.height,
            base_value_created,
            up_to_1h_value_created,
            exit,
        )?;
        self.adjusted_value_destroyed.height.compute_subtract(
            starting_indexes.height,
            base_value_destroyed,
            up_to_1h_value_destroyed,
            exit,
        )?;

        // Adjusted value created/destroyed rolling sums
        let window_starts = blocks.count.window_starts();
        self.adjusted_value_created_sum.compute_rolling_sum(
            starting_indexes.height,
            &window_starts,
            &self.adjusted_value_created.height,
            exit,
        )?;
        self.adjusted_value_destroyed_sum.compute_rolling_sum(
            starting_indexes.height,
            &window_starts,
            &self.adjusted_value_destroyed.height,
            exit,
        )?;

        // SOPR ratios from rolling sums
        for ((sopr, vc), vd) in self
            .adjusted_sopr
            .as_mut_array()
            .into_iter()
            .zip(self.adjusted_value_created_sum.as_array())
            .zip(self.adjusted_value_destroyed_sum.as_array())
        {
            sopr.compute_binary::<Cents, Cents, RatioCents64>(
                starting_indexes.height,
                &vc.height,
                &vd.height,
                exit,
            )?;
        }

        // Adjusted SOPR EMAs (based on 24h window)
        self.adjusted_sopr_ema.compute_from_24h(
            starting_indexes.height,
            &blocks.count.height_1w_ago,
            &blocks.count.height_1m_ago,
            &self.adjusted_sopr._24h.height,
            exit,
        )?;

        Ok(())
    }
}
