//! Import and validation methods for Vecs.
//!
//! This module contains methods for:
//! - `forced_import`: Creating a new Vecs instance from database
//! - `import_state`: Importing state when resuming from checkpoint
//! - `validate_computed_versions`: Version validation
//! - `min_height_vecs_len`: Finding minimum vector length

use brk_error::{Error, Result};
use brk_grouper::{CohortContext, Filter};
use brk_types::{DateIndex, Dollars, Height, Sats, StoredF32, StoredF64, Version};
use vecdb::{
    AnyVec, Database, EagerVec, GenericStoredVec, ImportableVec, IterableCloneableVec, PcoVec,
    StoredVec, TypedVecIterator,
};

use crate::{
    grouped::{
        ComputedHeightValueVecs, ComputedRatioVecsFromDateIndex, ComputedValueVecsFromDateIndex,
        ComputedValueVecsFromHeight, ComputedVecsFromDateIndex, ComputedVecsFromHeight,
        PricePercentiles, Source, VecBuilderOptions,
    },
    indexes, price,
    states::CohortState,
    utils::OptionExt,
};

use super::Vecs;

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
                EagerVec::<PcoVec<$idx, $val>>::forced_import(db, &suffix($name), version + $v)
                    .unwrap()
            };
        }
        macro_rules! computed_h {
            ($name:expr, $source:expr, $v:expr, $opts:expr $(,)?) => {
                ComputedVecsFromHeight::forced_import(
                    db,
                    &suffix($name),
                    $source,
                    version + $v,
                    indexes,
                    $opts,
                )
                .unwrap()
            };
        }
        macro_rules! computed_di {
            ($name:expr, $source:expr, $v:expr, $opts:expr $(,)?) => {
                ComputedVecsFromDateIndex::forced_import(
                    db,
                    &suffix($name),
                    $source,
                    version + $v,
                    indexes,
                    $opts,
                )
                .unwrap()
            };
        }

        // Common version patterns
        let v0 = Version::ZERO;
        let v1 = Version::ONE;
        let v2 = Version::TWO;
        let v3 = Version::new(3);
        let last = || VecBuilderOptions::default().add_last();
        let sum = || VecBuilderOptions::default().add_sum();
        let sum_cum = || VecBuilderOptions::default().add_sum().add_cumulative();

        // Pre-create dateindex vecs that are used in computed vecs
        let dateindex_to_supply_in_profit =
            compute_dollars.then(|| eager!(DateIndex, Sats, "supply_in_profit", v0));
        let dateindex_to_supply_in_loss =
            compute_dollars.then(|| eager!(DateIndex, Sats, "supply_in_loss", v0));
        let dateindex_to_unrealized_profit =
            compute_dollars.then(|| eager!(DateIndex, Dollars, "unrealized_profit", v0));
        let dateindex_to_unrealized_loss =
            compute_dollars.then(|| eager!(DateIndex, Dollars, "unrealized_loss", v0));

        Ok(Self {
            filter,

            // ==================== SUPPLY & UTXO COUNT ====================
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

            // ==================== ACTIVITY ====================
            height_to_sent: EagerVec::forced_import(db, &suffix("sent"), version + v0)?,
            indexes_to_sent: ComputedValueVecsFromHeight::forced_import(
                db,
                &suffix("sent"),
                Source::None,
                version + v0,
                sum(),
                compute_dollars,
                indexes,
            )?,
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

            // ==================== REALIZED CAP & PRICE ====================
            height_to_realized_cap: compute_dollars
                .then(|| eager!(Height, Dollars, "realized_cap", v0)),
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
            indexes_to_realized_cap_rel_to_own_market_cap: (compute_dollars && extended).then(
                || {
                    computed_h!(
                        "realized_cap_rel_to_own_market_cap",
                        Source::Compute,
                        v0,
                        last()
                    )
                },
            ),
            indexes_to_realized_cap_30d_delta: compute_dollars
                .then(|| computed_di!("realized_cap_30d_delta", Source::Compute, v0, last())),

            // ==================== REALIZED PROFIT & LOSS ====================
            height_to_realized_profit: compute_dollars
                .then(|| eager!(Height, Dollars, "realized_profit", v0)),
            indexes_to_realized_profit: compute_dollars
                .then(|| computed_h!("realized_profit", Source::None, v0, sum_cum())),
            height_to_realized_loss: compute_dollars
                .then(|| eager!(Height, Dollars, "realized_loss", v0)),
            indexes_to_realized_loss: compute_dollars
                .then(|| computed_h!("realized_loss", Source::None, v0, sum_cum())),
            indexes_to_neg_realized_loss: compute_dollars
                .then(|| computed_h!("neg_realized_loss", Source::Compute, v1, sum_cum())),
            indexes_to_net_realized_pnl: compute_dollars
                .then(|| computed_h!("net_realized_pnl", Source::Compute, v0, sum_cum())),
            indexes_to_realized_value: compute_dollars
                .then(|| computed_h!("realized_value", Source::Compute, v0, sum())),
            indexes_to_realized_profit_rel_to_realized_cap: compute_dollars.then(|| {
                computed_h!(
                    "realized_profit_rel_to_realized_cap",
                    Source::Compute,
                    v0,
                    sum()
                )
            }),
            indexes_to_realized_loss_rel_to_realized_cap: compute_dollars.then(|| {
                computed_h!(
                    "realized_loss_rel_to_realized_cap",
                    Source::Compute,
                    v0,
                    sum()
                )
            }),
            indexes_to_net_realized_pnl_rel_to_realized_cap: compute_dollars.then(|| {
                computed_h!(
                    "net_realized_pnl_rel_to_realized_cap",
                    Source::Compute,
                    v1,
                    sum()
                )
            }),
            height_to_total_realized_pnl: compute_dollars
                .then(|| eager!(Height, Dollars, "total_realized_pnl", v0)),
            indexes_to_total_realized_pnl: compute_dollars
                .then(|| computed_di!("total_realized_pnl", Source::Compute, v1, sum())),
            dateindex_to_realized_profit_to_loss_ratio: (compute_dollars && extended)
                .then(|| eager!(DateIndex, StoredF64, "realized_profit_to_loss_ratio", v1)),

            // ==================== VALUE CREATED & DESTROYED ====================
            height_to_value_created: compute_dollars
                .then(|| eager!(Height, Dollars, "value_created", v0)),
            indexes_to_value_created: compute_dollars
                .then(|| computed_h!("value_created", Source::None, v0, sum())),
            height_to_value_destroyed: compute_dollars
                .then(|| eager!(Height, Dollars, "value_destroyed", v0)),
            indexes_to_value_destroyed: compute_dollars
                .then(|| computed_h!("value_destroyed", Source::None, v0, sum())),
            height_to_adjusted_value_created: (compute_dollars && compute_adjusted)
                .then(|| eager!(Height, Dollars, "adjusted_value_created", v0)),
            indexes_to_adjusted_value_created: (compute_dollars && compute_adjusted)
                .then(|| computed_h!("adjusted_value_created", Source::None, v0, sum())),
            height_to_adjusted_value_destroyed: (compute_dollars && compute_adjusted)
                .then(|| eager!(Height, Dollars, "adjusted_value_destroyed", v0)),
            indexes_to_adjusted_value_destroyed: (compute_dollars && compute_adjusted)
                .then(|| computed_h!("adjusted_value_destroyed", Source::None, v0, sum())),

            // ==================== SOPR ====================
            dateindex_to_sopr: compute_dollars.then(|| eager!(DateIndex, StoredF64, "sopr", v1)),
            dateindex_to_sopr_7d_ema: compute_dollars
                .then(|| eager!(DateIndex, StoredF64, "sopr_7d_ema", v1)),
            dateindex_to_sopr_30d_ema: compute_dollars
                .then(|| eager!(DateIndex, StoredF64, "sopr_30d_ema", v1)),
            dateindex_to_adjusted_sopr: (compute_dollars && compute_adjusted)
                .then(|| eager!(DateIndex, StoredF64, "adjusted_sopr", v1)),
            dateindex_to_adjusted_sopr_7d_ema: (compute_dollars && compute_adjusted)
                .then(|| eager!(DateIndex, StoredF64, "adjusted_sopr_7d_ema", v1)),
            dateindex_to_adjusted_sopr_30d_ema: (compute_dollars && compute_adjusted)
                .then(|| eager!(DateIndex, StoredF64, "adjusted_sopr_30d_ema", v1)),

            // ==================== SELL SIDE RISK ====================
            dateindex_to_sell_side_risk_ratio: compute_dollars
                .then(|| eager!(DateIndex, StoredF32, "sell_side_risk_ratio", v1)),
            dateindex_to_sell_side_risk_ratio_7d_ema: compute_dollars
                .then(|| eager!(DateIndex, StoredF32, "sell_side_risk_ratio_7d_ema", v1)),
            dateindex_to_sell_side_risk_ratio_30d_ema: compute_dollars
                .then(|| eager!(DateIndex, StoredF32, "sell_side_risk_ratio_30d_ema", v1)),

            // ==================== SUPPLY IN PROFIT/LOSS ====================
            height_to_supply_in_profit: compute_dollars
                .then(|| eager!(Height, Sats, "supply_in_profit", v0)),
            indexes_to_supply_in_profit: compute_dollars.then(|| {
                ComputedValueVecsFromDateIndex::forced_import(
                    db,
                    &suffix("supply_in_profit"),
                    dateindex_to_supply_in_profit
                        .as_ref()
                        .map(|v| v.boxed_clone())
                        .into(),
                    version + v0,
                    last(),
                    compute_dollars,
                    indexes,
                )
                .unwrap()
            }),
            height_to_supply_in_loss: compute_dollars
                .then(|| eager!(Height, Sats, "supply_in_loss", v0)),
            indexes_to_supply_in_loss: compute_dollars.then(|| {
                ComputedValueVecsFromDateIndex::forced_import(
                    db,
                    &suffix("supply_in_loss"),
                    dateindex_to_supply_in_loss
                        .as_ref()
                        .map(|v| v.boxed_clone())
                        .into(),
                    version + v0,
                    last(),
                    compute_dollars,
                    indexes,
                )
                .unwrap()
            }),
            dateindex_to_supply_in_profit,
            dateindex_to_supply_in_loss,
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

            // ==================== UNREALIZED PROFIT & LOSS ====================
            height_to_unrealized_profit: compute_dollars
                .then(|| eager!(Height, Dollars, "unrealized_profit", v0)),
            indexes_to_unrealized_profit: compute_dollars.then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    db,
                    &suffix("unrealized_profit"),
                    dateindex_to_unrealized_profit
                        .as_ref()
                        .map(|v| v.boxed_clone())
                        .into(),
                    version + v0,
                    indexes,
                    last(),
                )
                .unwrap()
            }),
            height_to_unrealized_loss: compute_dollars
                .then(|| eager!(Height, Dollars, "unrealized_loss", v0)),
            indexes_to_unrealized_loss: compute_dollars.then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    db,
                    &suffix("unrealized_loss"),
                    dateindex_to_unrealized_loss
                        .as_ref()
                        .map(|v| v.boxed_clone())
                        .into(),
                    version + v0,
                    indexes,
                    last(),
                )
                .unwrap()
            }),
            dateindex_to_unrealized_profit,
            dateindex_to_unrealized_loss,
            height_to_neg_unrealized_loss: compute_dollars
                .then(|| eager!(Height, Dollars, "neg_unrealized_loss", v0)),
            indexes_to_neg_unrealized_loss: compute_dollars
                .then(|| computed_di!("neg_unrealized_loss", Source::Compute, v0, last())),
            height_to_net_unrealized_pnl: compute_dollars
                .then(|| eager!(Height, Dollars, "net_unrealized_pnl", v0)),
            indexes_to_net_unrealized_pnl: compute_dollars
                .then(|| computed_di!("net_unrealized_pnl", Source::Compute, v0, last())),
            height_to_total_unrealized_pnl: compute_dollars
                .then(|| eager!(Height, Dollars, "total_unrealized_pnl", v0)),
            indexes_to_total_unrealized_pnl: compute_dollars
                .then(|| computed_di!("total_unrealized_pnl", Source::Compute, v0, last())),

            // ==================== PRICE PAID ====================
            height_to_min_price_paid: compute_dollars
                .then(|| eager!(Height, Dollars, "min_price_paid", v0)),
            indexes_to_min_price_paid: compute_dollars
                .then(|| computed_h!("min_price_paid", Source::None, v0, last())),
            height_to_max_price_paid: compute_dollars
                .then(|| eager!(Height, Dollars, "max_price_paid", v0)),
            indexes_to_max_price_paid: compute_dollars
                .then(|| computed_h!("max_price_paid", Source::None, v0, last())),
            price_percentiles: (compute_dollars && extended).then(|| {
                PricePercentiles::forced_import(db, &suffix(""), version + v0, indexes, true)
                    .unwrap()
            }),

            // ==================== RELATIVE METRICS: UNREALIZED vs MARKET CAP ====================
            height_to_unrealized_profit_rel_to_market_cap: compute_dollars
                .then(|| eager!(Height, StoredF32, "unrealized_profit_rel_to_market_cap", v0)),
            height_to_unrealized_loss_rel_to_market_cap: compute_dollars
                .then(|| eager!(Height, StoredF32, "unrealized_loss_rel_to_market_cap", v0)),
            height_to_neg_unrealized_loss_rel_to_market_cap: compute_dollars.then(|| {
                eager!(
                    Height,
                    StoredF32,
                    "neg_unrealized_loss_rel_to_market_cap",
                    v0
                )
            }),
            height_to_net_unrealized_pnl_rel_to_market_cap: compute_dollars.then(|| {
                eager!(
                    Height,
                    StoredF32,
                    "net_unrealized_pnl_rel_to_market_cap",
                    v1
                )
            }),
            indexes_to_unrealized_profit_rel_to_market_cap: compute_dollars.then(|| {
                computed_di!(
                    "unrealized_profit_rel_to_market_cap",
                    Source::Compute,
                    v1,
                    last()
                )
            }),
            indexes_to_unrealized_loss_rel_to_market_cap: compute_dollars.then(|| {
                computed_di!(
                    "unrealized_loss_rel_to_market_cap",
                    Source::Compute,
                    v1,
                    last()
                )
            }),
            indexes_to_neg_unrealized_loss_rel_to_market_cap: compute_dollars.then(|| {
                computed_di!(
                    "neg_unrealized_loss_rel_to_market_cap",
                    Source::Compute,
                    v1,
                    last()
                )
            }),
            indexes_to_net_unrealized_pnl_rel_to_market_cap: compute_dollars.then(|| {
                computed_di!(
                    "net_unrealized_pnl_rel_to_market_cap",
                    Source::Compute,
                    v1,
                    last()
                )
            }),

            // ==================== RELATIVE METRICS: UNREALIZED vs OWN MARKET CAP ====================
            height_to_unrealized_profit_rel_to_own_market_cap: (compute_dollars
                && extended
                && compute_rel_to_all)
                .then(|| {
                    eager!(
                        Height,
                        StoredF32,
                        "unrealized_profit_rel_to_own_market_cap",
                        v1
                    )
                }),
            height_to_unrealized_loss_rel_to_own_market_cap: (compute_dollars
                && extended
                && compute_rel_to_all)
                .then(|| {
                    eager!(
                        Height,
                        StoredF32,
                        "unrealized_loss_rel_to_own_market_cap",
                        v1
                    )
                }),
            height_to_neg_unrealized_loss_rel_to_own_market_cap: (compute_dollars
                && extended
                && compute_rel_to_all)
                .then(|| {
                    eager!(
                        Height,
                        StoredF32,
                        "neg_unrealized_loss_rel_to_own_market_cap",
                        v1
                    )
                }),
            height_to_net_unrealized_pnl_rel_to_own_market_cap: (compute_dollars
                && extended
                && compute_rel_to_all)
                .then(|| {
                    eager!(
                        Height,
                        StoredF32,
                        "net_unrealized_pnl_rel_to_own_market_cap",
                        v2
                    )
                }),
            indexes_to_unrealized_profit_rel_to_own_market_cap: (compute_dollars
                && extended
                && compute_rel_to_all)
                .then(|| {
                    computed_di!(
                        "unrealized_profit_rel_to_own_market_cap",
                        Source::Compute,
                        v2,
                        last()
                    )
                }),
            indexes_to_unrealized_loss_rel_to_own_market_cap: (compute_dollars
                && extended
                && compute_rel_to_all)
                .then(|| {
                    computed_di!(
                        "unrealized_loss_rel_to_own_market_cap",
                        Source::Compute,
                        v2,
                        last()
                    )
                }),
            indexes_to_neg_unrealized_loss_rel_to_own_market_cap: (compute_dollars
                && extended
                && compute_rel_to_all)
                .then(|| {
                    computed_di!(
                        "neg_unrealized_loss_rel_to_own_market_cap",
                        Source::Compute,
                        v2,
                        last()
                    )
                }),
            indexes_to_net_unrealized_pnl_rel_to_own_market_cap: (compute_dollars
                && extended
                && compute_rel_to_all)
                .then(|| {
                    computed_di!(
                        "net_unrealized_pnl_rel_to_own_market_cap",
                        Source::Compute,
                        v2,
                        last()
                    )
                }),

            // ==================== RELATIVE METRICS: UNREALIZED vs OWN TOTAL UNREALIZED ====================
            height_to_unrealized_profit_rel_to_own_total_unrealized_pnl: (compute_dollars
                && extended)
                .then(|| {
                    eager!(
                        Height,
                        StoredF32,
                        "unrealized_profit_rel_to_own_total_unrealized_pnl",
                        v0
                    )
                }),
            height_to_unrealized_loss_rel_to_own_total_unrealized_pnl: (compute_dollars
                && extended)
                .then(|| {
                    eager!(
                        Height,
                        StoredF32,
                        "unrealized_loss_rel_to_own_total_unrealized_pnl",
                        v0
                    )
                }),
            height_to_neg_unrealized_loss_rel_to_own_total_unrealized_pnl: (compute_dollars
                && extended)
                .then(|| {
                    eager!(
                        Height,
                        StoredF32,
                        "neg_unrealized_loss_rel_to_own_total_unrealized_pnl",
                        v0
                    )
                }),
            height_to_net_unrealized_pnl_rel_to_own_total_unrealized_pnl: (compute_dollars
                && extended)
                .then(|| {
                    eager!(
                        Height,
                        StoredF32,
                        "net_unrealized_pnl_rel_to_own_total_unrealized_pnl",
                        v1
                    )
                }),
            indexes_to_unrealized_profit_rel_to_own_total_unrealized_pnl: (compute_dollars
                && extended)
                .then(|| {
                    computed_di!(
                        "unrealized_profit_rel_to_own_total_unrealized_pnl",
                        Source::Compute,
                        v1,
                        last()
                    )
                }),
            indexes_to_unrealized_loss_rel_to_own_total_unrealized_pnl: (compute_dollars
                && extended)
                .then(|| {
                    computed_di!(
                        "unrealized_loss_rel_to_own_total_unrealized_pnl",
                        Source::Compute,
                        v1,
                        last()
                    )
                }),
            indexes_to_neg_unrealized_loss_rel_to_own_total_unrealized_pnl: (compute_dollars
                && extended)
                .then(|| {
                    computed_di!(
                        "neg_unrealized_loss_rel_to_own_total_unrealized_pnl",
                        Source::Compute,
                        v1,
                        last()
                    )
                }),
            indexes_to_net_unrealized_pnl_rel_to_own_total_unrealized_pnl: (compute_dollars
                && extended)
                .then(|| {
                    computed_di!(
                        "net_unrealized_pnl_rel_to_own_total_unrealized_pnl",
                        Source::Compute,
                        v1,
                        last()
                    )
                }),

            // ==================== RELATIVE METRICS: SUPPLY vs CIRCULATING/OWN ====================
            indexes_to_supply_rel_to_circulating_supply: compute_rel_to_all.then(|| {
                computed_h!(
                    "supply_rel_to_circulating_supply",
                    Source::Compute,
                    v1,
                    last()
                )
            }),
            height_to_supply_in_profit_rel_to_own_supply: compute_dollars
                .then(|| eager!(Height, StoredF64, "supply_in_profit_rel_to_own_supply", v1)),
            height_to_supply_in_loss_rel_to_own_supply: compute_dollars
                .then(|| eager!(Height, StoredF64, "supply_in_loss_rel_to_own_supply", v1)),
            indexes_to_supply_in_profit_rel_to_own_supply: compute_dollars.then(|| {
                computed_di!(
                    "supply_in_profit_rel_to_own_supply",
                    Source::Compute,
                    v1,
                    last()
                )
            }),
            indexes_to_supply_in_loss_rel_to_own_supply: compute_dollars.then(|| {
                computed_di!(
                    "supply_in_loss_rel_to_own_supply",
                    Source::Compute,
                    v1,
                    last()
                )
            }),
            height_to_supply_in_profit_rel_to_circulating_supply: (compute_rel_to_all
                && compute_dollars)
                .then(|| {
                    eager!(
                        Height,
                        StoredF64,
                        "supply_in_profit_rel_to_circulating_supply",
                        v1
                    )
                }),
            height_to_supply_in_loss_rel_to_circulating_supply: (compute_rel_to_all
                && compute_dollars)
                .then(|| {
                    eager!(
                        Height,
                        StoredF64,
                        "supply_in_loss_rel_to_circulating_supply",
                        v1
                    )
                }),
            indexes_to_supply_in_profit_rel_to_circulating_supply: (compute_rel_to_all
                && compute_dollars)
                .then(|| {
                    computed_di!(
                        "supply_in_profit_rel_to_circulating_supply",
                        Source::Compute,
                        v1,
                        last()
                    )
                }),
            indexes_to_supply_in_loss_rel_to_circulating_supply: (compute_rel_to_all
                && compute_dollars)
                .then(|| {
                    computed_di!(
                        "supply_in_loss_rel_to_circulating_supply",
                        Source::Compute,
                        v1,
                        last()
                    )
                }),

            // ==================== NET REALIZED PNL DELTAS ====================
            indexes_to_net_realized_pnl_cumulative_30d_delta: compute_dollars.then(|| {
                computed_di!(
                    "net_realized_pnl_cumulative_30d_delta",
                    Source::Compute,
                    v3,
                    last()
                )
            }),
            indexes_to_net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap: compute_dollars
                .then(|| {
                    computed_di!(
                        "net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap",
                        Source::Compute,
                        v3,
                        last()
                    )
                }),
            indexes_to_net_realized_pnl_cumulative_30d_delta_rel_to_market_cap: compute_dollars
                .then(|| {
                    computed_di!(
                        "net_realized_pnl_cumulative_30d_delta_rel_to_market_cap",
                        Source::Compute,
                        v3,
                        last()
                    )
                }),
        })
    }

    /// Returns the minimum length of all height-indexed vectors.
    /// Used to determine the starting point for processing.
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

    /// Import state from a checkpoint when resuming processing.
    /// Returns the next height to process from.
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

    /// Validate that all computed versions match expected values, resetting if needed.
    pub fn validate_computed_versions(&mut self, base_version: Version) -> Result<()> {
        // Always-present vecs
        self.height_to_supply.validate_computed_version_or_reset(
            base_version + self.height_to_supply.inner_version(),
        )?;
        self.height_to_utxo_count
            .validate_computed_version_or_reset(
                base_version + self.height_to_utxo_count.inner_version(),
            )?;
        self.height_to_sent.validate_computed_version_or_reset(
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

        // Dollar-dependent vecs
        if let Some(height_to_realized_cap) = self.height_to_realized_cap.as_mut().as_mut() {
            height_to_realized_cap.validate_computed_version_or_reset(
                base_version + height_to_realized_cap.inner_version(),
            )?;

            Self::validate_optional_vec_version(&mut self.height_to_realized_profit, base_version)?;
            Self::validate_optional_vec_version(&mut self.height_to_realized_loss, base_version)?;
            Self::validate_optional_vec_version(&mut self.height_to_value_created, base_version)?;
            Self::validate_optional_vec_version(&mut self.height_to_value_destroyed, base_version)?;
            Self::validate_optional_vec_version(
                &mut self.height_to_supply_in_profit,
                base_version,
            )?;
            Self::validate_optional_vec_version(&mut self.height_to_supply_in_loss, base_version)?;
            Self::validate_optional_vec_version(
                &mut self.height_to_unrealized_profit,
                base_version,
            )?;
            Self::validate_optional_vec_version(&mut self.height_to_unrealized_loss, base_version)?;
            Self::validate_optional_vec_version(
                &mut self.dateindex_to_supply_in_profit,
                base_version,
            )?;
            Self::validate_optional_vec_version(
                &mut self.dateindex_to_supply_in_loss,
                base_version,
            )?;
            Self::validate_optional_vec_version(
                &mut self.dateindex_to_unrealized_profit,
                base_version,
            )?;
            Self::validate_optional_vec_version(
                &mut self.dateindex_to_unrealized_loss,
                base_version,
            )?;
            Self::validate_optional_vec_version(&mut self.height_to_min_price_paid, base_version)?;
            Self::validate_optional_vec_version(&mut self.height_to_max_price_paid, base_version)?;

            if self.height_to_adjusted_value_created.is_some() {
                Self::validate_optional_vec_version(
                    &mut self.height_to_adjusted_value_created,
                    base_version,
                )?;
                Self::validate_optional_vec_version(
                    &mut self.height_to_adjusted_value_destroyed,
                    base_version,
                )?;
            }
        }

        Ok(())
    }

    /// Helper to validate an optional vec's version.
    fn validate_optional_vec_version<V: StoredVec>(
        vec: &mut Option<EagerVec<V>>,
        base_version: Version,
    ) -> Result<()> {
        if let Some(v) = vec.as_mut() {
            v.validate_computed_version_or_reset(base_version + v.inner_version())?;
        }
        Ok(())
    }
}
