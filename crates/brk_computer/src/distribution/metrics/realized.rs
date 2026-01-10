use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Bitcoin, DateIndex, Dollars, Height, StoredF32, StoredF64, Version};
use rayon::prelude::*;
use vecdb::{
    AnyStoredVec, AnyVec, EagerVec, Exit, GenericStoredVec, Ident, ImportableVec,
    IterableCloneableVec, IterableVec, Negate, PcoVec,
};

use crate::{
    ComputeIndexes,
    distribution::state::RealizedState,
    indexes,
    internal::{
        ComputedBlockLast, ComputedBlockSum, ComputedBlockSumCum, ComputedDateLast,
        ComputedRatioVecsDate, DollarsMinus, LazyBinaryBlockSum, LazyBinaryBlockSumCum,
        LazyBlockSum, LazyBlockSumCum, LazyDateLast, PercentageDollarsF32, StoredF32Identity,
    },
    price,
};

use super::ImportConfig;

/// Realized cap and related metrics.
#[derive(Clone, Traversable)]
pub struct RealizedMetrics {
    // === Realized Cap ===
    pub realized_cap: ComputedBlockLast<Dollars>,
    pub realized_price: ComputedBlockLast<Dollars>,
    pub realized_price_extra: ComputedRatioVecsDate,
    pub realized_cap_rel_to_own_market_cap: Option<ComputedBlockLast<StoredF32>>,
    pub realized_cap_30d_delta: ComputedDateLast<Dollars>,

    // === MVRV (Market Value to Realized Value) ===
    // Proxy for realized_price_extra.ratio (close / realized_price = market_cap / realized_cap)
    pub mvrv: LazyDateLast<StoredF32>,

    // === Realized Profit/Loss ===
    pub realized_profit: ComputedBlockSumCum<Dollars>,
    pub realized_loss: ComputedBlockSumCum<Dollars>,
    pub neg_realized_loss: LazyBlockSumCum<Dollars>,
    pub net_realized_pnl: ComputedBlockSumCum<Dollars>,
    pub realized_value: ComputedBlockSum<Dollars>,

    // === Realized vs Realized Cap Ratios (lazy) ===
    pub realized_profit_rel_to_realized_cap: LazyBinaryBlockSumCum<StoredF32, Dollars, Dollars>,
    pub realized_loss_rel_to_realized_cap: LazyBinaryBlockSumCum<StoredF32, Dollars, Dollars>,
    pub net_realized_pnl_rel_to_realized_cap: LazyBinaryBlockSumCum<StoredF32, Dollars, Dollars>,

    // === Total Realized PnL ===
    pub total_realized_pnl: LazyBlockSum<Dollars>,
    pub realized_profit_to_loss_ratio: Option<EagerVec<PcoVec<DateIndex, StoredF64>>>,

    // === Value Created/Destroyed ===
    pub value_created: ComputedBlockSum<Dollars>,
    pub value_destroyed: ComputedBlockSum<Dollars>,

    // === Adjusted Value (lazy: cohort - up_to_1h) ===
    pub adjusted_value_created: Option<LazyBinaryBlockSum<Dollars, Dollars, Dollars>>,
    pub adjusted_value_destroyed: Option<LazyBinaryBlockSum<Dollars, Dollars, Dollars>>,

    // === SOPR (Spent Output Profit Ratio) ===
    pub sopr: EagerVec<PcoVec<DateIndex, StoredF64>>,
    pub sopr_7d_ema: EagerVec<PcoVec<DateIndex, StoredF64>>,
    pub sopr_30d_ema: EagerVec<PcoVec<DateIndex, StoredF64>>,
    pub adjusted_sopr: Option<EagerVec<PcoVec<DateIndex, StoredF64>>>,
    pub adjusted_sopr_7d_ema: Option<EagerVec<PcoVec<DateIndex, StoredF64>>>,
    pub adjusted_sopr_30d_ema: Option<EagerVec<PcoVec<DateIndex, StoredF64>>>,

