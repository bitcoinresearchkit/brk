use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{BasisPoints32, Cents, Dollars, Height, Indexes, StoredF64, Version};
use vecdb::{Exit, ReadableVec, Rw, StorageMode};

use crate::{
    blocks,
    internal::{
        ComputedFromHeightRatioExtension, PercentFromHeight, RatioCents64, RatioDollarsBp32,
        RollingWindows,
    },
};

use crate::distribution::metrics::ImportConfig;

use super::RealizedBase;

#[derive(Traversable)]
pub struct RealizedExtended<M: StorageMode = Rw> {
    pub realized_cap_rel_to_own_market_cap: PercentFromHeight<BasisPoints32, M>,

    pub realized_profit_sum: RollingWindows<Cents, M>,
    pub realized_loss_sum: RollingWindows<Cents, M>,

    pub realized_profit_to_loss_ratio: RollingWindows<StoredF64, M>,

    pub realized_price_ratio_ext: ComputedFromHeightRatioExtension<M>,
    pub investor_price_ratio_ext: ComputedFromHeightRatioExtension<M>,
}

impl RealizedExtended {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        Ok(RealizedExtended {
            realized_cap_rel_to_own_market_cap: cfg
                .import_percent_bp32("realized_cap_rel_to_own_market_cap", Version::ONE)?,
            realized_profit_sum: cfg.import_rolling("realized_profit", Version::ONE)?,
            realized_loss_sum: cfg.import_rolling("realized_loss", Version::ONE)?,
            realized_profit_to_loss_ratio: cfg
                .import_rolling("realized_profit_to_loss_ratio", Version::ONE)?,
            realized_price_ratio_ext: ComputedFromHeightRatioExtension::forced_import(
                cfg.db,
                &cfg.name("realized_price"),
                cfg.version + Version::ONE,
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
        starting_indexes: &Indexes,
        height_to_market_cap: &impl ReadableVec<Height, Dollars>,
        exit: &Exit,
    ) -> Result<()> {
        // Realized profit/loss rolling sums
        let window_starts = blocks.count.window_starts();
        self.realized_profit_sum.compute_rolling_sum(
            starting_indexes.height,
            &window_starts,
            &base.realized_profit.height,
            exit,
        )?;
        self.realized_loss_sum.compute_rolling_sum(
            starting_indexes.height,
            &window_starts,
            &base.realized_loss.height,
            exit,
        )?;

        // Realized cap relative to own market cap
        self.realized_cap_rel_to_own_market_cap
            .compute_binary::<Dollars, Dollars, RatioDollarsBp32>(
                starting_indexes.height,
                &base.realized_cap.height,
                height_to_market_cap,
                exit,
            )?;

        // Realized profit to loss ratios
        for ((ratio, profit), loss) in self
            .realized_profit_to_loss_ratio
            .as_mut_array()
            .into_iter()
            .zip(self.realized_profit_sum.as_array())
            .zip(self.realized_loss_sum.as_array())
        {
            ratio.compute_binary::<Cents, Cents, RatioCents64>(
                starting_indexes.height,
                &profit.height,
                &loss.height,
                exit,
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
