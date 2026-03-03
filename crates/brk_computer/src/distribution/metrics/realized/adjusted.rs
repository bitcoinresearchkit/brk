use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Cents, Height, StoredF64, Version};
use vecdb::{Exit, ReadableVec, Rw, StorageMode};

use crate::{
    ComputeIndexes, blocks,
    internal::{ComputedFromHeight, RatioCents64, RollingEmas1w1m, RollingWindows},
};

use crate::distribution::metrics::ImportConfig;

/// Adjusted realized metrics (only for adjusted cohorts: all, sth, max_age).
#[derive(Traversable)]
pub struct RealizedAdjusted<M: StorageMode = Rw> {
    // === Adjusted Value (computed: cohort - up_to_1h) ===
    pub adjusted_value_created: ComputedFromHeight<Cents, M>,
    pub adjusted_value_destroyed: ComputedFromHeight<Cents, M>,

    // === Adjusted Value Created/Destroyed Rolling Sums ===
    pub adjusted_value_created_sum: RollingWindows<Cents, M>,
    pub adjusted_value_destroyed_sum: RollingWindows<Cents, M>,

    // === Adjusted SOPR (rolling window ratios) ===
    pub adjusted_sopr: RollingWindows<StoredF64, M>,
    pub adjusted_sopr_ema: RollingEmas1w1m<StoredF64, M>,
}

impl RealizedAdjusted {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v1 = Version::ONE;

        let adjusted_value_created = ComputedFromHeight::forced_import(
            cfg.db,
            &cfg.name("adjusted_value_created"),
            cfg.version,
            cfg.indexes,
        )?;
        let adjusted_value_destroyed = ComputedFromHeight::forced_import(
            cfg.db,
            &cfg.name("adjusted_value_destroyed"),
            cfg.version,
            cfg.indexes,
        )?;

        let adjusted_value_created_sum = RollingWindows::forced_import(
            cfg.db, &cfg.name("adjusted_value_created"), cfg.version + v1, cfg.indexes,
        )?;
        let adjusted_value_destroyed_sum = RollingWindows::forced_import(
            cfg.db, &cfg.name("adjusted_value_destroyed"), cfg.version + v1, cfg.indexes,
        )?;
        let adjusted_sopr = RollingWindows::forced_import(
            cfg.db, &cfg.name("adjusted_sopr"), cfg.version + v1, cfg.indexes,
        )?;
        let adjusted_sopr_ema = RollingEmas1w1m::forced_import(
            cfg.db, &cfg.name("adjusted_sopr_24h"), cfg.version + v1, cfg.indexes,
        )?;

        Ok(RealizedAdjusted {
            adjusted_value_created,
            adjusted_value_destroyed,
            adjusted_value_created_sum,
            adjusted_value_destroyed_sum,
            adjusted_sopr,
            adjusted_sopr_ema,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn compute_rest_part2_adj(
        &mut self,
        blocks: &blocks::Vecs,
        starting_indexes: &ComputeIndexes,
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
            starting_indexes.height, &window_starts, &self.adjusted_value_created.height, exit,
        )?;
        self.adjusted_value_destroyed_sum.compute_rolling_sum(
            starting_indexes.height, &window_starts, &self.adjusted_value_destroyed.height, exit,
        )?;

        // SOPR ratios from rolling sums
        for ((sopr, vc), vd) in self.adjusted_sopr.as_mut_array().into_iter()
            .zip(self.adjusted_value_created_sum.as_array())
            .zip(self.adjusted_value_destroyed_sum.as_array())
        {
            sopr.compute_binary::<Cents, Cents, RatioCents64>(
                starting_indexes.height, &vc.height, &vd.height, exit,
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
