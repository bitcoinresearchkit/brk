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
    utils::OptionExt,
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
    ) -> Result<Self> {
        let compute_dollars = price.is_some();
        let extended = filter.is_extended(context);
        let compute_rel_to_all = filter.compute_rel_to_all();
        let compute_adjusted = filter.compute_adjusted(context);

        let version = parent_version + Version::ZERO;

        let name_prefix = filter.to_full_name(context);
        let suffix = |s: &str| {
            if name_prefix.is_empty() {
                s.to_string()
            } else {
                format!("{name_prefix}_{s}")
            }
        };

        // Helper macros for imports
        macro_rules! eager {
            ($idx:ty, $val:ty, $name:expr, $v:expr) => {
                EagerVec::<PcoVec<$idx, $val>>::forced_import(db, &suffix($name), version + $v).unwrap()
            };
        }
        macro_rules! computed_h {
            ($name:expr, $source:expr, $v:expr, $opts:expr $(,)?) => {
                ComputedVecsFromHeight::forced_import(db, &suffix($name), $source, version + $v, indexes, $opts).unwrap()
            };
        }
        macro_rules! computed_di {
            ($name:expr, $source:expr, $v:expr, $opts:expr $(,)?) => {
                ComputedVecsFromDateIndex::forced_import(db, &suffix($name), $source, version + $v, indexes, $opts).unwrap()
            };
        }

        // Common option patterns
        let v0 = Version::ZERO;
        let v1 = Version::ONE;
        let v2 = Version::TWO;
        let v3 = Version::new(3);
        let last = || VecBuilderOptions::default().add_last();
        let sum = || VecBuilderOptions::default().add_sum();
        let sum_cum = || VecBuilderOptions::default().add_sum().add_cumulative();

        // Pre-create dateindex vecs that are used in computed vecs
        let dateindex_to_supply_in_profit =
            compute_dollars.then(|| eager!(DateIndex, Sats,"supply_in_profit", v0));
        let dateindex_to_supply_in_loss = compute_dollars.then(|| eager!(DateIndex, Sats,"supply_in_loss", v0));
        let dateindex_to_unrealized_profit =
            compute_dollars.then(|| eager!(DateIndex, Dollars,"unrealized_profit", v0));
        let dateindex_to_unrealized_loss =
            compute_dollars.then(|| eager!(DateIndex, Dollars,"unrealized_loss", v0));

        Ok(Self {
            filter,

            // Supply & UTXO count (always computed)
            height_to_supply: EagerVec::forced_import(db, &suffix("supply"), version + v0)?,
            height_to_supply_value: ComputedHeightValueVecs::forced_import(
                db,
                &suffix("supply"),
                Source::None,
                version + v0,
                compute_dollars,
            )?,
            indexes_to_supply: ComputedValueVecsFromDateIndex::forced_import(
                db,
                &suffix("supply"),
                Source::Compute,
                version + v1,
                last(),
                compute_dollars,
                indexes,
            )?,
            height_to_utxo_count: EagerVec::forced_import(db, &suffix("utxo_count"), version + v0)?,
            indexes_to_utxo_count: computed_h!("utxo_count", Source::None, v0, last()),
            height_to_supply_half_value: ComputedHeightValueVecs::forced_import(
                db,
                &suffix("supply_half"),
                Source::Compute,
                version + v0,
                compute_dollars,
            )?,
            indexes_to_supply_half: ComputedValueVecsFromDateIndex::forced_import(
                db,
                &suffix("supply_half"),
                Source::Compute,
                version + v0,
                last(),
                compute_dollars,
                indexes,
            )?,

            // Sent & destroyed (always computed)
            height_to_sent: EagerVec::forced_import(db, &suffix("sent"), version + v0)?,
            height_to_satblocks_destroyed: EagerVec::forced_import(
                db,
                &suffix("satblocks_destroyed"),
                version + v0,
            )?,
            height_to_satdays_destroyed: EagerVec::forced_import(
                db,
                &suffix("satdays_destroyed"),
                version + v0,
            )?,
            indexes_to_sent: ComputedValueVecsFromHeight::forced_import(
                db,
                &suffix("sent"),
                Source::Compute,
                version + v0,
                sum(),
                compute_dollars,
                indexes,
            )?,
            indexes_to_coinblocks_destroyed: computed_h!(
                "coinblocks_destroyed",
                Source::Compute,
                v2,
                sum_cum(),
            ),
            indexes_to_coindays_destroyed: computed_h!(
                "coindays_destroyed",
                Source::Compute,
                v2,
                sum_cum(),
            ),

            // Realized cap & related (conditional on compute_dollars)
            height_to_realized_cap: compute_dollars.then(|| eager!(Height, Dollars,"realized_cap", v0)),
            indexes_to_realized_cap: compute_dollars
                .then(|| computed_h!("realized_cap", Source::None, v0, last())),
            indexes_to_realized_price: compute_dollars
                .then(|| computed_h!("realized_price", Source::Compute, v0, last())),
            indexes_to_realized_price_extra: compute_dollars.then(|| {
                ComputedRatioVecsFromDateIndex::forced_import(
                    db,
                    &suffix("realized_price"),
                    Source::None,
                    version + v0,
                    indexes,
                    extended,
                )
                .unwrap()
            }),
            indexes_to_realized_cap_rel_to_own_market_cap: (compute_dollars && extended).then(|| {
                computed_h!("realized_cap_rel_to_own_market_cap", Source::Compute, v0, last())
            }),
            indexes_to_realized_cap_30d_delta: compute_dollars
                .then(|| computed_di!("realized_cap_30d_delta", Source::Compute, v0, last())),

            // Realized profit & loss
            height_to_realized_profit: compute_dollars.then(|| eager!(Height, Dollars,"realized_profit", v0)),
            indexes_to_realized_profit: compute_dollars
                .then(|| computed_h!("realized_profit", Source::None, v0, sum_cum())),
            height_to_realized_loss: compute_dollars.then(|| eager!(Height, Dollars,"realized_loss", v0)),
            indexes_to_realized_loss: compute_dollars
                .then(|| computed_h!("realized_loss", Source::None, v0, sum_cum())),
            indexes_to_neg_realized_loss: compute_dollars
                .then(|| computed_h!("neg_realized_loss", Source::Compute, v1, sum_cum())),
            indexes_to_net_realized_pnl: compute_dollars
                .then(|| computed_h!("net_realized_pnl", Source::Compute, v0, sum_cum())),
            indexes_to_realized_value: compute_dollars
                .then(|| computed_h!("realized_value", Source::Compute, v0, sum())),
            indexes_to_realized_profit_rel_to_realized_cap: compute_dollars
                .then(|| computed_h!("realized_profit_rel_to_realized_cap", Source::Compute, v0, sum())),
            indexes_to_realized_loss_rel_to_realized_cap: compute_dollars
                .then(|| computed_h!("realized_loss_rel_to_realized_cap", Source::Compute, v0, sum())),
            indexes_to_net_realized_pnl_rel_to_realized_cap: compute_dollars
                .then(|| computed_h!("net_realized_pnl_rel_to_realized_cap", Source::Compute, v1, sum())),
            height_to_total_realized_pnl: compute_dollars.then(|| eager!(Height, Dollars,"total_realized_pnl", v0)),
            indexes_to_total_realized_pnl: compute_dollars
                .then(|| computed_di!("total_realized_pnl", Source::Compute, v1, sum())),
            dateindex_to_realized_profit_to_loss_ratio: (compute_dollars && extended)
                .then(|| eager!(DateIndex, StoredF64,"realized_profit_to_loss_ratio", v1)),

            // Value created & destroyed
            height_to_value_created: compute_dollars.then(|| eager!(Height, Dollars,"value_created", v0)),
            indexes_to_value_created: compute_dollars
                .then(|| computed_h!("value_created", Source::None, v0, sum())),
            height_to_value_destroyed: compute_dollars.then(|| eager!(Height, Dollars,"value_destroyed", v0)),
            indexes_to_value_destroyed: compute_dollars
                .then(|| computed_h!("value_destroyed", Source::None, v0, sum())),
            height_to_adjusted_value_created: (compute_dollars && compute_adjusted)
                .then(|| eager!(Height, Dollars,"adjusted_value_created", v0)),
            indexes_to_adjusted_value_created: (compute_dollars && compute_adjusted)
                .then(|| computed_h!("adjusted_value_created", Source::None, v0, sum())),
            height_to_adjusted_value_destroyed: (compute_dollars && compute_adjusted)
                .then(|| eager!(Height, Dollars,"adjusted_value_destroyed", v0)),
            indexes_to_adjusted_value_destroyed: (compute_dollars && compute_adjusted)
                .then(|| computed_h!("adjusted_value_destroyed", Source::None, v0, sum())),

            // SOPR
            dateindex_to_sopr: compute_dollars.then(|| eager!(DateIndex, StoredF64,"sopr", v1)),
            dateindex_to_sopr_7d_ema: compute_dollars.then(|| eager!(DateIndex, StoredF64,"sopr_7d_ema", v1)),
            dateindex_to_sopr_30d_ema: compute_dollars.then(|| eager!(DateIndex, StoredF64,"sopr_30d_ema", v1)),
            dateindex_to_adjusted_sopr: (compute_dollars && compute_adjusted)
                .then(|| eager!(DateIndex, StoredF64,"adjusted_sopr", v1)),
            dateindex_to_adjusted_sopr_7d_ema: (compute_dollars && compute_adjusted)
                .then(|| eager!(DateIndex, StoredF64,"adjusted_sopr_7d_ema", v1)),
            dateindex_to_adjusted_sopr_30d_ema: (compute_dollars && compute_adjusted)
                .then(|| eager!(DateIndex, StoredF64,"adjusted_sopr_30d_ema", v1)),

            // Sell side risk ratio
            dateindex_to_sell_side_risk_ratio: compute_dollars
                .then(|| eager!(DateIndex, StoredF32,"sell_side_risk_ratio", v1)),
            dateindex_to_sell_side_risk_ratio_7d_ema: compute_dollars
                .then(|| eager!(DateIndex, StoredF32,"sell_side_risk_ratio_7d_ema", v1)),
            dateindex_to_sell_side_risk_ratio_30d_ema: compute_dollars
                .then(|| eager!(DateIndex, StoredF32,"sell_side_risk_ratio_30d_ema", v1)),

            // Supply in profit/loss
            height_to_supply_in_profit: compute_dollars.then(|| eager!(Height, Sats,"supply_in_profit", v0)),
            indexes_to_supply_in_profit: compute_dollars.then(|| {
                ComputedValueVecsFromDateIndex::forced_import(
                    db,
                    &suffix("supply_in_profit"),
                    dateindex_to_supply_in_profit.as_ref().map(|v| v.boxed_clone()).into(),
                    version + v0,
                    last(),
                    compute_dollars,
                    indexes,
                )
                .unwrap()
            }),
            height_to_supply_in_loss: compute_dollars.then(|| eager!(Height, Sats,"supply_in_loss", v0)),
            indexes_to_supply_in_loss: compute_dollars.then(|| {
                ComputedValueVecsFromDateIndex::forced_import(
                    db,
                    &suffix("supply_in_loss"),
                    dateindex_to_supply_in_loss.as_ref().map(|v| v.boxed_clone()).into(),
                    version + v0,
                    last(),
                    compute_dollars,
                    indexes,
                )
                .unwrap()
            }),
            height_to_supply_in_loss_value: compute_dollars.then(|| {
                ComputedHeightValueVecs::forced_import(
                    db,
                    &suffix("supply_in_loss"),
                    Source::None,
                    version + v0,
                    compute_dollars,
                )
                .unwrap()
            }),
            height_to_supply_in_profit_value: compute_dollars.then(|| {
                ComputedHeightValueVecs::forced_import(
                    db,
                    &suffix("supply_in_profit"),
                    Source::None,
                    version + v0,
                    compute_dollars,
                )
                .unwrap()
            }),
            height_to_supply_in_loss_rel_to_own_supply: compute_dollars
                .then(|| eager!(Height, StoredF64,"supply_in_loss_rel_to_own_supply", v1)),
            height_to_supply_in_profit_rel_to_own_supply: compute_dollars
                .then(|| eager!(Height, StoredF64,"supply_in_profit_rel_to_own_supply", v1)),
            indexes_to_supply_in_loss_rel_to_own_supply: compute_dollars
                .then(|| computed_di!("supply_in_loss_rel_to_own_supply", Source::Compute, v1, last())),
            indexes_to_supply_in_profit_rel_to_own_supply: compute_dollars
                .then(|| computed_di!("supply_in_profit_rel_to_own_supply", Source::Compute, v1, last())),
            indexes_to_supply_rel_to_circulating_supply: compute_rel_to_all
                .then(|| computed_h!("supply_rel_to_circulating_supply", Source::Compute, v1, last())),
            height_to_supply_in_loss_rel_to_circulating_supply: (compute_rel_to_all && compute_dollars)
                .then(|| eager!(Height, StoredF64,"supply_in_loss_rel_to_circulating_supply", v1)),
            height_to_supply_in_profit_rel_to_circulating_supply: (compute_rel_to_all && compute_dollars)
                .then(|| eager!(Height, StoredF64,"supply_in_profit_rel_to_circulating_supply", v1)),
            indexes_to_supply_in_loss_rel_to_circulating_supply: (compute_rel_to_all && compute_dollars)
                .then(|| computed_di!("supply_in_loss_rel_to_circulating_supply", Source::Compute, v1, last())),
            indexes_to_supply_in_profit_rel_to_circulating_supply: (compute_rel_to_all && compute_dollars)
                .then(|| computed_di!("supply_in_profit_rel_to_circulating_supply", Source::Compute, v1, last())),
            dateindex_to_supply_in_profit,
            dateindex_to_supply_in_loss,

            // Unrealized profit/loss
            height_to_unrealized_profit: compute_dollars.then(|| eager!(Height, Dollars,"unrealized_profit", v0)),
            indexes_to_unrealized_profit: compute_dollars.then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    db,
                    &suffix("unrealized_profit"),
                    dateindex_to_unrealized_profit.as_ref().map(|v| v.boxed_clone()).into(),
                    version + v0,
                    indexes,
                    last(),
                )
                .unwrap()
            }),
            height_to_unrealized_loss: compute_dollars.then(|| eager!(Height, Dollars,"unrealized_loss", v0)),
            indexes_to_unrealized_loss: compute_dollars.then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    db,
                    &suffix("unrealized_loss"),
                    dateindex_to_unrealized_loss.as_ref().map(|v| v.boxed_clone()).into(),
                    version + v0,
                    indexes,
                    last(),
                )
                .unwrap()
            }),
            dateindex_to_unrealized_profit,
            dateindex_to_unrealized_loss,
            height_to_total_unrealized_pnl: compute_dollars.then(|| eager!(Height, Dollars,"total_unrealized_pnl", v0)),
            indexes_to_total_unrealized_pnl: compute_dollars
                .then(|| computed_di!("total_unrealized_pnl", Source::Compute, v0, last())),
            height_to_neg_unrealized_loss: compute_dollars.then(|| eager!(Height, Dollars,"neg_unrealized_loss", v0)),
            indexes_to_neg_unrealized_loss: compute_dollars
                .then(|| computed_di!("neg_unrealized_loss", Source::Compute, v0, last())),
            height_to_net_unrealized_pnl: compute_dollars.then(|| eager!(Height, Dollars,"net_unrealized_pnl", v0)),
            indexes_to_net_unrealized_pnl: compute_dollars
                .then(|| computed_di!("net_unrealized_pnl", Source::Compute, v0, last())),

            // Unrealized rel to market cap
            height_to_unrealized_profit_rel_to_market_cap: compute_dollars
                .then(|| eager!(Height, StoredF32,"unrealized_profit_rel_to_market_cap", v0)),
            height_to_unrealized_loss_rel_to_market_cap: compute_dollars
                .then(|| eager!(Height, StoredF32,"unrealized_loss_rel_to_market_cap", v0)),
            height_to_neg_unrealized_loss_rel_to_market_cap: compute_dollars
                .then(|| eager!(Height, StoredF32,"neg_unrealized_loss_rel_to_market_cap", v0)),
            height_to_net_unrealized_pnl_rel_to_market_cap: compute_dollars
                .then(|| eager!(Height, StoredF32,"net_unrealized_pnl_rel_to_market_cap", v1)),
            indexes_to_unrealized_profit_rel_to_market_cap: compute_dollars
                .then(|| computed_di!("unrealized_profit_rel_to_market_cap", Source::Compute, v1, last())),
            indexes_to_unrealized_loss_rel_to_market_cap: compute_dollars
                .then(|| computed_di!("unrealized_loss_rel_to_market_cap", Source::Compute, v1, last())),
            indexes_to_neg_unrealized_loss_rel_to_market_cap: compute_dollars
                .then(|| computed_di!("neg_unrealized_loss_rel_to_market_cap", Source::Compute, v1, last())),
            indexes_to_net_unrealized_pnl_rel_to_market_cap: compute_dollars
                .then(|| computed_di!("net_unrealized_pnl_rel_to_market_cap", Source::Compute, v1, last())),

            // Unrealized rel to own market cap
            height_to_unrealized_profit_rel_to_own_market_cap: (compute_dollars && extended && compute_rel_to_all)
                .then(|| eager!(Height, StoredF32,"unrealized_profit_rel_to_own_market_cap", v1)),
            height_to_unrealized_loss_rel_to_own_market_cap: (compute_dollars && extended && compute_rel_to_all)
                .then(|| eager!(Height, StoredF32,"unrealized_loss_rel_to_own_market_cap", v1)),
            height_to_neg_unrealized_loss_rel_to_own_market_cap: (compute_dollars && extended && compute_rel_to_all)
                .then(|| eager!(Height, StoredF32,"neg_unrealized_loss_rel_to_own_market_cap", v1)),
            height_to_net_unrealized_pnl_rel_to_own_market_cap: (compute_dollars && extended && compute_rel_to_all)
                .then(|| eager!(Height, StoredF32,"net_unrealized_pnl_rel_to_own_market_cap", v2)),
            indexes_to_unrealized_profit_rel_to_own_market_cap: (compute_dollars && extended && compute_rel_to_all)
                .then(|| computed_di!("unrealized_profit_rel_to_own_market_cap", Source::Compute, v2, last())),
            indexes_to_unrealized_loss_rel_to_own_market_cap: (compute_dollars && extended && compute_rel_to_all)
                .then(|| computed_di!("unrealized_loss_rel_to_own_market_cap", Source::Compute, v2, last())),
            indexes_to_neg_unrealized_loss_rel_to_own_market_cap: (compute_dollars && extended && compute_rel_to_all)
                .then(|| computed_di!("neg_unrealized_loss_rel_to_own_market_cap", Source::Compute, v2, last())),
            indexes_to_net_unrealized_pnl_rel_to_own_market_cap: (compute_dollars && extended && compute_rel_to_all)
                .then(|| computed_di!("net_unrealized_pnl_rel_to_own_market_cap", Source::Compute, v2, last())),

            // Unrealized rel to own total unrealized pnl
            height_to_unrealized_profit_rel_to_own_total_unrealized_pnl: (compute_dollars && extended)
                .then(|| eager!(Height, StoredF32,"unrealized_profit_rel_to_own_total_unrealized_pnl", v0)),
            height_to_unrealized_loss_rel_to_own_total_unrealized_pnl: (compute_dollars && extended)
                .then(|| eager!(Height, StoredF32,"unrealized_loss_rel_to_own_total_unrealized_pnl", v0)),
            height_to_neg_unrealized_loss_rel_to_own_total_unrealized_pnl: (compute_dollars && extended)
                .then(|| eager!(Height, StoredF32,"neg_unrealized_loss_rel_to_own_total_unrealized_pnl", v0)),
            height_to_net_unrealized_pnl_rel_to_own_total_unrealized_pnl: (compute_dollars && extended)
                .then(|| eager!(Height, StoredF32,"net_unrealized_pnl_rel_to_own_total_unrealized_pnl", v1)),
            indexes_to_unrealized_profit_rel_to_own_total_unrealized_pnl: (compute_dollars && extended)
                .then(|| computed_di!("unrealized_profit_rel_to_own_total_unrealized_pnl", Source::Compute, v1, last())),
            indexes_to_unrealized_loss_rel_to_own_total_unrealized_pnl: (compute_dollars && extended)
                .then(|| computed_di!("unrealized_loss_rel_to_own_total_unrealized_pnl", Source::Compute, v1, last())),
            indexes_to_neg_unrealized_loss_rel_to_own_total_unrealized_pnl: (compute_dollars && extended)
                .then(|| computed_di!("neg_unrealized_loss_rel_to_own_total_unrealized_pnl", Source::Compute, v1, last())),
            indexes_to_net_unrealized_pnl_rel_to_own_total_unrealized_pnl: (compute_dollars && extended)
                .then(|| computed_di!("net_unrealized_pnl_rel_to_own_total_unrealized_pnl", Source::Compute, v1, last())),

            // Price paid
            height_to_min_price_paid: compute_dollars.then(|| eager!(Height, Dollars,"min_price_paid", v0)),
            height_to_max_price_paid: compute_dollars.then(|| eager!(Height, Dollars,"max_price_paid", v0)),
            indexes_to_min_price_paid: compute_dollars
                .then(|| computed_h!("min_price_paid", Source::None, v0, last())),
            indexes_to_max_price_paid: compute_dollars
                .then(|| computed_h!("max_price_paid", Source::None, v0, last())),
            price_percentiles: (compute_dollars && extended).then(|| {
                PricePercentiles::forced_import(db, &suffix(""), version + v0, indexes, true).unwrap()
            }),

            // Net realized pnl cumulative deltas
            indexes_to_net_realized_pnl_cumulative_30d_delta: compute_dollars
                .then(|| computed_di!("net_realized_pnl_cumulative_30d_delta", Source::Compute, v3, last())),
            indexes_to_net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap: compute_dollars
                .then(|| computed_di!("net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap", Source::Compute, v3, last())),
            indexes_to_net_realized_pnl_cumulative_30d_delta_rel_to_market_cap: compute_dollars
                .then(|| computed_di!("net_realized_pnl_cumulative_30d_delta_rel_to_market_cap", Source::Compute, v3, last())),
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
                state.realized.um().cap =
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
                .u()
                .inner_version();
            self.height_to_realized_profit
                .um()
                .validate_computed_version_or_reset(
                    base_version + height_to_realized_profit_inner_version,
                )?;
            let height_to_realized_loss_inner_version = self
                .height_to_realized_loss
                .u()
                .inner_version();
            self.height_to_realized_loss
                .um()
                .validate_computed_version_or_reset(
                    base_version + height_to_realized_loss_inner_version,
                )?;
            let height_to_value_created_inner_version = self
                .height_to_value_created
                .u()
                .inner_version();
            self.height_to_value_created
                .um()
                .validate_computed_version_or_reset(
                    base_version + height_to_value_created_inner_version,
                )?;
            let height_to_value_destroyed_inner_version = self
                .height_to_value_destroyed
                .u()
                .inner_version();
            self.height_to_value_destroyed
                .um()
                .validate_computed_version_or_reset(
                    base_version + height_to_value_destroyed_inner_version,
                )?;
            let height_to_supply_in_profit_inner_version = self
                .height_to_supply_in_profit
                .u()
                .inner_version();
            self.height_to_supply_in_profit
                .um()
                .validate_computed_version_or_reset(
                    base_version + height_to_supply_in_profit_inner_version,
                )?;
            let height_to_supply_in_loss_inner_version = self
                .height_to_supply_in_loss
                .u()
                .inner_version();
            self.height_to_supply_in_loss
                .um()
                .validate_computed_version_or_reset(
                    base_version + height_to_supply_in_loss_inner_version,
                )?;
            let height_to_unrealized_profit_inner_version = self
                .height_to_unrealized_profit
                .u()
                .inner_version();
            self.height_to_unrealized_profit
                .um()
                .validate_computed_version_or_reset(
                    base_version + height_to_unrealized_profit_inner_version,
                )?;
            let height_to_unrealized_loss_inner_version = self
                .height_to_unrealized_loss
                .u()
                .inner_version();
            self.height_to_unrealized_loss
                .um()
                .validate_computed_version_or_reset(
                    base_version + height_to_unrealized_loss_inner_version,
                )?;
            let dateindex_to_supply_in_profit_inner_version = self
                .dateindex_to_supply_in_profit
                .u()
                .inner_version();
            self.dateindex_to_supply_in_profit
                .um()
                .validate_computed_version_or_reset(
                    base_version + dateindex_to_supply_in_profit_inner_version,
                )?;
            let dateindex_to_supply_in_loss_inner_version = self
                .dateindex_to_supply_in_loss
                .u()
                .inner_version();
            self.dateindex_to_supply_in_loss
                .um()
                .validate_computed_version_or_reset(
                    base_version + dateindex_to_supply_in_loss_inner_version,
                )?;
            let dateindex_to_unrealized_profit_inner_version = self
                .dateindex_to_unrealized_profit
                .u()
                .inner_version();
            self.dateindex_to_unrealized_profit
                .um()
                .validate_computed_version_or_reset(
                    base_version + dateindex_to_unrealized_profit_inner_version,
                )?;
            let dateindex_to_unrealized_loss_inner_version = self
                .dateindex_to_unrealized_loss
                .u()
                .inner_version();
            self.dateindex_to_unrealized_loss
                .um()
                .validate_computed_version_or_reset(
                    base_version + dateindex_to_unrealized_loss_inner_version,
                )?;
            let height_to_min_price_paid_inner_version = self
                .height_to_min_price_paid
                .u()
                .inner_version();
            self.height_to_min_price_paid
                .um()
                .validate_computed_version_or_reset(
                    base_version + height_to_min_price_paid_inner_version,
                )?;
            let height_to_max_price_paid_inner_version = self
                .height_to_max_price_paid
                .u()
                .inner_version();
            self.height_to_max_price_paid
                .um()
                .validate_computed_version_or_reset(
                    base_version + height_to_max_price_paid_inner_version,
                )?;

            if self.height_to_adjusted_value_created.is_some() {
                let height_to_adjusted_value_created_inner_version = self
                    .height_to_adjusted_value_created
                    .u()
                    .inner_version();
                self.height_to_adjusted_value_created
                    .um()
                    .validate_computed_version_or_reset(
                        base_version + height_to_adjusted_value_created_inner_version,
                    )?;
                let height_to_adjusted_value_destroyed_inner_version = self
                    .height_to_adjusted_value_destroyed
                    .u()
                    .inner_version();
                self.height_to_adjusted_value_destroyed
                    .um()
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
                .um()
                .truncate_push(height, realized.profit)?;
            self.height_to_realized_loss
                .um()
                .truncate_push(height, realized.loss)?;
            self.height_to_value_created
                .um()
                .truncate_push(height, realized.value_created)?;
            self.height_to_value_destroyed
                .um()
                .truncate_push(height, realized.value_destroyed)?;

            if self.height_to_adjusted_value_created.is_some() {
                self.height_to_adjusted_value_created
                    .um()
                    .truncate_push(height, realized.adj_value_created)?;
                self.height_to_adjusted_value_destroyed
                    .um()
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
                .um()
                .truncate_push(
                    height,
                    state
                        .price_to_amount_first_key_value()
                        .map(|(&dollars, _)| dollars)
                        .unwrap_or(Dollars::NAN),
                )?;
            self.height_to_max_price_paid
                .um()
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
                .um()
                .truncate_push(height, height_unrealized_state.supply_in_profit)?;
            self.height_to_supply_in_loss
                .um()
                .truncate_push(height, height_unrealized_state.supply_in_loss)?;
            self.height_to_unrealized_profit
                .um()
                .truncate_push(height, height_unrealized_state.unrealized_profit)?;
            self.height_to_unrealized_loss
                .um()
                .truncate_push(height, height_unrealized_state.unrealized_loss)?;

            if let Some(date_unrealized_state) = date_unrealized_state {
                let dateindex = dateindex.unwrap();

                self.dateindex_to_supply_in_profit
                    .um()
                    .truncate_push(dateindex, date_unrealized_state.supply_in_profit)?;
                self.dateindex_to_supply_in_loss
                    .um()
                    .truncate_push(dateindex, date_unrealized_state.supply_in_loss)?;
                self.dateindex_to_unrealized_profit
                    .um()
                    .truncate_push(dateindex, date_unrealized_state.unrealized_profit)?;
                self.dateindex_to_unrealized_loss
                    .um()
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
                .um()
                .safe_flush(exit)?;
            self.height_to_realized_loss
                .um()
                .safe_flush(exit)?;
            self.height_to_value_created
                .um()
                .safe_flush(exit)?;
            self.height_to_value_destroyed
                .um()
                .safe_flush(exit)?;
            self.height_to_supply_in_profit
                .um()
                .safe_flush(exit)?;
            self.height_to_supply_in_loss
                .um()
                .safe_flush(exit)?;
            self.height_to_unrealized_profit
                .um()
                .safe_flush(exit)?;
            self.height_to_unrealized_loss
                .um()
                .safe_flush(exit)?;
            self.dateindex_to_supply_in_profit
                .um()
                .safe_flush(exit)?;
            self.dateindex_to_supply_in_loss
                .um()
                .safe_flush(exit)?;
            self.dateindex_to_unrealized_profit
                .um()
                .safe_flush(exit)?;
            self.dateindex_to_unrealized_loss
                .um()
                .safe_flush(exit)?;
            self.height_to_min_price_paid
                .um()
                .safe_flush(exit)?;
            self.height_to_max_price_paid
                .um()
                .safe_flush(exit)?;

            if self.height_to_adjusted_value_created.is_some() {
                self.height_to_adjusted_value_created
                    .um()
                    .safe_flush(exit)?;
                self.height_to_adjusted_value_destroyed
                    .um()
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
                    .map(|v| v.height_to_realized_cap.u())
                    .collect::<Vec<_>>()
                    .as_slice(),
                exit,
            )?;

            self.height_to_min_price_paid
                .um()
                .compute_min_of_others(
                    starting_indexes.height,
                    others
                        .iter()
                        .map(|v| v.height_to_min_price_paid.u())
                        .collect::<Vec<_>>()
                        .as_slice(),
                    exit,
                )?;
            self.height_to_max_price_paid
                .um()
                .compute_max_of_others(
                    starting_indexes.height,
                    others
                        .iter()
                        .map(|v| v.height_to_max_price_paid.u())
                        .collect::<Vec<_>>()
                        .as_slice(),
                    exit,
                )?;
            self.height_to_realized_profit
                .um()
                .compute_sum_of_others(
                    starting_indexes.height,
                    others
                        .iter()
                        .map(|v| v.height_to_realized_profit.u())
                        .collect::<Vec<_>>()
                        .as_slice(),
                    exit,
                )?;
            self.height_to_realized_loss
                .um()
                .compute_sum_of_others(
                    starting_indexes.height,
                    others
                        .iter()
                        .map(|v| v.height_to_realized_loss.u())
                        .collect::<Vec<_>>()
                        .as_slice(),
                    exit,
                )?;
            self.height_to_value_created
                .um()
                .compute_sum_of_others(
                    starting_indexes.height,
                    others
                        .iter()
                        .map(|v| v.height_to_value_created.u())
                        .collect::<Vec<_>>()
                        .as_slice(),
                    exit,
                )?;
            self.height_to_value_destroyed
                .um()
                .compute_sum_of_others(
                    starting_indexes.height,
                    others
                        .iter()
                        .map(|v| v.height_to_value_destroyed.u())
                        .collect::<Vec<_>>()
                        .as_slice(),
                    exit,
                )?;
            self.height_to_supply_in_profit
                .um()
                .compute_sum_of_others(
                    starting_indexes.height,
                    others
                        .iter()
                        .map(|v| v.height_to_supply_in_profit.u())
                        .collect::<Vec<_>>()
                        .as_slice(),
                    exit,
                )?;
            self.height_to_supply_in_loss
                .um()
                .compute_sum_of_others(
                    starting_indexes.height,
                    others
                        .iter()
                        .map(|v| v.height_to_supply_in_loss.u())
                        .collect::<Vec<_>>()
                        .as_slice(),
                    exit,
                )?;
            self.height_to_unrealized_profit
                .um()
                .compute_sum_of_others(
                    starting_indexes.height,
                    others
                        .iter()
                        .map(|v| v.height_to_unrealized_profit.u())
                        .collect::<Vec<_>>()
                        .as_slice(),
                    exit,
                )?;
            self.height_to_unrealized_loss
                .um()
                .compute_sum_of_others(
                    starting_indexes.height,
                    others
                        .iter()
                        .map(|v| v.height_to_unrealized_loss.u())
                        .collect::<Vec<_>>()
                        .as_slice(),
                    exit,
                )?;
            self.dateindex_to_supply_in_profit
                .um()
                .compute_sum_of_others(
                    starting_indexes.dateindex,
                    others
                        .iter()
                        .map(|v| v.dateindex_to_supply_in_profit.u())
                        .collect::<Vec<_>>()
                        .as_slice(),
                    exit,
                )?;
            self.dateindex_to_supply_in_loss
                .um()
                .compute_sum_of_others(
                    starting_indexes.dateindex,
                    others
                        .iter()
                        .map(|v| v.dateindex_to_supply_in_loss.u())
                        .collect::<Vec<_>>()
                        .as_slice(),
                    exit,
                )?;
            self.dateindex_to_unrealized_profit
                .um()
                .compute_sum_of_others(
                    starting_indexes.dateindex,
                    others
                        .iter()
                        .map(|v| v.dateindex_to_unrealized_profit.u())
                        .collect::<Vec<_>>()
                        .as_slice(),
                    exit,
                )?;
            self.dateindex_to_unrealized_loss
                .um()
                .compute_sum_of_others(
                    starting_indexes.dateindex,
                    others
                        .iter()
                        .map(|v| v.dateindex_to_unrealized_loss.u())
                        .collect::<Vec<_>>()
                        .as_slice(),
                    exit,
                )?;
            self.height_to_min_price_paid
                .um()
                .compute_min_of_others(
                    starting_indexes.height,
                    others
                        .iter()
                        .map(|v| v.height_to_min_price_paid.u())
                        .collect::<Vec<_>>()
                        .as_slice(),
                    exit,
                )?;
            self.height_to_max_price_paid
                .um()
                .compute_max_of_others(
                    starting_indexes.height,
                    others
                        .iter()
                        .map(|v| v.height_to_max_price_paid.u())
                        .collect::<Vec<_>>()
                        .as_slice(),
                    exit,
                )?;

            if self.height_to_adjusted_value_created.is_some() {
                self.height_to_adjusted_value_created
                    .um()
                    .compute_sum_of_others(
                        starting_indexes.height,
                        others
                            .iter()
                            .map(|v| {
                                v.height_to_adjusted_value_created
                                    .as_ref()
                                    .unwrap_or(v.height_to_value_created.u())
                            })
                            .collect::<Vec<_>>()
                            .as_slice(),
                        exit,
                    )?;
                self.height_to_adjusted_value_destroyed
                    .um()
                    .compute_sum_of_others(
                        starting_indexes.height,
                        others
                            .iter()
                            .map(|v| {
                                v.height_to_adjusted_value_destroyed
                                    .as_ref()
                                    .unwrap_or(v.height_to_value_destroyed.u())
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
                    self.indexes_to_supply.sats.dateindex.u(),
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
                Some(self.height_to_realized_cap.u()),
            )?;

            self.indexes_to_realized_price
                .um()
                .compute_all(indexes, starting_indexes, exit, |vec| {
                    vec.compute_divide(
                        starting_indexes.height,
                        self.height_to_realized_cap.u(),
                        &self.height_to_supply_value.bitcoin,
                        exit,
                    )?;
                    Ok(())
                })?;

            self.indexes_to_realized_price_extra
                .um()
                .compute_rest(
                    price.u(),
                    starting_indexes,
                    exit,
                    Some(
                        self.indexes_to_realized_price
                            .u()
                            .dateindex
                            .unwrap_last(),
                    ),
                )?;

            self.indexes_to_realized_profit
                .um()
                .compute_rest(
                    indexes,
                    starting_indexes,
                    exit,
                    Some(self.height_to_realized_profit.u()),
                )?;

            self.indexes_to_realized_loss
                .um()
                .compute_rest(
                    indexes,
                    starting_indexes,
                    exit,
                    Some(self.height_to_realized_loss.u()),
                )?;

            self.indexes_to_neg_realized_loss
                .um()
                .compute_all(indexes, starting_indexes, exit, |vec| {
                    vec.compute_transform(
                        starting_indexes.height,
                        self.height_to_realized_loss.u(),
                        |(i, v, ..)| (i, v * -1_i64),
                        exit,
                    )?;
                    Ok(())
                })?;

            self.indexes_to_value_created
                .um()
                .compute_rest(
                    indexes,
                    starting_indexes,
                    exit,
                    Some(self.height_to_value_created.u()),
                )?;

            self.indexes_to_value_destroyed
                .um()
                .compute_rest(
                    indexes,
                    starting_indexes,
                    exit,
                    Some(self.height_to_value_destroyed.u()),
                )?;

            self.indexes_to_realized_cap_30d_delta
                .um()
                .compute_all(starting_indexes, exit, |vec| {
                    vec.compute_change(
                        starting_indexes.dateindex,
                        self.indexes_to_realized_cap
                            .u()
                            .dateindex
                            .unwrap_last(),
                        30,
                        exit,
                    )?;
                    Ok(())
                })?;

            self.indexes_to_net_realized_pnl
                .um()
                .compute_all(indexes, starting_indexes, exit, |vec| {
                    vec.compute_subtract(
                        starting_indexes.height,
                        self.height_to_realized_profit.u(),
                        self.height_to_realized_loss.u(),
                        exit,
                    )?;
                    Ok(())
                })?;

            self.indexes_to_realized_value
                .um()
                .compute_all(indexes, starting_indexes, exit, |vec| {
                    vec.compute_add(
                        starting_indexes.height,
                        self.height_to_realized_profit.u(),
                        self.height_to_realized_loss.u(),
                        exit,
                    )?;
                    Ok(())
                })?;

            self.dateindex_to_sopr.um().compute_divide(
                starting_indexes.dateindex,
                self.indexes_to_value_created
                    .u()
                    .dateindex
                    .unwrap_sum(),
                self.indexes_to_value_destroyed
                    .u()
                    .dateindex
                    .unwrap_sum(),
                exit,
            )?;

            self.dateindex_to_sopr_7d_ema
                .um()
                .compute_ema(
                    starting_indexes.dateindex,
                    self.dateindex_to_sopr.u(),
                    7,
                    exit,
                )?;

            self.dateindex_to_sopr_30d_ema
                .um()
                .compute_ema(
                    starting_indexes.dateindex,
                    self.dateindex_to_sopr.u(),
                    30,
                    exit,
                )?;

            self.dateindex_to_sell_side_risk_ratio
                .um()
                .compute_percentage(
                    starting_indexes.dateindex,
                    self.indexes_to_realized_value
                        .u()
                        .dateindex
                        .unwrap_sum(),
                    self.indexes_to_realized_cap
                        .u()
                        .dateindex
                        .unwrap_last(),
                    exit,
                )?;

            self.dateindex_to_sell_side_risk_ratio_7d_ema
                .um()
                .compute_ema(
                    starting_indexes.dateindex,
                    self.dateindex_to_sell_side_risk_ratio.u(),
                    7,
                    exit,
                )?;

            self.dateindex_to_sell_side_risk_ratio_30d_ema
                .um()
                .compute_ema(
                    starting_indexes.dateindex,
                    self.dateindex_to_sell_side_risk_ratio.u(),
                    30,
                    exit,
                )?;

            self.indexes_to_supply_in_profit
                .um()
                .compute_rest(
                    price,
                    starting_indexes,
                    exit,
                    Some(self.dateindex_to_supply_in_profit.u()),
                )?;
            self.indexes_to_supply_in_loss
                .um()
                .compute_rest(
                    price,
                    starting_indexes,
                    exit,
                    Some(self.dateindex_to_supply_in_loss.u()),
                )?;
            self.indexes_to_unrealized_profit
                .um()
                .compute_rest(
                    starting_indexes,
                    exit,
                    Some(self.dateindex_to_unrealized_profit.u()),
                )?;
            self.indexes_to_unrealized_loss
                .um()
                .compute_rest(
                    starting_indexes,
                    exit,
                    Some(self.dateindex_to_unrealized_loss.u()),
                )?;
            self.height_to_total_unrealized_pnl
                .um()
                .compute_add(
                    starting_indexes.height,
                    self.height_to_unrealized_profit.u(),
                    self.height_to_unrealized_loss.u(),
                    exit,
                )?;
            self.indexes_to_total_unrealized_pnl
                .um()
                .compute_all(starting_indexes, exit, |vec| {
                    vec.compute_add(
                        starting_indexes.dateindex,
                        self.dateindex_to_unrealized_profit.u(),
                        self.dateindex_to_unrealized_loss.u(),
                        exit,
                    )?;
                    Ok(())
                })?;
            self.height_to_total_realized_pnl
                .um()
                .compute_add(
                    starting_indexes.height,
                    self.height_to_realized_profit.u(),
                    self.height_to_realized_loss.u(),
                    exit,
                )?;
            self.indexes_to_total_realized_pnl
                .um()
                .compute_all(starting_indexes, exit, |vec| {
                    vec.compute_add(
                        starting_indexes.dateindex,
                        self.indexes_to_realized_profit
                            .u()
                            .dateindex
                            .unwrap_sum(),
                        self.indexes_to_realized_loss
                            .u()
                            .dateindex
                            .unwrap_sum(),
                        exit,
                    )?;
                    Ok(())
                })?;

            self.indexes_to_min_price_paid
                .um()
                .compute_rest(
                    indexes,
                    starting_indexes,
                    exit,
                    Some(self.height_to_min_price_paid.u()),
                )?;
            self.indexes_to_max_price_paid
                .um()
                .compute_rest(
                    indexes,
                    starting_indexes,
                    exit,
                    Some(self.height_to_max_price_paid.u()),
                )?;

            self.height_to_neg_unrealized_loss
                .um()
                .compute_transform(
                    starting_indexes.height,
                    self.height_to_unrealized_loss.u(),
                    |(h, v, ..)| (h, v * -1_i64),
                    exit,
                )?;
            self.indexes_to_neg_unrealized_loss
                .um()
                .compute_all(starting_indexes, exit, |v| {
                    v.compute_transform(
                        starting_indexes.dateindex,
                        self.dateindex_to_unrealized_loss.u(),
                        |(h, v, ..)| (h, v * -1_i64),
                        exit,
                    )?;
                    Ok(())
                })?;
            self.height_to_net_unrealized_pnl
                .um()
                .compute_subtract(
                    starting_indexes.height,
                    self.height_to_unrealized_profit.u(),
                    self.height_to_unrealized_loss.u(),
                    exit,
                )?;

            self.indexes_to_net_unrealized_pnl
                .um()
                .compute_all(starting_indexes, exit, |vec| {
                    vec.compute_subtract(
                        starting_indexes.dateindex,
                        self.dateindex_to_unrealized_profit.u(),
                        self.dateindex_to_unrealized_loss.u(),
                        exit,
                    )?;
                    Ok(())
                })?;
            self.height_to_unrealized_profit_rel_to_market_cap
                .um()
                .compute_percentage(
                    starting_indexes.height,
                    self.height_to_unrealized_profit.u(),
                    height_to_market_cap,
                    exit,
                )?;
            self.height_to_unrealized_loss_rel_to_market_cap
                .um()
                .compute_percentage(
                    starting_indexes.height,
                    self.height_to_unrealized_loss.u(),
                    height_to_market_cap,
                    exit,
                )?;
            self.height_to_neg_unrealized_loss_rel_to_market_cap
                .um()
                .compute_percentage(
                    starting_indexes.height,
                    self.height_to_neg_unrealized_loss.u(),
                    height_to_market_cap,
                    exit,
                )?;
            self.height_to_net_unrealized_pnl_rel_to_market_cap
                .um()
                .compute_percentage(
                    starting_indexes.height,
                    self.height_to_net_unrealized_pnl.u(),
                    height_to_market_cap,
                    exit,
                )?;
            self.indexes_to_unrealized_profit_rel_to_market_cap
                .um()
                .compute_all(starting_indexes, exit, |vec| {
                    vec.compute_percentage(
                        starting_indexes.dateindex,
                        self.dateindex_to_unrealized_profit.u(),
                        dateindex_to_market_cap,
                        exit,
                    )?;
                    Ok(())
                })?;
            self.indexes_to_unrealized_loss_rel_to_market_cap
                .um()
                .compute_all(starting_indexes, exit, |vec| {
                    vec.compute_percentage(
                        starting_indexes.dateindex,
                        self.dateindex_to_unrealized_loss.u(),
                        dateindex_to_market_cap,
                        exit,
                    )?;
                    Ok(())
                })?;
            self.indexes_to_neg_unrealized_loss_rel_to_market_cap
                .um()
                .compute_all(starting_indexes, exit, |vec| {
                    vec.compute_percentage(
                        starting_indexes.dateindex,
                        self.indexes_to_neg_unrealized_loss
                            .u()
                            .dateindex
                            .u(),
                        dateindex_to_market_cap,
                        exit,
                    )?;
                    Ok(())
                })?;
            self.indexes_to_net_unrealized_pnl_rel_to_market_cap
                .um()
                .compute_all(starting_indexes, exit, |vec| {
                    vec.compute_percentage(
                        starting_indexes.dateindex,
                        self.indexes_to_net_unrealized_pnl
                            .u()
                            .dateindex
                            .u(),
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
                    .um()
                    .compute_percentage(
                        starting_indexes.height,
                        self.height_to_unrealized_profit.u(),
                        self.height_to_supply_value.dollars.u(),
                        exit,
                    )?;
                self.height_to_unrealized_loss_rel_to_own_market_cap
                    .um()
                    .compute_percentage(
                        starting_indexes.height,
                        self.height_to_unrealized_loss.u(),
                        self.height_to_supply_value.dollars.u(),
                        exit,
                    )?;
                self.height_to_neg_unrealized_loss_rel_to_own_market_cap
                    .um()
                    .compute_percentage(
                        starting_indexes.height,
                        self.height_to_neg_unrealized_loss.u(),
                        self.height_to_supply_value.dollars.u(),
                        exit,
                    )?;
                self.height_to_net_unrealized_pnl_rel_to_own_market_cap
                    .um()
                    .compute_percentage(
                        starting_indexes.height,
                        self.height_to_net_unrealized_pnl.u(),
                        self.height_to_supply_value.dollars.u(),
                        exit,
                    )?;
                self.indexes_to_unrealized_profit_rel_to_own_market_cap
                    .um()
                    .compute_all(starting_indexes, exit, |vec| {
                        vec.compute_percentage(
                            starting_indexes.dateindex,
                            self.dateindex_to_unrealized_profit.u(),
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
                    .um()
                    .compute_all(starting_indexes, exit, |vec| {
                        vec.compute_percentage(
                            starting_indexes.dateindex,
                            self.dateindex_to_unrealized_loss.u(),
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
                    .um()
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
                    .um()
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
                    .um()
                    .compute_percentage(
                        starting_indexes.height,
                        self.height_to_unrealized_profit.u(),
                        self.height_to_total_unrealized_pnl.u(),
                        exit,
                    )?;
                self.height_to_unrealized_loss_rel_to_own_total_unrealized_pnl
                    .um()
                    .compute_percentage(
                        starting_indexes.height,
                        self.height_to_unrealized_loss.u(),
                        self.height_to_total_unrealized_pnl.u(),
                        exit,
                    )?;
                self.height_to_neg_unrealized_loss_rel_to_own_total_unrealized_pnl
                    .um()
                    .compute_percentage(
                        starting_indexes.height,
                        self.height_to_neg_unrealized_loss.u(),
                        self.height_to_total_unrealized_pnl.u(),
                        exit,
                    )?;
                self.height_to_net_unrealized_pnl_rel_to_own_total_unrealized_pnl
                    .um()
                    .compute_percentage(
                        starting_indexes.height,
                        self.height_to_net_unrealized_pnl.u(),
                        self.height_to_total_unrealized_pnl.u(),
                        exit,
                    )?;
                self.indexes_to_unrealized_profit_rel_to_own_total_unrealized_pnl
                    .um()
                    .compute_all(starting_indexes, exit, |vec| {
                        vec.compute_percentage(
                            starting_indexes.dateindex,
                            self.dateindex_to_unrealized_profit.u(),
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
                    .um()
                    .compute_all(starting_indexes, exit, |vec| {
                        vec.compute_percentage(
                            starting_indexes.dateindex,
                            self.dateindex_to_unrealized_loss.u(),
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
                    .um()
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
                    .um()
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
                .um()
                .compute_all(indexes, starting_indexes, exit, |vec| {
                    vec.compute_percentage(
                        starting_indexes.height,
                        self.height_to_realized_profit.u(),
                        *height_to_realized_cap.u(),
                        exit,
                    )?;
                    Ok(())
                })?;

            self.indexes_to_realized_loss_rel_to_realized_cap
                .um()
                .compute_all(indexes, starting_indexes, exit, |vec| {
                    vec.compute_percentage(
                        starting_indexes.height,
                        self.height_to_realized_loss.u(),
                        *height_to_realized_cap.u(),
                        exit,
                    )?;
                    Ok(())
                })?;

            self.indexes_to_net_realized_pnl_rel_to_realized_cap
                .um()
                .compute_all(indexes, starting_indexes, exit, |vec| {
                    vec.compute_percentage(
                        starting_indexes.height,
                        self.indexes_to_net_realized_pnl
                            .u()
                            .height
                            .u(),
                        *height_to_realized_cap.u(),
                        exit,
                    )?;
                    Ok(())
                })?;

            self.height_to_supply_in_loss_value
                .um()
                .compute_rest(
                    price,
                    starting_indexes,
                    exit,
                    Some(self.height_to_supply_in_loss.u()),
                )?;
            self.height_to_supply_in_profit_value
                .um()
                .compute_rest(
                    price,
                    starting_indexes,
                    exit,
                    Some(self.height_to_supply_in_profit.u()),
                )?;
            self.height_to_supply_in_loss_rel_to_own_supply
                .um()
                .compute_percentage(
                    starting_indexes.height,
                    &self
                        .height_to_supply_in_loss_value
                        .u()
                        .bitcoin,
                    &self.height_to_supply_value.bitcoin,
                    exit,
                )?;
            self.height_to_supply_in_profit_rel_to_own_supply
                .um()
                .compute_percentage(
                    starting_indexes.height,
                    &self
                        .height_to_supply_in_profit_value
                        .u()
                        .bitcoin,
                    &self.height_to_supply_value.bitcoin,
                    exit,
                )?;
            self.indexes_to_supply_in_loss_rel_to_own_supply
                .um()
                .compute_all(starting_indexes, exit, |v| {
                    v.compute_percentage(
                        starting_indexes.dateindex,
                        self.indexes_to_supply_in_loss
                            .u()
                            .bitcoin
                            .dateindex
                            .u(),
                        self.indexes_to_supply.bitcoin.dateindex.u(),
                        exit,
                    )?;
                    Ok(())
                })?;
            self.indexes_to_supply_in_profit_rel_to_own_supply
                .um()
                .compute_all(starting_indexes, exit, |v| {
                    v.compute_percentage(
                        starting_indexes.dateindex,
                        self.indexes_to_supply_in_profit
                            .u()
                            .bitcoin
                            .dateindex
                            .u(),
                        self.indexes_to_supply.bitcoin.dateindex.u(),
                        exit,
                    )?;
                    Ok(())
                })?;

            self.indexes_to_net_realized_pnl_cumulative_30d_delta
                .um()
                .compute_all(starting_indexes, exit, |v| {
                    v.compute_change(
                        starting_indexes.dateindex,
                        self.indexes_to_net_realized_pnl
                            .u()
                            .dateindex
                            .unwrap_cumulative(),
                        30,
                        exit,
                    )?;
                    Ok(())
                })?;

            self.indexes_to_net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap
                .um()
                .compute_all(starting_indexes, exit, |v| {
                    v.compute_percentage(
                        starting_indexes.dateindex,
                        self.indexes_to_net_realized_pnl_cumulative_30d_delta
                            .u()
                            .dateindex
                            .u(),
                        *dateindex_to_realized_cap.u(),
                        exit,
                    )?;
                    Ok(())
                })?;

            self.indexes_to_net_realized_pnl_cumulative_30d_delta_rel_to_market_cap
                .um()
                .compute_all(starting_indexes, exit, |v| {
                    v.compute_percentage(
                        starting_indexes.dateindex,
                        self.indexes_to_net_realized_pnl_cumulative_30d_delta
                            .u()
                            .dateindex
                            .u(),
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
                    .um()
                    .compute_percentage(
                        starting_indexes.height,
                        &self
                            .height_to_supply_in_loss_value
                            .u()
                            .bitcoin,
                        height_to_supply,
                        exit,
                    )?;
                self.height_to_supply_in_profit_rel_to_circulating_supply
                    .um()
                    .compute_percentage(
                        starting_indexes.height,
                        &self
                            .height_to_supply_in_profit_value
                            .u()
                            .bitcoin,
                        height_to_supply,
                        exit,
                    )?;
                self.indexes_to_supply_in_loss_rel_to_circulating_supply
                    .um()
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
                    .um()
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
                    .um()
                    .compute_rest(
                        indexes,
                        starting_indexes,
                        exit,
                        Some(self.height_to_adjusted_value_created.u()),
                    )?;

                self.indexes_to_adjusted_value_destroyed
                    .um()
                    .compute_rest(
                        indexes,
                        starting_indexes,
                        exit,
                        Some(self.height_to_adjusted_value_destroyed.u()),
                    )?;

                self.dateindex_to_adjusted_sopr
                    .um()
                    .compute_divide(
                        starting_indexes.dateindex,
                        self.indexes_to_adjusted_value_created
                            .u()
                            .dateindex
                            .unwrap_sum(),
                        self.indexes_to_adjusted_value_destroyed
                            .u()
                            .dateindex
                            .unwrap_sum(),
                        exit,
                    )?;

                self.dateindex_to_adjusted_sopr_7d_ema
                    .um()
                    .compute_ema(
                        starting_indexes.dateindex,
                        self.dateindex_to_adjusted_sopr.u(),
                        7,
                        exit,
                    )?;

                self.dateindex_to_adjusted_sopr_30d_ema
                    .um()
                    .compute_ema(
                        starting_indexes.dateindex,
                        self.dateindex_to_adjusted_sopr.u(),
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
                            self.height_to_realized_cap.u(),
                            self.height_to_supply_value.dollars.u(),
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
                    .u()
                    .dateindex
                    .unwrap_sum(),
                self.indexes_to_realized_loss
                    .u()
                    .dateindex
                    .unwrap_sum(),
                exit,
            )?;
        }

        Ok(())
    }
}
