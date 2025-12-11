use brk_grouper::Filter;
use brk_traversable::Traversable;
use brk_types::{DateIndex, Dollars, Height, Sats, StoredF32, StoredF64, StoredU64};
use vecdb::{EagerVec, PcoVec};

use crate::grouped::{
    ComputedHeightValueVecs, ComputedRatioVecsFromDateIndex, ComputedValueVecsFromDateIndex,
    ComputedValueVecsFromHeight, ComputedVecsFromDateIndex, ComputedVecsFromHeight,
    PricePercentiles,
};

/// Common vectors shared between UTXO and Address cohorts.
///
/// This struct contains all the computed vectors for a single cohort. The fields are
/// organized into logical groups matching the initialization order in `forced_import`.
///
/// ## Field Groups
/// - **Supply & UTXO count**: Basic supply metrics (always computed)
/// - **Activity**: Sent amounts, satblocks/satdays destroyed
/// - **Realized**: Realized cap, profit/loss, value created/destroyed, SOPR
/// - **Unrealized**: Unrealized profit/loss, supply in profit/loss
/// - **Price**: Min/max price paid, price percentiles
/// - **Relative metrics**: Ratios relative to market cap, realized cap, etc.
#[derive(Clone, Traversable)]
pub struct Vecs {
    #[traversable(skip)]
    pub filter: Filter,

    // ==================== SUPPLY & UTXO COUNT ====================
    // Always computed - core supply metrics
    pub height_to_supply: EagerVec<PcoVec<Height, Sats>>,
    pub height_to_supply_value: ComputedHeightValueVecs,
    pub indexes_to_supply: ComputedValueVecsFromDateIndex,
    pub height_to_utxo_count: EagerVec<PcoVec<Height, StoredU64>>,
    pub indexes_to_utxo_count: ComputedVecsFromHeight<StoredU64>,
    pub height_to_supply_half_value: ComputedHeightValueVecs,
    pub indexes_to_supply_half: ComputedValueVecsFromDateIndex,

    // ==================== ACTIVITY ====================
    // Always computed - transaction activity metrics
    pub height_to_sent: EagerVec<PcoVec<Height, Sats>>,
    pub indexes_to_sent: ComputedValueVecsFromHeight,
    pub height_to_satblocks_destroyed: EagerVec<PcoVec<Height, Sats>>,
    pub height_to_satdays_destroyed: EagerVec<PcoVec<Height, Sats>>,
    pub indexes_to_coinblocks_destroyed: ComputedVecsFromHeight<StoredF64>,
    pub indexes_to_coindays_destroyed: ComputedVecsFromHeight<StoredF64>,

    // ==================== REALIZED CAP & PRICE ====================
    // Conditional on compute_dollars
    pub height_to_realized_cap: Option<EagerVec<PcoVec<Height, Dollars>>>,
    pub indexes_to_realized_cap: Option<ComputedVecsFromHeight<Dollars>>,
    pub indexes_to_realized_price: Option<ComputedVecsFromHeight<Dollars>>,
    pub indexes_to_realized_price_extra: Option<ComputedRatioVecsFromDateIndex>,
    pub indexes_to_realized_cap_rel_to_own_market_cap: Option<ComputedVecsFromHeight<StoredF32>>,
    pub indexes_to_realized_cap_30d_delta: Option<ComputedVecsFromDateIndex<Dollars>>,

    // ==================== REALIZED PROFIT & LOSS ====================
    // Conditional on compute_dollars
    pub height_to_realized_profit: Option<EagerVec<PcoVec<Height, Dollars>>>,
    pub indexes_to_realized_profit: Option<ComputedVecsFromHeight<Dollars>>,
    pub height_to_realized_loss: Option<EagerVec<PcoVec<Height, Dollars>>>,
    pub indexes_to_realized_loss: Option<ComputedVecsFromHeight<Dollars>>,
    pub indexes_to_neg_realized_loss: Option<ComputedVecsFromHeight<Dollars>>,
    pub indexes_to_net_realized_pnl: Option<ComputedVecsFromHeight<Dollars>>,
    pub indexes_to_realized_value: Option<ComputedVecsFromHeight<Dollars>>,
    pub indexes_to_realized_profit_rel_to_realized_cap: Option<ComputedVecsFromHeight<StoredF32>>,
    pub indexes_to_realized_loss_rel_to_realized_cap: Option<ComputedVecsFromHeight<StoredF32>>,
    pub indexes_to_net_realized_pnl_rel_to_realized_cap: Option<ComputedVecsFromHeight<StoredF32>>,
    pub height_to_total_realized_pnl: Option<EagerVec<PcoVec<Height, Dollars>>>,
    pub indexes_to_total_realized_pnl: Option<ComputedVecsFromDateIndex<Dollars>>,
    pub dateindex_to_realized_profit_to_loss_ratio: Option<EagerVec<PcoVec<DateIndex, StoredF64>>>,