    // === Sell Side Risk ===
    pub sell_side_risk_ratio: EagerVec<PcoVec<DateIndex, StoredF32>>,
    pub sell_side_risk_ratio_7d_ema: EagerVec<PcoVec<DateIndex, StoredF32>>,
    pub sell_side_risk_ratio_30d_ema: EagerVec<PcoVec<DateIndex, StoredF32>>,

    // === Net Realized PnL Deltas ===
    pub net_realized_pnl_cumulative_30d_delta: ComputedDateLast<Dollars>,
    pub net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap: ComputedDateLast<StoredF32>,
    pub net_realized_pnl_cumulative_30d_delta_rel_to_market_cap: ComputedDateLast<StoredF32>,
}

impl RealizedMetrics {
    /// Import realized metrics from database.
    pub fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v1 = Version::ONE;
        let v3 = Version::new(3);
        let extended = cfg.extended();
        let compute_adjusted = cfg.compute_adjusted();

        // Import combined types using forced_import which handles height + derived
        let realized_cap = ComputedBlockLast::forced_import(
            cfg.db,
            &cfg.name("realized_cap"),
            cfg.version,
            cfg.indexes,
        )?;

        let realized_profit = ComputedBlockSumCum::forced_import(
            cfg.db,
            &cfg.name("realized_profit"),
            cfg.version,
            cfg.indexes,
        )?;

        let realized_loss = ComputedBlockSumCum::forced_import(
            cfg.db,
            &cfg.name("realized_loss"),
            cfg.version,
            cfg.indexes,
        )?;

        let neg_realized_loss = LazyBlockSumCum::from_computed::<Negate>(
            &cfg.name("neg_realized_loss"),
            cfg.version + v1,
            realized_loss.height.boxed_clone(),
            &realized_loss,
        );

        let net_realized_pnl = ComputedBlockSumCum::forced_import(
            cfg.db,
            &cfg.name("net_realized_pnl"),
            cfg.version,
            cfg.indexes,
        )?;

        // realized_value is the source for total_realized_pnl (they're identical)
        let realized_value = ComputedBlockSum::forced_import(
            cfg.db,
            &cfg.name("realized_value"),
            cfg.version,
            cfg.indexes,
        )?;

        // total_realized_pnl is a lazy alias to realized_value
        let total_realized_pnl = LazyBlockSum::from_computed::<Ident>(
            &cfg.name("total_realized_pnl"),
            cfg.version + v1,
            realized_value.height.boxed_clone(),
            &realized_value,
        );

        // Construct lazy ratio vecs
        let realized_profit_rel_to_realized_cap =
            LazyBinaryBlockSumCum::from_computed_last::<PercentageDollarsF32>(
                &cfg.name("realized_profit_rel_to_realized_cap"),
                cfg.version + v1,
                realized_profit.height.boxed_clone(),
                realized_cap.height.boxed_clone(),
                &realized_profit,
                &realized_cap,
            );

        let realized_loss_rel_to_realized_cap =
            LazyBinaryBlockSumCum::from_computed_last::<PercentageDollarsF32>(
                &cfg.name("realized_loss_rel_to_realized_cap"),
                cfg.version + v1,
                realized_loss.height.boxed_clone(),
                realized_cap.height.boxed_clone(),
                &realized_loss,
                &realized_cap,
            );

        let net_realized_pnl_rel_to_realized_cap =
            LazyBinaryBlockSumCum::from_computed_last::<PercentageDollarsF32>(
                &cfg.name("net_realized_pnl_rel_to_realized_cap"),
                cfg.version + v1,
                net_realized_pnl.height.boxed_clone(),
                realized_cap.height.boxed_clone(),
                &net_realized_pnl,
                &realized_cap,
            );

        let realized_price = ComputedBlockLast::forced_import(
            cfg.db,
            &cfg.name("realized_price"),
            cfg.version + v1,
            cfg.indexes,
        )?;

        let value_created = ComputedBlockSum::forced_import(
            cfg.db,
            &cfg.name("value_created"),
            cfg.version,
            cfg.indexes,
        )?;

