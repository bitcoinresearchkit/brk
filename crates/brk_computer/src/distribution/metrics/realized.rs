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
        BinaryBlockSum, BinaryBlockSumCumLast, ComputedBlockLast, ComputedBlockSum,
        ComputedBlockSumCum, ComputedDateLast, ComputedRatioVecsDate, DerivedComputedBlockLast,
        DerivedComputedBlockSum, DerivedComputedBlockSumCum, DollarsMinus, LazyBlockSum,
        LazyBlockSumCum, LazyDateLast, PercentageDollarsF32, StoredF32Identity,
    },
    price,
};

use super::ImportConfig;

/// Realized cap and related metrics.
#[derive(Clone, Traversable)]
pub struct RealizedMetrics {
    // === Realized Cap ===
    pub height_to_realized_cap: EagerVec<PcoVec<Height, Dollars>>,
    pub indexes_to_realized_cap: DerivedComputedBlockLast<Dollars>,
    pub indexes_to_realized_price: ComputedBlockLast<Dollars>,
    pub indexes_to_realized_price_extra: ComputedRatioVecsDate,
    pub indexes_to_realized_cap_rel_to_own_market_cap: Option<ComputedBlockLast<StoredF32>>,
    pub indexes_to_realized_cap_30d_delta: ComputedDateLast<Dollars>,

    // === MVRV (Market Value to Realized Value) ===
    // Proxy for indexes_to_realized_price_extra.ratio (close / realized_price = market_cap / realized_cap)
    pub indexes_to_mvrv: LazyDateLast<StoredF32>,

    // === Realized Profit/Loss ===
    pub height_to_realized_profit: EagerVec<PcoVec<Height, Dollars>>,
    pub indexes_to_realized_profit: DerivedComputedBlockSumCum<Dollars>,
    pub height_to_realized_loss: EagerVec<PcoVec<Height, Dollars>>,
    pub indexes_to_realized_loss: DerivedComputedBlockSumCum<Dollars>,
    pub indexes_to_neg_realized_loss: LazyBlockSumCum<Dollars>,
    pub indexes_to_net_realized_pnl: ComputedBlockSumCum<Dollars>,
    pub indexes_to_realized_value: ComputedBlockSum<Dollars>,

    // === Realized vs Realized Cap Ratios (lazy) ===
    pub indexes_to_realized_profit_rel_to_realized_cap:
        BinaryBlockSumCumLast<StoredF32, Dollars, Dollars>,
    pub indexes_to_realized_loss_rel_to_realized_cap:
        BinaryBlockSumCumLast<StoredF32, Dollars, Dollars>,
    pub indexes_to_net_realized_pnl_rel_to_realized_cap:
        BinaryBlockSumCumLast<StoredF32, Dollars, Dollars>,

    // === Total Realized PnL ===
    pub indexes_to_total_realized_pnl: LazyBlockSum<Dollars>,
    pub dateindex_to_realized_profit_to_loss_ratio: Option<EagerVec<PcoVec<DateIndex, StoredF64>>>,

    // === Value Created/Destroyed ===
    pub height_to_value_created: EagerVec<PcoVec<Height, Dollars>>,
    #[traversable(rename = "value_created_sum")]
    pub indexes_to_value_created: DerivedComputedBlockSum<Dollars>,
    pub height_to_value_destroyed: EagerVec<PcoVec<Height, Dollars>>,
    #[traversable(rename = "value_destroyed_sum")]
    pub indexes_to_value_destroyed: DerivedComputedBlockSum<Dollars>,

    // === Adjusted Value (lazy: cohort - up_to_1h) ===
    pub indexes_to_adjusted_value_created: Option<BinaryBlockSum<Dollars, Dollars, Dollars>>,
    pub indexes_to_adjusted_value_destroyed: Option<BinaryBlockSum<Dollars, Dollars, Dollars>>,

