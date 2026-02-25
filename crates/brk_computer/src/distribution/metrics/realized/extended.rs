use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Dollars, Height, StoredF32, StoredF64, Version};
use vecdb::{Exit, ReadableVec, Rw, StorageMode};

use crate::{
    ComputeIndexes, blocks,
    internal::{
        ComputedFromHeightLast, LazyBinaryFromHeightLast, Ratio64,
    },
};

use crate::distribution::metrics::ImportConfig;

use super::RealizedBase;

/// Extended realized metrics (only for extended cohorts: all, sth, lth, age_range).
#[derive(Traversable)]
pub struct RealizedExtended<M: StorageMode = Rw> {
    pub realized_cap_rel_to_own_market_cap: ComputedFromHeightLast<StoredF32, M>,

    // === Realized Profit/Loss Rolling Sums ===
    pub realized_profit_24h: ComputedFromHeightLast<Dollars, M>,
    pub realized_profit_7d: ComputedFromHeightLast<Dollars, M>,
    pub realized_profit_30d: ComputedFromHeightLast<Dollars, M>,
    pub realized_profit_1y: ComputedFromHeightLast<Dollars, M>,
    pub realized_loss_24h: ComputedFromHeightLast<Dollars, M>,
    pub realized_loss_7d: ComputedFromHeightLast<Dollars, M>,
    pub realized_loss_30d: ComputedFromHeightLast<Dollars, M>,
    pub realized_loss_1y: ComputedFromHeightLast<Dollars, M>,

    // === Realized Profit to Loss Ratio (lazy from rolling sums) ===
    pub realized_profit_to_loss_ratio_24h: LazyBinaryFromHeightLast<StoredF64, Dollars, Dollars>,
    pub realized_profit_to_loss_ratio_7d: LazyBinaryFromHeightLast<StoredF64, Dollars, Dollars>,
    pub realized_profit_to_loss_ratio_30d: LazyBinaryFromHeightLast<StoredF64, Dollars, Dollars>,
    pub realized_profit_to_loss_ratio_1y: LazyBinaryFromHeightLast<StoredF64, Dollars, Dollars>,
}

impl RealizedExtended {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v1 = Version::ONE;

        macro_rules! import_rolling {
            ($name:expr) => {
                ComputedFromHeightLast::forced_import(cfg.db, &cfg.name($name), cfg.version + v1, cfg.indexes)?
            };
        }

        let realized_profit_24h = import_rolling!("realized_profit_24h");
        let realized_profit_7d = import_rolling!("realized_profit_7d");
        let realized_profit_30d = import_rolling!("realized_profit_30d");
        let realized_profit_1y = import_rolling!("realized_profit_1y");
        let realized_loss_24h = import_rolling!("realized_loss_24h");
        let realized_loss_7d = import_rolling!("realized_loss_7d");
        let realized_loss_30d = import_rolling!("realized_loss_30d");
        let realized_loss_1y = import_rolling!("realized_loss_1y");

        let realized_profit_to_loss_ratio_24h = LazyBinaryFromHeightLast::from_computed_last::<Ratio64>(
            &cfg.name("realized_profit_to_loss_ratio_24h"), cfg.version + v1, &realized_profit_24h, &realized_loss_24h,
        );
        let realized_profit_to_loss_ratio_7d = LazyBinaryFromHeightLast::from_computed_last::<Ratio64>(
            &cfg.name("realized_profit_to_loss_ratio_7d"), cfg.version + v1, &realized_profit_7d, &realized_loss_7d,
        );
        let realized_profit_to_loss_ratio_30d = LazyBinaryFromHeightLast::from_computed_last::<Ratio64>(
            &cfg.name("realized_profit_to_loss_ratio_30d"), cfg.version + v1, &realized_profit_30d, &realized_loss_30d,
        );
        let realized_profit_to_loss_ratio_1y = LazyBinaryFromHeightLast::from_computed_last::<Ratio64>(
            &cfg.name("realized_profit_to_loss_ratio_1y"), cfg.version + v1, &realized_profit_1y, &realized_loss_1y,
        );

        Ok(RealizedExtended {
            realized_cap_rel_to_own_market_cap: ComputedFromHeightLast::forced_import(
                cfg.db,
                &cfg.name("realized_cap_rel_to_own_market_cap"),
                cfg.version,
                cfg.indexes,
            )?,
            realized_profit_24h,
            realized_profit_7d,
            realized_profit_30d,
            realized_profit_1y,
            realized_loss_24h,
            realized_loss_7d,
            realized_loss_30d,
            realized_loss_1y,
            realized_profit_to_loss_ratio_24h,
            realized_profit_to_loss_ratio_7d,
            realized_profit_to_loss_ratio_30d,
            realized_profit_to_loss_ratio_1y,
        })
    }

    pub(crate) fn compute_rest_part2_ext(
        &mut self,
        base: &RealizedBase,
        blocks: &blocks::Vecs,
        starting_indexes: &ComputeIndexes,
        height_to_market_cap: &impl ReadableVec<Height, Dollars>,
        exit: &Exit,
    ) -> Result<()> {
        // Realized profit/loss rolling sums
        self.realized_profit_24h.height.compute_rolling_sum(starting_indexes.height, &blocks.count.height_24h_ago, &base.realized_profit.height, exit)?;
        self.realized_profit_7d.height.compute_rolling_sum(starting_indexes.height, &blocks.count.height_1w_ago, &base.realized_profit.height, exit)?;
        self.realized_profit_30d.height.compute_rolling_sum(starting_indexes.height, &blocks.count.height_1m_ago, &base.realized_profit.height, exit)?;
        self.realized_profit_1y.height.compute_rolling_sum(starting_indexes.height, &blocks.count.height_1y_ago, &base.realized_profit.height, exit)?;
        self.realized_loss_24h.height.compute_rolling_sum(starting_indexes.height, &blocks.count.height_24h_ago, &base.realized_loss.height, exit)?;
        self.realized_loss_7d.height.compute_rolling_sum(starting_indexes.height, &blocks.count.height_1w_ago, &base.realized_loss.height, exit)?;
        self.realized_loss_30d.height.compute_rolling_sum(starting_indexes.height, &blocks.count.height_1m_ago, &base.realized_loss.height, exit)?;
        self.realized_loss_1y.height.compute_rolling_sum(starting_indexes.height, &blocks.count.height_1y_ago, &base.realized_loss.height, exit)?;

        // Realized cap relative to own market cap
        self.realized_cap_rel_to_own_market_cap.height.compute_percentage(
            starting_indexes.height,
            &base.realized_cap.height,
            height_to_market_cap,
            exit,
        )?;

        Ok(())
    }
}
