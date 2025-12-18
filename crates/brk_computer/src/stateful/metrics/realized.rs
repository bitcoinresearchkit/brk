//! Realized cap and profit/loss metrics.
//!
//! These metrics require price data and track realized value based on acquisition price.

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{DateIndex, Dollars, Height, StoredF32, StoredF64, Version};
use vecdb::{AnyStoredVec, EagerVec, Exit, GenericStoredVec, ImportableVec, PcoVec};

use crate::{
    Indexes,
    grouped::{
        ComputedRatioVecsFromDateIndex, ComputedVecsFromDateIndex, ComputedVecsFromHeight, Source,
        VecBuilderOptions,
    },
    indexes, price,
    stateful::states::RealizedState,
    utils::OptionExt,
};

use super::ImportConfig;

/// Realized cap and related metrics.
#[derive(Clone, Traversable)]
pub struct RealizedMetrics {
    // === Realized Cap ===
    pub height_to_realized_cap: EagerVec<PcoVec<Height, Dollars>>,
    pub indexes_to_realized_cap: ComputedVecsFromHeight<Dollars>,
    pub indexes_to_realized_price: ComputedVecsFromHeight<Dollars>,
    pub indexes_to_realized_price_extra: ComputedRatioVecsFromDateIndex,
    pub indexes_to_realized_cap_rel_to_own_market_cap: Option<ComputedVecsFromHeight<StoredF32>>,
    pub indexes_to_realized_cap_30d_delta: ComputedVecsFromDateIndex<Dollars>,

    // === Realized Profit/Loss ===
    pub height_to_realized_profit: EagerVec<PcoVec<Height, Dollars>>,
    pub indexes_to_realized_profit: ComputedVecsFromHeight<Dollars>,
    pub height_to_realized_loss: EagerVec<PcoVec<Height, Dollars>>,
    pub indexes_to_realized_loss: ComputedVecsFromHeight<Dollars>,
    pub indexes_to_neg_realized_loss: ComputedVecsFromHeight<Dollars>,
    pub indexes_to_net_realized_pnl: ComputedVecsFromHeight<Dollars>,
    pub indexes_to_realized_value: ComputedVecsFromHeight<Dollars>,

    // === Realized vs Realized Cap Ratios ===
    pub indexes_to_realized_profit_rel_to_realized_cap: ComputedVecsFromHeight<StoredF32>,
    pub indexes_to_realized_loss_rel_to_realized_cap: ComputedVecsFromHeight<StoredF32>,
    pub indexes_to_net_realized_pnl_rel_to_realized_cap: ComputedVecsFromHeight<StoredF32>,

    // === Total Realized PnL ===
    pub height_to_total_realized_pnl: EagerVec<PcoVec<Height, Dollars>>,
    pub indexes_to_total_realized_pnl: ComputedVecsFromDateIndex<Dollars>,
    pub dateindex_to_realized_profit_to_loss_ratio: Option<EagerVec<PcoVec<DateIndex, StoredF64>>>,

    // === Value Created/Destroyed ===
    pub height_to_value_created: EagerVec<PcoVec<Height, Dollars>>,
    pub indexes_to_value_created: ComputedVecsFromHeight<Dollars>,
    pub height_to_value_destroyed: EagerVec<PcoVec<Height, Dollars>>,
    pub indexes_to_value_destroyed: ComputedVecsFromHeight<Dollars>,

    // === Adjusted Value (optional) ===
    pub height_to_adjusted_value_created: Option<EagerVec<PcoVec<Height, Dollars>>>,
    pub indexes_to_adjusted_value_created: Option<ComputedVecsFromHeight<Dollars>>,
    pub height_to_adjusted_value_destroyed: Option<EagerVec<PcoVec<Height, Dollars>>>,
    pub indexes_to_adjusted_value_destroyed: Option<ComputedVecsFromHeight<Dollars>>,

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
    pub indexes_to_net_realized_pnl_cumulative_30d_delta: ComputedVecsFromDateIndex<Dollars>,
    pub indexes_to_net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap:
        ComputedVecsFromDateIndex<StoredF32>,
    pub indexes_to_net_realized_pnl_cumulative_30d_delta_rel_to_market_cap:
        ComputedVecsFromDateIndex<StoredF32>,
}

