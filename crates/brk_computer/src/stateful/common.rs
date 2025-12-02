use brk_error::{Error, Result};
use brk_grouper::{CohortContext, Filter};
use brk_traversable::Traversable;
use brk_types::{
    Bitcoin, DateIndex, Dollars, Height, Sats, StoredF32, StoredF64, StoredU64, Version,
};
use vecdb::{
    AnyStoredVec, AnyVec, Database, EagerVec, Exit, GenericStoredVec, ImportableVec,
    IterableCloneableVec, IterableVec, PcoVec, TypedVecIterator,
};

use crate::{
    Indexes,
    grouped::{
        ComputedHeightValueVecs, ComputedRatioVecsFromDateIndex, ComputedValueVecsFromDateIndex,
        ComputedValueVecsFromHeight, ComputedVecsFromDateIndex, ComputedVecsFromHeight,
        PricePercentiles, Source, VecBuilderOptions,
    },
    indexes, price,
    states::CohortState,
};

#[derive(Clone, Traversable)]
pub struct Vecs {
    #[traversable(skip)]
    pub filter: Filter,

    // Cumulative
    pub height_to_realized_cap: Option<EagerVec<PcoVec<Height, Dollars>>>,
    pub height_to_supply: EagerVec<PcoVec<Height, Sats>>,
    pub height_to_utxo_count: EagerVec<PcoVec<Height, StoredU64>>,
    // Single
    pub dateindex_to_supply_in_loss: Option<EagerVec<PcoVec<DateIndex, Sats>>>,
    pub dateindex_to_supply_in_profit: Option<EagerVec<PcoVec<DateIndex, Sats>>>,
    pub dateindex_to_unrealized_loss: Option<EagerVec<PcoVec<DateIndex, Dollars>>>,
    pub dateindex_to_unrealized_profit: Option<EagerVec<PcoVec<DateIndex, Dollars>>>,
    pub height_to_adjusted_value_created: Option<EagerVec<PcoVec<Height, Dollars>>>,
    pub height_to_adjusted_value_destroyed: Option<EagerVec<PcoVec<Height, Dollars>>>,
    pub height_to_max_price_paid: Option<EagerVec<PcoVec<Height, Dollars>>>,
    pub height_to_min_price_paid: Option<EagerVec<PcoVec<Height, Dollars>>>,
    pub height_to_realized_loss: Option<EagerVec<PcoVec<Height, Dollars>>>,
    pub height_to_realized_profit: Option<EagerVec<PcoVec<Height, Dollars>>>,
    pub height_to_supply_in_loss: Option<EagerVec<PcoVec<Height, Sats>>>,
    pub height_to_supply_in_profit: Option<EagerVec<PcoVec<Height, Sats>>>,
    pub height_to_unrealized_loss: Option<EagerVec<PcoVec<Height, Dollars>>>,
    pub height_to_unrealized_profit: Option<EagerVec<PcoVec<Height, Dollars>>>,
    pub height_to_value_created: Option<EagerVec<PcoVec<Height, Dollars>>>,
    pub height_to_value_destroyed: Option<EagerVec<PcoVec<Height, Dollars>>>,
    pub height_to_sent: EagerVec<PcoVec<Height, Sats>>,
    pub height_to_satblocks_destroyed: EagerVec<PcoVec<Height, Sats>>,
    pub height_to_satdays_destroyed: EagerVec<PcoVec<Height, Sats>>,

    pub indexes_to_sent: ComputedValueVecsFromHeight,
    pub indexes_to_coinblocks_destroyed: ComputedVecsFromHeight<StoredF64>,
    pub indexes_to_coindays_destroyed: ComputedVecsFromHeight<StoredF64>,
    pub dateindex_to_sopr: Option<EagerVec<PcoVec<DateIndex, StoredF64>>>,
    pub dateindex_to_sopr_7d_ema: Option<EagerVec<PcoVec<DateIndex, StoredF64>>>,
    pub dateindex_to_sopr_30d_ema: Option<EagerVec<PcoVec<DateIndex, StoredF64>>>,
    pub dateindex_to_adjusted_sopr: Option<EagerVec<PcoVec<DateIndex, StoredF64>>>,
    pub dateindex_to_adjusted_sopr_7d_ema: Option<EagerVec<PcoVec<DateIndex, StoredF64>>>,
    pub dateindex_to_adjusted_sopr_30d_ema: Option<EagerVec<PcoVec<DateIndex, StoredF64>>>,
    pub indexes_to_realized_cap_30d_delta: Option<ComputedVecsFromDateIndex<Dollars>>,
    pub dateindex_to_sell_side_risk_ratio: Option<EagerVec<PcoVec<DateIndex, StoredF32>>>,
    pub dateindex_to_sell_side_risk_ratio_7d_ema: Option<EagerVec<PcoVec<DateIndex, StoredF32>>>,
    pub dateindex_to_sell_side_risk_ratio_30d_ema: Option<EagerVec<PcoVec<DateIndex, StoredF32>>>,
    pub indexes_to_adjusted_value_created: Option<ComputedVecsFromHeight<Dollars>>,
    pub indexes_to_adjusted_value_destroyed: Option<ComputedVecsFromHeight<Dollars>>,
    pub indexes_to_neg_realized_loss: Option<ComputedVecsFromHeight<Dollars>>,
    pub indexes_to_net_realized_pnl: Option<ComputedVecsFromHeight<Dollars>>,
    pub indexes_to_realized_cap: Option<ComputedVecsFromHeight<Dollars>>,
    pub indexes_to_realized_loss: Option<ComputedVecsFromHeight<Dollars>>,
    pub indexes_to_realized_price: Option<ComputedVecsFromHeight<Dollars>>,
    pub indexes_to_realized_price_extra: Option<ComputedRatioVecsFromDateIndex>,
    pub indexes_to_realized_profit: Option<ComputedVecsFromHeight<Dollars>>,
    pub indexes_to_realized_value: Option<ComputedVecsFromHeight<Dollars>>,
    pub height_to_supply_value: ComputedHeightValueVecs,
    pub indexes_to_supply: ComputedValueVecsFromDateIndex,
    pub indexes_to_utxo_count: ComputedVecsFromHeight<StoredU64>,
    pub indexes_to_value_created: Option<ComputedVecsFromHeight<Dollars>>,
    pub indexes_to_value_destroyed: Option<ComputedVecsFromHeight<Dollars>>,
    pub indexes_to_unrealized_profit: Option<ComputedVecsFromDateIndex<Dollars>>,
    pub indexes_to_unrealized_loss: Option<ComputedVecsFromDateIndex<Dollars>>,
    pub height_to_total_unrealized_pnl: Option<EagerVec<PcoVec<Height, Dollars>>>,
    pub indexes_to_total_unrealized_pnl: Option<ComputedVecsFromDateIndex<Dollars>>,
    pub height_to_total_realized_pnl: Option<EagerVec<PcoVec<Height, Dollars>>>,
    pub indexes_to_total_realized_pnl: Option<ComputedVecsFromDateIndex<Dollars>>,
    pub indexes_to_min_price_paid: Option<ComputedVecsFromHeight<Dollars>>,
    pub indexes_to_max_price_paid: Option<ComputedVecsFromHeight<Dollars>>,
    pub price_percentiles: Option<PricePercentiles>,
    pub height_to_supply_half_value: ComputedHeightValueVecs,
    pub indexes_to_supply_half: ComputedValueVecsFromDateIndex,
    pub height_to_neg_unrealized_loss: Option<EagerVec<PcoVec<Height, Dollars>>>,
    pub indexes_to_neg_unrealized_loss: Option<ComputedVecsFromDateIndex<Dollars>>,
    pub height_to_net_unrealized_pnl: Option<EagerVec<PcoVec<Height, Dollars>>>,
    pub indexes_to_net_unrealized_pnl: Option<ComputedVecsFromDateIndex<Dollars>>,
    pub height_to_unrealized_profit_rel_to_market_cap: Option<EagerVec<PcoVec<Height, StoredF32>>>,
    pub height_to_unrealized_loss_rel_to_market_cap: Option<EagerVec<PcoVec<Height, StoredF32>>>,
    pub height_to_neg_unrealized_loss_rel_to_market_cap:
        Option<EagerVec<PcoVec<Height, StoredF32>>>,
    pub height_to_net_unrealized_pnl_rel_to_market_cap: Option<EagerVec<PcoVec<Height, StoredF32>>>,
    pub indexes_to_unrealized_profit_rel_to_market_cap:
        Option<ComputedVecsFromDateIndex<StoredF32>>,
    pub indexes_to_unrealized_loss_rel_to_market_cap: Option<ComputedVecsFromDateIndex<StoredF32>>,
    pub indexes_to_neg_unrealized_loss_rel_to_market_cap:
        Option<ComputedVecsFromDateIndex<StoredF32>>,
    pub indexes_to_net_unrealized_pnl_rel_to_market_cap:
        Option<ComputedVecsFromDateIndex<StoredF32>>,
    pub height_to_unrealized_profit_rel_to_own_market_cap:
        Option<EagerVec<PcoVec<Height, StoredF32>>>,
    pub height_to_unrealized_loss_rel_to_own_market_cap:
        Option<EagerVec<PcoVec<Height, StoredF32>>>,
    pub height_to_neg_unrealized_loss_rel_to_own_market_cap:
        Option<EagerVec<PcoVec<Height, StoredF32>>>,
    pub height_to_net_unrealized_pnl_rel_to_own_market_cap:
        Option<EagerVec<PcoVec<Height, StoredF32>>>,
    pub indexes_to_unrealized_profit_rel_to_own_market_cap:
        Option<ComputedVecsFromDateIndex<StoredF32>>,
    pub indexes_to_unrealized_loss_rel_to_own_market_cap:
        Option<ComputedVecsFromDateIndex<StoredF32>>,
    pub indexes_to_neg_unrealized_loss_rel_to_own_market_cap:
        Option<ComputedVecsFromDateIndex<StoredF32>>,
    pub indexes_to_net_unrealized_pnl_rel_to_own_market_cap:
        Option<ComputedVecsFromDateIndex<StoredF32>>,
    pub height_to_unrealized_profit_rel_to_own_total_unrealized_pnl:
        Option<EagerVec<PcoVec<Height, StoredF32>>>,
    pub height_to_unrealized_loss_rel_to_own_total_unrealized_pnl:
        Option<EagerVec<PcoVec<Height, StoredF32>>>,
    pub height_to_neg_unrealized_loss_rel_to_own_total_unrealized_pnl:
        Option<EagerVec<PcoVec<Height, StoredF32>>>,
    pub height_to_net_unrealized_pnl_rel_to_own_total_unrealized_pnl:
        Option<EagerVec<PcoVec<Height, StoredF32>>>,
    pub indexes_to_unrealized_profit_rel_to_own_total_unrealized_pnl:
        Option<ComputedVecsFromDateIndex<StoredF32>>,
    pub indexes_to_unrealized_loss_rel_to_own_total_unrealized_pnl:
        Option<ComputedVecsFromDateIndex<StoredF32>>,
    pub indexes_to_neg_unrealized_loss_rel_to_own_total_unrealized_pnl:
        Option<ComputedVecsFromDateIndex<StoredF32>>,
    pub indexes_to_net_unrealized_pnl_rel_to_own_total_unrealized_pnl:
        Option<ComputedVecsFromDateIndex<StoredF32>>,
    pub indexes_to_realized_cap_rel_to_own_market_cap: Option<ComputedVecsFromHeight<StoredF32>>,
    pub indexes_to_realized_profit_rel_to_realized_cap: Option<ComputedVecsFromHeight<StoredF32>>,
    pub indexes_to_realized_loss_rel_to_realized_cap: Option<ComputedVecsFromHeight<StoredF32>>,
    pub indexes_to_net_realized_pnl_rel_to_realized_cap: Option<ComputedVecsFromHeight<StoredF32>>,
    pub height_to_supply_in_loss_value: Option<ComputedHeightValueVecs>,
    pub height_to_supply_in_profit_value: Option<ComputedHeightValueVecs>,
    pub indexes_to_supply_in_loss: Option<ComputedValueVecsFromDateIndex>,
    pub indexes_to_supply_in_profit: Option<ComputedValueVecsFromDateIndex>,
    pub height_to_supply_in_loss_rel_to_own_supply: Option<EagerVec<PcoVec<Height, StoredF64>>>,
    pub height_to_supply_in_profit_rel_to_own_supply: Option<EagerVec<PcoVec<Height, StoredF64>>>,
    pub indexes_to_supply_in_loss_rel_to_own_supply: Option<ComputedVecsFromDateIndex<StoredF64>>,
    pub indexes_to_supply_in_profit_rel_to_own_supply: Option<ComputedVecsFromDateIndex<StoredF64>>,
    pub indexes_to_supply_rel_to_circulating_supply: Option<ComputedVecsFromHeight<StoredF64>>,
    pub height_to_supply_in_loss_rel_to_circulating_supply:
        Option<EagerVec<PcoVec<Height, StoredF64>>>,
    pub height_to_supply_in_profit_rel_to_circulating_supply:
        Option<EagerVec<PcoVec<Height, StoredF64>>>,
    pub indexes_to_supply_in_loss_rel_to_circulating_supply:
        Option<ComputedVecsFromDateIndex<StoredF64>>,
    pub indexes_to_supply_in_profit_rel_to_circulating_supply:
        Option<ComputedVecsFromDateIndex<StoredF64>>,
    pub indexes_to_net_realized_pnl_cumulative_30d_delta:
        Option<ComputedVecsFromDateIndex<Dollars>>,
    pub indexes_to_net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap:
        Option<ComputedVecsFromDateIndex<StoredF32>>,
    pub indexes_to_net_realized_pnl_cumulative_30d_delta_rel_to_market_cap:
        Option<ComputedVecsFromDateIndex<StoredF32>>,
    pub dateindex_to_realized_profit_to_loss_ratio: Option<EagerVec<PcoVec<DateIndex, StoredF64>>>,
}