    // ==================== VALUE CREATED & DESTROYED ====================
    // Conditional on compute_dollars
    pub height_to_value_created: Option<EagerVec<PcoVec<Height, Dollars>>>,
    pub indexes_to_value_created: Option<ComputedVecsFromHeight<Dollars>>,
    pub height_to_value_destroyed: Option<EagerVec<PcoVec<Height, Dollars>>>,
    pub indexes_to_value_destroyed: Option<ComputedVecsFromHeight<Dollars>>,
    pub height_to_adjusted_value_created: Option<EagerVec<PcoVec<Height, Dollars>>>,
    pub indexes_to_adjusted_value_created: Option<ComputedVecsFromHeight<Dollars>>,
    pub height_to_adjusted_value_destroyed: Option<EagerVec<PcoVec<Height, Dollars>>>,
    pub indexes_to_adjusted_value_destroyed: Option<ComputedVecsFromHeight<Dollars>>,

    // ==================== SOPR ====================
    // Spent Output Profit Ratio - conditional on compute_dollars
    pub dateindex_to_sopr: Option<EagerVec<PcoVec<DateIndex, StoredF64>>>,
    pub dateindex_to_sopr_7d_ema: Option<EagerVec<PcoVec<DateIndex, StoredF64>>>,
    pub dateindex_to_sopr_30d_ema: Option<EagerVec<PcoVec<DateIndex, StoredF64>>>,
    pub dateindex_to_adjusted_sopr: Option<EagerVec<PcoVec<DateIndex, StoredF64>>>,
    pub dateindex_to_adjusted_sopr_7d_ema: Option<EagerVec<PcoVec<DateIndex, StoredF64>>>,
    pub dateindex_to_adjusted_sopr_30d_ema: Option<EagerVec<PcoVec<DateIndex, StoredF64>>>,

    // ==================== SELL SIDE RISK ====================
    // Conditional on compute_dollars
    pub dateindex_to_sell_side_risk_ratio: Option<EagerVec<PcoVec<DateIndex, StoredF32>>>,
    pub dateindex_to_sell_side_risk_ratio_7d_ema: Option<EagerVec<PcoVec<DateIndex, StoredF32>>>,
    pub dateindex_to_sell_side_risk_ratio_30d_ema: Option<EagerVec<PcoVec<DateIndex, StoredF32>>>,

    // ==================== SUPPLY IN PROFIT/LOSS ====================
    // Conditional on compute_dollars
    pub height_to_supply_in_profit: Option<EagerVec<PcoVec<Height, Sats>>>,
    pub indexes_to_supply_in_profit: Option<ComputedValueVecsFromDateIndex>,
    pub height_to_supply_in_loss: Option<EagerVec<PcoVec<Height, Sats>>>,
    pub indexes_to_supply_in_loss: Option<ComputedValueVecsFromDateIndex>,
    pub dateindex_to_supply_in_profit: Option<EagerVec<PcoVec<DateIndex, Sats>>>,
    pub dateindex_to_supply_in_loss: Option<EagerVec<PcoVec<DateIndex, Sats>>>,
    pub height_to_supply_in_profit_value: Option<ComputedHeightValueVecs>,
    pub height_to_supply_in_loss_value: Option<ComputedHeightValueVecs>,