        let value_destroyed = ComputedBlockSum::forced_import(
            cfg.db,
            &cfg.name("value_destroyed"),
            cfg.version,
            cfg.indexes,
        )?;

        // Create lazy adjusted vecs if compute_adjusted and up_to_1h is available
        let adjusted_value_created =
            (compute_adjusted && cfg.up_to_1h_realized.is_some()).then(|| {
                let up_to_1h = cfg.up_to_1h_realized.unwrap();
                LazyBinaryBlockSum::from_computed::<DollarsMinus>(
                    &cfg.name("adjusted_value_created"),
                    cfg.version,
                    &value_created,
                    &up_to_1h.value_created,
                )
            });
        let adjusted_value_destroyed =
            (compute_adjusted && cfg.up_to_1h_realized.is_some()).then(|| {
                let up_to_1h = cfg.up_to_1h_realized.unwrap();
                LazyBinaryBlockSum::from_computed::<DollarsMinus>(
                    &cfg.name("adjusted_value_destroyed"),
                    cfg.version,
                    &value_destroyed,
                    &up_to_1h.value_destroyed,
                )
            });

        // Create realized_price_extra first so we can reference its ratio for MVRV proxy
        let realized_price_extra = ComputedRatioVecsDate::forced_import(
            cfg.db,
            &cfg.name("realized_price"),
            Some(&realized_price),
            cfg.version + v1,
            cfg.indexes,
            extended,
            cfg.price,
        )?;

        // MVRV is a lazy proxy for realized_price_extra.ratio
        // ratio = close / realized_price = market_cap / realized_cap = MVRV
        let mvrv = LazyDateLast::from_source::<StoredF32Identity>(
            &cfg.name("mvrv"),
            cfg.version,
            &realized_price_extra.ratio,
        );