impl Vecs {
    #[allow(clippy::too_many_arguments)]
    pub fn forced_import(
        db: &Database,
        filter: Filter,
        context: CohortContext,
        parent_version: Version,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
        extended: bool,
        compute_rel_to_all: bool,
        compute_adjusted: bool,
    ) -> Result<Self> {
        let compute_dollars = price.is_some();

        let version = parent_version + Version::ZERO;

        let name_prefix = filter.to_full_name(context);
        let suffix = |s: &str| {
            if name_prefix.is_empty() {
                s.to_string()
            } else {
                format!("{name_prefix}_{s}")
            }
        };

        let dateindex_to_supply_in_profit = compute_dollars.then(|| {
            EagerVec::forced_import(db, &suffix("supply_in_profit"), version + Version::ZERO)
                .unwrap()
        });

        let dateindex_to_supply_in_loss = compute_dollars.then(|| {
            EagerVec::forced_import(db, &suffix("supply_in_loss"), version + Version::ZERO).unwrap()
        });

        let dateindex_to_unrealized_profit = compute_dollars.then(|| {
            EagerVec::forced_import(db, &suffix("unrealized_profit"), version + Version::ZERO)
                .unwrap()
        });

        let dateindex_to_unrealized_loss = compute_dollars.then(|| {
            EagerVec::forced_import(db, &suffix("unrealized_loss"), version + Version::ZERO)
                .unwrap()
        });

        Ok(Self {
            filter,

            height_to_supply_in_profit: compute_dollars.then(|| {
                EagerVec::forced_import(db, &suffix("supply_in_profit"), version + Version::ZERO)
                    .unwrap()
            }),
            indexes_to_supply_in_profit: compute_dollars.then(|| {
                ComputedValueVecsFromDateIndex::forced_import(
                    db,
                    &suffix("supply_in_profit"),
                    dateindex_to_supply_in_profit
                        .as_ref()
                        .map(|v| v.boxed_clone())
                        .into(),
                    version + Version::ZERO,
                    VecBuilderOptions::default().add_last(),
                    compute_dollars,
                    indexes,
                )
                .unwrap()
            }),
            dateindex_to_supply_in_profit,
            height_to_supply_in_loss: compute_dollars.then(|| {
                EagerVec::forced_import(db, &suffix("supply_in_loss"), version + Version::ZERO)
                    .unwrap()
            }),
            indexes_to_supply_in_loss: compute_dollars.then(|| {
                ComputedValueVecsFromDateIndex::forced_import(
                    db,
                    &suffix("supply_in_loss"),
                    dateindex_to_supply_in_loss
                        .as_ref()
                        .map(|v| v.boxed_clone())
                        .into(),
                    version + Version::ZERO,
                    VecBuilderOptions::default().add_last(),
                    compute_dollars,
                    indexes,
                )
                .unwrap()
            }),
            dateindex_to_supply_in_loss,
            height_to_unrealized_profit: compute_dollars.then(|| {
                EagerVec::forced_import(db, &suffix("unrealized_profit"), version + Version::ZERO)
                    .unwrap()
            }),
            indexes_to_unrealized_profit: compute_dollars.then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    db,
                    &suffix("unrealized_profit"),
                    dateindex_to_unrealized_profit
                        .as_ref()
                        .map(|v| v.boxed_clone())
                        .into(),
                    version + Version::ZERO,
                    indexes,
                    VecBuilderOptions::default().add_last(),
                )
                .unwrap()
            }),
            dateindex_to_unrealized_profit,
            height_to_unrealized_loss: compute_dollars.then(|| {
                EagerVec::forced_import(db, &suffix("unrealized_loss"), version + Version::ZERO)
                    .unwrap()
            }),
            height_to_min_price_paid: compute_dollars.then(|| {
                EagerVec::forced_import(db, &suffix("min_price_paid"), version + Version::ZERO)
                    .unwrap()
            }),
            height_to_max_price_paid: compute_dollars.then(|| {
                EagerVec::forced_import(db, &suffix("max_price_paid"), version + Version::ZERO)
                    .unwrap()
            }),
            indexes_to_unrealized_loss: compute_dollars.then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    db,
                    &suffix("unrealized_loss"),
                    dateindex_to_unrealized_loss
                        .as_ref()
                        .map(|v| v.boxed_clone())
                        .into(),
                    version + Version::ZERO,
                    indexes,
                    VecBuilderOptions::default().add_last(),
                )
                .unwrap()
            }),
            height_to_total_unrealized_pnl: compute_dollars.then(|| {
                EagerVec::forced_import(
                    db,
                    &suffix("total_unrealized_pnl"),
                    version + Version::ZERO,
                )
                .unwrap()
            }),
            indexes_to_total_unrealized_pnl: compute_dollars.then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    db,
                    &suffix("total_unrealized_pnl"),
                    Source::Compute,
                    version + Version::ZERO,
                    indexes,
                    VecBuilderOptions::default().add_last(),
                )
                .unwrap()
            }),
            height_to_total_realized_pnl: compute_dollars.then(|| {
                EagerVec::forced_import(db, &suffix("total_realized_pnl"), version + Version::ZERO)
                    .unwrap()
            }),
            indexes_to_total_realized_pnl: compute_dollars.then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    db,
                    &suffix("total_realized_pnl"),
                    Source::Compute,
                    version + Version::ONE,
                    indexes,
                    VecBuilderOptions::default().add_sum(),
                )
                .unwrap()
            }),
            dateindex_to_unrealized_loss,
            height_to_realized_cap: compute_dollars.then(|| {
                EagerVec::forced_import(db, &suffix("realized_cap"), version + Version::ZERO)
                    .unwrap()
            }),
            indexes_to_realized_cap: compute_dollars.then(|| {
                ComputedVecsFromHeight::forced_import(
                    db,
                    &suffix("realized_cap"),
                    Source::None,
                    version + Version::ZERO,
                    indexes,
                    VecBuilderOptions::default().add_last(),
                )
                .unwrap()
            }),
            indexes_to_min_price_paid: compute_dollars.then(|| {
                ComputedVecsFromHeight::forced_import(
                    db,
                    &suffix("min_price_paid"),
                    Source::None,
                    version + Version::ZERO,
                    indexes,
                    VecBuilderOptions::default().add_last(),
                )
                .unwrap()
            }),
            indexes_to_max_price_paid: compute_dollars.then(|| {
                ComputedVecsFromHeight::forced_import(
                    db,
                    &suffix("max_price_paid"),
                    Source::None,
                    version + Version::ZERO,
                    indexes,
                    VecBuilderOptions::default().add_last(),
                )
                .unwrap()
            }),
            price_percentiles: (compute_dollars && extended).then(|| {
                PricePercentiles::forced_import(
                    db,
                    &suffix(""),
                    version + Version::ZERO,
                    indexes,
                    true,
                )
                .unwrap()
            }),
            height_to_supply: EagerVec::forced_import(
                db,
                &suffix("supply"),
                version + Version::ZERO,
            )?,
            height_to_supply_value: ComputedHeightValueVecs::forced_import(
                db,
                &suffix("supply"),
                Source::None,
                version + Version::ZERO,
                compute_dollars,
            )?,
            indexes_to_supply: ComputedValueVecsFromDateIndex::forced_import(
                db,
                &suffix("supply"),
                Source::Compute,
                version + Version::ONE,
                VecBuilderOptions::default().add_last(),
                compute_dollars,
                indexes,
            )?,
            height_to_utxo_count: EagerVec::forced_import(
                db,
                &suffix("utxo_count"),
                version + Version::ZERO,
            )?,
            indexes_to_utxo_count: ComputedVecsFromHeight::forced_import(
                db,
                &suffix("utxo_count"),
                Source::None,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            indexes_to_realized_price: compute_dollars.then(|| {
                ComputedVecsFromHeight::forced_import(
                    db,
                    &suffix("realized_price"),
                    Source::Compute,
                    version + Version::ZERO,
                    indexes,
                    VecBuilderOptions::default().add_last(),
                )
                .unwrap()
            }),
            indexes_to_realized_price_extra: compute_dollars.then(|| {
                ComputedRatioVecsFromDateIndex::forced_import(
                    db,
                    &suffix("realized_price"),
                    Source::None,
                    version + Version::ZERO,
                    indexes,
                    extended,
                )
                .unwrap()
            }),
            indexes_to_realized_cap_rel_to_own_market_cap: (compute_dollars && extended).then(
                || {
                    ComputedVecsFromHeight::forced_import(
                        db,
                        &suffix("realized_cap_rel_to_own_market_cap"),
                        Source::Compute,
                        version + Version::ZERO,
                        indexes,
                        VecBuilderOptions::default().add_last(),
                    )
                    .unwrap()
                },
            ),
            height_to_realized_profit: compute_dollars.then(|| {
                EagerVec::forced_import(db, &suffix("realized_profit"), version + Version::ZERO)
                    .unwrap()
            }),
            indexes_to_realized_profit: compute_dollars.then(|| {
                ComputedVecsFromHeight::forced_import(
                    db,
                    &suffix("realized_profit"),
                    Source::None,
                    version + Version::ZERO,
                    indexes,
                    VecBuilderOptions::default().add_sum().add_cumulative(),
                )
                .unwrap()
            }),
            height_to_realized_loss: compute_dollars.then(|| {
                EagerVec::forced_import(db, &suffix("realized_loss"), version + Version::ZERO)
                    .unwrap()
            }),
            indexes_to_realized_loss: compute_dollars.then(|| {
                ComputedVecsFromHeight::forced_import(
                    db,
                    &suffix("realized_loss"),
                    Source::None,
                    version + Version::ZERO,
                    indexes,
                    VecBuilderOptions::default().add_sum().add_cumulative(),
                )
                .unwrap()
            }),
            indexes_to_neg_realized_loss: compute_dollars.then(|| {
                ComputedVecsFromHeight::forced_import(
                    db,
                    &suffix("neg_realized_loss"),
                    Source::Compute,
                    version + Version::ONE,
                    indexes,
                    VecBuilderOptions::default().add_sum().add_cumulative(),
                )
                .unwrap()
            }),
            height_to_value_created: compute_dollars.then(|| {
                EagerVec::forced_import(db, &suffix("value_created"), version + Version::ZERO)
                    .unwrap()
            }),
            indexes_to_value_created: compute_dollars.then(|| {
                ComputedVecsFromHeight::forced_import(
                    db,
                    &suffix("value_created"),
                    Source::None,
                    version + Version::ZERO,
                    indexes,
                    VecBuilderOptions::default().add_sum(),
                )
                .unwrap()
            }),
            indexes_to_realized_value: compute_dollars.then(|| {
                ComputedVecsFromHeight::forced_import(
                    db,
                    &suffix("realized_value"),
                    Source::Compute,
                    version + Version::ZERO,
                    indexes,
                    VecBuilderOptions::default().add_sum(),
                )
                .unwrap()
            }),
            height_to_adjusted_value_created: (compute_dollars && compute_adjusted).then(|| {
                EagerVec::forced_import(
                    db,
                    &suffix("adjusted_value_created"),
                    version + Version::ZERO,
                )
                .unwrap()
            }),
            indexes_to_adjusted_value_created: (compute_dollars && compute_adjusted).then(|| {
                ComputedVecsFromHeight::forced_import(
                    db,
                    &suffix("adjusted_value_created"),
                    Source::None,
                    version + Version::ZERO,
                    indexes,
                    VecBuilderOptions::default().add_sum(),
                )
                .unwrap()
            }),
            height_to_value_destroyed: compute_dollars.then(|| {
                EagerVec::forced_import(db, &suffix("value_destroyed"), version + Version::ZERO)
                    .unwrap()
            }),
            indexes_to_value_destroyed: compute_dollars.then(|| {
                ComputedVecsFromHeight::forced_import(
                    db,
                    &suffix("value_destroyed"),
                    Source::None,
                    version + Version::ZERO,
                    indexes,
                    VecBuilderOptions::default().add_sum(),
                )
                .unwrap()
            }),
            height_to_adjusted_value_destroyed: (compute_dollars && compute_adjusted).then(|| {
                EagerVec::forced_import(
                    db,
                    &suffix("adjusted_value_destroyed"),
                    version + Version::ZERO,
                )
                .unwrap()
            }),
            indexes_to_adjusted_value_destroyed: (compute_dollars && compute_adjusted).then(|| {
                ComputedVecsFromHeight::forced_import(
                    db,
                    &suffix("adjusted_value_destroyed"),
                    Source::None,
                    version + Version::ZERO,
                    indexes,
                    VecBuilderOptions::default().add_sum(),
                )
                .unwrap()
            }),
            indexes_to_realized_cap_30d_delta: compute_dollars.then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    db,
                    &suffix("realized_cap_30d_delta"),
                    Source::Compute,
                    version + Version::ZERO,
                    indexes,
                    VecBuilderOptions::default().add_last(),
                )
                .unwrap()
            }),
            indexes_to_net_realized_pnl: compute_dollars.then(|| {
                ComputedVecsFromHeight::forced_import(
                    db,
                    &suffix("net_realized_pnl"),
                    Source::Compute,
                    version + Version::ZERO,
                    indexes,
                    VecBuilderOptions::default().add_sum().add_cumulative(),
                )
                .unwrap()
            }),
            dateindex_to_sell_side_risk_ratio: compute_dollars.then(|| {
                EagerVec::forced_import(db, &suffix("sell_side_risk_ratio"), version + Version::ONE)
                    .unwrap()
            }),
            dateindex_to_sell_side_risk_ratio_7d_ema: compute_dollars.then(|| {
                EagerVec::forced_import(
                    db,
                    &suffix("sell_side_risk_ratio_7d_ema"),
                    version + Version::ONE,
                )
                .unwrap()
            }),
            dateindex_to_sell_side_risk_ratio_30d_ema: compute_dollars.then(|| {
                EagerVec::forced_import(
                    db,
                    &suffix("sell_side_risk_ratio_30d_ema"),
                    version + Version::ONE,
                )
                .unwrap()
            }),
            dateindex_to_sopr: compute_dollars.then(|| {
                EagerVec::forced_import(db, &suffix("sopr"), version + Version::ONE).unwrap()
            }),
            dateindex_to_sopr_7d_ema: compute_dollars.then(|| {
                EagerVec::forced_import(db, &suffix("sopr_7d_ema"), version + Version::ONE).unwrap()
            }),
            dateindex_to_sopr_30d_ema: compute_dollars.then(|| {
                EagerVec::forced_import(db, &suffix("sopr_30d_ema"), version + Version::ONE)
                    .unwrap()
            }),
            dateindex_to_adjusted_sopr: (compute_dollars && compute_adjusted).then(|| {
                EagerVec::forced_import(db, &suffix("adjusted_sopr"), version + Version::ONE)
                    .unwrap()
            }),
            dateindex_to_adjusted_sopr_7d_ema: (compute_dollars && compute_adjusted).then(|| {
                EagerVec::forced_import(db, &suffix("adjusted_sopr_7d_ema"), version + Version::ONE)
                    .unwrap()
            }),
            dateindex_to_adjusted_sopr_30d_ema: (compute_dollars && compute_adjusted).then(|| {
                EagerVec::forced_import(
                    db,
                    &suffix("adjusted_sopr_30d_ema"),
                    version + Version::ONE,
                )
                .unwrap()
            }),
            height_to_supply_half_value: ComputedHeightValueVecs::forced_import(
                db,
                &suffix("supply_half"),
                Source::Compute,
                version + Version::ZERO,
                compute_dollars,
            )?,
            indexes_to_supply_half: ComputedValueVecsFromDateIndex::forced_import(
                db,
                &suffix("supply_half"),
                Source::Compute,
                version + Version::ZERO,
                VecBuilderOptions::default().add_last(),
                compute_dollars,
                indexes,
            )?,
            height_to_neg_unrealized_loss: compute_dollars.then(|| {
                EagerVec::forced_import(db, &suffix("neg_unrealized_loss"), version + Version::ZERO)
                    .unwrap()
            }),
            indexes_to_neg_unrealized_loss: compute_dollars.then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    db,
                    &suffix("neg_unrealized_loss"),
                    Source::Compute,
                    version + Version::ZERO,
                    indexes,
                    VecBuilderOptions::default().add_last(),
                )
                .unwrap()
            }),
            height_to_net_unrealized_pnl: compute_dollars.then(|| {
                EagerVec::forced_import(db, &suffix("net_unrealized_pnl"), version + Version::ZERO)
                    .unwrap()
            }),
            indexes_to_net_unrealized_pnl: compute_dollars.then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    db,
                    &suffix("net_unrealized_pnl"),
                    Source::Compute,
                    version + Version::ZERO,
                    indexes,
                    VecBuilderOptions::default().add_last(),
                )
                .unwrap()
            }),
            height_to_unrealized_profit_rel_to_market_cap: compute_dollars.then(|| {
                EagerVec::forced_import(
                    db,
                    &suffix("unrealized_profit_rel_to_market_cap"),
                    version + Version::ZERO,
                )
                .unwrap()
            }),
            height_to_unrealized_loss_rel_to_market_cap: compute_dollars.then(|| {
                EagerVec::forced_import(
                    db,
                    &suffix("unrealized_loss_rel_to_market_cap"),
                    version + Version::ZERO,
                )
                .unwrap()
            }),
            height_to_neg_unrealized_loss_rel_to_market_cap: compute_dollars.then(|| {
                EagerVec::forced_import(
                    db,
                    &suffix("neg_unrealized_loss_rel_to_market_cap"),
                    version + Version::ZERO,
                )
                .unwrap()
            }),
            height_to_net_unrealized_pnl_rel_to_market_cap: compute_dollars.then(|| {
                EagerVec::forced_import(
                    db,
                    &suffix("net_unrealized_pnl_rel_to_market_cap"),
                    version + Version::ONE,
                )
                .unwrap()
            }),
            indexes_to_unrealized_profit_rel_to_market_cap: compute_dollars.then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    db,
                    &suffix("unrealized_profit_rel_to_market_cap"),
                    Source::Compute,
                    version + Version::ONE,
                    indexes,
                    VecBuilderOptions::default().add_last(),
                )
                .unwrap()
            }),
            indexes_to_unrealized_loss_rel_to_market_cap: compute_dollars.then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    db,
                    &suffix("unrealized_loss_rel_to_market_cap"),
                    Source::Compute,
                    version + Version::ONE,
                    indexes,
                    VecBuilderOptions::default().add_last(),
                )
                .unwrap()
            }),
            indexes_to_neg_unrealized_loss_rel_to_market_cap: compute_dollars.then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    db,
                    &suffix("neg_unrealized_loss_rel_to_market_cap"),
                    Source::Compute,
                    version + Version::ONE,
                    indexes,
                    VecBuilderOptions::default().add_last(),
                )
                .unwrap()
            }),
            indexes_to_net_unrealized_pnl_rel_to_market_cap: compute_dollars.then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    db,
                    &suffix("net_unrealized_pnl_rel_to_market_cap"),
                    Source::Compute,
                    version + Version::ONE,
                    indexes,
                    VecBuilderOptions::default().add_last(),
                )
                .unwrap()
            }),
            height_to_unrealized_profit_rel_to_own_market_cap: (compute_dollars
                && extended
                && compute_rel_to_all)
                .then(|| {
                    EagerVec::forced_import(
                        db,
                        &suffix("unrealized_profit_rel_to_own_market_cap"),
                        version + Version::ONE,
                    )
                    .unwrap()
                }),
            height_to_unrealized_loss_rel_to_own_market_cap: (compute_dollars
                && extended
                && compute_rel_to_all)
                .then(|| {
                    EagerVec::forced_import(
                        db,
                        &suffix("unrealized_loss_rel_to_own_market_cap"),
                        version + Version::ONE,
                    )
                    .unwrap()
                }),
            height_to_neg_unrealized_loss_rel_to_own_market_cap: (compute_dollars
                && extended
                && compute_rel_to_all)
                .then(|| {
                    EagerVec::forced_import(
                        db,
                        &suffix("neg_unrealized_loss_rel_to_own_market_cap"),
                        version + Version::ONE,
                    )
                    .unwrap()
                }),
            height_to_net_unrealized_pnl_rel_to_own_market_cap: (compute_dollars
                && extended
                && compute_rel_to_all)
                .then(|| {
                    EagerVec::forced_import(
                        db,
                        &suffix("net_unrealized_pnl_rel_to_own_market_cap"),
                        version + Version::TWO,
                    )
                    .unwrap()
                }),
            indexes_to_unrealized_profit_rel_to_own_market_cap: (compute_dollars
                && extended
                && compute_rel_to_all)
                .then(|| {
                    ComputedVecsFromDateIndex::forced_import(
                        db,
                        &suffix("unrealized_profit_rel_to_own_market_cap"),
                        Source::Compute,
                        version + Version::TWO,
                        indexes,
                        VecBuilderOptions::default().add_last(),
                    )
                    .unwrap()
                }),
            indexes_to_unrealized_loss_rel_to_own_market_cap: (compute_dollars
                && extended
                && compute_rel_to_all)
                .then(|| {
                    ComputedVecsFromDateIndex::forced_import(
                        db,
                        &suffix("unrealized_loss_rel_to_own_market_cap"),
                        Source::Compute,
                        version + Version::TWO,
                        indexes,
                        VecBuilderOptions::default().add_last(),
                    )
                    .unwrap()
                }),
            indexes_to_neg_unrealized_loss_rel_to_own_market_cap: (compute_dollars
                && extended
                && compute_rel_to_all)
                .then(|| {
                    ComputedVecsFromDateIndex::forced_import(
                        db,
                        &suffix("neg_unrealized_loss_rel_to_own_market_cap"),
                        Source::Compute,
                        version + Version::TWO,
                        indexes,
                        VecBuilderOptions::default().add_last(),
                    )
                    .unwrap()
                }),
            indexes_to_net_unrealized_pnl_rel_to_own_market_cap: (compute_dollars
                && extended
                && compute_rel_to_all)
                .then(|| {
                    ComputedVecsFromDateIndex::forced_import(
                        db,
                        &suffix("net_unrealized_pnl_rel_to_own_market_cap"),
                        Source::Compute,
                        version + Version::TWO,
                        indexes,
                        VecBuilderOptions::default().add_last(),
                    )
                    .unwrap()
                }),
            height_to_unrealized_profit_rel_to_own_total_unrealized_pnl: (compute_dollars
                && extended)
                .then(|| {
                    EagerVec::forced_import(
                        db,
                        &suffix("unrealized_profit_rel_to_own_total_unrealized_pnl"),
                        version + Version::ZERO,
                    )
                    .unwrap()
                }),
            height_to_unrealized_loss_rel_to_own_total_unrealized_pnl: (compute_dollars
                && extended)
                .then(|| {
                    EagerVec::forced_import(
                        db,
                        &suffix("unrealized_loss_rel_to_own_total_unrealized_pnl"),
                        version + Version::ZERO,
                    )
                    .unwrap()
                }),
            height_to_neg_unrealized_loss_rel_to_own_total_unrealized_pnl: (compute_dollars
                && extended)
                .then(|| {
                    EagerVec::forced_import(
                        db,
                        &suffix("neg_unrealized_loss_rel_to_own_total_unrealized_pnl"),
                        version + Version::ZERO,
                    )
                    .unwrap()
                }),
            height_to_net_unrealized_pnl_rel_to_own_total_unrealized_pnl: (compute_dollars
                && extended)
                .then(|| {
                    EagerVec::forced_import(
                        db,
                        &suffix("net_unrealized_pnl_rel_to_own_total_unrealized_pnl"),
                        version + Version::ONE,
                    )
                    .unwrap()
                }),
            indexes_to_unrealized_profit_rel_to_own_total_unrealized_pnl: (compute_dollars
                && extended)
                .then(|| {
                    ComputedVecsFromDateIndex::forced_import(
                        db,
                        &suffix("unrealized_profit_rel_to_own_total_unrealized_pnl"),
                        Source::Compute,
                        version + Version::ONE,
                        indexes,
                        VecBuilderOptions::default().add_last(),
                    )
                    .unwrap()
                }),
            indexes_to_unrealized_loss_rel_to_own_total_unrealized_pnl: (compute_dollars
                && extended)
                .then(|| {
                    ComputedVecsFromDateIndex::forced_import(
                        db,
                        &suffix("unrealized_loss_rel_to_own_total_unrealized_pnl"),
                        Source::Compute,
                        version + Version::ONE,
                        indexes,
                        VecBuilderOptions::default().add_last(),
                    )
                    .unwrap()
                }),
            indexes_to_neg_unrealized_loss_rel_to_own_total_unrealized_pnl: (compute_dollars
                && extended)
                .then(|| {
                    ComputedVecsFromDateIndex::forced_import(
                        db,
                        &suffix("neg_unrealized_loss_rel_to_own_total_unrealized_pnl"),
                        Source::Compute,
                        version + Version::ONE,
                        indexes,
                        VecBuilderOptions::default().add_last(),
                    )
                    .unwrap()
                }),
            indexes_to_net_unrealized_pnl_rel_to_own_total_unrealized_pnl: (compute_dollars
                && extended)
                .then(|| {
                    ComputedVecsFromDateIndex::forced_import(
                        db,
                        &suffix("net_unrealized_pnl_rel_to_own_total_unrealized_pnl"),
                        Source::Compute,
                        version + Version::ONE,
                        indexes,
                        VecBuilderOptions::default().add_last(),
                    )
                    .unwrap()
                }),
            indexes_to_realized_profit_rel_to_realized_cap: compute_dollars.then(|| {
                ComputedVecsFromHeight::forced_import(
                    db,
                    &suffix("realized_profit_rel_to_realized_cap"),
                    Source::Compute,
                    version + Version::ZERO,
                    indexes,
                    VecBuilderOptions::default().add_sum(),
                )
                .unwrap()
            }),
            indexes_to_realized_loss_rel_to_realized_cap: compute_dollars.then(|| {
                ComputedVecsFromHeight::forced_import(
                    db,
                    &suffix("realized_loss_rel_to_realized_cap"),
                    Source::Compute,
                    version + Version::ZERO,
                    indexes,
                    VecBuilderOptions::default().add_sum(),
                )
                .unwrap()
            }),
            indexes_to_net_realized_pnl_rel_to_realized_cap: compute_dollars.then(|| {
                ComputedVecsFromHeight::forced_import(
                    db,
                    &suffix("net_realized_pnl_rel_to_realized_cap"),
                    Source::Compute,
                    version + Version::ONE,
                    indexes,
                    VecBuilderOptions::default().add_sum(),
                )
                .unwrap()
            }),
            height_to_supply_in_loss_value: compute_dollars.then(|| {
                ComputedHeightValueVecs::forced_import(
                    db,
                    &suffix("supply_in_loss"),
                    Source::None,
                    version + Version::ZERO,
                    compute_dollars,
                )
                .unwrap()
            }),
            height_to_supply_in_profit_value: compute_dollars.then(|| {
                ComputedHeightValueVecs::forced_import(
                    db,
                    &suffix("supply_in_profit"),
                    Source::None,
                    version + Version::ZERO,
                    compute_dollars,
                )
                .unwrap()
            }),
            height_to_supply_in_loss_rel_to_own_supply: compute_dollars.then(|| {
                EagerVec::forced_import(
                    db,
                    &suffix("supply_in_loss_rel_to_own_supply"),
                    version + Version::ONE,
                )
                .unwrap()
            }),
            height_to_supply_in_profit_rel_to_own_supply: compute_dollars.then(|| {
                EagerVec::forced_import(
                    db,
                    &suffix("supply_in_profit_rel_to_own_supply"),
                    version + Version::ONE,
                )
                .unwrap()
            }),
            indexes_to_supply_in_loss_rel_to_own_supply: compute_dollars.then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    db,
                    &suffix("supply_in_loss_rel_to_own_supply"),
                    Source::Compute,
                    version + Version::ONE,
                    indexes,
                    VecBuilderOptions::default().add_last(),
                )
                .unwrap()
            }),
            indexes_to_supply_in_profit_rel_to_own_supply: compute_dollars.then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    db,
                    &suffix("supply_in_profit_rel_to_own_supply"),
                    Source::Compute,
                    version + Version::ONE,
                    indexes,
                    VecBuilderOptions::default().add_last(),
                )
                .unwrap()
            }),
            indexes_to_supply_rel_to_circulating_supply: compute_rel_to_all.then(|| {
                ComputedVecsFromHeight::forced_import(
                    db,
                    &suffix("supply_rel_to_circulating_supply"),
                    Source::Compute,
                    version + Version::ONE,
                    indexes,
                    VecBuilderOptions::default().add_last(),
                )
                .unwrap()
            }),
            height_to_supply_in_loss_rel_to_circulating_supply: (compute_rel_to_all
                && compute_dollars)
                .then(|| {
                    EagerVec::forced_import(
                        db,
                        &suffix("supply_in_loss_rel_to_circulating_supply"),
                        version + Version::ONE,
                    )
                    .unwrap()
                }),
            height_to_supply_in_profit_rel_to_circulating_supply: (compute_rel_to_all
                && compute_dollars)
                .then(|| {
                    EagerVec::forced_import(
                        db,
                        &suffix("supply_in_profit_rel_to_circulating_supply"),
                        version + Version::ONE,
                    )
                    .unwrap()
                }),
            indexes_to_supply_in_loss_rel_to_circulating_supply: (compute_rel_to_all
                && compute_dollars)
                .then(|| {
                    ComputedVecsFromDateIndex::forced_import(
                        db,
                        &suffix("supply_in_loss_rel_to_circulating_supply"),
                        Source::Compute,
                        version + Version::ONE,
                        indexes,
                        VecBuilderOptions::default().add_last(),
                    )
                    .unwrap()
                }),
            indexes_to_supply_in_profit_rel_to_circulating_supply: (compute_rel_to_all
                && compute_dollars)
                .then(|| {
                    ComputedVecsFromDateIndex::forced_import(
                        db,
                        &suffix("supply_in_profit_rel_to_circulating_supply"),
                        Source::Compute,
                        version + Version::ONE,
                        indexes,
                        VecBuilderOptions::default().add_last(),
                    )
                    .unwrap()
                }),
            height_to_sent: EagerVec::forced_import(
                db,
                &suffix("sent"),
                version + Version::ZERO,
            )?,
            height_to_satblocks_destroyed: EagerVec::forced_import(
                db,
                &suffix("satblocks_destroyed"),
                version + Version::ZERO,
            )?,
            height_to_satdays_destroyed: EagerVec::forced_import(
                db,
                &suffix("satdays_destroyed"),
                version + Version::ZERO,
            )?,
            indexes_to_sent: ComputedValueVecsFromHeight::forced_import(
                db,
                &suffix("sent"),
                Source::Compute,
                version + Version::ZERO,
                VecBuilderOptions::default().add_sum(),
                compute_dollars,
                indexes,
            )?,
            indexes_to_coinblocks_destroyed: ComputedVecsFromHeight::forced_import(
                db,
                &suffix("coinblocks_destroyed"),
                Source::Compute,
                version + Version::TWO,
                indexes,
                VecBuilderOptions::default().add_sum().add_cumulative(),
            )?,
            indexes_to_coindays_destroyed: ComputedVecsFromHeight::forced_import(
                db,
                &suffix("coindays_destroyed"),
                Source::Compute,
                version + Version::TWO,
                indexes,
                VecBuilderOptions::default().add_sum().add_cumulative(),
            )?,
            indexes_to_net_realized_pnl_cumulative_30d_delta: compute_dollars.then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    db,
                    &suffix("net_realized_pnl_cumulative_30d_delta"),
                    Source::Compute,
                    version + Version::new(3),
                    indexes,
                    VecBuilderOptions::default().add_last(),
                )
                .unwrap()
            }),
            indexes_to_net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap: compute_dollars
                .then(|| {
                    ComputedVecsFromDateIndex::forced_import(
                        db,
                        &suffix("net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap"),
                        Source::Compute,
                        version + Version::new(3),
                        indexes,
                        VecBuilderOptions::default().add_last(),
                    )
                    .unwrap()
                }),
            indexes_to_net_realized_pnl_cumulative_30d_delta_rel_to_market_cap: compute_dollars
                .then(|| {
                    ComputedVecsFromDateIndex::forced_import(
                        db,
                        &suffix("net_realized_pnl_cumulative_30d_delta_rel_to_market_cap"),
                        Source::Compute,
                        version + Version::new(3),
                        indexes,
                        VecBuilderOptions::default().add_last(),
                    )
                    .unwrap()
                }),
            dateindex_to_realized_profit_to_loss_ratio: (compute_dollars && extended).then(|| {
                EagerVec::forced_import(
                    db,
                    &suffix("realized_profit_to_loss_ratio"),
                    version + Version::ONE,
                )
                .unwrap()
            }),
        })
    }

    pub fn min_height_vecs_len(&self) -> usize {
        [
            self.height_to_supply.len(),
            self.height_to_utxo_count.len(),
            self.height_to_realized_cap
                .as_ref()
                .map_or(usize::MAX, |v| v.len()),
            self.height_to_realized_profit
                .as_ref()
                .map_or(usize::MAX, |v| v.len()),
            self.height_to_realized_loss
                .as_ref()
                .map_or(usize::MAX, |v| v.len()),
            self.height_to_value_created
                .as_ref()
                .map_or(usize::MAX, |v| v.len()),
            self.height_to_adjusted_value_created
                .as_ref()
                .map_or(usize::MAX, |v| v.len()),
            self.height_to_value_destroyed
                .as_ref()
                .map_or(usize::MAX, |v| v.len()),
            self.height_to_adjusted_value_destroyed
                .as_ref()
                .map_or(usize::MAX, |v| v.len()),
            self.height_to_supply_in_profit
                .as_ref()
                .map_or(usize::MAX, |v| v.len()),
            self.height_to_supply_in_loss
                .as_ref()
                .map_or(usize::MAX, |v| v.len()),
            self.height_to_unrealized_profit
                .as_ref()
                .map_or(usize::MAX, |v| v.len()),
            self.height_to_unrealized_loss
                .as_ref()
                .map_or(usize::MAX, |v| v.len()),
            self.height_to_min_price_paid
                .as_ref()
                .map_or(usize::MAX, |v| v.len()),
            self.height_to_max_price_paid
                .as_ref()
                .map_or(usize::MAX, |v| v.len()),
            self.height_to_sent.len(),
            self.height_to_satdays_destroyed.len(),
            self.height_to_satblocks_destroyed.len(),
        ]
        .into_iter()
        .min()
        .unwrap()
    }

    pub fn import_state(
        &mut self,
        starting_height: Height,
        state: &mut CohortState,
    ) -> Result<Height> {
        if let Some(mut prev_height) = starting_height.decremented() {
            if self.height_to_realized_cap.as_mut().is_some() {
                prev_height = state.import_at_or_before(prev_height)?;
            }

            state.supply.value = self.height_to_supply.into_iter().get_unwrap(prev_height);
            state.supply.utxo_count = *self
                .height_to_utxo_count
                .into_iter()
                .get_unwrap(prev_height);

            if let Some(height_to_realized_cap) = self.height_to_realized_cap.as_mut() {
                state.realized.as_mut().unwrap().cap =
                    height_to_realized_cap.into_iter().get_unwrap(prev_height);
            }

            Ok(prev_height.incremented())
        } else {
            Err(Error::Str("Unset"))
        }
    }

    pub fn validate_computed_versions(&mut self, base_version: Version) -> Result<()> {
        self.height_to_supply.validate_computed_version_or_reset(
            base_version + self.height_to_supply.inner_version(),
        )?;

        self.height_to_utxo_count
            .validate_computed_version_or_reset(
                base_version + self.height_to_utxo_count.inner_version(),
            )?;

        self.height_to_sent
            .validate_computed_version_or_reset(
                base_version + self.height_to_sent.inner_version(),
            )?;

        self.height_to_satblocks_destroyed
            .validate_computed_version_or_reset(
                base_version + self.height_to_satblocks_destroyed.inner_version(),
            )?;

        self.height_to_satdays_destroyed
            .validate_computed_version_or_reset(
                base_version + self.height_to_satdays_destroyed.inner_version(),
            )?;

        if let Some(height_to_realized_cap) = self.height_to_realized_cap.as_mut().as_mut() {
            height_to_realized_cap.validate_computed_version_or_reset(
                base_version + height_to_realized_cap.inner_version(),
            )?;

            let height_to_realized_profit_inner_version = self
                .height_to_realized_profit
                .as_ref()
                .unwrap()
                .inner_version();
            self.height_to_realized_profit
                .as_mut()
                .unwrap()
                .validate_computed_version_or_reset(
                    base_version + height_to_realized_profit_inner_version,
                )?;
            let height_to_realized_loss_inner_version = self
                .height_to_realized_loss
                .as_ref()
                .unwrap()
                .inner_version();
            self.height_to_realized_loss
                .as_mut()
                .unwrap()
                .validate_computed_version_or_reset(
                    base_version + height_to_realized_loss_inner_version,
                )?;
            let height_to_value_created_inner_version = self
                .height_to_value_created
                .as_ref()
                .unwrap()
                .inner_version();
            self.height_to_value_created
                .as_mut()
                .unwrap()
                .validate_computed_version_or_reset(
                    base_version + height_to_value_created_inner_version,
                )?;
            let height_to_value_destroyed_inner_version = self
                .height_to_value_destroyed
                .as_ref()
                .unwrap()
                .inner_version();
            self.height_to_value_destroyed
                .as_mut()
                .unwrap()
                .validate_computed_version_or_reset(
                    base_version + height_to_value_destroyed_inner_version,
                )?;
            let height_to_supply_in_profit_inner_version = self
                .height_to_supply_in_profit
                .as_ref()
                .unwrap()
                .inner_version();
            self.height_to_supply_in_profit
                .as_mut()
                .unwrap()
                .validate_computed_version_or_reset(
                    base_version + height_to_supply_in_profit_inner_version,
                )?;
            let height_to_supply_in_loss_inner_version = self
                .height_to_supply_in_loss
                .as_ref()
                .unwrap()
                .inner_version();
            self.height_to_supply_in_loss
                .as_mut()
                .unwrap()
                .validate_computed_version_or_reset(
                    base_version + height_to_supply_in_loss_inner_version,
                )?;
            let height_to_unrealized_profit_inner_version = self
                .height_to_unrealized_profit
                .as_ref()
                .unwrap()
                .inner_version();
            self.height_to_unrealized_profit
                .as_mut()
                .unwrap()
                .validate_computed_version_or_reset(
                    base_version + height_to_unrealized_profit_inner_version,
                )?;
            let height_to_unrealized_loss_inner_version = self
                .height_to_unrealized_loss
                .as_ref()
                .unwrap()
                .inner_version();
            self.height_to_unrealized_loss
                .as_mut()
                .unwrap()
                .validate_computed_version_or_reset(
                    base_version + height_to_unrealized_loss_inner_version,
                )?;
            let dateindex_to_supply_in_profit_inner_version = self
                .dateindex_to_supply_in_profit
                .as_ref()
                .unwrap()
                .inner_version();
            self.dateindex_to_supply_in_profit
                .as_mut()
                .unwrap()
                .validate_computed_version_or_reset(
                    base_version + dateindex_to_supply_in_profit_inner_version,
                )?;
            let dateindex_to_supply_in_loss_inner_version = self
                .dateindex_to_supply_in_loss
                .as_ref()
                .unwrap()
                .inner_version();
            self.dateindex_to_supply_in_loss
                .as_mut()
                .unwrap()
                .validate_computed_version_or_reset(
                    base_version + dateindex_to_supply_in_loss_inner_version,
                )?;
            let dateindex_to_unrealized_profit_inner_version = self
                .dateindex_to_unrealized_profit
                .as_ref()
                .unwrap()
                .inner_version();
            self.dateindex_to_unrealized_profit
                .as_mut()
                .unwrap()
                .validate_computed_version_or_reset(
                    base_version + dateindex_to_unrealized_profit_inner_version,
                )?;
            let dateindex_to_unrealized_loss_inner_version = self
                .dateindex_to_unrealized_loss
                .as_ref()
                .unwrap()
                .inner_version();
            self.dateindex_to_unrealized_loss
                .as_mut()
                .unwrap()
                .validate_computed_version_or_reset(
                    base_version + dateindex_to_unrealized_loss_inner_version,
                )?;
            let height_to_min_price_paid_inner_version = self
                .height_to_min_price_paid
                .as_ref()
                .unwrap()
                .inner_version();
            self.height_to_min_price_paid
                .as_mut()
                .unwrap()
                .validate_computed_version_or_reset(
                    base_version + height_to_min_price_paid_inner_version,
                )?;
            let height_to_max_price_paid_inner_version = self
                .height_to_max_price_paid
                .as_ref()
                .unwrap()
                .inner_version();
            self.height_to_max_price_paid
                .as_mut()
                .unwrap()
                .validate_computed_version_or_reset(
                    base_version + height_to_max_price_paid_inner_version,
                )?;

            if self.height_to_adjusted_value_created.is_some() {
                let height_to_adjusted_value_created_inner_version = self
                    .height_to_adjusted_value_created
                    .as_ref()
                    .unwrap()
                    .inner_version();
                self.height_to_adjusted_value_created
                    .as_mut()
                    .unwrap()
                    .validate_computed_version_or_reset(
                        base_version + height_to_adjusted_value_created_inner_version,
                    )?;
                let height_to_adjusted_value_destroyed_inner_version = self
                    .height_to_adjusted_value_destroyed
                    .as_ref()
                    .unwrap()
                    .inner_version();
                self.height_to_adjusted_value_destroyed
                    .as_mut()
                    .unwrap()
                    .validate_computed_version_or_reset(
                        base_version + height_to_adjusted_value_destroyed_inner_version,
                    )?;
            }
        }

        Ok(())
    }

    pub fn truncate_push(&mut self, height: Height, state: &CohortState) -> Result<()> {
        self.height_to_supply
            .truncate_push(height, state.supply.value)?;

        self.height_to_utxo_count
            .truncate_push(height, StoredU64::from(state.supply.utxo_count))?;

        self.height_to_sent.truncate_push(height, state.sent)?;

        self.height_to_satblocks_destroyed
            .truncate_push(height, state.satblocks_destroyed)?;

        self.height_to_satdays_destroyed
            .truncate_push(height, state.satdays_destroyed)?;

        if let Some(height_to_realized_cap) = self.height_to_realized_cap.as_mut() {
            let realized = state.realized.as_ref().unwrap_or_else(|| {
                dbg!((&state.realized, &state.supply));
                panic!();
            });

            height_to_realized_cap.truncate_push(height, realized.cap)?;

            self.height_to_realized_profit
                .as_mut()
                .unwrap()
                .truncate_push(height, realized.profit)?;
            self.height_to_realized_loss
                .as_mut()
                .unwrap()
                .truncate_push(height, realized.loss)?;
            self.height_to_value_created
                .as_mut()
                .unwrap()
                .truncate_push(height, realized.value_created)?;
            self.height_to_value_destroyed
                .as_mut()
                .unwrap()
                .truncate_push(height, realized.value_destroyed)?;

            if self.height_to_adjusted_value_created.is_some() {
                self.height_to_adjusted_value_created
                    .as_mut()
                    .unwrap()
                    .truncate_push(height, realized.adj_value_created)?;
                self.height_to_adjusted_value_destroyed
                    .as_mut()
                    .unwrap()
                    .truncate_push(height, realized.adj_value_destroyed)?;
            }
        }
        Ok(())
    }

    pub fn compute_then_truncate_push_unrealized_states(
        &mut self,
        height: Height,
        height_price: Option<Dollars>,
        dateindex: Option<DateIndex>,
        date_price: Option<Option<Dollars>>,
        state: &CohortState,
    ) -> Result<()> {
        if let Some(height_price) = height_price {
            self.height_to_min_price_paid
                .as_mut()
                .unwrap()
                .truncate_push(
                    height,
                    state
                        .price_to_amount_first_key_value()
                        .map(|(&dollars, _)| dollars)
                        .unwrap_or(Dollars::NAN),
                )?;
            self.height_to_max_price_paid
                .as_mut()
                .unwrap()
                .truncate_push(
                    height,
                    state
                        .price_to_amount_last_key_value()
                        .map(|(&dollars, _)| dollars)
                        .unwrap_or(Dollars::NAN),
                )?;

            let (height_unrealized_state, date_unrealized_state) =
                state.compute_unrealized_states(height_price, date_price.unwrap());

            self.height_to_supply_in_profit
                .as_mut()
                .unwrap()
                .truncate_push(height, height_unrealized_state.supply_in_profit)?;
            self.height_to_supply_in_loss
                .as_mut()
                .unwrap()
                .truncate_push(height, height_unrealized_state.supply_in_loss)?;
            self.height_to_unrealized_profit
                .as_mut()
                .unwrap()
                .truncate_push(height, height_unrealized_state.unrealized_profit)?;
            self.height_to_unrealized_loss
                .as_mut()
                .unwrap()
                .truncate_push(height, height_unrealized_state.unrealized_loss)?;

            if let Some(date_unrealized_state) = date_unrealized_state {
                let dateindex = dateindex.unwrap();

                self.dateindex_to_supply_in_profit
                    .as_mut()
                    .unwrap()
                    .truncate_push(dateindex, date_unrealized_state.supply_in_profit)?;
                self.dateindex_to_supply_in_loss
                    .as_mut()
                    .unwrap()
                    .truncate_push(dateindex, date_unrealized_state.supply_in_loss)?;
                self.dateindex_to_unrealized_profit
                    .as_mut()
                    .unwrap()
                    .truncate_push(dateindex, date_unrealized_state.unrealized_profit)?;
                self.dateindex_to_unrealized_loss
                    .as_mut()
                    .unwrap()
                    .truncate_push(dateindex, date_unrealized_state.unrealized_loss)?;
            }

            // Compute and push price percentiles
            if let Some(price_percentiles) = self.price_percentiles.as_mut() {
                let percentile_prices = state.compute_percentile_prices();
                price_percentiles.truncate_push(height, &percentile_prices)?;
            }
        }

        Ok(())
    }

    pub fn safe_flush_stateful_vecs(
        &mut self,
        height: Height,
        exit: &Exit,
        state: &mut CohortState,
    ) -> Result<()> {
        self.height_to_supply.safe_flush(exit)?;
        self.height_to_utxo_count.safe_flush(exit)?;
        self.height_to_sent.safe_flush(exit)?;
        self.height_to_satdays_destroyed.safe_flush(exit)?;
        self.height_to_satblocks_destroyed.safe_flush(exit)?;

        if let Some(height_to_realized_cap) = self.height_to_realized_cap.as_mut() {
            height_to_realized_cap.safe_flush(exit)?;
            self.height_to_realized_profit
                .as_mut()
                .unwrap()
                .safe_flush(exit)?;
            self.height_to_realized_loss
                .as_mut()
                .unwrap()
                .safe_flush(exit)?;
            self.height_to_value_created
                .as_mut()
                .unwrap()
                .safe_flush(exit)?;
            self.height_to_value_destroyed
                .as_mut()
                .unwrap()
                .safe_flush(exit)?;
            self.height_to_supply_in_profit
                .as_mut()
                .unwrap()
                .safe_flush(exit)?;
            self.height_to_supply_in_loss
                .as_mut()
                .unwrap()
                .safe_flush(exit)?;
            self.height_to_unrealized_profit
                .as_mut()
                .unwrap()
                .safe_flush(exit)?;
            self.height_to_unrealized_loss
                .as_mut()
                .unwrap()
                .safe_flush(exit)?;
            self.dateindex_to_supply_in_profit
                .as_mut()
                .unwrap()
                .safe_flush(exit)?;
            self.dateindex_to_supply_in_loss
                .as_mut()
                .unwrap()
                .safe_flush(exit)?;
            self.dateindex_to_unrealized_profit
                .as_mut()
                .unwrap()
                .safe_flush(exit)?;
            self.dateindex_to_unrealized_loss
                .as_mut()
                .unwrap()
                .safe_flush(exit)?;
            self.height_to_min_price_paid
                .as_mut()
                .unwrap()
                .safe_flush(exit)?;
            self.height_to_max_price_paid
                .as_mut()
                .unwrap()
                .safe_flush(exit)?;

            if self.height_to_adjusted_value_created.is_some() {
                self.height_to_adjusted_value_created
                    .as_mut()
                    .unwrap()
                    .safe_flush(exit)?;
                self.height_to_adjusted_value_destroyed
                    .as_mut()
                    .unwrap()
                    .safe_flush(exit)?;
            }
        }

        state.commit(height)?;

        Ok(())
    }

    pub fn compute_from_stateful(
        &mut self,
        starting_indexes: &Indexes,
        others: &[&Self],
        exit: &Exit,
    ) -> Result<()> {
        self.height_to_supply.compute_sum_of_others(
            starting_indexes.height,
            others
                .iter()
                .map(|v| &v.height_to_supply)
                .collect::<Vec<_>>()
                .as_slice(),
            exit,
        )?;
        self.height_to_utxo_count.compute_sum_of_others(
            starting_indexes.height,
            others
                .iter()
                .map(|v| &v.height_to_utxo_count)
                .collect::<Vec<_>>()
                .as_slice(),
            exit,
        )?;
        self.height_to_sent.compute_sum_of_others(
            starting_indexes.height,
            others
                .iter()
                .map(|v| &v.height_to_sent)
                .collect::<Vec<_>>()
                .as_slice(),
            exit,
        )?;
        self.height_to_satblocks_destroyed.compute_sum_of_others(
            starting_indexes.height,
            others
                .iter()
                .map(|v| &v.height_to_satblocks_destroyed)
                .collect::<Vec<_>>()
                .as_slice(),
            exit,
        )?;
        self.height_to_satdays_destroyed.compute_sum_of_others(
            starting_indexes.height,
            others
                .iter()
                .map(|v| &v.height_to_satdays_destroyed)
                .collect::<Vec<_>>()
                .as_slice(),
            exit,
        )?;

        if let Some(height_to_realized_cap) = &mut self.height_to_realized_cap {
            height_to_realized_cap.compute_sum_of_others(
                starting_indexes.height,
                others
                    .iter()
                    .map(|v| v.height_to_realized_cap.as_ref().unwrap())
                    .collect::<Vec<_>>()
                    .as_slice(),
                exit,
            )?;

            self.height_to_min_price_paid
                .as_mut()
                .unwrap()
                .compute_min_of_others(
                    starting_indexes.height,
                    others
                        .iter()
                        .map(|v| v.height_to_min_price_paid.as_ref().unwrap())
                        .collect::<Vec<_>>()
                        .as_slice(),
                    exit,
                )?;
            self.height_to_max_price_paid
                .as_mut()
                .unwrap()
                .compute_max_of_others(
                    starting_indexes.height,
                    others
                        .iter()
                        .map(|v| v.height_to_max_price_paid.as_ref().unwrap())
                        .collect::<Vec<_>>()
                        .as_slice(),
                    exit,
                )?;
            self.height_to_realized_profit
                .as_mut()
                .unwrap()
                .compute_sum_of_others(
                    starting_indexes.height,
                    others
                        .iter()
                        .map(|v| v.height_to_realized_profit.as_ref().unwrap())
                        .collect::<Vec<_>>()
                        .as_slice(),
                    exit,
                )?;
            self.height_to_realized_loss
                .as_mut()
                .unwrap()
                .compute_sum_of_others(
                    starting_indexes.height,
                    others
                        .iter()
                        .map(|v| v.height_to_realized_loss.as_ref().unwrap())
                        .collect::<Vec<_>>()
                        .as_slice(),
                    exit,
                )?;
            self.height_to_value_created
                .as_mut()
                .unwrap()
                .compute_sum_of_others(
                    starting_indexes.height,
                    others
                        .iter()
                        .map(|v| v.height_to_value_created.as_ref().unwrap())
                        .collect::<Vec<_>>()
                        .as_slice(),
                    exit,
                )?;
            self.height_to_value_destroyed
                .as_mut()
                .unwrap()
                .compute_sum_of_others(
                    starting_indexes.height,
                    others
                        .iter()
                        .map(|v| v.height_to_value_destroyed.as_ref().unwrap())
                        .collect::<Vec<_>>()
                        .as_slice(),
                    exit,
                )?;
            self.height_to_supply_in_profit
                .as_mut()
                .unwrap()
                .compute_sum_of_others(
                    starting_indexes.height,
                    others
                        .iter()
                        .map(|v| v.height_to_supply_in_profit.as_ref().unwrap())
                        .collect::<Vec<_>>()
                        .as_slice(),
                    exit,
                )?;
            self.height_to_supply_in_loss
                .as_mut()
                .unwrap()
                .compute_sum_of_others(
                    starting_indexes.height,
                    others
                        .iter()
                        .map(|v| v.height_to_supply_in_loss.as_ref().unwrap())
                        .collect::<Vec<_>>()
                        .as_slice(),
                    exit,
                )?;
            self.height_to_unrealized_profit
                .as_mut()
                .unwrap()
                .compute_sum_of_others(
                    starting_indexes.height,
                    others
                        .iter()
                        .map(|v| v.height_to_unrealized_profit.as_ref().unwrap())
                        .collect::<Vec<_>>()
                        .as_slice(),
                    exit,
                )?;
            self.height_to_unrealized_loss
                .as_mut()
                .unwrap()
                .compute_sum_of_others(
                    starting_indexes.height,
                    others
                        .iter()
                        .map(|v| v.height_to_unrealized_loss.as_ref().unwrap())
                        .collect::<Vec<_>>()
                        .as_slice(),
                    exit,
                )?;
            self.dateindex_to_supply_in_profit
                .as_mut()
                .unwrap()
                .compute_sum_of_others(
                    starting_indexes.dateindex,
                    others
                        .iter()
                        .map(|v| v.dateindex_to_supply_in_profit.as_ref().unwrap())
                        .collect::<Vec<_>>()
                        .as_slice(),
                    exit,
                )?;
            self.dateindex_to_supply_in_loss
                .as_mut()
                .unwrap()
                .compute_sum_of_others(
                    starting_indexes.dateindex,
                    others
                        .iter()
                        .map(|v| v.dateindex_to_supply_in_loss.as_ref().unwrap())
                        .collect::<Vec<_>>()
                        .as_slice(),
                    exit,
                )?;
            self.dateindex_to_unrealized_profit
                .as_mut()
                .unwrap()
                .compute_sum_of_others(
                    starting_indexes.dateindex,
                    others
                        .iter()
                        .map(|v| v.dateindex_to_unrealized_profit.as_ref().unwrap())
                        .collect::<Vec<_>>()
                        .as_slice(),
                    exit,
                )?;
            self.dateindex_to_unrealized_loss
                .as_mut()
                .unwrap()
                .compute_sum_of_others(
                    starting_indexes.dateindex,
                    others
                        .iter()
                        .map(|v| v.dateindex_to_unrealized_loss.as_ref().unwrap())
                        .collect::<Vec<_>>()
                        .as_slice(),
                    exit,
                )?;
            self.height_to_min_price_paid
                .as_mut()
                .unwrap()
                .compute_min_of_others(
                    starting_indexes.height,
                    others
                        .iter()
                        .map(|v| v.height_to_min_price_paid.as_ref().unwrap())
                        .collect::<Vec<_>>()
                        .as_slice(),
                    exit,
                )?;
            self.height_to_max_price_paid
                .as_mut()
                .unwrap()
                .compute_max_of_others(
                    starting_indexes.height,
                    others
                        .iter()
                        .map(|v| v.height_to_max_price_paid.as_ref().unwrap())
                        .collect::<Vec<_>>()
                        .as_slice(),
                    exit,
                )?;

            if self.height_to_adjusted_value_created.is_some() {
                self.height_to_adjusted_value_created
                    .as_mut()
                    .unwrap()
                    .compute_sum_of_others(
                        starting_indexes.height,
                        others
                            .iter()
                            .map(|v| {
                                v.height_to_adjusted_value_created
                                    .as_ref()
                                    .unwrap_or(v.height_to_value_created.as_ref().unwrap())
                            })
                            .collect::<Vec<_>>()
                            .as_slice(),
                        exit,
                    )?;
                self.height_to_adjusted_value_destroyed
                    .as_mut()
                    .unwrap()
                    .compute_sum_of_others(
                        starting_indexes.height,
                        others
                            .iter()
                            .map(|v| {
                                v.height_to_adjusted_value_destroyed
                                    .as_ref()
                                    .unwrap_or(v.height_to_value_destroyed.as_ref().unwrap())
                            })
                            .collect::<Vec<_>>()
                            .as_slice(),
                        exit,
                    )?;
            }
        }

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn compute_rest_part1(
        &mut self,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.height_to_supply_value.compute_rest(
            price,
            starting_indexes,
            exit,
            Some(&self.height_to_supply),
        )?;

        self.indexes_to_supply
            .compute_all(price, starting_indexes, exit, |v| {
                let mut dateindex_to_height_count_iter =
                    indexes.dateindex_to_height_count.into_iter();
                let mut height_to_supply_iter = self.height_to_supply.into_iter();
                v.compute_transform(
                    starting_indexes.dateindex,
                    &indexes.dateindex_to_first_height,
                    |(i, height, ..)| {
                        let count = dateindex_to_height_count_iter.get_unwrap(i);
                        if count == StoredU64::default() {
                            unreachable!()
                        }
                        let supply = height_to_supply_iter.get_unwrap(height + (*count - 1));
                        (i, supply)
                    },
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_utxo_count.compute_rest(
            indexes,
            starting_indexes,
            exit,
            Some(&self.height_to_utxo_count),
        )?;

        self.height_to_supply_half_value
            .compute_all(price, starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.height,
                    &self.height_to_supply,
                    |(h, v, ..)| (h, v / 2),
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_supply_half
            .compute_all(price, starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.dateindex,
                    self.indexes_to_supply.sats.dateindex.as_ref().unwrap(),
                    |(i, sats, ..)| (i, sats / 2),
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_sent
            .compute_rest(indexes, price, starting_indexes, exit, Some(&self.height_to_sent))?;

        self.indexes_to_coinblocks_destroyed
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.height,
                    &self.height_to_satblocks_destroyed,
                    |(i, v, ..)| (i, StoredF64::from(Bitcoin::from(v))),
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_coindays_destroyed
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.height,
                    &self.height_to_satdays_destroyed,
                    |(i, v, ..)| (i, StoredF64::from(Bitcoin::from(v))),
                    exit,
                )?;
                Ok(())
            })?;

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn compute_rest_part2(
        &mut self,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
        starting_indexes: &Indexes,
        height_to_supply: &impl IterableVec<Height, Bitcoin>,
        dateindex_to_supply: &impl IterableVec<DateIndex, Bitcoin>,
        height_to_market_cap: Option<&impl IterableVec<Height, Dollars>>,
        dateindex_to_market_cap: Option<&impl IterableVec<DateIndex, Dollars>>,
        height_to_realized_cap: Option<&impl IterableVec<Height, Dollars>>,
        dateindex_to_realized_cap: Option<&impl IterableVec<DateIndex, Dollars>>,
        exit: &Exit,
    ) -> Result<()> {
        if let Some(v) = self.indexes_to_supply_rel_to_circulating_supply.as_mut() {
            v.compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_percentage(
                    starting_indexes.height,
                    &self.height_to_supply_value.bitcoin,
                    height_to_supply,
                    exit,
                )?;
                Ok(())
            })?;
        }

        if let Some(indexes_to_realized_cap) = self.indexes_to_realized_cap.as_mut() {
            let height_to_market_cap = height_to_market_cap.unwrap();
            let dateindex_to_market_cap = dateindex_to_market_cap.unwrap();

            indexes_to_realized_cap.compute_rest(
                indexes,
                starting_indexes,
                exit,
                Some(self.height_to_realized_cap.as_ref().unwrap()),
            )?;

            self.indexes_to_realized_price
                .as_mut()
                .unwrap()
                .compute_all(indexes, starting_indexes, exit, |vec| {
                    vec.compute_divide(
                        starting_indexes.height,
                        self.height_to_realized_cap.as_ref().unwrap(),
                        &self.height_to_supply_value.bitcoin,
                        exit,
                    )?;
                    Ok(())
                })?;

            self.indexes_to_realized_price_extra
                .as_mut()
                .unwrap()
                .compute_rest(
                    price.as_ref().unwrap(),
                    starting_indexes,
                    exit,
                    Some(
                        self.indexes_to_realized_price
                            .as_ref()
                            .unwrap()
                            .dateindex
                            .unwrap_last(),
                    ),
                )?;

            self.indexes_to_realized_profit
                .as_mut()
                .unwrap()
                .compute_rest(
                    indexes,
                    starting_indexes,
                    exit,
                    Some(self.height_to_realized_profit.as_ref().unwrap()),
                )?;

            self.indexes_to_realized_loss
                .as_mut()
                .unwrap()
                .compute_rest(
                    indexes,
                    starting_indexes,
                    exit,
                    Some(self.height_to_realized_loss.as_ref().unwrap()),
                )?;

            self.indexes_to_neg_realized_loss
                .as_mut()
                .unwrap()
                .compute_all(indexes, starting_indexes, exit, |vec| {
                    vec.compute_transform(
                        starting_indexes.height,
                        self.height_to_realized_loss.as_ref().unwrap(),
                        |(i, v, ..)| (i, v * -1_i64),
                        exit,
                    )?;
                    Ok(())
                })?;

            self.indexes_to_value_created
                .as_mut()
                .unwrap()
                .compute_rest(
                    indexes,
                    starting_indexes,
                    exit,
                    Some(self.height_to_value_created.as_ref().unwrap()),
                )?;

            self.indexes_to_value_destroyed
                .as_mut()
                .unwrap()
                .compute_rest(
                    indexes,
                    starting_indexes,
                    exit,
                    Some(self.height_to_value_destroyed.as_ref().unwrap()),
                )?;

            self.indexes_to_realized_cap_30d_delta
                .as_mut()
                .unwrap()
                .compute_all(starting_indexes, exit, |vec| {
                    vec.compute_change(
                        starting_indexes.dateindex,
                        self.indexes_to_realized_cap
                            .as_ref()
                            .unwrap()
                            .dateindex
                            .unwrap_last(),
                        30,
                        exit,
                    )?;
                    Ok(())
                })?;

            self.indexes_to_net_realized_pnl
                .as_mut()
                .unwrap()
                .compute_all(indexes, starting_indexes, exit, |vec| {
                    vec.compute_subtract(
                        starting_indexes.height,
                        self.height_to_realized_profit.as_ref().unwrap(),
                        self.height_to_realized_loss.as_ref().unwrap(),
                        exit,
                    )?;
                    Ok(())
                })?;

            self.indexes_to_realized_value
                .as_mut()
                .unwrap()
                .compute_all(indexes, starting_indexes, exit, |vec| {
                    vec.compute_add(
                        starting_indexes.height,
                        self.height_to_realized_profit.as_ref().unwrap(),
                        self.height_to_realized_loss.as_ref().unwrap(),
                        exit,
                    )?;
                    Ok(())
                })?;

            self.dateindex_to_sopr.as_mut().unwrap().compute_divide(
                starting_indexes.dateindex,
                self.indexes_to_value_created
                    .as_ref()
                    .unwrap()
                    .dateindex
                    .unwrap_sum(),
                self.indexes_to_value_destroyed
                    .as_ref()
                    .unwrap()
                    .dateindex
                    .unwrap_sum(),
                exit,
            )?;

            self.dateindex_to_sopr_7d_ema
                .as_mut()
                .unwrap()
                .compute_ema(
                    starting_indexes.dateindex,
                    self.dateindex_to_sopr.as_ref().unwrap(),
                    7,
                    exit,
                )?;

            self.dateindex_to_sopr_30d_ema
                .as_mut()
                .unwrap()
                .compute_ema(
                    starting_indexes.dateindex,
                    self.dateindex_to_sopr.as_ref().unwrap(),
                    30,
                    exit,
                )?;

            self.dateindex_to_sell_side_risk_ratio
                .as_mut()
                .unwrap()
                .compute_percentage(
                    starting_indexes.dateindex,
                    self.indexes_to_realized_value
                        .as_ref()
                        .unwrap()
                        .dateindex
                        .unwrap_sum(),
                    self.indexes_to_realized_cap
                        .as_ref()
                        .unwrap()
                        .dateindex
                        .unwrap_last(),
                    exit,
                )?;

            self.dateindex_to_sell_side_risk_ratio_7d_ema
                .as_mut()
                .unwrap()
                .compute_ema(
                    starting_indexes.dateindex,
                    self.dateindex_to_sell_side_risk_ratio.as_ref().unwrap(),
                    7,
                    exit,
                )?;

            self.dateindex_to_sell_side_risk_ratio_30d_ema
                .as_mut()
                .unwrap()
                .compute_ema(
                    starting_indexes.dateindex,
                    self.dateindex_to_sell_side_risk_ratio.as_ref().unwrap(),
                    30,
                    exit,
                )?;

            self.indexes_to_supply_in_profit
                .as_mut()
                .unwrap()
                .compute_rest(
                    price,
                    starting_indexes,
                    exit,
                    Some(self.dateindex_to_supply_in_profit.as_ref().unwrap()),
                )?;
            self.indexes_to_supply_in_loss
                .as_mut()
                .unwrap()
                .compute_rest(
                    price,
                    starting_indexes,
                    exit,
                    Some(self.dateindex_to_supply_in_loss.as_ref().unwrap()),
                )?;
            self.indexes_to_unrealized_profit
                .as_mut()
                .unwrap()
                .compute_rest(
                    starting_indexes,
                    exit,
                    Some(self.dateindex_to_unrealized_profit.as_ref().unwrap()),
                )?;
            self.indexes_to_unrealized_loss
                .as_mut()
                .unwrap()
                .compute_rest(
                    starting_indexes,
                    exit,
                    Some(self.dateindex_to_unrealized_loss.as_ref().unwrap()),
                )?;
            self.height_to_total_unrealized_pnl
                .as_mut()
                .unwrap()
                .compute_add(
                    starting_indexes.height,
                    self.height_to_unrealized_profit.as_ref().unwrap(),
                    self.height_to_unrealized_loss.as_ref().unwrap(),
                    exit,
                )?;
            self.indexes_to_total_unrealized_pnl
                .as_mut()
                .unwrap()
                .compute_all(starting_indexes, exit, |vec| {
                    vec.compute_add(
                        starting_indexes.dateindex,
                        self.dateindex_to_unrealized_profit.as_ref().unwrap(),
                        self.dateindex_to_unrealized_loss.as_ref().unwrap(),
                        exit,
                    )?;
                    Ok(())
                })?;
            self.height_to_total_realized_pnl
                .as_mut()
                .unwrap()
                .compute_add(
                    starting_indexes.height,
                    self.height_to_realized_profit.as_ref().unwrap(),
                    self.height_to_realized_loss.as_ref().unwrap(),
                    exit,
                )?;
            self.indexes_to_total_realized_pnl
                .as_mut()
                .unwrap()
                .compute_all(starting_indexes, exit, |vec| {
                    vec.compute_add(
                        starting_indexes.dateindex,
                        self.indexes_to_realized_profit
                            .as_ref()
                            .unwrap()
                            .dateindex
                            .unwrap_sum(),
                        self.indexes_to_realized_loss
                            .as_ref()
                            .unwrap()
                            .dateindex
                            .unwrap_sum(),
                        exit,
                    )?;
                    Ok(())
                })?;

            self.indexes_to_min_price_paid
                .as_mut()
                .unwrap()
                .compute_rest(
                    indexes,
                    starting_indexes,
                    exit,
                    Some(self.height_to_min_price_paid.as_ref().unwrap()),
                )?;
            self.indexes_to_max_price_paid
                .as_mut()
                .unwrap()
                .compute_rest(
                    indexes,
                    starting_indexes,
                    exit,
                    Some(self.height_to_max_price_paid.as_ref().unwrap()),
                )?;

            self.height_to_neg_unrealized_loss
                .as_mut()
                .unwrap()
                .compute_transform(
                    starting_indexes.height,
                    self.height_to_unrealized_loss.as_ref().unwrap(),
                    |(h, v, ..)| (h, v * -1_i64),
                    exit,
                )?;
            self.indexes_to_neg_unrealized_loss
                .as_mut()
                .unwrap()
                .compute_all(starting_indexes, exit, |v| {
                    v.compute_transform(
                        starting_indexes.dateindex,
                        self.dateindex_to_unrealized_loss.as_ref().unwrap(),
                        |(h, v, ..)| (h, v * -1_i64),
                        exit,
                    )?;
                    Ok(())
                })?;
            self.height_to_net_unrealized_pnl
                .as_mut()
                .unwrap()
                .compute_subtract(
                    starting_indexes.height,
                    self.height_to_unrealized_profit.as_ref().unwrap(),
                    self.height_to_unrealized_loss.as_ref().unwrap(),
                    exit,
                )?;

            self.indexes_to_net_unrealized_pnl
                .as_mut()
                .unwrap()
                .compute_all(starting_indexes, exit, |vec| {
                    vec.compute_subtract(
                        starting_indexes.dateindex,
                        self.dateindex_to_unrealized_profit.as_ref().unwrap(),
                        self.dateindex_to_unrealized_loss.as_ref().unwrap(),
                        exit,
                    )?;
                    Ok(())
                })?;
            self.height_to_unrealized_profit_rel_to_market_cap
                .as_mut()
                .unwrap()
                .compute_percentage(
                    starting_indexes.height,
                    self.height_to_unrealized_profit.as_ref().unwrap(),
                    height_to_market_cap,
                    exit,
                )?;
            self.height_to_unrealized_loss_rel_to_market_cap
                .as_mut()
                .unwrap()
                .compute_percentage(
                    starting_indexes.height,
                    self.height_to_unrealized_loss.as_ref().unwrap(),
                    height_to_market_cap,
                    exit,
                )?;
            self.height_to_neg_unrealized_loss_rel_to_market_cap
                .as_mut()
                .unwrap()
                .compute_percentage(
                    starting_indexes.height,
                    self.height_to_neg_unrealized_loss.as_ref().unwrap(),
                    height_to_market_cap,
                    exit,
                )?;
            self.height_to_net_unrealized_pnl_rel_to_market_cap
                .as_mut()
                .unwrap()
                .compute_percentage(
                    starting_indexes.height,
                    self.height_to_net_unrealized_pnl.as_ref().unwrap(),
                    height_to_market_cap,
                    exit,
                )?;
            self.indexes_to_unrealized_profit_rel_to_market_cap
                .as_mut()
                .unwrap()
                .compute_all(starting_indexes, exit, |vec| {
                    vec.compute_percentage(
                        starting_indexes.dateindex,
                        self.dateindex_to_unrealized_profit.as_ref().unwrap(),
                        dateindex_to_market_cap,
                        exit,
                    )?;
                    Ok(())
                })?;
            self.indexes_to_unrealized_loss_rel_to_market_cap
                .as_mut()
                .unwrap()
                .compute_all(starting_indexes, exit, |vec| {
                    vec.compute_percentage(
                        starting_indexes.dateindex,
                        self.dateindex_to_unrealized_loss.as_ref().unwrap(),
                        dateindex_to_market_cap,
                        exit,
                    )?;
                    Ok(())
                })?;
            self.indexes_to_neg_unrealized_loss_rel_to_market_cap
                .as_mut()
                .unwrap()
                .compute_all(starting_indexes, exit, |vec| {
                    vec.compute_percentage(
                        starting_indexes.dateindex,
                        self.indexes_to_neg_unrealized_loss
                            .as_ref()
                            .unwrap()
                            .dateindex
                            .as_ref()
                            .unwrap(),
                        dateindex_to_market_cap,
                        exit,
                    )?;
                    Ok(())
                })?;
            self.indexes_to_net_unrealized_pnl_rel_to_market_cap
                .as_mut()
                .unwrap()
                .compute_all(starting_indexes, exit, |vec| {
                    vec.compute_percentage(
                        starting_indexes.dateindex,
                        self.indexes_to_net_unrealized_pnl
                            .as_ref()
                            .unwrap()
                            .dateindex
                            .as_ref()
                            .unwrap(),
                        dateindex_to_market_cap,
                        exit,
                    )?;
                    Ok(())
                })?;

            if self
                .height_to_unrealized_profit_rel_to_own_market_cap
                .is_some()
            {
                self.height_to_unrealized_profit_rel_to_own_market_cap
                    .as_mut()
                    .unwrap()
                    .compute_percentage(
                        starting_indexes.height,
                        self.height_to_unrealized_profit.as_ref().unwrap(),
                        self.height_to_supply_value.dollars.as_ref().unwrap(),
                        exit,
                    )?;
                self.height_to_unrealized_loss_rel_to_own_market_cap
                    .as_mut()
                    .unwrap()
                    .compute_percentage(
                        starting_indexes.height,
                        self.height_to_unrealized_loss.as_ref().unwrap(),
                        self.height_to_supply_value.dollars.as_ref().unwrap(),
                        exit,
                    )?;
                self.height_to_neg_unrealized_loss_rel_to_own_market_cap
                    .as_mut()
                    .unwrap()
                    .compute_percentage(
                        starting_indexes.height,
                        self.height_to_neg_unrealized_loss.as_ref().unwrap(),
                        self.height_to_supply_value.dollars.as_ref().unwrap(),
                        exit,
                    )?;
                self.height_to_net_unrealized_pnl_rel_to_own_market_cap
                    .as_mut()
                    .unwrap()
                    .compute_percentage(
                        starting_indexes.height,
                        self.height_to_net_unrealized_pnl.as_ref().unwrap(),
                        self.height_to_supply_value.dollars.as_ref().unwrap(),
                        exit,
                    )?;
                self.indexes_to_unrealized_profit_rel_to_own_market_cap
                    .as_mut()
                    .unwrap()
                    .compute_all(starting_indexes, exit, |vec| {
                        vec.compute_percentage(
                            starting_indexes.dateindex,
                            self.dateindex_to_unrealized_profit.as_ref().unwrap(),
                            self.indexes_to_supply
                                .dollars
                                .as_ref()
                                .unwrap()
                                .dateindex
                                .as_ref()
                                .unwrap(),
                            exit,
                        )?;
                        Ok(())
                    })?;
                self.indexes_to_unrealized_loss_rel_to_own_market_cap
                    .as_mut()
                    .unwrap()
                    .compute_all(starting_indexes, exit, |vec| {
                        vec.compute_percentage(
                            starting_indexes.dateindex,
                            self.dateindex_to_unrealized_loss.as_ref().unwrap(),
                            self.indexes_to_supply
                                .dollars
                                .as_ref()
                                .unwrap()
                                .dateindex
                                .as_ref()
                                .unwrap(),
                            exit,
                        )?;
                        Ok(())
                    })?;
                self.indexes_to_neg_unrealized_loss_rel_to_own_market_cap
                    .as_mut()
                    .unwrap()
                    .compute_all(starting_indexes, exit, |vec| {
                        vec.compute_percentage(
                            starting_indexes.dateindex,
                            self.indexes_to_neg_unrealized_loss
                                .as_ref()
                                .unwrap()
                                .dateindex
                                .as_ref()
                                .unwrap(),
                            self.indexes_to_supply
                                .dollars
                                .as_ref()
                                .unwrap()
                                .dateindex
                                .as_ref()
                                .unwrap(),
                            exit,
                        )?;
                        Ok(())
                    })?;
                self.indexes_to_net_unrealized_pnl_rel_to_own_market_cap
                    .as_mut()
                    .unwrap()
                    .compute_all(starting_indexes, exit, |vec| {
                        vec.compute_percentage(
                            starting_indexes.dateindex,
                            self.indexes_to_net_unrealized_pnl
                                .as_ref()
                                .unwrap()
                                .dateindex
                                .as_ref()
                                .unwrap(),
                            self.indexes_to_supply
                                .dollars
                                .as_ref()
                                .unwrap()
                                .dateindex
                                .as_ref()
                                .unwrap(),
                            exit,
                        )?;
                        Ok(())
                    })?;
            }

            if self
                .height_to_unrealized_profit_rel_to_own_total_unrealized_pnl
                .is_some()
            {
                self.height_to_unrealized_profit_rel_to_own_total_unrealized_pnl
                    .as_mut()
                    .unwrap()
                    .compute_percentage(
                        starting_indexes.height,
                        self.height_to_unrealized_profit.as_ref().unwrap(),
                        self.height_to_total_unrealized_pnl.as_ref().unwrap(),
                        exit,
                    )?;
                self.height_to_unrealized_loss_rel_to_own_total_unrealized_pnl
                    .as_mut()
                    .unwrap()
                    .compute_percentage(
                        starting_indexes.height,
                        self.height_to_unrealized_loss.as_ref().unwrap(),
                        self.height_to_total_unrealized_pnl.as_ref().unwrap(),
                        exit,
                    )?;
                self.height_to_neg_unrealized_loss_rel_to_own_total_unrealized_pnl
                    .as_mut()
                    .unwrap()
                    .compute_percentage(
                        starting_indexes.height,
                        self.height_to_neg_unrealized_loss.as_ref().unwrap(),
                        self.height_to_total_unrealized_pnl.as_ref().unwrap(),
                        exit,
                    )?;
                self.height_to_net_unrealized_pnl_rel_to_own_total_unrealized_pnl
                    .as_mut()
                    .unwrap()
                    .compute_percentage(
                        starting_indexes.height,
                        self.height_to_net_unrealized_pnl.as_ref().unwrap(),
                        self.height_to_total_unrealized_pnl.as_ref().unwrap(),
                        exit,
                    )?;
                self.indexes_to_unrealized_profit_rel_to_own_total_unrealized_pnl
                    .as_mut()
                    .unwrap()
                    .compute_all(starting_indexes, exit, |vec| {
                        vec.compute_percentage(
                            starting_indexes.dateindex,
                            self.dateindex_to_unrealized_profit.as_ref().unwrap(),
                            self.indexes_to_total_unrealized_pnl
                                .as_ref()
                                .unwrap()
                                .dateindex
                                .as_ref()
                                .unwrap(),
                            exit,
                        )?;
                        Ok(())
                    })?;
                self.indexes_to_unrealized_loss_rel_to_own_total_unrealized_pnl
                    .as_mut()
                    .unwrap()
                    .compute_all(starting_indexes, exit, |vec| {
                        vec.compute_percentage(
                            starting_indexes.dateindex,
                            self.dateindex_to_unrealized_loss.as_ref().unwrap(),
                            self.indexes_to_total_unrealized_pnl
                                .as_ref()
                                .unwrap()
                                .dateindex
                                .as_ref()
                                .unwrap(),
                            exit,
                        )?;
                        Ok(())
                    })?;
                self.indexes_to_neg_unrealized_loss_rel_to_own_total_unrealized_pnl
                    .as_mut()
                    .unwrap()
                    .compute_all(starting_indexes, exit, |vec| {
                        vec.compute_percentage(
                            starting_indexes.dateindex,
                            self.indexes_to_neg_unrealized_loss
                                .as_ref()
                                .unwrap()
                                .dateindex
                                .as_ref()
                                .unwrap(),
                            self.indexes_to_total_unrealized_pnl
                                .as_ref()
                                .unwrap()
                                .dateindex
                                .as_ref()
                                .unwrap(),
                            exit,
                        )?;
                        Ok(())
                    })?;
                self.indexes_to_net_unrealized_pnl_rel_to_own_total_unrealized_pnl
                    .as_mut()
                    .unwrap()
                    .compute_all(starting_indexes, exit, |vec| {
                        vec.compute_percentage(
                            starting_indexes.dateindex,
                            self.indexes_to_net_unrealized_pnl
                                .as_ref()
                                .unwrap()
                                .dateindex
                                .as_ref()
                                .unwrap(),
                            self.indexes_to_total_unrealized_pnl
                                .as_ref()
                                .unwrap()
                                .dateindex
                                .as_ref()
                                .unwrap(),
                            exit,
                        )?;
                        Ok(())
                    })?;
            }

            self.indexes_to_realized_profit_rel_to_realized_cap
                .as_mut()
                .unwrap()
                .compute_all(indexes, starting_indexes, exit, |vec| {
                    vec.compute_percentage(
                        starting_indexes.height,
                        self.height_to_realized_profit.as_ref().unwrap(),
                        *height_to_realized_cap.as_ref().unwrap(),
                        exit,
                    )?;
                    Ok(())
                })?;

            self.indexes_to_realized_loss_rel_to_realized_cap
                .as_mut()
                .unwrap()
                .compute_all(indexes, starting_indexes, exit, |vec| {
                    vec.compute_percentage(
                        starting_indexes.height,
                        self.height_to_realized_loss.as_ref().unwrap(),
                        *height_to_realized_cap.as_ref().unwrap(),
                        exit,
                    )?;
                    Ok(())
                })?;

            self.indexes_to_net_realized_pnl_rel_to_realized_cap
                .as_mut()
                .unwrap()
                .compute_all(indexes, starting_indexes, exit, |vec| {
                    vec.compute_percentage(
                        starting_indexes.height,
                        self.indexes_to_net_realized_pnl
                            .as_ref()
                            .unwrap()
                            .height
                            .as_ref()
                            .unwrap(),
                        *height_to_realized_cap.as_ref().unwrap(),
                        exit,
                    )?;
                    Ok(())
                })?;

            self.height_to_supply_in_loss_value
                .as_mut()
                .unwrap()
                .compute_rest(
                    price,
                    starting_indexes,
                    exit,
                    Some(self.height_to_supply_in_loss.as_ref().unwrap()),
                )?;
            self.height_to_supply_in_profit_value
                .as_mut()
                .unwrap()
                .compute_rest(
                    price,
                    starting_indexes,
                    exit,
                    Some(self.height_to_supply_in_profit.as_ref().unwrap()),
                )?;
            self.height_to_supply_in_loss_rel_to_own_supply
                .as_mut()
                .unwrap()
                .compute_percentage(
                    starting_indexes.height,
                    &self
                        .height_to_supply_in_loss_value
                        .as_ref()
                        .unwrap()
                        .bitcoin,
                    &self.height_to_supply_value.bitcoin,
                    exit,
                )?;
            self.height_to_supply_in_profit_rel_to_own_supply
                .as_mut()
                .unwrap()
                .compute_percentage(
                    starting_indexes.height,
                    &self
                        .height_to_supply_in_profit_value
                        .as_ref()
                        .unwrap()
                        .bitcoin,
                    &self.height_to_supply_value.bitcoin,
                    exit,
                )?;
            self.indexes_to_supply_in_loss_rel_to_own_supply
                .as_mut()
                .unwrap()
                .compute_all(starting_indexes, exit, |v| {
                    v.compute_percentage(
                        starting_indexes.dateindex,
                        self.indexes_to_supply_in_loss
                            .as_ref()
                            .unwrap()
                            .bitcoin
                            .dateindex
                            .as_ref()
                            .unwrap(),
                        self.indexes_to_supply.bitcoin.dateindex.as_ref().unwrap(),
                        exit,
                    )?;
                    Ok(())
                })?;
            self.indexes_to_supply_in_profit_rel_to_own_supply
                .as_mut()
                .unwrap()
                .compute_all(starting_indexes, exit, |v| {
                    v.compute_percentage(
                        starting_indexes.dateindex,
                        self.indexes_to_supply_in_profit
                            .as_ref()
                            .unwrap()
                            .bitcoin
                            .dateindex
                            .as_ref()
                            .unwrap(),
                        self.indexes_to_supply.bitcoin.dateindex.as_ref().unwrap(),
                        exit,
                    )?;
                    Ok(())
                })?;

            self.indexes_to_net_realized_pnl_cumulative_30d_delta
                .as_mut()
                .unwrap()
                .compute_all(starting_indexes, exit, |v| {
                    v.compute_change(
                        starting_indexes.dateindex,
                        self.indexes_to_net_realized_pnl
                            .as_ref()
                            .unwrap()
                            .dateindex
                            .unwrap_cumulative(),
                        30,
                        exit,
                    )?;
                    Ok(())
                })?;

            self.indexes_to_net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap
                .as_mut()
                .unwrap()
                .compute_all(starting_indexes, exit, |v| {
                    v.compute_percentage(
                        starting_indexes.dateindex,
                        self.indexes_to_net_realized_pnl_cumulative_30d_delta
                            .as_ref()
                            .unwrap()
                            .dateindex
                            .as_ref()
                            .unwrap(),
                        *dateindex_to_realized_cap.as_ref().unwrap(),
                        exit,
                    )?;
                    Ok(())
                })?;

            self.indexes_to_net_realized_pnl_cumulative_30d_delta_rel_to_market_cap
                .as_mut()
                .unwrap()
                .compute_all(starting_indexes, exit, |v| {
                    v.compute_percentage(
                        starting_indexes.dateindex,
                        self.indexes_to_net_realized_pnl_cumulative_30d_delta
                            .as_ref()
                            .unwrap()
                            .dateindex
                            .as_ref()
                            .unwrap(),
                        dateindex_to_market_cap,
                        exit,
                    )?;
                    Ok(())
                })?;

            if self
                .height_to_supply_in_profit_rel_to_circulating_supply
                .as_mut()
                .is_some()
            {
                self.height_to_supply_in_loss_rel_to_circulating_supply
                    .as_mut()
                    .unwrap()
                    .compute_percentage(
                        starting_indexes.height,
                        &self
                            .height_to_supply_in_loss_value
                            .as_ref()
                            .unwrap()
                            .bitcoin,
                        height_to_supply,
                        exit,
                    )?;
                self.height_to_supply_in_profit_rel_to_circulating_supply
                    .as_mut()
                    .unwrap()
                    .compute_percentage(
                        starting_indexes.height,
                        &self
                            .height_to_supply_in_profit_value
                            .as_ref()
                            .unwrap()
                            .bitcoin,
                        height_to_supply,
                        exit,
                    )?;
                self.indexes_to_supply_in_loss_rel_to_circulating_supply
                    .as_mut()
                    .unwrap()
                    .compute_all(starting_indexes, exit, |v| {
                        v.compute_percentage(
                            starting_indexes.dateindex,
                            self.indexes_to_supply_in_loss
                                .as_ref()
                                .unwrap()
                                .bitcoin
                                .dateindex
                                .as_ref()
                                .unwrap(),
                            dateindex_to_supply,
                            exit,
                        )?;
                        Ok(())
                    })?;
                self.indexes_to_supply_in_profit_rel_to_circulating_supply
                    .as_mut()
                    .unwrap()
                    .compute_all(starting_indexes, exit, |v| {
                        v.compute_percentage(
                            starting_indexes.dateindex,
                            self.indexes_to_supply_in_profit
                                .as_ref()
                                .unwrap()
                                .bitcoin
                                .dateindex
                                .as_ref()
                                .unwrap(),
                            dateindex_to_supply,
                            exit,
                        )?;
                        Ok(())
                    })?;
            }

            if self.indexes_to_adjusted_value_created.is_some() {
                self.indexes_to_adjusted_value_created
                    .as_mut()
                    .unwrap()
                    .compute_rest(
                        indexes,
                        starting_indexes,
                        exit,
                        Some(self.height_to_adjusted_value_created.as_ref().unwrap()),
                    )?;

                self.indexes_to_adjusted_value_destroyed
                    .as_mut()
                    .unwrap()
                    .compute_rest(
                        indexes,
                        starting_indexes,
                        exit,
                        Some(self.height_to_adjusted_value_destroyed.as_ref().unwrap()),
                    )?;

                self.dateindex_to_adjusted_sopr
                    .as_mut()
                    .unwrap()
                    .compute_divide(
                        starting_indexes.dateindex,
                        self.indexes_to_adjusted_value_created
                            .as_ref()
                            .unwrap()
                            .dateindex
                            .unwrap_sum(),
                        self.indexes_to_adjusted_value_destroyed
                            .as_ref()
                            .unwrap()
                            .dateindex
                            .unwrap_sum(),
                        exit,
                    )?;

                self.dateindex_to_adjusted_sopr_7d_ema
                    .as_mut()
                    .unwrap()
                    .compute_ema(
                        starting_indexes.dateindex,
                        self.dateindex_to_adjusted_sopr.as_ref().unwrap(),
                        7,
                        exit,
                    )?;

                self.dateindex_to_adjusted_sopr_30d_ema
                    .as_mut()
                    .unwrap()
                    .compute_ema(
                        starting_indexes.dateindex,
                        self.dateindex_to_adjusted_sopr.as_ref().unwrap(),
                        30,
                        exit,
                    )?;
            }

            if let Some(indexes_to_realized_cap_rel_to_own_market_cap) =
                self.indexes_to_realized_cap_rel_to_own_market_cap.as_mut()
            {
                indexes_to_realized_cap_rel_to_own_market_cap.compute_all(
                    indexes,
                    starting_indexes,
                    exit,
                    |v| {
                        v.compute_percentage(
                            starting_indexes.height,
                            self.height_to_realized_cap.as_ref().unwrap(),
                            self.height_to_supply_value.dollars.as_ref().unwrap(),
                            exit,
                        )?;
                        Ok(())
                    },
                )?;
            }
        }

        if let Some(dateindex_to_realized_profit_to_loss_ratio) =
            self.dateindex_to_realized_profit_to_loss_ratio.as_mut()
        {
            dateindex_to_realized_profit_to_loss_ratio.compute_divide(
                starting_indexes.dateindex,
                self.indexes_to_realized_profit
                    .as_ref()
                    .unwrap()
                    .dateindex
                    .unwrap_sum(),
                self.indexes_to_realized_loss
                    .as_ref()
                    .unwrap()
                    .dateindex
                    .unwrap_sum(),
                exit,
            )?;
        }

        Ok(())
    }
}