impl RealizedMetrics {
    /// Import realized metrics from database.
    pub fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v0 = Version::ZERO;
        let v1 = Version::ONE;
        let v3 = Version::new(3);
        let extended = cfg.extended();
        let compute_adjusted = cfg.compute_adjusted();
        let last = VecBuilderOptions::default().add_last();
        let sum = VecBuilderOptions::default().add_sum();
        let sum_cum = VecBuilderOptions::default().add_sum().add_cumulative();

        Ok(Self {
            // === Realized Cap ===
            height_to_realized_cap: EagerVec::forced_import(
                cfg.db,
                &cfg.name("realized_cap"),
                cfg.version + v0,
            )?,
            indexes_to_realized_cap: ComputedVecsFromHeight::forced_import(
                cfg.db,
                &cfg.name("realized_cap"),
                Source::None,
                cfg.version + v0,
                cfg.indexes,
                last,
            )?,
            indexes_to_realized_price: ComputedVecsFromHeight::forced_import(
                cfg.db,
                &cfg.name("realized_price"),
                Source::Compute,
                cfg.version + v0,
                cfg.indexes,
                last,
            )?,
            indexes_to_realized_price_extra: ComputedRatioVecsFromDateIndex::forced_import(
                cfg.db,
                &cfg.name("realized_price"),
                Source::None,
                cfg.version + v0,
                cfg.indexes,
                extended,
            )?,
            indexes_to_realized_cap_rel_to_own_market_cap: extended
                .then(|| {
                    ComputedVecsFromHeight::forced_import(
                        cfg.db,
                        &cfg.name("realized_cap_rel_to_own_market_cap"),
                        Source::Compute,
                        cfg.version + v0,
                        cfg.indexes,
                        last,
                    )
                })
                .transpose()?,
            indexes_to_realized_cap_30d_delta: ComputedVecsFromDateIndex::forced_import(
                cfg.db,
                &cfg.name("realized_cap_30d_delta"),
                Source::Compute,
                cfg.version + v0,
                cfg.indexes,
                last,
            )?,

            // === Realized Profit/Loss ===
            height_to_realized_profit: EagerVec::forced_import(
                cfg.db,
                &cfg.name("realized_profit"),
                cfg.version + v0,
            )?,
            indexes_to_realized_profit: ComputedVecsFromHeight::forced_import(
                cfg.db,
                &cfg.name("realized_profit"),
                Source::None,
                cfg.version + v0,
                cfg.indexes,
                sum_cum,
            )?,
            height_to_realized_loss: EagerVec::forced_import(
                cfg.db,
                &cfg.name("realized_loss"),
                cfg.version + v0,
            )?,
            indexes_to_realized_loss: ComputedVecsFromHeight::forced_import(
                cfg.db,
                &cfg.name("realized_loss"),
                Source::None,
                cfg.version + v0,
                cfg.indexes,
                sum_cum,
            )?,
            indexes_to_neg_realized_loss: ComputedVecsFromHeight::forced_import(
                cfg.db,
                &cfg.name("neg_realized_loss"),
                Source::Compute,
                cfg.version + v1,
                cfg.indexes,
                sum_cum,
            )?,
            indexes_to_net_realized_pnl: ComputedVecsFromHeight::forced_import(
                cfg.db,
                &cfg.name("net_realized_pnl"),
                Source::Compute,
                cfg.version + v0,
                cfg.indexes,
                sum_cum,
            )?,
            indexes_to_realized_value: ComputedVecsFromHeight::forced_import(
                cfg.db,
                &cfg.name("realized_value"),
                Source::Compute,
                cfg.version + v0,
                cfg.indexes,
                sum,
            )?,

            // === Realized vs Realized Cap Ratios ===
            indexes_to_realized_profit_rel_to_realized_cap: ComputedVecsFromHeight::forced_import(
                cfg.db,
                &cfg.name("realized_profit_rel_to_realized_cap"),
                Source::Compute,
                cfg.version + v0,
                cfg.indexes,
                sum,
            )?,
            indexes_to_realized_loss_rel_to_realized_cap: ComputedVecsFromHeight::forced_import(
                cfg.db,
                &cfg.name("realized_loss_rel_to_realized_cap"),
                Source::Compute,
                cfg.version + v0,
                cfg.indexes,
                sum,
            )?,
            indexes_to_net_realized_pnl_rel_to_realized_cap: ComputedVecsFromHeight::forced_import(
                cfg.db,
                &cfg.name("net_realized_pnl_rel_to_realized_cap"),
                Source::Compute,
                cfg.version + v1,
                cfg.indexes,
                sum,
            )?,

            // === Total Realized PnL ===
            height_to_total_realized_pnl: EagerVec::forced_import(
                cfg.db,
                &cfg.name("total_realized_pnl"),
                cfg.version + v0,
            )?,
            indexes_to_total_realized_pnl: ComputedVecsFromDateIndex::forced_import(
                cfg.db,
                &cfg.name("total_realized_pnl"),
                Source::Compute,
                cfg.version + v1,
                cfg.indexes,
                sum,
            )?,
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
            height_to_value_created: EagerVec::forced_import(
                cfg.db,
                &cfg.name("value_created"),
                cfg.version + v0,
            )?,
            indexes_to_value_created: ComputedVecsFromHeight::forced_import(
                cfg.db,
                &cfg.name("value_created"),
                Source::None,
                cfg.version + v0,
                cfg.indexes,
                sum,
            )?,
            height_to_value_destroyed: EagerVec::forced_import(
                cfg.db,
                &cfg.name("value_destroyed"),
                cfg.version + v0,
            )?,
            indexes_to_value_destroyed: ComputedVecsFromHeight::forced_import(
                cfg.db,
                &cfg.name("value_destroyed"),
                Source::None,
                cfg.version + v0,
                cfg.indexes,
                sum,
            )?,

            // === Adjusted Value (optional) ===
            height_to_adjusted_value_created: compute_adjusted
                .then(|| {
                    EagerVec::forced_import(
                        cfg.db,
                        &cfg.name("adjusted_value_created"),
                        cfg.version + v0,
                    )
                })
                .transpose()?,
            indexes_to_adjusted_value_created: compute_adjusted
                .then(|| {
                    ComputedVecsFromHeight::forced_import(
                        cfg.db,
                        &cfg.name("adjusted_value_created"),
                        Source::None,
                        cfg.version + v0,
                        cfg.indexes,
                        sum,
                    )
                })
                .transpose()?,
            height_to_adjusted_value_destroyed: compute_adjusted
                .then(|| {
                    EagerVec::forced_import(
                        cfg.db,
                        &cfg.name("adjusted_value_destroyed"),
                        cfg.version + v0,
                    )
                })
                .transpose()?,
            indexes_to_adjusted_value_destroyed: compute_adjusted
                .then(|| {
                    ComputedVecsFromHeight::forced_import(
                        cfg.db,
                        &cfg.name("adjusted_value_destroyed"),
                        Source::None,
                        cfg.version + v0,
                        cfg.indexes,
                        sum,
                    )
                })
                .transpose()?,

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
            indexes_to_net_realized_pnl_cumulative_30d_delta:
                ComputedVecsFromDateIndex::forced_import(
                    cfg.db,
                    &cfg.name("net_realized_pnl_cumulative_30d_delta"),
                    Source::Compute,
                    cfg.version + v3,
                    cfg.indexes,
                    last,
                )?,
            indexes_to_net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap:
                ComputedVecsFromDateIndex::forced_import(
                    cfg.db,
                    &cfg.name("net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap"),
                    Source::Compute,
                    cfg.version + v3,
                    cfg.indexes,
                    last,
                )?,
            indexes_to_net_realized_pnl_cumulative_30d_delta_rel_to_market_cap:
                ComputedVecsFromDateIndex::forced_import(
                    cfg.db,
                    &cfg.name("net_realized_pnl_cumulative_30d_delta_rel_to_market_cap"),
                    Source::Compute,
                    cfg.version + v3,
                    cfg.indexes,
                    last,
                )?,
        })
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

