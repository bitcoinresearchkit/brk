use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Cents, Dollars, Height, StoredF32, StoredF64, Version};
use vecdb::{Exit, ReadableVec, Rw, StorageMode};

use crate::{
    ComputeIndexes, blocks,
    internal::{ComputedFromHeight, ComputedFromHeightRatioExtension, RatioCents64},
};

use crate::distribution::metrics::ImportConfig;

use super::RealizedBase;

/// Extended realized metrics (only for extended cohorts: all, sth, lth, age_range).
#[derive(Traversable)]
pub struct RealizedExtended<M: StorageMode = Rw> {
    pub realized_cap_rel_to_own_market_cap: ComputedFromHeight<StoredF32, M>,

    // === Realized Profit/Loss Rolling Sums ===
    pub realized_profit_24h: ComputedFromHeight<Cents, M>,
    pub realized_profit_7d: ComputedFromHeight<Cents, M>,
    pub realized_profit_30d: ComputedFromHeight<Cents, M>,
    pub realized_profit_1y: ComputedFromHeight<Cents, M>,
    pub realized_loss_24h: ComputedFromHeight<Cents, M>,
    pub realized_loss_7d: ComputedFromHeight<Cents, M>,
    pub realized_loss_30d: ComputedFromHeight<Cents, M>,
    pub realized_loss_1y: ComputedFromHeight<Cents, M>,

    // === Realized Profit to Loss Ratio (from rolling sums) ===
    pub realized_profit_to_loss_ratio_24h: ComputedFromHeight<StoredF64, M>,
    pub realized_profit_to_loss_ratio_7d: ComputedFromHeight<StoredF64, M>,
    pub realized_profit_to_loss_ratio_30d: ComputedFromHeight<StoredF64, M>,
    pub realized_profit_to_loss_ratio_1y: ComputedFromHeight<StoredF64, M>,

    // === Extended ratio metrics for realized/investor price ===
    pub realized_price_ratio_ext: ComputedFromHeightRatioExtension<M>,
    pub investor_price_ratio_ext: ComputedFromHeightRatioExtension<M>,
}

impl RealizedExtended {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v1 = Version::ONE;

        macro_rules! import_rolling {
            ($name:expr) => {
                ComputedFromHeight::forced_import(
                    cfg.db,
                    &cfg.name($name),
                    cfg.version + v1,
                    cfg.indexes,
                )?
            };
        }