    // === SOPR (Spent Output Profit Ratio) ===
    pub dateindex_to_sopr: EagerVec<PcoVec<DateIndex, StoredF64>>,
    pub dateindex_to_sopr_7d_ema: EagerVec<PcoVec<DateIndex, StoredF64>>,
    pub dateindex_to_sopr_30d_ema: EagerVec<PcoVec<DateIndex, StoredF64>>,
    pub dateindex_to_adjusted_sopr: Option<EagerVec<PcoVec<DateIndex, StoredF64>>>,
    pub dateindex_to_adjusted_sopr_7d_ema: Option<EagerVec<PcoVec<DateIndex, StoredF64>>>,
    pub dateindex_to_adjusted_sopr_30d_ema: Option<EagerVec<PcoVec<DateIndex, StoredF64>>>,

    // === Sell Side Risk ===
    pub dateindex_to_sell_side_risk_ratio: EagerVec<PcoVec<DateIndex, StoredF32>>,
    pub dateindex_to_sell_side_risk_ratio_7d_ema: EagerVec<PcoVec<DateIndex, StoredF32>>,
    pub dateindex_to_sell_side_risk_ratio_30d_ema: EagerVec<PcoVec<DateIndex, StoredF32>>,

    // === Net Realized PnL Deltas ===
    pub indexes_to_net_realized_pnl_cumulative_30d_delta: ComputedDateLast<Dollars>,
    pub indexes_to_net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap:
        ComputedDateLast<StoredF32>,
    pub indexes_to_net_realized_pnl_cumulative_30d_delta_rel_to_market_cap:
        ComputedDateLast<StoredF32>,
}

impl RealizedMetrics {
    /// Import realized metrics from database.
    pub fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v1 = Version::ONE;
        let v3 = Version::new(3);
        let extended = cfg.extended();
        let compute_adjusted = cfg.compute_adjusted();

        let height_to_realized_loss: EagerVec<PcoVec<Height, Dollars>> =
            EagerVec::forced_import(cfg.db, &cfg.name("realized_loss"), cfg.version)?;

        let indexes_to_realized_loss = DerivedComputedBlockSumCum::forced_import(
            cfg.db,
            &cfg.name("realized_loss"),
            height_to_realized_loss.boxed_clone(),
            cfg.version,
            cfg.indexes,
        )?;

        let indexes_to_neg_realized_loss = LazyBlockSumCum::from_derived::<Negate>(
            &cfg.name("neg_realized_loss"),
            cfg.version + v1,
            height_to_realized_loss.boxed_clone(),
            &indexes_to_realized_loss,
        );

        // realized_value is the source for total_realized_pnl (they're identical)
        let indexes_to_realized_value = ComputedBlockSum::forced_import(
            cfg.db,
            &cfg.name("realized_value"),
            cfg.version,
            cfg.indexes,
        )?;

        // total_realized_pnl is a lazy alias to realized_value
        let indexes_to_total_realized_pnl = LazyBlockSum::from_computed::<Ident>(
            &cfg.name("total_realized_pnl"),
            cfg.version + v1,
            indexes_to_realized_value.height.boxed_clone(),
            &indexes_to_realized_value,
        );

        // Extract vecs needed for lazy ratio construction
        let height_to_realized_cap: EagerVec<PcoVec<Height, Dollars>> =
            EagerVec::forced_import(cfg.db, &cfg.name("realized_cap"), cfg.version)?;

        let indexes_to_realized_cap = DerivedComputedBlockLast::forced_import(
            cfg.db,
            &cfg.name("realized_cap"),
            height_to_realized_cap.boxed_clone(),
            cfg.version,
            cfg.indexes,
        )?;

        let height_to_realized_profit: EagerVec<PcoVec<Height, Dollars>> =
            EagerVec::forced_import(cfg.db, &cfg.name("realized_profit"), cfg.version)?;

