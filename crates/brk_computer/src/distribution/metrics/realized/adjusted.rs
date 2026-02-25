use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Dollars, StoredF64, Version};
use vecdb::{Exit, Ident, ReadableCloneableVec, Rw, StorageMode};

use crate::{
    ComputeIndexes, blocks,
    internal::{
        ComputedFromHeightLast, DollarsMinus, LazyBinaryFromHeightLast,
        LazyFromHeightLast, Ratio64,
    },
};

use crate::distribution::metrics::ImportConfig;

use super::RealizedBase;

/// Adjusted realized metrics (only for adjusted cohorts: all, sth, max_age).
#[derive(Traversable)]
pub struct RealizedAdjusted<M: StorageMode = Rw> {
    // === Adjusted Value (lazy: cohort - up_to_1h) ===
    pub adjusted_value_created: LazyBinaryFromHeightLast<Dollars, Dollars, Dollars>,
    pub adjusted_value_destroyed: LazyBinaryFromHeightLast<Dollars, Dollars, Dollars>,

    // === Adjusted Value Created/Destroyed Rolling Sums ===
    pub adjusted_value_created_24h: ComputedFromHeightLast<Dollars, M>,
    pub adjusted_value_created_7d: ComputedFromHeightLast<Dollars, M>,
    pub adjusted_value_created_30d: ComputedFromHeightLast<Dollars, M>,
    pub adjusted_value_created_1y: ComputedFromHeightLast<Dollars, M>,
    pub adjusted_value_destroyed_24h: ComputedFromHeightLast<Dollars, M>,
    pub adjusted_value_destroyed_7d: ComputedFromHeightLast<Dollars, M>,
    pub adjusted_value_destroyed_30d: ComputedFromHeightLast<Dollars, M>,
    pub adjusted_value_destroyed_1y: ComputedFromHeightLast<Dollars, M>,

    // === Adjusted SOPR (rolling window ratios) ===
    pub adjusted_sopr: LazyFromHeightLast<StoredF64>,
    pub adjusted_sopr_24h: LazyBinaryFromHeightLast<StoredF64, Dollars, Dollars>,
    pub adjusted_sopr_7d: LazyBinaryFromHeightLast<StoredF64, Dollars, Dollars>,
    pub adjusted_sopr_30d: LazyBinaryFromHeightLast<StoredF64, Dollars, Dollars>,
    pub adjusted_sopr_1y: LazyBinaryFromHeightLast<StoredF64, Dollars, Dollars>,
    pub adjusted_sopr_24h_7d_ema: ComputedFromHeightLast<StoredF64, M>,
    pub adjusted_sopr_7d_ema: LazyFromHeightLast<StoredF64>,
    pub adjusted_sopr_24h_30d_ema: ComputedFromHeightLast<StoredF64, M>,
    pub adjusted_sopr_30d_ema: LazyFromHeightLast<StoredF64>,
}

