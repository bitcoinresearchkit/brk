use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{BasisPoints16, Cents, Dollars, Height, StoredF64, Version};
use vecdb::{Exit, ReadableVec, Rw, StorageMode};

use crate::{
    ComputeIndexes, blocks,
    internal::{
        Bp16ToFloat, Bp16ToPercent, ComputedFromHeightRatioExtension, PercentFromHeight,
        RatioCents64, RatioDollarsBp16, RollingWindows,
    },
};

use crate::distribution::metrics::ImportConfig;

use super::RealizedBase;

/// Extended realized metrics (only for extended cohorts: all, sth, lth, age_range).
#[derive(Traversable)]
pub struct RealizedExtended<M: StorageMode = Rw> {
    pub realized_cap_rel_to_own_market_cap: PercentFromHeight<BasisPoints16, M>,

    // === Realized Profit/Loss Rolling Sums ===
    pub realized_profit_sum: RollingWindows<Cents, M>,
    pub realized_loss_sum: RollingWindows<Cents, M>,

    // === Realized Profit to Loss Ratio (from rolling sums) ===
    pub realized_profit_to_loss_ratio: RollingWindows<StoredF64, M>,

    // === Extended ratio metrics for realized/investor price ===
    pub realized_price_ratio_ext: ComputedFromHeightRatioExtension<M>,
    pub investor_price_ratio_ext: ComputedFromHeightRatioExtension<M>,
}

impl RealizedExtended {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v1 = Version::ONE;

        Ok(RealizedExtended {
            realized_cap_rel_to_own_market_cap: PercentFromHeight::forced_import::<Bp16ToFloat, Bp16ToPercent>(
                cfg.db,
                &cfg.name("realized_cap_rel_to_own_market_cap"),
                cfg.version,
                cfg.indexes,
            )?,
            realized_profit_sum: RollingWindows::forced_import(
                cfg.db, &cfg.name("realized_profit"), cfg.version + v1, cfg.indexes,
            )?,
            realized_loss_sum: RollingWindows::forced_import(
                cfg.db, &cfg.name("realized_loss"), cfg.version + v1, cfg.indexes,
            )?,
            realized_profit_to_loss_ratio: RollingWindows::forced_import(
                cfg.db, &cfg.name("realized_profit_to_loss_ratio"), cfg.version + v1, cfg.indexes,
            )?,
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
        let window_starts = blocks.count.window_starts();
        self.realized_profit_sum.compute_rolling_sum(
            starting_indexes.height, &window_starts, &base.realized_profit.height, exit,
        )?;
        self.realized_loss_sum.compute_rolling_sum(
            starting_indexes.height, &window_starts, &base.realized_loss.height, exit,
        )?;

        // Realized cap relative to own market cap
        self.realized_cap_rel_to_own_market_cap
            .compute_binary::<Dollars, Dollars, RatioDollarsBp16>(
                starting_indexes.height,
                &base.realized_cap.height,
                height_to_market_cap,
                exit,
            )?;

        // Realized profit to loss ratios
        for ((ratio, profit), loss) in self.realized_profit_to_loss_ratio.as_mut_array().into_iter()
            .zip(self.realized_profit_sum.as_array())
            .zip(self.realized_loss_sum.as_array())
        {
            ratio.compute_binary::<Cents, Cents, RatioCents64>(
                starting_indexes.height, &profit.height, &loss.height, exit,
            )?;
        }

        // Extended ratio metrics
        self.realized_price_ratio_ext.compute_rest(
            blocks,
            starting_indexes,
            exit,
            &base.realized_price_ratio.ratio.height,
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
            &base.investor_price_ratio.ratio.height,
        )?;
        self.investor_price_ratio_ext.compute_cents_bands(
            starting_indexes,
            &base.investor_price.cents.height,
            exit,
        )?;

        Ok(())
    }
}