        let indexes_to_realized_profit = DerivedComputedBlockSumCum::forced_import(
            cfg.db,
            &cfg.name("realized_profit"),
            height_to_realized_profit.boxed_clone(),
            cfg.version,
            cfg.indexes,
        )?;

        let indexes_to_net_realized_pnl = ComputedBlockSumCum::forced_import(
            cfg.db,
            &cfg.name("net_realized_pnl"),
            cfg.version,
            cfg.indexes,
        )?;

        // Construct lazy ratio vecs (before struct assignment to satisfy borrow checker)
        let indexes_to_realized_profit_rel_to_realized_cap =
            BinaryBlockSumCumLast::from_derived::<PercentageDollarsF32>(
                &cfg.name("realized_profit_rel_to_realized_cap"),
                cfg.version + v1,
                height_to_realized_profit.boxed_clone(),
                height_to_realized_cap.boxed_clone(),
                &indexes_to_realized_profit,
                &indexes_to_realized_cap,
            );

        let indexes_to_realized_loss_rel_to_realized_cap =
            BinaryBlockSumCumLast::from_derived::<PercentageDollarsF32>(
                &cfg.name("realized_loss_rel_to_realized_cap"),
                cfg.version + v1,
                height_to_realized_loss.boxed_clone(),
                height_to_realized_cap.boxed_clone(),
                &indexes_to_realized_loss,
                &indexes_to_realized_cap,
            );

        let indexes_to_net_realized_pnl_rel_to_realized_cap =
            BinaryBlockSumCumLast::from_computed_derived::<PercentageDollarsF32>(
                &cfg.name("net_realized_pnl_rel_to_realized_cap"),
                cfg.version + v1,
                indexes_to_net_realized_pnl.height.boxed_clone(),
                height_to_realized_cap.boxed_clone(),
                &indexes_to_net_realized_pnl,
                &indexes_to_realized_cap,
            );

        let indexes_to_realized_price = ComputedBlockLast::forced_import(
            cfg.db,
            &cfg.name("realized_price"),
            cfg.version + v1,
            cfg.indexes,
        )?;

        let height_to_value_created =
            EagerVec::forced_import(cfg.db, &cfg.name("value_created"), cfg.version)?;
        let height_to_value_destroyed =
            EagerVec::forced_import(cfg.db, &cfg.name("value_destroyed"), cfg.version)?;

        let indexes_to_value_created = DerivedComputedBlockSum::forced_import(
            cfg.db,
            &cfg.name("value_created"),
            height_to_value_created.boxed_clone(),
            cfg.version,
            cfg.indexes,
        )?;
        let indexes_to_value_destroyed = DerivedComputedBlockSum::forced_import(
            cfg.db,
            &cfg.name("value_destroyed"),
            height_to_value_destroyed.boxed_clone(),
            cfg.version,
            cfg.indexes,
        )?;

        // Create lazy adjusted vecs if compute_adjusted and up_to_1h is available
        let indexes_to_adjusted_value_created =
            (compute_adjusted && cfg.up_to_1h_realized.is_some()).then(|| {
                let up_to_1h = cfg.up_to_1h_realized.unwrap();
                BinaryBlockSum::from_derived::<DollarsMinus>(
                    &cfg.name("adjusted_value_created"),
                    cfg.version,
                    height_to_value_created.boxed_clone(),
                    up_to_1h.height_to_value_created.boxed_clone(),
                    &indexes_to_value_created,
                    &up_to_1h.indexes_to_value_created,
                )
            });
        let indexes_to_adjusted_value_destroyed =
            (compute_adjusted && cfg.up_to_1h_realized.is_some()).then(|| {
                let up_to_1h = cfg.up_to_1h_realized.unwrap();
                BinaryBlockSum::from_derived::<DollarsMinus>(
                    &cfg.name("adjusted_value_destroyed"),
                    cfg.version,
                    height_to_value_destroyed.boxed_clone(),
                    up_to_1h.height_to_value_destroyed.boxed_clone(),
                    &indexes_to_value_destroyed,
                    &up_to_1h.indexes_to_value_destroyed,
                )
            });

