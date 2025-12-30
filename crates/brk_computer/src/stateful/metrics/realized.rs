use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Bitcoin, DateIndex, Dollars, Height, StoredF32, StoredF64, Version};
use rayon::prelude::*;
use vecdb::{
    AnyStoredVec, EagerVec, Exit, GenericStoredVec, Ident, ImportableVec, IterableCloneableVec,
    IterableVec, Negate, PcoVec,
};

use crate::{
    Indexes,
    grouped::{
        ComputedRatioVecsFromDateIndex, ComputedVecsFromDateIndex, ComputedVecsFromHeight,
        LazyVecsFrom2FromHeight, LazyVecsFromHeight, PercentageDollarsF32, Source,
        VecBuilderOptions,
    },
    indexes, price,
    stateful::state::RealizedState,
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
    pub indexes_to_neg_realized_loss: LazyVecsFromHeight<Dollars>,
    pub indexes_to_net_realized_pnl: ComputedVecsFromHeight<Dollars>,
    pub indexes_to_realized_value: ComputedVecsFromHeight<Dollars>,

    // === Realized vs Realized Cap Ratios (lazy) ===
    pub indexes_to_realized_profit_rel_to_realized_cap:
        LazyVecsFrom2FromHeight<StoredF32, Dollars, Dollars>,
    pub indexes_to_realized_loss_rel_to_realized_cap:
        LazyVecsFrom2FromHeight<StoredF32, Dollars, Dollars>,
    pub indexes_to_net_realized_pnl_rel_to_realized_cap:
        LazyVecsFrom2FromHeight<StoredF32, Dollars, Dollars>,

    // === Total Realized PnL ===
    pub indexes_to_total_realized_pnl: LazyVecsFromHeight<Dollars>,
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

        let height_to_realized_loss: EagerVec<PcoVec<Height, Dollars>> =
            EagerVec::forced_import(cfg.db, &cfg.name("realized_loss"), cfg.version + v0)?;

        let indexes_to_realized_loss = ComputedVecsFromHeight::forced_import(
            cfg.db,
            &cfg.name("realized_loss"),
            Source::Vec(height_to_realized_loss.boxed_clone()),
            cfg.version + v0,
            cfg.indexes,
            sum_cum,
        )?;

        let indexes_to_neg_realized_loss = LazyVecsFromHeight::from_computed::<Negate>(
            &cfg.name("neg_realized_loss"),
            cfg.version + v1,
            height_to_realized_loss.boxed_clone(),
            &indexes_to_realized_loss,
        );

        // realized_value is the source for total_realized_pnl (they're identical)
        let indexes_to_realized_value = ComputedVecsFromHeight::forced_import(
            cfg.db,
            &cfg.name("realized_value"),
            Source::Compute,
            cfg.version + v0,
            cfg.indexes,
            sum,
        )?;

        // total_realized_pnl is a lazy alias to realized_value
        let indexes_to_total_realized_pnl = LazyVecsFromHeight::from_computed::<Ident>(
            &cfg.name("total_realized_pnl"),
            cfg.version + v1,
            indexes_to_realized_value
                .height
                .as_ref()
                .unwrap()
                .boxed_clone(),
            &indexes_to_realized_value,
        );

        // Extract vecs needed for lazy ratio construction
        let height_to_realized_cap: EagerVec<PcoVec<Height, Dollars>> =
            EagerVec::forced_import(cfg.db, &cfg.name("realized_cap"), cfg.version + v0)?;

        let indexes_to_realized_cap = ComputedVecsFromHeight::forced_import(
            cfg.db,
            &cfg.name("realized_cap"),
            Source::Vec(height_to_realized_cap.boxed_clone()),
            cfg.version + v0,
            cfg.indexes,
            last,
        )?;

        let height_to_realized_profit: EagerVec<PcoVec<Height, Dollars>> =
            EagerVec::forced_import(cfg.db, &cfg.name("realized_profit"), cfg.version + v0)?;

        let indexes_to_realized_profit = ComputedVecsFromHeight::forced_import(
            cfg.db,
            &cfg.name("realized_profit"),
            Source::Vec(height_to_realized_profit.boxed_clone()),
            cfg.version + v0,
            cfg.indexes,
            sum_cum,
        )?;

        let indexes_to_net_realized_pnl = ComputedVecsFromHeight::forced_import(
            cfg.db,
            &cfg.name("net_realized_pnl"),
            Source::Compute,
            cfg.version + v0,
            cfg.indexes,
            sum_cum,
        )?;

        // Construct lazy ratio vecs (before struct assignment to satisfy borrow checker)
        let indexes_to_realized_profit_rel_to_realized_cap =
            LazyVecsFrom2FromHeight::from_computed::<PercentageDollarsF32>(
                &cfg.name("realized_profit_rel_to_realized_cap"),
                cfg.version + v1,
                height_to_realized_profit.boxed_clone(),
                height_to_realized_cap.boxed_clone(),
                &indexes_to_realized_profit,
                &indexes_to_realized_cap,
            );

        let indexes_to_realized_loss_rel_to_realized_cap =
            LazyVecsFrom2FromHeight::from_computed::<PercentageDollarsF32>(
                &cfg.name("realized_loss_rel_to_realized_cap"),
                cfg.version + v1,
                height_to_realized_loss.boxed_clone(),
                height_to_realized_cap.boxed_clone(),
                &indexes_to_realized_loss,
                &indexes_to_realized_cap,
            );

        let indexes_to_net_realized_pnl_rel_to_realized_cap =
            LazyVecsFrom2FromHeight::from_computed::<PercentageDollarsF32>(
                &cfg.name("net_realized_pnl_rel_to_realized_cap"),
                cfg.version + v1,
                indexes_to_net_realized_pnl
                    .height
                    .as_ref()
                    .unwrap()
                    .boxed_clone(),
                height_to_realized_cap.boxed_clone(),
                &indexes_to_net_realized_pnl,
                &indexes_to_realized_cap,
            );

        let indexes_to_realized_price = ComputedVecsFromHeight::forced_import(
            cfg.db,
            &cfg.name("realized_price"),
            Source::Compute,
            cfg.version + v0,
            cfg.indexes,
            last,
        )?;

        let height_to_value_created =
            EagerVec::forced_import(cfg.db, &cfg.name("value_created"), cfg.version + v0)?;
        let height_to_value_destroyed =
            EagerVec::forced_import(cfg.db, &cfg.name("value_destroyed"), cfg.version + v0)?;

        let height_to_adjusted_value_created = compute_adjusted
            .then(|| {
                EagerVec::forced_import(
                    cfg.db,
                    &cfg.name("adjusted_value_created"),
                    cfg.version + v0,
                )
            })
            .transpose()?;
        let height_to_adjusted_value_destroyed = compute_adjusted
            .then(|| {
                EagerVec::forced_import(
                    cfg.db,
                    &cfg.name("adjusted_value_destroyed"),
                    cfg.version + v0,
                )
            })
            .transpose()?;

        Ok(Self {
            // === Realized Cap ===
            height_to_realized_cap,
            indexes_to_realized_cap,

            indexes_to_realized_price_extra: ComputedRatioVecsFromDateIndex::forced_import(
                cfg.db,
                &cfg.name("realized_price"),
                Some(&indexes_to_realized_price),
                cfg.version + v0,
                cfg.indexes,
                extended,
                cfg.price,
            )?,
            indexes_to_realized_price,
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
            indexes_to_value_created: ComputedVecsFromHeight::forced_import(
                cfg.db,
                &cfg.name("value_created"),
                Source::Vec(height_to_value_created.boxed_clone()),
                cfg.version + v0,
                cfg.indexes,
                sum,
            )?,
            indexes_to_value_destroyed: ComputedVecsFromHeight::forced_import(
                cfg.db,
                &cfg.name("value_destroyed"),
                Source::Vec(height_to_value_destroyed.boxed_clone()),
                cfg.version + v0,
                cfg.indexes,
                sum,
            )?,
            height_to_value_created,
            height_to_value_destroyed,

            // === Adjusted Value (optional) ===
            indexes_to_adjusted_value_created: compute_adjusted
                .then(|| {
                    ComputedVecsFromHeight::forced_import(
                        cfg.db,
                        &cfg.name("adjusted_value_created"),
                        Source::Vec(
                            height_to_adjusted_value_created
                                .as_ref()
                                .unwrap()
                                .boxed_clone(),
                        ),
                        cfg.version + v0,
                        cfg.indexes,
                        sum,
                    )
                })
                .transpose()?,
            indexes_to_adjusted_value_destroyed: compute_adjusted
                .then(|| {
                    ComputedVecsFromHeight::forced_import(
                        cfg.db,
                        &cfg.name("adjusted_value_destroyed"),
                        Source::Vec(
                            height_to_adjusted_value_destroyed
                                .as_ref()
                                .unwrap()
                                .boxed_clone(),
                        ),
                        cfg.version + v0,
                        cfg.indexes,
                        sum,
                    )
                })
                .transpose()?,
            height_to_adjusted_value_created,
            height_to_adjusted_value_destroyed,

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

    /// Returns a parallel iterator over all vecs for parallel writing.
    pub fn par_iter_mut(&mut self) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        let mut vecs: Vec<&mut dyn AnyStoredVec> = vec![
            &mut self.height_to_realized_cap,
            &mut self.height_to_realized_profit,
            &mut self.height_to_realized_loss,
            &mut self.height_to_value_created,
            &mut self.height_to_value_destroyed,
        ];
        if let Some(v) = self.height_to_adjusted_value_created.as_mut() {
            vecs.push(v);
        }
        if let Some(v) = self.height_to_adjusted_value_destroyed.as_mut() {
            vecs.push(v);
        }
        vecs.into_par_iter()
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

        // Optional: adjusted value
        if let Some(adjusted_value_created) = self.indexes_to_adjusted_value_created.as_mut() {
            adjusted_value_created.compute_rest(
                indexes,
                starting_indexes,
                exit,
                self.height_to_adjusted_value_created.as_ref(),
            )?;
        }

        if let Some(adjusted_value_destroyed) = self.indexes_to_adjusted_value_destroyed.as_mut() {
            adjusted_value_destroyed.compute_rest(
                indexes,
                starting_indexes,
                exit,
                self.height_to_adjusted_value_destroyed.as_ref(),
            )?;
        }

        Ok(())
    }

    /// Second phase of computed metrics (realized price from realized cap / supply).
    #[allow(clippy::too_many_arguments)]
    pub fn compute_rest_part2(
        &mut self,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
        starting_indexes: &Indexes,
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
                Some(self.indexes_to_realized_price.dateindex.unwrap_last()),
            )?;
        }

        // realized_cap_30d_delta
        self.indexes_to_realized_cap_30d_delta
            .compute_all(starting_indexes, exit, |vec| {
                vec.compute_change(
                    starting_indexes.dateindex,
                    self.indexes_to_realized_cap.dateindex.unwrap_last(),
                    30,
                    exit,
                )?;
                Ok(())
            })?;

        // SOPR = value_created / value_destroyed
        self.dateindex_to_sopr.compute_divide(
            starting_indexes.dateindex,
            self.indexes_to_value_created.dateindex.unwrap_sum(),
            self.indexes_to_value_destroyed.dateindex.unwrap_sum(),
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

        // Optional: adjusted SOPR
        if let (Some(adjusted_sopr), Some(adj_created), Some(adj_destroyed)) = (
            self.dateindex_to_adjusted_sopr.as_mut(),
            self.indexes_to_adjusted_value_created.as_ref(),
            self.indexes_to_adjusted_value_destroyed.as_ref(),
        ) {
            adjusted_sopr.compute_divide(
                starting_indexes.dateindex,
                adj_created.dateindex.unwrap_sum(),
                adj_destroyed.dateindex.unwrap_sum(),
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
            self.indexes_to_realized_value.dateindex.unwrap_sum(),
            self.indexes_to_realized_cap.dateindex.unwrap_last(),
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
                    self.indexes_to_net_realized_pnl
                        .dateindex
                        .unwrap_cumulative(),
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
                    self.indexes_to_net_realized_pnl_cumulative_30d_delta
                        .dateindex
                        .u(),
                    self.indexes_to_realized_cap.dateindex.unwrap_last(),
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
                        self.indexes_to_net_realized_pnl_cumulative_30d_delta
                            .dateindex
                            .u(),
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
                self.indexes_to_realized_profit.dateindex.unwrap_sum(),
                self.indexes_to_realized_loss.dateindex.unwrap_sum(),
                exit,
            )?;
        }

        Ok(())
    }
}