impl RealizedAdjusted {
    pub(crate) fn forced_import(
        cfg: &ImportConfig,
        base: &RealizedBase,
        up_to_1h: &RealizedBase,
    ) -> Result<Self> {
        let v1 = Version::ONE;

        macro_rules! import_rolling {
            ($name:expr) => {
                ComputedFromHeightLast::forced_import(cfg.db, &cfg.name($name), cfg.version + v1, cfg.indexes)?
            };
        }

        let adjusted_value_created = LazyBinaryFromHeightLast::from_both_binary_block::<
            DollarsMinus, Dollars, Dollars, Dollars, Dollars,
        >(
            &cfg.name("adjusted_value_created"),
            cfg.version,
            &base.value_created,
            &up_to_1h.value_created,
        );
        let adjusted_value_destroyed = LazyBinaryFromHeightLast::from_both_binary_block::<
            DollarsMinus, Dollars, Dollars, Dollars, Dollars,
        >(
            &cfg.name("adjusted_value_destroyed"),
            cfg.version,
            &base.value_destroyed,
            &up_to_1h.value_destroyed,
        );

        let adjusted_value_created_24h = import_rolling!("adjusted_value_created_24h");
        let adjusted_value_created_7d = import_rolling!("adjusted_value_created_7d");
        let adjusted_value_created_30d = import_rolling!("adjusted_value_created_30d");
        let adjusted_value_created_1y = import_rolling!("adjusted_value_created_1y");
        let adjusted_value_destroyed_24h = import_rolling!("adjusted_value_destroyed_24h");
        let adjusted_value_destroyed_7d = import_rolling!("adjusted_value_destroyed_7d");
        let adjusted_value_destroyed_30d = import_rolling!("adjusted_value_destroyed_30d");
        let adjusted_value_destroyed_1y = import_rolling!("adjusted_value_destroyed_1y");

        let adjusted_sopr_24h = LazyBinaryFromHeightLast::from_computed_last::<Ratio64>(
            &cfg.name("adjusted_sopr_24h"), cfg.version + v1, &adjusted_value_created_24h, &adjusted_value_destroyed_24h,
        );
        let adjusted_sopr_7d = LazyBinaryFromHeightLast::from_computed_last::<Ratio64>(
            &cfg.name("adjusted_sopr_7d"), cfg.version + v1, &adjusted_value_created_7d, &adjusted_value_destroyed_7d,
        );
        let adjusted_sopr_30d = LazyBinaryFromHeightLast::from_computed_last::<Ratio64>(
            &cfg.name("adjusted_sopr_30d"), cfg.version + v1, &adjusted_value_created_30d, &adjusted_value_destroyed_30d,
        );
        let adjusted_sopr_1y = LazyBinaryFromHeightLast::from_computed_last::<Ratio64>(
            &cfg.name("adjusted_sopr_1y"), cfg.version + v1, &adjusted_value_created_1y, &adjusted_value_destroyed_1y,
        );
        let adjusted_sopr = LazyFromHeightLast::from_binary::<Ident, Dollars, Dollars>(
            &cfg.name("adjusted_sopr"), cfg.version + v1, &adjusted_sopr_24h,
        );

        let adjusted_sopr_24h_7d_ema = import_rolling!("adjusted_sopr_24h_7d_ema");
        let adjusted_sopr_7d_ema = LazyFromHeightLast::from_computed::<Ident>(
            &cfg.name("adjusted_sopr_7d_ema"), cfg.version + v1,
            adjusted_sopr_24h_7d_ema.height.read_only_boxed_clone(), &adjusted_sopr_24h_7d_ema,
        );
        let adjusted_sopr_24h_30d_ema = import_rolling!("adjusted_sopr_24h_30d_ema");
        let adjusted_sopr_30d_ema = LazyFromHeightLast::from_computed::<Ident>(
            &cfg.name("adjusted_sopr_30d_ema"), cfg.version + v1,
            adjusted_sopr_24h_30d_ema.height.read_only_boxed_clone(), &adjusted_sopr_24h_30d_ema,
        );

        Ok(RealizedAdjusted {
            adjusted_value_created,
            adjusted_value_destroyed,
            adjusted_value_created_24h,
            adjusted_value_created_7d,
            adjusted_value_created_30d,
            adjusted_value_created_1y,
            adjusted_value_destroyed_24h,
            adjusted_value_destroyed_7d,
            adjusted_value_destroyed_30d,
            adjusted_value_destroyed_1y,
            adjusted_sopr,
            adjusted_sopr_24h,
            adjusted_sopr_7d,
            adjusted_sopr_30d,
            adjusted_sopr_1y,
            adjusted_sopr_24h_7d_ema,
            adjusted_sopr_7d_ema,
            adjusted_sopr_24h_30d_ema,
            adjusted_sopr_30d_ema,
        })
    }

    pub(crate) fn compute_rest_part2_adj(
        &mut self,
        blocks: &blocks::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        // Adjusted value created/destroyed rolling sums
        self.adjusted_value_created_24h.height.compute_rolling_sum(starting_indexes.height, &blocks.count.height_24h_ago, &self.adjusted_value_created.height, exit)?;
        self.adjusted_value_created_7d.height.compute_rolling_sum(starting_indexes.height, &blocks.count.height_1w_ago, &self.adjusted_value_created.height, exit)?;
        self.adjusted_value_created_30d.height.compute_rolling_sum(starting_indexes.height, &blocks.count.height_1m_ago, &self.adjusted_value_created.height, exit)?;
        self.adjusted_value_created_1y.height.compute_rolling_sum(starting_indexes.height, &blocks.count.height_1y_ago, &self.adjusted_value_created.height, exit)?;

        self.adjusted_value_destroyed_24h.height.compute_rolling_sum(starting_indexes.height, &blocks.count.height_24h_ago, &self.adjusted_value_destroyed.height, exit)?;
        self.adjusted_value_destroyed_7d.height.compute_rolling_sum(starting_indexes.height, &blocks.count.height_1w_ago, &self.adjusted_value_destroyed.height, exit)?;
        self.adjusted_value_destroyed_30d.height.compute_rolling_sum(starting_indexes.height, &blocks.count.height_1m_ago, &self.adjusted_value_destroyed.height, exit)?;
        self.adjusted_value_destroyed_1y.height.compute_rolling_sum(starting_indexes.height, &blocks.count.height_1y_ago, &self.adjusted_value_destroyed.height, exit)?;

        // Adjusted SOPR EMAs
        self.adjusted_sopr_24h_7d_ema.height.compute_rolling_average(
            starting_indexes.height,
            &blocks.count.height_1w_ago,
            &self.adjusted_sopr.height,
            exit,
        )?;
        self.adjusted_sopr_24h_30d_ema.height.compute_rolling_average(
            starting_indexes.height,
            &blocks.count.height_1m_ago,
            &self.adjusted_sopr.height,
            exit,
        )?;

        Ok(())
    }
}