        // Create realized_price_extra first so we can reference its ratio for MVRV proxy
        let indexes_to_realized_price_extra = ComputedRatioVecsDate::forced_import(
            cfg.db,
            &cfg.name("realized_price"),
            Some(&indexes_to_realized_price),
            cfg.version + v1,
            cfg.indexes,
            extended,
            cfg.price,
        )?;

        // MVRV is a lazy proxy for realized_price_extra.ratio
        // ratio = close / realized_price = market_cap / realized_cap = MVRV
        let indexes_to_mvrv = LazyDateLast::from_source::<StoredF32Identity>(
            &cfg.name("mvrv"),
            cfg.version,
            &indexes_to_realized_price_extra.ratio,
        );

        Ok(Self {
            // === Realized Cap ===
            height_to_realized_cap,
            indexes_to_realized_cap,

            indexes_to_realized_price_extra,
            indexes_to_realized_price,
            indexes_to_mvrv,
            indexes_to_realized_cap_rel_to_own_market_cap: extended
                .then(|| {
                    ComputedBlockLast::forced_import(
                        cfg.db,
                        &cfg.name("realized_cap_rel_to_own_market_cap"),
                        cfg.version,
                        cfg.indexes,
                    )
                })
                .transpose()?,
            indexes_to_realized_cap_30d_delta: ComputedDateLast::forced_import(
                cfg.db,
                &cfg.name("realized_cap_30d_delta"),
                cfg.version,
                cfg.indexes,
            )?,

            // === Realized Profit/Loss ===
            height_to_realized_profit,
            indexes_to_realized_profit,
            height_to_realized_loss,
            indexes_to_realized_loss,
            indexes_to_neg_realized_loss,
            indexes_to_net_realized_pnl,
            indexes_to_realized_value,

            // === Realized vs Realized Cap Ratios (lazy) ===
            indexes_to_realized_profit_rel_to_realized_cap,
            indexes_to_realized_loss_rel_to_realized_cap,
            indexes_to_net_realized_pnl_rel_to_realized_cap,

            // === Total Realized PnL ===
            indexes_to_total_realized_pnl,
            dateindex_to_realized_profit_to_loss_ratio: extended
                .then(|| {
                    EagerVec::forced_import(
                        cfg.db,
                        &cfg.name("realized_profit_to_loss_ratio"),
                        cfg.version + v1,
                    )
                })
                .transpose()?,

            // === Value Created/Destroyed ===
            height_to_value_created,
            indexes_to_value_created,
            height_to_value_destroyed,
            indexes_to_value_destroyed,

            // === Adjusted Value (lazy: cohort - up_to_1h) ===
            indexes_to_adjusted_value_created,
            indexes_to_adjusted_value_destroyed,

            // === SOPR ===
            dateindex_to_sopr: EagerVec::forced_import(
                cfg.db,
                &cfg.name("sopr"),
                cfg.version + v1,
            )?,
            dateindex_to_sopr_7d_ema: EagerVec::forced_import(
                cfg.db,
                &cfg.name("sopr_7d_ema"),
                cfg.version + v1,
            )?,
            dateindex_to_sopr_30d_ema: EagerVec::forced_import(
                cfg.db,
                &cfg.name("sopr_30d_ema"),
                cfg.version + v1,
            )?,
            dateindex_to_adjusted_sopr: compute_adjusted
                .then(|| {
                    EagerVec::forced_import(cfg.db, &cfg.name("adjusted_sopr"), cfg.version + v1)
                })
                .transpose()?,
            dateindex_to_adjusted_sopr_7d_ema: compute_adjusted
                .then(|| {
                    EagerVec::forced_import(
                        cfg.db,
                        &cfg.name("adjusted_sopr_7d_ema"),
                        cfg.version + v1,
                    )
                })
                .transpose()?,
            dateindex_to_adjusted_sopr_30d_ema: compute_adjusted
                .then(|| {
                    EagerVec::forced_import(
                        cfg.db,
                        &cfg.name("adjusted_sopr_30d_ema"),
                        cfg.version + v1,
                    )
                })
                .transpose()?,

            // === Sell Side Risk ===
            dateindex_to_sell_side_risk_ratio: EagerVec::forced_import(
                cfg.db,
                &cfg.name("sell_side_risk_ratio"),
                cfg.version + v1,
            )?,
            dateindex_to_sell_side_risk_ratio_7d_ema: EagerVec::forced_import(
                cfg.db,
                &cfg.name("sell_side_risk_ratio_7d_ema"),
                cfg.version + v1,
            )?,
            dateindex_to_sell_side_risk_ratio_30d_ema: EagerVec::forced_import(
                cfg.db,
                &cfg.name("sell_side_risk_ratio_30d_ema"),
                cfg.version + v1,
            )?,

            // === Net Realized PnL Deltas ===
            indexes_to_net_realized_pnl_cumulative_30d_delta: ComputedDateLast::forced_import(
                cfg.db,
                &cfg.name("net_realized_pnl_cumulative_30d_delta"),
                cfg.version + v3,
                cfg.indexes,
            )?,
            indexes_to_net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap:
                ComputedDateLast::forced_import(
                    cfg.db,
                    &cfg.name("net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap"),
                    cfg.version + v3,
                    cfg.indexes,
                )?,
            indexes_to_net_realized_pnl_cumulative_30d_delta_rel_to_market_cap:
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
        self.height_to_realized_cap
            .len()
            .min(self.height_to_realized_profit.len())
            .min(self.height_to_realized_loss.len())
            .min(self.height_to_value_created.len())
            .min(self.height_to_value_destroyed.len())
    }

    /// Push realized state values to height-indexed vectors.
    pub fn truncate_push(&mut self, height: Height, state: &RealizedState) -> Result<()> {
        self.height_to_realized_cap
            .truncate_push(height, state.cap)?;
        self.height_to_realized_profit
            .truncate_push(height, state.profit)?;
        self.height_to_realized_loss
            .truncate_push(height, state.loss)?;
        self.height_to_value_created
            .truncate_push(height, state.value_created)?;
        self.height_to_value_destroyed
            .truncate_push(height, state.value_destroyed)?;

        Ok(())
    }

    /// Write height-indexed vectors to disk.
    pub fn write(&mut self) -> Result<()> {
        self.height_to_realized_cap.write()?;
        self.height_to_realized_profit.write()?;
        self.height_to_realized_loss.write()?;
        self.height_to_value_created.write()?;
        self.height_to_value_destroyed.write()?;
        Ok(())
    }

    /// Returns a parallel iterator over all vecs for parallel writing.
    pub fn par_iter_mut(&mut self) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        [
            &mut self.height_to_realized_cap as &mut dyn AnyStoredVec,
            &mut self.height_to_realized_profit,
            &mut self.height_to_realized_loss,
            &mut self.height_to_value_created,
            &mut self.height_to_value_destroyed,
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
        self.height_to_realized_cap.compute_sum_of_others(
            starting_indexes.height,
            &others
                .iter()
                .map(|v| &v.height_to_realized_cap)
                .collect::<Vec<_>>(),
            exit,
        )?;
        self.height_to_realized_profit.compute_sum_of_others(
            starting_indexes.height,
            &others
                .iter()
                .map(|v| &v.height_to_realized_profit)
                .collect::<Vec<_>>(),
            exit,
        )?;
        self.height_to_realized_loss.compute_sum_of_others(
            starting_indexes.height,
            &others
                .iter()
                .map(|v| &v.height_to_realized_loss)
                .collect::<Vec<_>>(),
            exit,
        )?;
        self.height_to_value_created.compute_sum_of_others(
            starting_indexes.height,
            &others
                .iter()
                .map(|v| &v.height_to_value_created)
                .collect::<Vec<_>>(),
            exit,
        )?;
        self.height_to_value_destroyed.compute_sum_of_others(
            starting_indexes.height,
            &others
                .iter()
                .map(|v| &v.height_to_value_destroyed)
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
        self.indexes_to_realized_cap.derive_from(
            indexes,
            starting_indexes,
            &self.height_to_realized_cap,
            exit,
        )?;

        self.indexes_to_realized_profit.derive_from(
            indexes,
            starting_indexes,
            &self.height_to_realized_profit,
            exit,
        )?;

        self.indexes_to_realized_loss.derive_from(
            indexes,
            starting_indexes,
            &self.height_to_realized_loss,
            exit,
        )?;

        // net_realized_pnl = profit - loss
        self.indexes_to_net_realized_pnl
            .compute_all(indexes, starting_indexes, exit, |vec| {
                vec.compute_subtract(
                    starting_indexes.height,
                    &self.height_to_realized_profit,
                    &self.height_to_realized_loss,
                    exit,
                )?;
                Ok(())
            })?;

        // realized_value = profit + loss
        // Note: total_realized_pnl is a lazy alias to realized_value since both
        // compute profit + loss with sum aggregation, making them identical.
        self.indexes_to_realized_value
            .compute_all(indexes, starting_indexes, exit, |vec| {
                vec.compute_add(
                    starting_indexes.height,
                    &self.height_to_realized_profit,
                    &self.height_to_realized_loss,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_value_created.derive_from(
            indexes,
            starting_indexes,
            &self.height_to_value_created,
            exit,
        )?;

        self.indexes_to_value_destroyed.derive_from(
            indexes,
            starting_indexes,
            &self.height_to_value_destroyed,
            exit,
        )?;

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
        self.indexes_to_realized_price
            .compute_all(indexes, starting_indexes, exit, |vec| {
                vec.compute_divide(
                    starting_indexes.height,
                    &self.height_to_realized_cap,
                    height_to_supply,
                    exit,
                )?;
                Ok(())
            })?;

        if let Some(price) = price {
            self.indexes_to_realized_price_extra.compute_rest(
                price,
                starting_indexes,
                exit,
                Some(&self.indexes_to_realized_price.dateindex.0),
            )?;
        }

        // realized_cap_30d_delta
        self.indexes_to_realized_cap_30d_delta
            .compute_all(starting_indexes, exit, |vec| {
                vec.compute_change(
                    starting_indexes.dateindex,
                    &self.indexes_to_realized_cap.dateindex.0,
                    30,
                    exit,
                )?;
                Ok(())
            })?;

        // SOPR = value_created / value_destroyed
        self.dateindex_to_sopr.compute_divide(
            starting_indexes.dateindex,
            &self.indexes_to_value_created.dateindex.0,
            &self.indexes_to_value_destroyed.dateindex.0,
            exit,
        )?;

        self.dateindex_to_sopr_7d_ema.compute_ema(
            starting_indexes.dateindex,
            &self.dateindex_to_sopr,
            7,
            exit,
        )?;

        self.dateindex_to_sopr_30d_ema.compute_ema(
            starting_indexes.dateindex,
            &self.dateindex_to_sopr,
            30,
            exit,
        )?;

        // Optional: adjusted SOPR (lazy: cohort - up_to_1h)
        if let (Some(adjusted_sopr), Some(adj_created), Some(adj_destroyed)) = (
            self.dateindex_to_adjusted_sopr.as_mut(),
            self.indexes_to_adjusted_value_created.as_ref(),
            self.indexes_to_adjusted_value_destroyed.as_ref(),
        ) {
            adjusted_sopr.compute_divide(
                starting_indexes.dateindex,
                &*adj_created.dateindex,
                &*adj_destroyed.dateindex,
                exit,
            )?;

            if let Some(ema_7d) = self.dateindex_to_adjusted_sopr_7d_ema.as_mut() {
                ema_7d.compute_ema(
                    starting_indexes.dateindex,
                    self.dateindex_to_adjusted_sopr.as_ref().unwrap(),
                    7,
                    exit,
                )?;
            }

            if let Some(ema_30d) = self.dateindex_to_adjusted_sopr_30d_ema.as_mut() {
                ema_30d.compute_ema(
                    starting_indexes.dateindex,
                    self.dateindex_to_adjusted_sopr.as_ref().unwrap(),
                    30,
                    exit,
                )?;
            }
        }

        // sell_side_risk_ratio = realized_value / realized_cap
        self.dateindex_to_sell_side_risk_ratio.compute_percentage(
            starting_indexes.dateindex,
            &self.indexes_to_realized_value.dateindex.0,
            &self.indexes_to_realized_cap.dateindex.0,
            exit,
        )?;

        self.dateindex_to_sell_side_risk_ratio_7d_ema.compute_ema(
            starting_indexes.dateindex,
            &self.dateindex_to_sell_side_risk_ratio,
            7,
            exit,
        )?;

        self.dateindex_to_sell_side_risk_ratio_30d_ema.compute_ema(
            starting_indexes.dateindex,
            &self.dateindex_to_sell_side_risk_ratio,
            30,
            exit,
        )?;

        // Net realized PnL cumulative 30d delta
        self.indexes_to_net_realized_pnl_cumulative_30d_delta
            .compute_all(starting_indexes, exit, |vec| {
                vec.compute_change(
                    starting_indexes.dateindex,
                    &self.indexes_to_net_realized_pnl.dateindex.cumulative.0,
                    30,
                    exit,
                )?;
                Ok(())
            })?;

        // Relative to realized cap
        self.indexes_to_net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap
            .compute_all(starting_indexes, exit, |vec| {
                vec.compute_percentage(
                    starting_indexes.dateindex,
                    &self
                        .indexes_to_net_realized_pnl_cumulative_30d_delta
                        .dateindex,
                    &self.indexes_to_realized_cap.dateindex.0,
                    exit,
                )?;
                Ok(())
            })?;

        // Relative to market cap
        if let Some(dateindex_to_market_cap) = dateindex_to_market_cap {
            self.indexes_to_net_realized_pnl_cumulative_30d_delta_rel_to_market_cap
                .compute_all(starting_indexes, exit, |vec| {
                    vec.compute_percentage(
                        starting_indexes.dateindex,
                        &self
                            .indexes_to_net_realized_pnl_cumulative_30d_delta
                            .dateindex,
                        dateindex_to_market_cap,
                        exit,
                    )?;
                    Ok(())
                })?;
        }

        // Optional: realized_cap_rel_to_own_market_cap
        if let (Some(rel_vec), Some(height_to_market_cap)) = (
            self.indexes_to_realized_cap_rel_to_own_market_cap.as_mut(),
            height_to_market_cap,
        ) {
            rel_vec.compute_all(indexes, starting_indexes, exit, |vec| {
                vec.compute_percentage(
                    starting_indexes.height,
                    &self.height_to_realized_cap,
                    height_to_market_cap,
                    exit,
                )?;
                Ok(())
            })?;
        }

        // Optional: realized_profit_to_loss_ratio
        if let Some(ratio) = self.dateindex_to_realized_profit_to_loss_ratio.as_mut() {
            ratio.compute_divide(
                starting_indexes.dateindex,
                &self.indexes_to_realized_profit.dateindex.sum.0,
                &self.indexes_to_realized_loss.dateindex.sum.0,
                exit,
            )?;
        }

        Ok(())
    }
}