        Ok(RealizedExtended {
            realized_cap_rel_to_own_market_cap: ComputedFromHeight::forced_import(
                cfg.db,
                &cfg.name("realized_cap_rel_to_own_market_cap"),
                cfg.version,
                cfg.indexes,
            )?,
            realized_profit_24h: import_rolling!("realized_profit_24h"),
            realized_profit_7d: import_rolling!("realized_profit_7d"),
            realized_profit_30d: import_rolling!("realized_profit_30d"),
            realized_profit_1y: import_rolling!("realized_profit_1y"),
            realized_loss_24h: import_rolling!("realized_loss_24h"),
            realized_loss_7d: import_rolling!("realized_loss_7d"),
            realized_loss_30d: import_rolling!("realized_loss_30d"),
            realized_loss_1y: import_rolling!("realized_loss_1y"),
            realized_profit_to_loss_ratio_24h: import_rolling!("realized_profit_to_loss_ratio_24h"),
            realized_profit_to_loss_ratio_7d: import_rolling!("realized_profit_to_loss_ratio_7d"),
            realized_profit_to_loss_ratio_30d: import_rolling!("realized_profit_to_loss_ratio_30d"),
            realized_profit_to_loss_ratio_1y: import_rolling!("realized_profit_to_loss_ratio_1y"),
            realized_price_ratio_ext: ComputedFromHeightRatioExtension::forced_import(
                cfg.db,
                &cfg.name("realized_price"),
                cfg.version + v1,
                cfg.indexes,
            )?,
            investor_price_ratio_ext: ComputedFromHeightRatioExtension::forced_import(
                cfg.db,
                &cfg.name("investor_price"),
                cfg.version,
                cfg.indexes,
            )?,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn compute_rest_part2_ext(
        &mut self,
        base: &RealizedBase,
        blocks: &blocks::Vecs,
        starting_indexes: &ComputeIndexes,
        height_to_market_cap: &impl ReadableVec<Height, Dollars>,
        exit: &Exit,
    ) -> Result<()> {
        // Realized profit/loss rolling sums
        self.realized_profit_24h.height.compute_rolling_sum(
            starting_indexes.height,
            &blocks.count.height_24h_ago,
            &base.realized_profit.height,
            exit,
        )?;
        self.realized_profit_7d.height.compute_rolling_sum(
            starting_indexes.height,
            &blocks.count.height_1w_ago,
            &base.realized_profit.height,
            exit,
        )?;
        self.realized_profit_30d.height.compute_rolling_sum(
            starting_indexes.height,
            &blocks.count.height_1m_ago,
            &base.realized_profit.height,
            exit,
        )?;
        self.realized_profit_1y.height.compute_rolling_sum(
            starting_indexes.height,
            &blocks.count.height_1y_ago,
            &base.realized_profit.height,
            exit,
        )?;
        self.realized_loss_24h.height.compute_rolling_sum(
            starting_indexes.height,
            &blocks.count.height_24h_ago,
            &base.realized_loss.height,
            exit,
        )?;
        self.realized_loss_7d.height.compute_rolling_sum(
            starting_indexes.height,
            &blocks.count.height_1w_ago,
            &base.realized_loss.height,
            exit,
        )?;
        self.realized_loss_30d.height.compute_rolling_sum(
            starting_indexes.height,
            &blocks.count.height_1m_ago,
            &base.realized_loss.height,
            exit,
        )?;
        self.realized_loss_1y.height.compute_rolling_sum(
            starting_indexes.height,
            &blocks.count.height_1y_ago,
            &base.realized_loss.height,
            exit,
        )?;

        // Realized cap relative to own market cap
        self.realized_cap_rel_to_own_market_cap
            .height
            .compute_percentage(
                starting_indexes.height,
                &base.realized_cap.height,
                height_to_market_cap,
                exit,
            )?;

        // Realized profit to loss ratios
        self.realized_profit_to_loss_ratio_24h
            .compute_binary::<Cents, Cents, RatioCents64>(
                starting_indexes.height,
                &self.realized_profit_24h.height,
                &self.realized_loss_24h.height,
                exit,
            )?;
        self.realized_profit_to_loss_ratio_7d
            .compute_binary::<Cents, Cents, RatioCents64>(
                starting_indexes.height,
                &self.realized_profit_7d.height,
                &self.realized_loss_7d.height,
                exit,
            )?;
        self.realized_profit_to_loss_ratio_30d
            .compute_binary::<Cents, Cents, RatioCents64>(
                starting_indexes.height,
                &self.realized_profit_30d.height,
                &self.realized_loss_30d.height,
                exit,
            )?;
        self.realized_profit_to_loss_ratio_1y
            .compute_binary::<Cents, Cents, RatioCents64>(
                starting_indexes.height,
                &self.realized_profit_1y.height,
                &self.realized_loss_1y.height,
                exit,
            )?;

        // Extended ratio metrics
        self.realized_price_ratio_ext.compute_rest(
            blocks,
            starting_indexes,
            exit,
            &base.realized_price_extra.ratio.height,
        )?;
        self.realized_price_ratio_ext.compute_cents_bands(
            starting_indexes,
            &base.realized_price.cents.height,
            exit,
        )?;

        self.investor_price_ratio_ext.compute_rest(
            blocks,
            starting_indexes,
            exit,
            &base.investor_price_extra.ratio.height,
        )?;
        self.investor_price_ratio_ext.compute_cents_bands(
            starting_indexes,
            &base.investor_price.cents.height,
            exit,
        )?;

        Ok(())
    }
}