    // ==================== UNREALIZED PROFIT & LOSS ====================
    // Conditional on compute_dollars
    pub height_to_unrealized_profit: Option<EagerVec<PcoVec<Height, Dollars>>>,
    pub indexes_to_unrealized_profit: Option<ComputedVecsFromDateIndex<Dollars>>,
    pub height_to_unrealized_loss: Option<EagerVec<PcoVec<Height, Dollars>>>,
    pub indexes_to_unrealized_loss: Option<ComputedVecsFromDateIndex<Dollars>>,
    pub dateindex_to_unrealized_profit: Option<EagerVec<PcoVec<DateIndex, Dollars>>>,
    pub dateindex_to_unrealized_loss: Option<EagerVec<PcoVec<DateIndex, Dollars>>>,
    pub height_to_neg_unrealized_loss: Option<EagerVec<PcoVec<Height, Dollars>>>,
    pub indexes_to_neg_unrealized_loss: Option<ComputedVecsFromDateIndex<Dollars>>,
    pub height_to_net_unrealized_pnl: Option<EagerVec<PcoVec<Height, Dollars>>>,
    pub indexes_to_net_unrealized_pnl: Option<ComputedVecsFromDateIndex<Dollars>>,
    pub height_to_total_unrealized_pnl: Option<EagerVec<PcoVec<Height, Dollars>>>,
    pub indexes_to_total_unrealized_pnl: Option<ComputedVecsFromDateIndex<Dollars>>,

    // ==================== PRICE PAID ====================
    // Conditional on compute_dollars
    pub height_to_min_price_paid: Option<EagerVec<PcoVec<Height, Dollars>>>,
    pub indexes_to_min_price_paid: Option<ComputedVecsFromHeight<Dollars>>,
    pub height_to_max_price_paid: Option<EagerVec<PcoVec<Height, Dollars>>>,
    pub indexes_to_max_price_paid: Option<ComputedVecsFromHeight<Dollars>>,
    pub price_percentiles: Option<PricePercentiles>,

    // ==================== RELATIVE METRICS: UNREALIZED vs MARKET CAP ====================
    // Conditional on compute_dollars
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

    // ==================== RELATIVE METRICS: UNREALIZED vs OWN MARKET CAP ====================
    // Conditional on compute_dollars && extended && compute_rel_to_all
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

    // ==================== RELATIVE METRICS: UNREALIZED vs OWN TOTAL UNREALIZED ====================
    // Conditional on compute_dollars && extended
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

    // ==================== RELATIVE METRICS: SUPPLY vs CIRCULATING/OWN ====================
    // Conditional on compute_dollars
    pub indexes_to_supply_rel_to_circulating_supply: Option<ComputedVecsFromHeight<StoredF64>>,
    pub height_to_supply_in_profit_rel_to_own_supply: Option<EagerVec<PcoVec<Height, StoredF64>>>,
    pub height_to_supply_in_loss_rel_to_own_supply: Option<EagerVec<PcoVec<Height, StoredF64>>>,
    pub indexes_to_supply_in_profit_rel_to_own_supply: Option<ComputedVecsFromDateIndex<StoredF64>>,
    pub indexes_to_supply_in_loss_rel_to_own_supply: Option<ComputedVecsFromDateIndex<StoredF64>>,
    pub height_to_supply_in_profit_rel_to_circulating_supply:
        Option<EagerVec<PcoVec<Height, StoredF64>>>,
    pub height_to_supply_in_loss_rel_to_circulating_supply:
        Option<EagerVec<PcoVec<Height, StoredF64>>>,
    pub indexes_to_supply_in_profit_rel_to_circulating_supply:
        Option<ComputedVecsFromDateIndex<StoredF64>>,
    pub indexes_to_supply_in_loss_rel_to_circulating_supply:
        Option<ComputedVecsFromDateIndex<StoredF64>>,

    // ==================== NET REALIZED PNL DELTAS ====================
    // Conditional on compute_dollars
    pub indexes_to_net_realized_pnl_cumulative_30d_delta:
        Option<ComputedVecsFromDateIndex<Dollars>>,
    pub indexes_to_net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap:
        Option<ComputedVecsFromDateIndex<StoredF32>>,
    pub indexes_to_net_realized_pnl_cumulative_30d_delta_rel_to_market_cap:
        Option<ComputedVecsFromDateIndex<StoredF32>>,
}