        Ok(Self {
            // === Realized Cap ===
            realized_cap,
            realized_price,
            realized_price_extra,
            realized_cap_rel_to_own_market_cap: extended
                .then(|| {
                    ComputedBlockLast::forced_import(
                        cfg.db,
                        &cfg.name("realized_cap_rel_to_own_market_cap"),
                        cfg.version,
                        cfg.indexes,
                    )
                })
                .transpose()?,
            realized_cap_30d_delta: ComputedDateLast::forced_import(
                cfg.db,
                &cfg.name("realized_cap_30d_delta"),
                cfg.version,
                cfg.indexes,
            )?,

            // === MVRV ===
            mvrv,

            // === Realized Profit/Loss ===
            realized_profit,
            realized_loss,
            neg_realized_loss,
            net_realized_pnl,
            realized_value,

            // === Realized vs Realized Cap Ratios (lazy) ===
            realized_profit_rel_to_realized_cap,
            realized_loss_rel_to_realized_cap,
            net_realized_pnl_rel_to_realized_cap,

            // === Total Realized PnL ===
            total_realized_pnl,
            realized_profit_to_loss_ratio: extended
                .then(|| {
                    EagerVec::forced_import(
                        cfg.db,
                        &cfg.name("realized_profit_to_loss_ratio"),
                        cfg.version + v1,
                    )
                })
                .transpose()?,

            // === Value Created/Destroyed ===
            value_created,
            value_destroyed,

            // === Adjusted Value (lazy: cohort - up_to_1h) ===
            adjusted_value_created,
            adjusted_value_destroyed,

            // === SOPR ===
            sopr: EagerVec::forced_import(cfg.db, &cfg.name("sopr"), cfg.version + v1)?,
            sopr_7d_ema: EagerVec::forced_import(cfg.db, &cfg.name("sopr_7d_ema"), cfg.version + v1)?,
            sopr_30d_ema: EagerVec::forced_import(
                cfg.db,
                &cfg.name("sopr_30d_ema"),
                cfg.version + v1,
            )?,
            adjusted_sopr: compute_adjusted
                .then(|| {
                    EagerVec::forced_import(cfg.db, &cfg.name("adjusted_sopr"), cfg.version + v1)
                })
                .transpose()?,
            adjusted_sopr_7d_ema: compute_adjusted
                .then(|| {
                    EagerVec::forced_import(
                        cfg.db,
                        &cfg.name("adjusted_sopr_7d_ema"),
                        cfg.version + v1,
                    )
                })
                .transpose()?,
            adjusted_sopr_30d_ema: compute_adjusted
                .then(|| {
                    EagerVec::forced_import(
                        cfg.db,
                        &cfg.name("adjusted_sopr_30d_ema"),
                        cfg.version + v1,
                    )
                })
                .transpose()?,

            // === Sell Side Risk ===
            sell_side_risk_ratio: EagerVec::forced_import(
                cfg.db,
                &cfg.name("sell_side_risk_ratio"),
                cfg.version + v1,
            )?,
            sell_side_risk_ratio_7d_ema: EagerVec::forced_import(
                cfg.db,
                &cfg.name("sell_side_risk_ratio_7d_ema"),
                cfg.version + v1,
            )?,
            sell_side_risk_ratio_30d_ema: EagerVec::forced_import(
                cfg.db,
                &cfg.name("sell_side_risk_ratio_30d_ema"),
                cfg.version + v1,
            )?,

            // === Net Realized PnL Deltas ===
            net_realized_pnl_cumulative_30d_delta: ComputedDateLast::forced_import(
                cfg.db,
                &cfg.name("net_realized_pnl_cumulative_30d_delta"),
                cfg.version + v3,
                cfg.indexes,
            )?,
            net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap:
                ComputedDateLast::forced_import(
                    cfg.db,
                    &cfg.name("net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap"),
                    cfg.version + v3,
                    cfg.indexes,
                )?,
            net_realized_pnl_cumulative_30d_delta_rel_to_market_cap:
                ComputedDateLast::forced_import(
                    cfg.db,
                    &cfg.name("net_realized_pnl_cumulative_30d_delta_rel_to_market_cap"),
                    cfg.version + v3,
                    cfg.indexes,
                )?,
        })
    }

    /// Get minimum length across height-indexed vectors written in block loop.
    pub fn min_stateful_height_len(&self) -> usize {
        self.realized_cap
            .height
            .len()
            .min(self.realized_profit.height.len())
            .min(self.realized_loss.height.len())
            .min(self.value_created.height.len())
            .min(self.value_destroyed.height.len())
    }

    /// Push realized state values to height-indexed vectors.
    pub fn truncate_push(&mut self, height: Height, state: &RealizedState) -> Result<()> {
        self.realized_cap.height.truncate_push(height, state.cap)?;
        self.realized_profit
            .height
            .truncate_push(height, state.profit)?;
        self.realized_loss
            .height
            .truncate_push(height, state.loss)?;
        self.value_created
            .height
            .truncate_push(height, state.value_created)?;
        self.value_destroyed
            .height
            .truncate_push(height, state.value_destroyed)?;

        Ok(())
    }

    /// Write height-indexed vectors to disk.
    pub fn write(&mut self) -> Result<()> {
        self.realized_cap.height.write()?;
        self.realized_profit.height.write()?;
        self.realized_loss.height.write()?;
        self.value_created.height.write()?;
        self.value_destroyed.height.write()?;
        Ok(())
    }

    /// Returns a parallel iterator over all vecs for parallel writing.
    pub fn par_iter_mut(&mut self) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        [
            &mut self.realized_cap.height as &mut dyn AnyStoredVec,
            &mut self.realized_profit.height,
            &mut self.realized_loss.height,
            &mut self.value_created.height,
            &mut self.value_destroyed.height,
        ]
        .into_par_iter()
    }

    /// Validate computed versions against base version.
    pub fn validate_computed_versions(&mut self, _base_version: Version) -> Result<()> {
        // Validation logic for computed vecs
        Ok(())
    }

    /// Compute aggregate values from separate cohorts.
    pub fn compute_from_stateful(
        &mut self,
        starting_indexes: &ComputeIndexes,
        others: &[&Self],
        exit: &Exit,
    ) -> Result<()> {
        self.realized_cap.height.compute_sum_of_others(
            starting_indexes.height,
            &others
                .iter()
                .map(|v| &v.realized_cap.height)
                .collect::<Vec<_>>(),
            exit,
        )?;
        self.realized_profit.height.compute_sum_of_others(
            starting_indexes.height,
            &others
                .iter()
                .map(|v| &v.realized_profit.height)
                .collect::<Vec<_>>(),
            exit,
        )?;
        self.realized_loss.height.compute_sum_of_others(
            starting_indexes.height,
            &others
                .iter()
                .map(|v| &v.realized_loss.height)
                .collect::<Vec<_>>(),
            exit,
        )?;
        self.value_created.height.compute_sum_of_others(
            starting_indexes.height,
            &others
                .iter()
                .map(|v| &v.value_created.height)
                .collect::<Vec<_>>(),
            exit,
        )?;
        self.value_destroyed.height.compute_sum_of_others(
            starting_indexes.height,
            &others
                .iter()
                .map(|v| &v.value_destroyed.height)
                .collect::<Vec<_>>(),
            exit,
        )?;

        Ok(())
    }

    /// First phase of computed metrics (indexes from height).
    pub fn compute_rest_part1(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.realized_cap.compute_rest(indexes, starting_indexes, exit)?;
        self.realized_profit.compute_rest(indexes, starting_indexes, exit)?;
        self.realized_loss.compute_rest(indexes, starting_indexes, exit)?;

        // net_realized_pnl = profit - loss
        self.net_realized_pnl
            .compute_all(indexes, starting_indexes, exit, |vec| {
                vec.compute_subtract(
                    starting_indexes.height,
                    &self.realized_profit.height,
                    &self.realized_loss.height,
                    exit,
                )?;
                Ok(())
            })?;

        // realized_value = profit + loss
        // Note: total_realized_pnl is a lazy alias to realized_value since both
        // compute profit + loss with sum aggregation, making them identical.
        self.realized_value
            .compute_all(indexes, starting_indexes, exit, |vec| {
                vec.compute_add(
                    starting_indexes.height,
                    &self.realized_profit.height,
                    &self.realized_loss.height,
                    exit,
                )?;
                Ok(())
            })?;

        self.value_created.compute_rest(indexes, starting_indexes, exit)?;
        self.value_destroyed.compute_rest(indexes, starting_indexes, exit)?;

        Ok(())
    }

    /// Second phase of computed metrics (realized price from realized cap / supply).
    #[allow(clippy::too_many_arguments)]
    pub fn compute_rest_part2(
        &mut self,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
        starting_indexes: &ComputeIndexes,
        height_to_supply: &impl IterableVec<Height, Bitcoin>,
        height_to_market_cap: Option<&impl IterableVec<Height, Dollars>>,
        dateindex_to_market_cap: Option<&impl IterableVec<DateIndex, Dollars>>,
        exit: &Exit,
    ) -> Result<()> {
        // realized_price = realized_cap / supply
        self.realized_price
            .compute_all(indexes, starting_indexes, exit, |vec| {
                vec.compute_divide(
                    starting_indexes.height,
                    &self.realized_cap.height,
                    height_to_supply,
                    exit,
                )?;
                Ok(())
            })?;

        if let Some(price) = price {
            self.realized_price_extra.compute_rest(
                price,
                starting_indexes,
                exit,
                Some(&self.realized_price.dateindex.0),
            )?;
        }

        // realized_cap_30d_delta
        self.realized_cap_30d_delta
            .compute_all(starting_indexes, exit, |vec| {
                vec.compute_change(
                    starting_indexes.dateindex,
                    &self.realized_cap.dateindex.0,
                    30,
                    exit,
                )?;
                Ok(())
            })?;

        // SOPR = value_created / value_destroyed
        self.sopr.compute_divide(
            starting_indexes.dateindex,
            &self.value_created.dateindex.0,
            &self.value_destroyed.dateindex.0,
            exit,
        )?;

        self.sopr_7d_ema
            .compute_ema(starting_indexes.dateindex, &self.sopr, 7, exit)?;

        self.sopr_30d_ema
            .compute_ema(starting_indexes.dateindex, &self.sopr, 30, exit)?;

        // Optional: adjusted SOPR (lazy: cohort - up_to_1h)
        if let (Some(adjusted_sopr), Some(adj_created), Some(adj_destroyed)) = (
            self.adjusted_sopr.as_mut(),
            self.adjusted_value_created.as_ref(),
            self.adjusted_value_destroyed.as_ref(),
        ) {
            adjusted_sopr.compute_divide(
                starting_indexes.dateindex,
                &*adj_created.dateindex,
                &*adj_destroyed.dateindex,
                exit,
            )?;

            if let Some(ema_7d) = self.adjusted_sopr_7d_ema.as_mut() {
                ema_7d.compute_ema(
                    starting_indexes.dateindex,
                    self.adjusted_sopr.as_ref().unwrap(),
                    7,
                    exit,
                )?;
            }

            if let Some(ema_30d) = self.adjusted_sopr_30d_ema.as_mut() {
                ema_30d.compute_ema(
                    starting_indexes.dateindex,
                    self.adjusted_sopr.as_ref().unwrap(),
                    30,
                    exit,
                )?;
            }
        }

        // sell_side_risk_ratio = realized_value / realized_cap
        self.sell_side_risk_ratio.compute_percentage(
            starting_indexes.dateindex,
            &self.realized_value.dateindex.0,
            &self.realized_cap.dateindex.0,
            exit,
        )?;

        self.sell_side_risk_ratio_7d_ema
            .compute_ema(starting_indexes.dateindex, &self.sell_side_risk_ratio, 7, exit)?;

        self.sell_side_risk_ratio_30d_ema.compute_ema(
            starting_indexes.dateindex,
            &self.sell_side_risk_ratio,
            30,
            exit,
        )?;

        // Net realized PnL cumulative 30d delta
        self.net_realized_pnl_cumulative_30d_delta
            .compute_all(starting_indexes, exit, |vec| {
                vec.compute_change(
                    starting_indexes.dateindex,
                    &self.net_realized_pnl.dateindex.cumulative.0,
                    30,
                    exit,
                )?;
                Ok(())
            })?;

        // Relative to realized cap
        self.net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap
            .compute_all(starting_indexes, exit, |vec| {
                vec.compute_percentage(
                    starting_indexes.dateindex,
                    &self.net_realized_pnl_cumulative_30d_delta.dateindex,
                    &self.realized_cap.dateindex.0,
                    exit,
                )?;
                Ok(())
            })?;

        // Relative to market cap
        if let Some(dateindex_to_market_cap) = dateindex_to_market_cap {
            self.net_realized_pnl_cumulative_30d_delta_rel_to_market_cap
                .compute_all(starting_indexes, exit, |vec| {
                    vec.compute_percentage(
                        starting_indexes.dateindex,
                        &self.net_realized_pnl_cumulative_30d_delta.dateindex,
                        dateindex_to_market_cap,
                        exit,
                    )?;
                    Ok(())
                })?;
        }

        // Optional: realized_cap_rel_to_own_market_cap
        if let (Some(rel_vec), Some(height_to_market_cap)) = (
            self.realized_cap_rel_to_own_market_cap.as_mut(),
            height_to_market_cap,
        ) {
            rel_vec.compute_all(indexes, starting_indexes, exit, |vec| {
                vec.compute_percentage(
                    starting_indexes.height,
                    &self.realized_cap.height,
                    height_to_market_cap,
                    exit,
                )?;
                Ok(())
            })?;
        }

        // Optional: realized_profit_to_loss_ratio
        if let Some(ratio) = self.realized_profit_to_loss_ratio.as_mut() {
            ratio.compute_divide(
                starting_indexes.dateindex,
                &self.realized_profit.dateindex.sum.0,
                &self.realized_loss.dateindex.sum.0,
                exit,
            )?;
        }

        Ok(())
    }
}