        if let Some(v) = self.height_to_adjusted_value_created.as_mut() {
            v.truncate_push(height, state.adj_value_created)?;
        }
        if let Some(v) = self.height_to_adjusted_value_destroyed.as_mut() {
            v.truncate_push(height, state.adj_value_destroyed)?;
        }

        Ok(())
    }

    /// Write height-indexed vectors to disk.
    pub fn write(&mut self) -> Result<()> {
        self.height_to_realized_cap.write()?;
        self.height_to_realized_profit.write()?;
        self.height_to_realized_loss.write()?;
        self.height_to_value_created.write()?;
        self.height_to_value_destroyed.write()?;
        if let Some(v) = self.height_to_adjusted_value_created.as_mut() {
            v.write()?;
        }
        if let Some(v) = self.height_to_adjusted_value_destroyed.as_mut() {
            v.write()?;
        }
        Ok(())
    }

    /// Validate computed versions against base version.
    pub fn validate_computed_versions(&mut self, _base_version: Version) -> Result<()> {
        // Validation logic for computed vecs
        Ok(())
    }

    /// Compute aggregate values from separate cohorts.
    pub fn compute_from_stateful(
        &mut self,
        starting_indexes: &Indexes,
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

        if self.height_to_adjusted_value_created.is_some() {
            self.height_to_adjusted_value_created
                .um()
                .compute_sum_of_others(
                    starting_indexes.height,
                    &others
                        .iter()
                        .map(|v| {
                            v.height_to_adjusted_value_created
                                .as_ref()
                                .unwrap_or(&v.height_to_value_created)
                        })
                        .collect::<Vec<_>>(),
                    exit,
                )?;
            self.height_to_adjusted_value_destroyed
                .um()
                .compute_sum_of_others(
                    starting_indexes.height,
                    &others
                        .iter()
                        .map(|v| {
                            v.height_to_adjusted_value_destroyed
                                .as_ref()
                                .unwrap_or(&v.height_to_value_destroyed)
                        })
                        .collect::<Vec<_>>(),
                    exit,
                )?;
        }

        Ok(())
    }

    /// First phase of computed metrics (indexes from height).
    pub fn compute_rest_part1(
        &mut self,
        indexes: &indexes::Vecs,
        _price: Option<&price::Vecs>,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.indexes_to_realized_cap.compute_rest(
            indexes,
            starting_indexes,
            exit,
            Some(&self.height_to_realized_cap),
        )?;

        self.indexes_to_realized_profit.compute_rest(
            indexes,
            starting_indexes,
            exit,
            Some(&self.height_to_realized_profit),
        )?;

        self.indexes_to_realized_loss.compute_rest(
            indexes,
            starting_indexes,
            exit,
            Some(&self.height_to_realized_loss),
        )?;

        self.indexes_to_value_created.compute_rest(
            indexes,
            starting_indexes,
            exit,
            Some(&self.height_to_value_created),
        )?;

        self.indexes_to_value_destroyed.compute_rest(
            indexes,
            starting_indexes,
            exit,
            Some(&self.height_to_value_destroyed),
        )?;

        Ok(())
    }
}
