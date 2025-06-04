use std::{fs, path::Path};

use brk_core::{
    DateIndex, Dollars, Height, Result, Sats, StoredF32, StoredF64, StoredUsize, Version,
};
use brk_exit::Exit;
use brk_indexer::Indexer;
use brk_state::CohortState;
use brk_vec::{
    AnyCollectableVec, AnyIterableVec, AnyVec, Computation, EagerVec, Format, StoredIndex,
    VecIterator,
};
use fjall::TransactionalKeyspace;

use crate::vecs::{
    Indexes, fetched,
    grouped::{
        ComputedHeightValueVecs, ComputedRatioVecsFromDateIndex, ComputedValueVecsFromDateIndex,
        ComputedValueVecsFromHeight, ComputedVecsFromDateIndex, ComputedVecsFromHeight,
        StorableVecGeneatorOptions,
    },
    indexes, market,
};

const VERSION: Version = Version::ZERO;

#[derive(Clone)]
pub struct Vecs {
    starting_height: Height,
    pub state: CohortState,

    // Cumulative
    pub height_to_realized_cap: Option<EagerVec<Height, Dollars>>,
    pub height_to_supply: EagerVec<Height, Sats>,
    pub height_to_utxo_count: EagerVec<Height, StoredUsize>,
    // Single
    pub dateindex_to_supply_even: Option<EagerVec<DateIndex, Sats>>,
    pub dateindex_to_supply_in_loss: Option<EagerVec<DateIndex, Sats>>,
    pub dateindex_to_supply_in_profit: Option<EagerVec<DateIndex, Sats>>,
    pub dateindex_to_unrealized_loss: Option<EagerVec<DateIndex, Dollars>>,
    pub dateindex_to_unrealized_profit: Option<EagerVec<DateIndex, Dollars>>,
    pub height_to_adjusted_value_created: Option<EagerVec<Height, Dollars>>,
    pub height_to_adjusted_value_destroyed: Option<EagerVec<Height, Dollars>>,
    pub height_to_max_price_paid: Option<EagerVec<Height, Dollars>>,
    pub height_to_min_price_paid: Option<EagerVec<Height, Dollars>>,
    pub height_to_realized_loss: Option<EagerVec<Height, Dollars>>,
    pub height_to_realized_profit: Option<EagerVec<Height, Dollars>>,
    pub height_to_supply_even: Option<EagerVec<Height, Sats>>,
    pub height_to_supply_in_loss: Option<EagerVec<Height, Sats>>,
    pub height_to_supply_in_profit: Option<EagerVec<Height, Sats>>,
    pub height_to_unrealized_loss: Option<EagerVec<Height, Dollars>>,
    pub height_to_unrealized_profit: Option<EagerVec<Height, Dollars>>,
    pub height_to_value_created: Option<EagerVec<Height, Dollars>>,
    pub height_to_value_destroyed: Option<EagerVec<Height, Dollars>>,

    pub dateindex_to_adjusted_spent_output_profit_ratio: Option<EagerVec<DateIndex, StoredF32>>,
    pub dateindex_to_realized_cap_30d_change: Option<EagerVec<DateIndex, Dollars>>,
    pub dateindex_to_sell_side_risk_ratio: Option<EagerVec<DateIndex, StoredF32>>,
    pub dateindex_to_spent_output_profit_ratio: Option<EagerVec<DateIndex, StoredF32>>,
    pub indexes_to_adjusted_value_created: Option<ComputedVecsFromHeight<Dollars>>,
    pub indexes_to_adjusted_value_destroyed: Option<ComputedVecsFromHeight<Dollars>>,
    pub indexes_to_negative_realized_loss: Option<ComputedVecsFromHeight<Dollars>>,
    pub indexes_to_net_realized_profit_and_loss: Option<ComputedVecsFromHeight<Dollars>>,
    pub indexes_to_realized_cap: Option<ComputedVecsFromHeight<Dollars>>,
    pub indexes_to_realized_loss: Option<ComputedVecsFromHeight<Dollars>>,
    pub indexes_to_realized_price: Option<ComputedVecsFromHeight<Dollars>>,
    pub indexes_to_realized_price_extra: Option<ComputedRatioVecsFromDateIndex>,
    pub indexes_to_realized_profit: Option<ComputedVecsFromHeight<Dollars>>,
    pub indexes_to_realized_value: Option<ComputedVecsFromHeight<Dollars>>,
    pub indexes_to_supply: ComputedValueVecsFromHeight,
    pub indexes_to_utxo_count: ComputedVecsFromHeight<StoredUsize>,
    pub indexes_to_value_created: Option<ComputedVecsFromHeight<Dollars>>,
    pub indexes_to_value_destroyed: Option<ComputedVecsFromHeight<Dollars>>,
    pub indexes_to_unrealized_profit: Option<ComputedVecsFromDateIndex<Dollars>>,
    pub indexes_to_unrealized_loss: Option<ComputedVecsFromDateIndex<Dollars>>,
    pub indexes_to_min_price_paid: Option<ComputedVecsFromHeight<Dollars>>,
    pub indexes_to_max_price_paid: Option<ComputedVecsFromHeight<Dollars>>,
    pub indexes_to_halved_supply: ComputedValueVecsFromHeight,
    pub height_to_negative_unrealized_loss: Option<EagerVec<Height, Dollars>>,
    pub indexes_to_negative_unrealized_loss: Option<ComputedVecsFromDateIndex<Dollars>>,
    pub height_to_net_unrealized_profit_and_loss: Option<EagerVec<Height, Dollars>>,
    pub indexes_to_net_unrealized_profit_and_loss: Option<ComputedVecsFromDateIndex<Dollars>>,
    pub height_to_net_unrealized_profit_and_loss_relative_to_market_cap:
        Option<EagerVec<Height, StoredF32>>,
    pub indexes_to_net_unrealized_profit_and_loss_relative_to_market_cap:
        Option<ComputedVecsFromDateIndex<StoredF32>>,
    pub indexes_to_net_realized_profit_and_loss_relative_to_realized_cap:
        Option<ComputedVecsFromHeight<StoredF32>>,
    pub height_to_supply_even_value: Option<ComputedHeightValueVecs>,
    pub height_to_supply_in_loss_value: Option<ComputedHeightValueVecs>,
    pub height_to_supply_in_profit_value: Option<ComputedHeightValueVecs>,
    pub indexes_to_supply_even: Option<ComputedValueVecsFromDateIndex>,
    pub indexes_to_supply_in_loss: Option<ComputedValueVecsFromDateIndex>,
    pub indexes_to_supply_in_profit: Option<ComputedValueVecsFromDateIndex>,
    pub height_to_supply_even_relative_to_own_supply: Option<EagerVec<Height, StoredF64>>,
    pub height_to_supply_in_loss_relative_to_own_supply: Option<EagerVec<Height, StoredF64>>,
    pub height_to_supply_in_profit_relative_to_own_supply: Option<EagerVec<Height, StoredF64>>,
    pub indexes_to_supply_even_relative_to_own_supply: Option<ComputedVecsFromDateIndex<StoredF64>>,
    pub indexes_to_supply_in_loss_relative_to_own_supply:
        Option<ComputedVecsFromDateIndex<StoredF64>>,
    pub indexes_to_supply_in_profit_relative_to_own_supply:
        Option<ComputedVecsFromDateIndex<StoredF64>>,
    pub indexes_to_supply_relative_to_circulating_supply: Option<ComputedVecsFromHeight<StoredF64>>,
    pub height_to_supply_even_relative_to_circulating_supply: Option<EagerVec<Height, StoredF64>>,
    pub height_to_supply_in_loss_relative_to_circulating_supply:
        Option<EagerVec<Height, StoredF64>>,
    pub height_to_supply_in_profit_relative_to_circulating_supply:
        Option<EagerVec<Height, StoredF64>>,
    pub indexes_to_supply_even_relative_to_circulating_supply:
        Option<ComputedVecsFromDateIndex<StoredF64>>,
    pub indexes_to_supply_in_loss_relative_to_circulating_supply:
        Option<ComputedVecsFromDateIndex<StoredF64>>,
    pub indexes_to_supply_in_profit_relative_to_circulating_supply:
        Option<ComputedVecsFromDateIndex<StoredF64>>,
}

impl Vecs {
    #[allow(clippy::too_many_arguments)]
    pub fn forced_import(
        path: &Path,
        cohort_name: Option<&str>,
        _computation: Computation,
        format: Format,
        version: Version,
        fetched: Option<&fetched::Vecs>,
        keyspace: &TransactionalKeyspace,
        stores_path: &Path,
        compute_relative_to_all: bool,
    ) -> color_eyre::Result<Self> {
        let compute_dollars = fetched.is_some();

        fs::create_dir_all(path)?;

        // let prefix = |s: &str| cohort_name.map_or(s.to_string(), |name| format!("{s}_{name}"));

        let suffix = |s: &str| cohort_name.map_or(s.to_string(), |name| format!("{name}_{s}"));

        let state = CohortState::default_and_import(
            keyspace,
            stores_path,
            cohort_name.unwrap_or_default(),
            version,
            compute_dollars,
        )?;

        Ok(Self {
            starting_height: Height::ZERO,
            state,

            height_to_supply_in_profit: compute_dollars.then(|| {
                EagerVec::forced_import(
                    path,
                    &suffix("supply_in_profit"),
                    version + VERSION + Version::ZERO,
                    format,
                )
                .unwrap()
            }),
            dateindex_to_supply_in_profit: compute_dollars.then(|| {
                EagerVec::forced_import(
                    path,
                    &suffix("supply_in_profit"),
                    version + VERSION + Version::ZERO,
                    format,
                )
                .unwrap()
            }),
            indexes_to_supply_in_profit: compute_dollars.then(|| {
                ComputedValueVecsFromDateIndex::forced_import(
                    path,
                    &suffix("supply_in_profit"),
                    false,
                    version + VERSION + Version::ZERO,
                    format,
                    StorableVecGeneatorOptions::default().add_last(),
                    compute_dollars,
                )
                .unwrap()
            }),
            height_to_supply_even: compute_dollars.then(|| {
                EagerVec::forced_import(
                    path,
                    &suffix("supply_even"),
                    version + VERSION + Version::ZERO,
                    format,
                )
                .unwrap()
            }),
            dateindex_to_supply_even: compute_dollars.then(|| {
                EagerVec::forced_import(
                    path,
                    &suffix("supply_even"),
                    version + VERSION + Version::ZERO,
                    format,
                )
                .unwrap()
            }),
            indexes_to_supply_even: compute_dollars.then(|| {
                ComputedValueVecsFromDateIndex::forced_import(
                    path,
                    &suffix("supply_even"),
                    false,
                    version + VERSION + Version::ZERO,
                    format,
                    StorableVecGeneatorOptions::default().add_last(),
                    compute_dollars,
                )
                .unwrap()
            }),
            height_to_supply_in_loss: compute_dollars.then(|| {
                EagerVec::forced_import(
                    path,
                    &suffix("supply_in_loss"),
                    version + VERSION + Version::ZERO,
                    format,
                )
                .unwrap()
            }),
            dateindex_to_supply_in_loss: compute_dollars.then(|| {
                EagerVec::forced_import(
                    path,
                    &suffix("supply_in_loss"),
                    version + VERSION + Version::ZERO,
                    format,
                )
                .unwrap()
            }),
            indexes_to_supply_in_loss: compute_dollars.then(|| {
                ComputedValueVecsFromDateIndex::forced_import(
                    path,
                    &suffix("supply_in_loss"),
                    false,
                    version + VERSION + Version::ZERO,
                    format,
                    StorableVecGeneatorOptions::default().add_last(),
                    compute_dollars,
                )
                .unwrap()
            }),
            height_to_unrealized_profit: compute_dollars.then(|| {
                EagerVec::forced_import(
                    path,
                    &suffix("unrealized_profit"),
                    version + VERSION + Version::ZERO,
                    format,
                )
                .unwrap()
            }),
            dateindex_to_unrealized_profit: compute_dollars.then(|| {
                EagerVec::forced_import(
                    path,
                    &suffix("unrealized_profit"),
                    version + VERSION + Version::ZERO,
                    format,
                )
                .unwrap()
            }),
            indexes_to_unrealized_profit: compute_dollars.then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    path,
                    &suffix("unrealized_profit"),
                    false,
                    version + VERSION + Version::ZERO,
                    format,
                    StorableVecGeneatorOptions::default().add_last(),
                )
                .unwrap()
            }),
            height_to_unrealized_loss: compute_dollars.then(|| {
                EagerVec::forced_import(
                    path,
                    &suffix("unrealized_loss"),
                    version + VERSION + Version::ZERO,
                    format,
                )
                .unwrap()
            }),
            height_to_min_price_paid: compute_dollars.then(|| {
                EagerVec::forced_import(
                    path,
                    &suffix("min_price_paid"),
                    version + VERSION + Version::ZERO,
                    format,
                )
                .unwrap()
            }),
            height_to_max_price_paid: compute_dollars.then(|| {
                EagerVec::forced_import(
                    path,
                    &suffix("max_price_paid"),
                    version + VERSION + Version::ZERO,
                    format,
                )
                .unwrap()
            }),
            dateindex_to_unrealized_loss: compute_dollars.then(|| {
                EagerVec::forced_import(
                    path,
                    &suffix("unrealized_loss"),
                    version + VERSION + Version::ZERO,
                    format,
                )
                .unwrap()
            }),
            indexes_to_unrealized_loss: compute_dollars.then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    path,
                    &suffix("unrealized_loss"),
                    false,
                    version + VERSION + Version::ZERO,
                    format,
                    StorableVecGeneatorOptions::default().add_last(),
                )
                .unwrap()
            }),
            height_to_realized_cap: compute_dollars.then(|| {
                EagerVec::forced_import(
                    path,
                    &suffix("realized_cap"),
                    version + VERSION + Version::ZERO,
                    format,
                )
                .unwrap()
            }),
            indexes_to_realized_cap: compute_dollars.then(|| {
                ComputedVecsFromHeight::forced_import(
                    path,
                    &suffix("realized_cap"),
                    false,
                    version + VERSION + Version::ZERO,
                    format,
                    StorableVecGeneatorOptions::default().add_last(),
                )
                .unwrap()
            }),
            indexes_to_min_price_paid: compute_dollars.then(|| {
                ComputedVecsFromHeight::forced_import(
                    path,
                    &suffix("min_price_paid"),
                    false,
                    version + VERSION + Version::ZERO,
                    format,
                    StorableVecGeneatorOptions::default().add_last(),
                )
                .unwrap()
            }),
            indexes_to_max_price_paid: compute_dollars.then(|| {
                ComputedVecsFromHeight::forced_import(
                    path,
                    &suffix("max_price_paid"),
                    false,
                    version + VERSION + Version::ZERO,
                    format,
                    StorableVecGeneatorOptions::default().add_last(),
                )
                .unwrap()
            }),
            height_to_supply: EagerVec::forced_import(
                path,
                &suffix("supply"),
                version + VERSION + Version::ZERO,
                format,
            )?,
            indexes_to_supply: ComputedValueVecsFromHeight::forced_import(
                path,
                &suffix("supply"),
                false,
                version + VERSION + Version::ZERO,
                format,
                StorableVecGeneatorOptions::default().add_last(),
                compute_dollars,
            )?,
            height_to_utxo_count: EagerVec::forced_import(
                path,
                &suffix("utxo_count"),
                version + VERSION + Version::ZERO,
                format,
            )?,
            indexes_to_utxo_count: ComputedVecsFromHeight::forced_import(
                path,
                &suffix("utxo_count"),
                false,
                version + VERSION + Version::ZERO,
                format,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            indexes_to_realized_price: compute_dollars.then(|| {
                ComputedVecsFromHeight::forced_import(
                    path,
                    &suffix("realized_price"),
                    true,
                    version + VERSION + Version::ZERO,
                    format,
                    StorableVecGeneatorOptions::default().add_last(),
                )
                .unwrap()
            }),
            indexes_to_realized_price_extra: compute_dollars.then(|| {
                ComputedRatioVecsFromDateIndex::forced_import(
                    path,
                    &suffix("realized_price"),
                    false,
                    version + VERSION + Version::ZERO,
                    format,
                    StorableVecGeneatorOptions::default().add_last(),
                )
                .unwrap()
            }),
            height_to_realized_profit: compute_dollars.then(|| {
                EagerVec::forced_import(
                    path,
                    &suffix("realized_profit"),
                    version + VERSION + Version::ZERO,
                    format,
                )
                .unwrap()
            }),
            indexes_to_realized_profit: compute_dollars.then(|| {
                ComputedVecsFromHeight::forced_import(
                    path,
                    &suffix("realized_profit"),
                    false,
                    version + VERSION + Version::ZERO,
                    format,
                    StorableVecGeneatorOptions::default().add_sum(),
                )
                .unwrap()
            }),
            height_to_realized_loss: compute_dollars.then(|| {
                EagerVec::forced_import(
                    path,
                    &suffix("realized_loss"),
                    version + VERSION + Version::ZERO,
                    format,
                )
                .unwrap()
            }),
            indexes_to_realized_loss: compute_dollars.then(|| {
                ComputedVecsFromHeight::forced_import(
                    path,
                    &suffix("realized_loss"),
                    false,
                    version + VERSION + Version::ZERO,
                    format,
                    StorableVecGeneatorOptions::default().add_sum(),
                )
                .unwrap()
            }),
            indexes_to_negative_realized_loss: compute_dollars.then(|| {
                ComputedVecsFromHeight::forced_import(
                    path,
                    &suffix("negative_realized_loss"),
                    true,
                    version + VERSION + Version::ONE,
                    format,
                    StorableVecGeneatorOptions::default().add_sum(),
                )
                .unwrap()
            }),
            height_to_value_created: compute_dollars.then(|| {
                EagerVec::forced_import(
                    path,
                    &suffix("value_created"),
                    version + VERSION + Version::ZERO,
                    format,
                )
                .unwrap()
            }),
            indexes_to_value_created: compute_dollars.then(|| {
                ComputedVecsFromHeight::forced_import(
                    path,
                    &suffix("value_created"),
                    false,
                    version + VERSION + Version::ZERO,
                    format,
                    StorableVecGeneatorOptions::default().add_sum(),
                )
                .unwrap()
            }),
            indexes_to_realized_value: compute_dollars.then(|| {
                ComputedVecsFromHeight::forced_import(
                    path,
                    &suffix("realized_value"),
                    true,
                    version + VERSION + Version::ZERO,
                    format,
                    StorableVecGeneatorOptions::default().add_sum(),
                )
                .unwrap()
            }),
            height_to_adjusted_value_created: compute_dollars.then(|| {
                EagerVec::forced_import(
                    path,
                    &suffix("adjusted_value_created"),
                    version + VERSION + Version::ZERO,
                    format,
                )
                .unwrap()
            }),
            indexes_to_adjusted_value_created: compute_dollars.then(|| {
                ComputedVecsFromHeight::forced_import(
                    path,
                    &suffix("adjusted_value_created"),
                    false,
                    version + VERSION + Version::ZERO,
                    format,
                    StorableVecGeneatorOptions::default().add_sum(),
                )
                .unwrap()
            }),
            height_to_value_destroyed: compute_dollars.then(|| {
                EagerVec::forced_import(
                    path,
                    &suffix("value_destroyed"),
                    version + VERSION + Version::ZERO,
                    format,
                )
                .unwrap()
            }),
            indexes_to_value_destroyed: compute_dollars.then(|| {
                ComputedVecsFromHeight::forced_import(
                    path,
                    &suffix("value_destroyed"),
                    false,
                    version + VERSION + Version::ZERO,
                    format,
                    StorableVecGeneatorOptions::default().add_sum(),
                )
                .unwrap()
            }),
            height_to_adjusted_value_destroyed: compute_dollars.then(|| {
                EagerVec::forced_import(
                    path,
                    &suffix("adjusted_value_destroyed"),
                    version + VERSION + Version::ZERO,
                    format,
                )
                .unwrap()
            }),
            indexes_to_adjusted_value_destroyed: compute_dollars.then(|| {
                ComputedVecsFromHeight::forced_import(
                    path,
                    &suffix("adjusted_value_destroyed"),
                    false,
                    version + VERSION + Version::ZERO,
                    format,
                    StorableVecGeneatorOptions::default().add_sum(),
                )
                .unwrap()
            }),
            dateindex_to_realized_cap_30d_change: compute_dollars.then(|| {
                EagerVec::forced_import(
                    path,
                    &suffix("realized_cap_30d_change"),
                    version + VERSION + Version::ZERO,
                    format,
                )
                .unwrap()
            }),
            indexes_to_net_realized_profit_and_loss: compute_dollars.then(|| {
                ComputedVecsFromHeight::forced_import(
                    path,
                    &suffix("net_realized_profit_and_loss"),
                    true,
                    version + VERSION + Version::ZERO,
                    format,
                    StorableVecGeneatorOptions::default().add_sum(),
                )
                .unwrap()
            }),
            dateindex_to_sell_side_risk_ratio: compute_dollars.then(|| {
                EagerVec::forced_import(
                    path,
                    &suffix("sell_side_risk_ratio"),
                    version + VERSION + Version::ONE,
                    format,
                )
                .unwrap()
            }),
            dateindex_to_spent_output_profit_ratio: compute_dollars.then(|| {
                EagerVec::forced_import(
                    path,
                    &suffix("spent_output_profit_ratio"),
                    version + VERSION + Version::ZERO,
                    format,
                )
                .unwrap()
            }),
            dateindex_to_adjusted_spent_output_profit_ratio: compute_dollars.then(|| {
                EagerVec::forced_import(
                    path,
                    &suffix("adjusted_spent_output_profit_ratio"),
                    version + VERSION + Version::ZERO,
                    format,
                )
                .unwrap()
            }),
            indexes_to_halved_supply: ComputedValueVecsFromHeight::forced_import(
                path,
                &suffix("halved_supply"),
                true,
                version + VERSION + Version::ZERO,
                format,
                StorableVecGeneatorOptions::default().add_last(),
                compute_dollars,
            )?,

            height_to_negative_unrealized_loss: compute_dollars.then(|| {
                EagerVec::forced_import(
                    path,
                    &suffix("negative_unrealized_loss"),
                    version + VERSION + Version::ZERO,
                    format,
                )
                .unwrap()
            }),
            indexes_to_negative_unrealized_loss: compute_dollars.then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    path,
                    &suffix("negative_unrealized_loss"),
                    true,
                    version + VERSION + Version::ZERO,
                    format,
                    StorableVecGeneatorOptions::default().add_last(),
                )
                .unwrap()
            }),
            height_to_net_unrealized_profit_and_loss: compute_dollars.then(|| {
                EagerVec::forced_import(
                    path,
                    &suffix("net_unrealized_profit_and_loss"),
                    version + VERSION + Version::ZERO,
                    format,
                )
                .unwrap()
            }),
            indexes_to_net_unrealized_profit_and_loss: compute_dollars.then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    path,
                    &suffix("net_unrealized_profit_and_loss"),
                    true,
                    version + VERSION + Version::ZERO,
                    format,
                    StorableVecGeneatorOptions::default().add_last(),
                )
                .unwrap()
            }),
            height_to_net_unrealized_profit_and_loss_relative_to_market_cap: compute_dollars.then(
                || {
                    EagerVec::forced_import(
                        path,
                        &suffix("net_unrealized_profit_and_loss_relative_to_market_cap"),
                        version + VERSION + Version::ZERO,
                        format,
                    )
                    .unwrap()
                },
            ),
            indexes_to_net_unrealized_profit_and_loss_relative_to_market_cap: compute_dollars.then(
                || {
                    ComputedVecsFromDateIndex::forced_import(
                        path,
                        &suffix("net_unrealized_profit_and_loss_relative_to_market_cap"),
                        true,
                        version + VERSION + Version::ZERO,
                        format,
                        StorableVecGeneatorOptions::default().add_last(),
                    )
                    .unwrap()
                },
            ),
            indexes_to_net_realized_profit_and_loss_relative_to_realized_cap: compute_dollars.then(
                || {
                    ComputedVecsFromHeight::forced_import(
                        path,
                        &suffix("net_realized_profit_and_loss_relative_to_realized_cap"),
                        true,
                        version + VERSION + Version::ZERO,
                        format,
                        StorableVecGeneatorOptions::default().add_last(),
                    )
                    .unwrap()
                },
            ),
            height_to_supply_even_value: compute_dollars.then(|| {
                ComputedHeightValueVecs::forced_import(
                    path,
                    &suffix("supply_even"),
                    false,
                    version,
                    format,
                    compute_dollars,
                )
                .unwrap()
            }),
            height_to_supply_in_loss_value: compute_dollars.then(|| {
                ComputedHeightValueVecs::forced_import(
                    path,
                    &suffix("supply_in_loss"),
                    false,
                    version,
                    format,
                    compute_dollars,
                )
                .unwrap()
            }),
            height_to_supply_in_profit_value: compute_dollars.then(|| {
                ComputedHeightValueVecs::forced_import(
                    path,
                    &suffix("supply_in_profit"),
                    false,
                    version,
                    format,
                    compute_dollars,
                )
                .unwrap()
            }),
            height_to_supply_even_relative_to_own_supply: compute_dollars.then(|| {
                EagerVec::forced_import(
                    path,
                    &suffix("supply_even_relative_to_own_supply"),
                    version,
                    format,
                )
                .unwrap()
            }),
            height_to_supply_in_loss_relative_to_own_supply: compute_dollars.then(|| {
                EagerVec::forced_import(
                    path,
                    &suffix("supply_in_loss_relative_to_own_supply"),
                    version,
                    format,
                )
                .unwrap()
            }),
            height_to_supply_in_profit_relative_to_own_supply: compute_dollars.then(|| {
                EagerVec::forced_import(
                    path,
                    &suffix("supply_in_profit_relative_to_own_supply"),
                    version,
                    format,
                )
                .unwrap()
            }),
            indexes_to_supply_even_relative_to_own_supply: compute_dollars.then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    path,
                    &suffix("supply_even_relative_to_own_supply"),
                    true,
                    version,
                    format,
                    StorableVecGeneatorOptions::default().add_last(),
                )
                .unwrap()
            }),
            indexes_to_supply_in_loss_relative_to_own_supply: compute_dollars.then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    path,
                    &suffix("supply_in_loss_relative_to_own_supply"),
                    true,
                    version,
                    format,
                    StorableVecGeneatorOptions::default().add_last(),
                )
                .unwrap()
            }),
            indexes_to_supply_in_profit_relative_to_own_supply: compute_dollars.then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    path,
                    &suffix("supply_in_profit_relative_to_own_supply"),
                    true,
                    version,
                    format,
                    StorableVecGeneatorOptions::default().add_last(),
                )
                .unwrap()
            }),
            indexes_to_supply_relative_to_circulating_supply: compute_relative_to_all.then(|| {
                ComputedVecsFromHeight::forced_import(
                    path,
                    &suffix("supply_relative_to_circulating_supply"),
                    true,
                    version,
                    format,
                    StorableVecGeneatorOptions::default().add_last(),
                )
                .unwrap()
            }),
            height_to_supply_even_relative_to_circulating_supply: (compute_relative_to_all
                && compute_dollars)
                .then(|| {
                    EagerVec::forced_import(
                        path,
                        &suffix("supply_even_relative_to_circulating_supply"),
                        version,
                        format,
                    )
                    .unwrap()
                }),
            height_to_supply_in_loss_relative_to_circulating_supply: (compute_relative_to_all
                && compute_dollars)
                .then(|| {
                    EagerVec::forced_import(
                        path,
                        &suffix("supply_in_loss_relative_to_circulating_supply"),
                        version,
                        format,
                    )
                    .unwrap()
                }),
            height_to_supply_in_profit_relative_to_circulating_supply: (compute_relative_to_all
                && compute_dollars)
                .then(|| {
                    EagerVec::forced_import(
                        path,
                        &suffix("supply_in_profit_relative_to_circulating_supply"),
                        version,
                        format,
                    )
                    .unwrap()
                }),
            indexes_to_supply_even_relative_to_circulating_supply: (compute_relative_to_all
                && compute_dollars)
                .then(|| {
                    ComputedVecsFromDateIndex::forced_import(
                        path,
                        &suffix("supply_even_relative_to_circulating_supply"),
                        true,
                        version,
                        format,
                        StorableVecGeneatorOptions::default().add_last(),
                    )
                    .unwrap()
                }),
            indexes_to_supply_in_loss_relative_to_circulating_supply: (compute_relative_to_all
                && compute_dollars)
                .then(|| {
                    ComputedVecsFromDateIndex::forced_import(
                        path,
                        &suffix("supply_in_loss_relative_to_circulating_supply"),
                        true,
                        version,
                        format,
                        StorableVecGeneatorOptions::default().add_last(),
                    )
                    .unwrap()
                }),
            indexes_to_supply_in_profit_relative_to_circulating_supply: (compute_relative_to_all
                && compute_dollars)
                .then(|| {
                    ComputedVecsFromDateIndex::forced_import(
                        path,
                        &suffix("supply_in_profit_relative_to_circulating_supply"),
                        true,
                        version,
                        format,
                        StorableVecGeneatorOptions::default().add_last(),
                    )
                    .unwrap()
                }),
        })
    }

    pub fn starting_height(&self) -> Height {
        [
            self.state
                .price_to_amount
                .height()
                .map_or(usize::MAX, |h| h.incremented().unwrap_to_usize()),
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
            self.height_to_supply_even
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
        ]
        .into_iter()
        .map(Height::from)
        .min()
        .unwrap()
    }

    pub fn init(&mut self, starting_height: Height) {
        if starting_height > self.starting_height() {
            unreachable!()
        }

        self.starting_height = starting_height;

        if let Some(prev_height) = starting_height.decremented() {
            self.state.supply.value = self
                .height_to_supply
                .into_iter()
                .unwrap_get_inner(prev_height);
            self.state.supply.utxos = *self
                .height_to_utxo_count
                .into_iter()
                .unwrap_get_inner(prev_height);

            self.state.price_to_amount.copy_db_to_puts();

            if let Some(height_to_realized_cap) = self.height_to_realized_cap.as_mut() {
                self.state.realized.as_mut().unwrap().cap = height_to_realized_cap
                    .into_iter()
                    .unwrap_get_inner(prev_height);
            }
        }
    }

    pub fn validate_computed_versions(&mut self, base_version: Version) -> Result<()> {
        self.height_to_supply
            .validate_computed_version_or_reset_file(
                base_version + self.height_to_supply.inner_version(),
            )?;

        self.height_to_utxo_count
            .validate_computed_version_or_reset_file(
                base_version + self.height_to_utxo_count.inner_version(),
            )?;

        if let Some(height_to_realized_cap) = self.height_to_realized_cap.as_mut().as_mut() {
            height_to_realized_cap.validate_computed_version_or_reset_file(
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
                .validate_computed_version_or_reset_file(
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
                .validate_computed_version_or_reset_file(
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
                .validate_computed_version_or_reset_file(
                    base_version + height_to_value_created_inner_version,
                )?;
            let height_to_adjusted_value_created_inner_version = self
                .height_to_adjusted_value_created
                .as_ref()
                .unwrap()
                .inner_version();
            self.height_to_adjusted_value_created
                .as_mut()
                .unwrap()
                .validate_computed_version_or_reset_file(
                    base_version + height_to_adjusted_value_created_inner_version,
                )?;
            let height_to_value_destroyed_inner_version = self
                .height_to_value_destroyed
                .as_ref()
                .unwrap()
                .inner_version();
            self.height_to_value_destroyed
                .as_mut()
                .unwrap()
                .validate_computed_version_or_reset_file(
                    base_version + height_to_value_destroyed_inner_version,
                )?;
            let height_to_adjusted_value_destroyed_inner_version = self
                .height_to_adjusted_value_destroyed
                .as_ref()
                .unwrap()
                .inner_version();
            self.height_to_adjusted_value_destroyed
                .as_mut()
                .unwrap()
                .validate_computed_version_or_reset_file(
                    base_version + height_to_adjusted_value_destroyed_inner_version,
                )?;
            let height_to_supply_in_profit_inner_version = self
                .height_to_supply_in_profit
                .as_ref()
                .unwrap()
                .inner_version();
            self.height_to_supply_in_profit
                .as_mut()
                .unwrap()
                .validate_computed_version_or_reset_file(
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
                .validate_computed_version_or_reset_file(
                    base_version + height_to_supply_in_loss_inner_version,
                )?;
            let height_to_supply_even_inner_version =
                self.height_to_supply_even.as_ref().unwrap().inner_version();
            self.height_to_supply_even
                .as_mut()
                .unwrap()
                .validate_computed_version_or_reset_file(
                    base_version + height_to_supply_even_inner_version,
                )?;
            let height_to_unrealized_profit_inner_version = self
                .height_to_unrealized_profit
                .as_ref()
                .unwrap()
                .inner_version();
            self.height_to_unrealized_profit
                .as_mut()
                .unwrap()
                .validate_computed_version_or_reset_file(
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
                .validate_computed_version_or_reset_file(
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
                .validate_computed_version_or_reset_file(
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
                .validate_computed_version_or_reset_file(
                    base_version + dateindex_to_supply_in_loss_inner_version,
                )?;
            let dateindex_to_supply_even_inner_version = self
                .dateindex_to_supply_even
                .as_ref()
                .unwrap()
                .inner_version();
            self.dateindex_to_supply_even
                .as_mut()
                .unwrap()
                .validate_computed_version_or_reset_file(
                    base_version + dateindex_to_supply_even_inner_version,
                )?;
            let dateindex_to_unrealized_profit_inner_version = self
                .dateindex_to_unrealized_profit
                .as_ref()
                .unwrap()
                .inner_version();
            self.dateindex_to_unrealized_profit
                .as_mut()
                .unwrap()
                .validate_computed_version_or_reset_file(
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
                .validate_computed_version_or_reset_file(
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
                .validate_computed_version_or_reset_file(
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
                .validate_computed_version_or_reset_file(
                    base_version + height_to_max_price_paid_inner_version,
                )?;
        }

        Ok(())
    }

    pub fn forced_pushed_at(&mut self, height: Height, exit: &Exit) -> Result<()> {
        if self.starting_height > height {
            return Ok(());
        }

        self.height_to_supply
            .forced_push_at(height, self.state.supply.value, exit)?;

        self.height_to_utxo_count.forced_push_at(
            height,
            StoredUsize::from(self.state.supply.utxos),
            exit,
        )?;

        if let Some(height_to_realized_cap) = self.height_to_realized_cap.as_mut() {
            let realized = self.state.realized.as_ref().unwrap_or_else(|| {
                dbg!((&self.state.realized, &self.state.supply));
                panic!();
            });

            height_to_realized_cap.forced_push_at(height, realized.cap, exit)?;

            self.height_to_realized_profit
                .as_mut()
                .unwrap()
                .forced_push_at(height, realized.profit, exit)?;
            self.height_to_realized_loss
                .as_mut()
                .unwrap()
                .forced_push_at(height, realized.loss, exit)?;
            self.height_to_value_created
                .as_mut()
                .unwrap()
                .forced_push_at(height, realized.value_created, exit)?;
            self.height_to_adjusted_value_created
                .as_mut()
                .unwrap()
                .forced_push_at(height, realized.adj_value_created, exit)?;
            self.height_to_value_destroyed
                .as_mut()
                .unwrap()
                .forced_push_at(height, realized.value_destroyed, exit)?;
            self.height_to_adjusted_value_destroyed
                .as_mut()
                .unwrap()
                .forced_push_at(height, realized.adj_value_destroyed, exit)?;
        }
        Ok(())
    }

    pub fn compute_then_force_push_unrealized_states(
        &mut self,
        height: Height,
        height_price: Option<Dollars>,
        dateindex: Option<DateIndex>,
        date_price: Option<Option<Dollars>>,
        exit: &Exit,
    ) -> Result<()> {
        if let Some(height_price) = height_price {
            self.height_to_min_price_paid
                .as_mut()
                .unwrap()
                .forced_push_at(
                    height,
                    self.state
                        .price_to_amount
                        .puts_first_key_value()
                        .map(|(&dollars, _)| dollars)
                        .unwrap_or(Dollars::NAN),
                    exit,
                )?;
            self.height_to_max_price_paid
                .as_mut()
                .unwrap()
                .forced_push_at(
                    height,
                    self.state
                        .price_to_amount
                        .puts_last_key_value()
                        .map(|(&dollars, _)| dollars)
                        .unwrap_or(Dollars::NAN),
                    exit,
                )?;

            let (height_unrealized_state, date_unrealized_state) = self
                .state
                .compute_unrealized_states(height_price, date_price.unwrap());

            self.height_to_supply_even
                .as_mut()
                .unwrap()
                .forced_push_at(height, height_unrealized_state.supply_even, exit)?;
            self.height_to_supply_in_profit
                .as_mut()
                .unwrap()
                .forced_push_at(height, height_unrealized_state.supply_in_profit, exit)?;
            self.height_to_supply_in_loss
                .as_mut()
                .unwrap()
                .forced_push_at(height, height_unrealized_state.supply_in_loss, exit)?;
            self.height_to_unrealized_profit
                .as_mut()
                .unwrap()
                .forced_push_at(height, height_unrealized_state.unrealized_profit, exit)?;
            self.height_to_unrealized_loss
                .as_mut()
                .unwrap()
                .forced_push_at(height, height_unrealized_state.unrealized_loss, exit)?;

            if let Some(date_unrealized_state) = date_unrealized_state {
                let dateindex = dateindex.unwrap();

                self.dateindex_to_supply_even
                    .as_mut()
                    .unwrap()
                    .forced_push_at(dateindex, date_unrealized_state.supply_even, exit)?;
                self.dateindex_to_supply_in_profit
                    .as_mut()
                    .unwrap()
                    .forced_push_at(dateindex, date_unrealized_state.supply_in_profit, exit)?;
                self.dateindex_to_supply_in_loss
                    .as_mut()
                    .unwrap()
                    .forced_push_at(dateindex, date_unrealized_state.supply_in_loss, exit)?;
                self.dateindex_to_unrealized_profit
                    .as_mut()
                    .unwrap()
                    .forced_push_at(dateindex, date_unrealized_state.unrealized_profit, exit)?;
                self.dateindex_to_unrealized_loss
                    .as_mut()
                    .unwrap()
                    .forced_push_at(dateindex, date_unrealized_state.unrealized_loss, exit)?;
            }
        }

        Ok(())
    }

    pub fn safe_flush_stateful_vecs(&mut self, height: Height, exit: &Exit) -> Result<()> {
        self.height_to_supply.safe_flush(exit)?;

        self.height_to_utxo_count.safe_flush(exit)?;

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
            self.height_to_adjusted_value_created
                .as_mut()
                .unwrap()
                .safe_flush(exit)?;
            self.height_to_value_destroyed
                .as_mut()
                .unwrap()
                .safe_flush(exit)?;
            self.height_to_adjusted_value_destroyed
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
            self.height_to_supply_even
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
            self.dateindex_to_supply_even
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
        }

        self.state.commit(height)?;

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
            self.height_to_adjusted_value_created
                .as_mut()
                .unwrap()
                .compute_sum_of_others(
                    starting_indexes.height,
                    others
                        .iter()
                        .map(|v| v.height_to_adjusted_value_created.as_ref().unwrap())
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
            self.height_to_adjusted_value_destroyed
                .as_mut()
                .unwrap()
                .compute_sum_of_others(
                    starting_indexes.height,
                    others
                        .iter()
                        .map(|v| v.height_to_adjusted_value_destroyed.as_ref().unwrap())
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
            self.height_to_supply_even
                .as_mut()
                .unwrap()
                .compute_sum_of_others(
                    starting_indexes.height,
                    others
                        .iter()
                        .map(|v| v.height_to_supply_even.as_ref().unwrap())
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
            self.dateindex_to_supply_even
                .as_mut()
                .unwrap()
                .compute_sum_of_others(
                    starting_indexes.dateindex,
                    others
                        .iter()
                        .map(|v| v.dateindex_to_supply_even.as_ref().unwrap())
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
        }

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn compute_rest_part1(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        fetched: Option<&fetched::Vecs>,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> color_eyre::Result<()> {
        self.indexes_to_supply.compute_rest(
            indexer,
            indexes,
            fetched,
            starting_indexes,
            exit,
            Some(&self.height_to_supply),
        )?;

        self.indexes_to_utxo_count.compute_rest(
            indexes,
            starting_indexes,
            exit,
            Some(&self.height_to_utxo_count),
        )?;

        self.indexes_to_halved_supply.compute_all(
            indexer,
            indexes,
            fetched,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                v.compute_transform(
                    starting_indexes.height,
                    &self.height_to_supply,
                    |(h, sats, ..)| (h, sats / 2),
                    exit,
                )
            },
        )?;

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn compute_rest_part2(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        fetched: Option<&fetched::Vecs>,
        starting_indexes: &Indexes,
        market: &market::Vecs,
        height_to_supply: &impl AnyIterableVec<Height, Sats>,
        dateindex_to_supply: &impl AnyIterableVec<DateIndex, Sats>,
        height_to_realized_cap: Option<&impl AnyIterableVec<Height, Dollars>>,
        exit: &Exit,
    ) -> color_eyre::Result<()> {
        if let Some(v) = self
            .indexes_to_supply_relative_to_circulating_supply
            .as_mut()
        {
            v.compute_all(
                indexer,
                indexes,
                starting_indexes,
                exit,
                |v, _, _, starting_indexes, exit| {
                    v.compute_percentage(
                        starting_indexes.height,
                        &self.height_to_supply,
                        height_to_supply,
                        exit,
                    )
                },
            )?;
        }

        if let Some(indexes_to_realized_cap) = self.indexes_to_realized_cap.as_mut() {
            indexes_to_realized_cap.compute_rest(
                indexes,
                starting_indexes,
                exit,
                Some(self.height_to_realized_cap.as_ref().unwrap()),
            )?;

            self.indexes_to_realized_price
                .as_mut()
                .unwrap()
                .compute_all(
                    indexer,
                    indexes,
                    starting_indexes,
                    exit,
                    |vec, _, _, starting_indexes, exit| {
                        vec.compute_divide(
                            starting_indexes.height,
                            self.height_to_realized_cap.as_ref().unwrap(),
                            self.indexes_to_supply.bitcoin.height.as_ref().unwrap(),
                            exit,
                        )
                    },
                )?;

            self.indexes_to_realized_price_extra
                .as_mut()
                .unwrap()
                .compute_rest(
                    indexer,
                    indexes,
                    fetched.as_ref().unwrap(),
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

            self.indexes_to_negative_realized_loss
                .as_mut()
                .unwrap()
                .compute_all(
                    indexer,
                    indexes,
                    starting_indexes,
                    exit,
                    |vec, _, _, starting_indexes, exit| {
                        vec.compute_transform(
                            starting_indexes.height,
                            self.height_to_realized_loss.as_ref().unwrap(),
                            |(i, v, ..)| (i, v * -1_i64),
                            exit,
                        )
                    },
                )?;

            self.indexes_to_value_created
                .as_mut()
                .unwrap()
                .compute_rest(
                    indexes,
                    starting_indexes,
                    exit,
                    Some(self.height_to_value_created.as_ref().unwrap()),
                )?;

            self.indexes_to_adjusted_value_created
                .as_mut()
                .unwrap()
                .compute_rest(
                    indexes,
                    starting_indexes,
                    exit,
                    Some(self.height_to_adjusted_value_created.as_ref().unwrap()),
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

            self.indexes_to_adjusted_value_destroyed
                .as_mut()
                .unwrap()
                .compute_rest(
                    indexes,
                    starting_indexes,
                    exit,
                    Some(self.height_to_adjusted_value_destroyed.as_ref().unwrap()),
                )?;

            self.dateindex_to_realized_cap_30d_change
                .as_mut()
                .unwrap()
                .compute_change(
                    starting_indexes.dateindex,
                    self.indexes_to_realized_cap
                        .as_ref()
                        .unwrap()
                        .dateindex
                        .unwrap_last(),
                    30,
                    exit,
                )?;

            self.indexes_to_net_realized_profit_and_loss
                .as_mut()
                .unwrap()
                .compute_all(
                    indexer,
                    indexes,
                    starting_indexes,
                    exit,
                    |vec, _, _, starting_indexes, exit| {
                        vec.compute_subtract(
                            starting_indexes.height,
                            self.height_to_realized_profit.as_ref().unwrap(),
                            self.height_to_realized_loss.as_ref().unwrap(),
                            exit,
                        )
                    },
                )?;

            self.indexes_to_realized_value
                .as_mut()
                .unwrap()
                .compute_all(
                    indexer,
                    indexes,
                    starting_indexes,
                    exit,
                    |vec, _, _, starting_indexes, exit| {
                        vec.compute_add(
                            starting_indexes.height,
                            self.height_to_realized_profit.as_ref().unwrap(),
                            self.height_to_realized_loss.as_ref().unwrap(),
                            exit,
                        )
                    },
                )?;

            self.dateindex_to_spent_output_profit_ratio
                .as_mut()
                .unwrap()
                .compute_divide(
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

            self.dateindex_to_adjusted_spent_output_profit_ratio
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

            self.indexes_to_supply_in_profit
                .as_mut()
                .unwrap()
                .compute_rest(
                    indexer,
                    indexes,
                    fetched,
                    starting_indexes,
                    exit,
                    Some(self.dateindex_to_supply_in_profit.as_ref().unwrap()),
                )?;
            self.indexes_to_supply_in_loss
                .as_mut()
                .unwrap()
                .compute_rest(
                    indexer,
                    indexes,
                    fetched,
                    starting_indexes,
                    exit,
                    Some(self.dateindex_to_supply_in_loss.as_ref().unwrap()),
                )?;
            self.indexes_to_supply_even.as_mut().unwrap().compute_rest(
                indexer,
                indexes,
                fetched,
                starting_indexes,
                exit,
                Some(self.dateindex_to_supply_even.as_ref().unwrap()),
            )?;
            self.indexes_to_unrealized_profit
                .as_mut()
                .unwrap()
                .compute_rest(
                    indexes,
                    starting_indexes,
                    exit,
                    Some(self.dateindex_to_unrealized_profit.as_ref().unwrap()),
                )?;
            self.indexes_to_unrealized_loss
                .as_mut()
                .unwrap()
                .compute_rest(
                    indexes,
                    starting_indexes,
                    exit,
                    Some(self.dateindex_to_unrealized_loss.as_ref().unwrap()),
                )?;

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

            self.height_to_negative_unrealized_loss
                .as_mut()
                .unwrap()
                .compute_transform(
                    starting_indexes.height,
                    self.height_to_unrealized_loss.as_ref().unwrap(),
                    |(h, v, ..)| (h, v * -1_i64),
                    exit,
                )?;
            self.indexes_to_negative_unrealized_loss
                .as_mut()
                .unwrap()
                .compute_all(
                    indexer,
                    indexes,
                    starting_indexes,
                    exit,
                    |v, _, _, starting_indexes, exit| {
                        v.compute_transform(
                            starting_indexes.dateindex,
                            self.dateindex_to_unrealized_loss.as_ref().unwrap(),
                            |(h, v, ..)| (h, v * -1_i64),
                            exit,
                        )
                    },
                )?;
            self.height_to_net_unrealized_profit_and_loss
                .as_mut()
                .unwrap()
                .compute_subtract(
                    starting_indexes.height,
                    self.height_to_unrealized_profit.as_ref().unwrap(),
                    self.height_to_unrealized_loss.as_ref().unwrap(),
                    exit,
                )?;

            self.indexes_to_net_unrealized_profit_and_loss
                .as_mut()
                .unwrap()
                .compute_all(
                    indexer,
                    indexes,
                    starting_indexes,
                    exit,
                    |vec, _, _, starting_indexes, exit| {
                        vec.compute_subtract(
                            starting_indexes.dateindex,
                            self.dateindex_to_unrealized_profit.as_ref().unwrap(),
                            self.dateindex_to_unrealized_loss.as_ref().unwrap(),
                            exit,
                        )
                    },
                )?;
            self.height_to_net_unrealized_profit_and_loss_relative_to_market_cap
                .as_mut()
                .unwrap()
                .compute_percentage(
                    starting_indexes.height,
                    self.height_to_net_unrealized_profit_and_loss
                        .as_ref()
                        .unwrap(),
                    &market.height_to_marketcap,
                    exit,
                )?;
            self.indexes_to_net_unrealized_profit_and_loss_relative_to_market_cap
                .as_mut()
                .unwrap()
                .compute_all(
                    indexer,
                    indexes,
                    starting_indexes,
                    exit,
                    |vec, _, _, starting_indexes, exit| {
                        vec.compute_percentage(
                            starting_indexes.dateindex,
                            self.indexes_to_net_unrealized_profit_and_loss
                                .as_ref()
                                .unwrap()
                                .dateindex
                                .as_ref()
                                .unwrap(),
                            market.indexes_to_marketcap.dateindex.as_ref().unwrap(),
                            exit,
                        )
                    },
                )?;

            self.indexes_to_net_realized_profit_and_loss_relative_to_realized_cap
                .as_mut()
                .unwrap()
                .compute_all(
                    indexer,
                    indexes,
                    starting_indexes,
                    exit,
                    |vec, _, _, starting_indexes, exit| {
                        vec.compute_percentage(
                            starting_indexes.height,
                            self.indexes_to_net_realized_profit_and_loss
                                .as_ref()
                                .unwrap()
                                .height
                                .as_ref()
                                .unwrap(),
                            *height_to_realized_cap.as_ref().unwrap(),
                            exit,
                        )
                    },
                )?;

            self.height_to_supply_even_value
                .as_mut()
                .unwrap()
                .compute_rest(
                    fetched,
                    starting_indexes,
                    exit,
                    Some(self.height_to_supply_even.as_ref().unwrap()),
                )?;
            self.height_to_supply_in_loss_value
                .as_mut()
                .unwrap()
                .compute_rest(
                    fetched,
                    starting_indexes,
                    exit,
                    Some(self.height_to_supply_in_loss.as_ref().unwrap()),
                )?;
            self.height_to_supply_in_profit_value
                .as_mut()
                .unwrap()
                .compute_rest(
                    fetched,
                    starting_indexes,
                    exit,
                    Some(self.height_to_supply_in_profit.as_ref().unwrap()),
                )?;
            self.height_to_supply_even_relative_to_own_supply
                .as_mut()
                .unwrap()
                .compute_percentage(
                    starting_indexes.height,
                    self.height_to_supply_even.as_ref().unwrap(),
                    &self.height_to_supply,
                    exit,
                )?;
            self.height_to_supply_in_loss_relative_to_own_supply
                .as_mut()
                .unwrap()
                .compute_percentage(
                    starting_indexes.height,
                    self.height_to_supply_in_loss.as_ref().unwrap(),
                    &self.height_to_supply,
                    exit,
                )?;
            self.height_to_supply_in_profit_relative_to_own_supply
                .as_mut()
                .unwrap()
                .compute_percentage(
                    starting_indexes.height,
                    self.height_to_supply_even.as_ref().unwrap(),
                    &self.height_to_supply,
                    exit,
                )?;
            self.indexes_to_supply_even_relative_to_own_supply
                .as_mut()
                .unwrap()
                .compute_all(
                    indexer,
                    indexes,
                    starting_indexes,
                    exit,
                    |v, _, _, starting_indexes, exit| {
                        v.compute_percentage(
                            starting_indexes.dateindex,
                            self.dateindex_to_supply_even.as_ref().unwrap(),
                            self.indexes_to_supply.sats.dateindex.unwrap_last(),
                            exit,
                        )
                    },
                )?;
            self.indexes_to_supply_in_loss_relative_to_own_supply
                .as_mut()
                .unwrap()
                .compute_all(
                    indexer,
                    indexes,
                    starting_indexes,
                    exit,
                    |v, _, _, starting_indexes, exit| {
                        v.compute_percentage(
                            starting_indexes.dateindex,
                            self.dateindex_to_supply_even.as_ref().unwrap(),
                            self.indexes_to_supply.sats.dateindex.unwrap_last(),
                            exit,
                        )
                    },
                )?;
            self.indexes_to_supply_in_profit_relative_to_own_supply
                .as_mut()
                .unwrap()
                .compute_all(
                    indexer,
                    indexes,
                    starting_indexes,
                    exit,
                    |v, _, _, starting_indexes, exit| {
                        v.compute_percentage(
                            starting_indexes.dateindex,
                            self.dateindex_to_supply_even.as_ref().unwrap(),
                            self.indexes_to_supply.sats.dateindex.unwrap_last(),
                            exit,
                        )
                    },
                )?;

            if let Some(height_to_supply_even_relative_to_circulating_supply) = self
                .height_to_supply_even_relative_to_circulating_supply
                .as_mut()
            {
                height_to_supply_even_relative_to_circulating_supply.compute_percentage(
                    starting_indexes.height,
                    self.height_to_supply_even.as_ref().unwrap(),
                    height_to_supply,
                    exit,
                )?;
                self.height_to_supply_in_loss_relative_to_circulating_supply
                    .as_mut()
                    .unwrap()
                    .compute_percentage(
                        starting_indexes.height,
                        self.height_to_supply_in_loss.as_ref().unwrap(),
                        height_to_supply,
                        exit,
                    )?;
                self.height_to_supply_in_profit_relative_to_circulating_supply
                    .as_mut()
                    .unwrap()
                    .compute_percentage(
                        starting_indexes.height,
                        self.height_to_supply_in_profit.as_ref().unwrap(),
                        height_to_supply,
                        exit,
                    )?;
                self.indexes_to_supply_even_relative_to_circulating_supply
                    .as_mut()
                    .unwrap()
                    .compute_all(
                        indexer,
                        indexes,
                        starting_indexes,
                        exit,
                        |v, _, _, starting_indexes, exit| {
                            v.compute_percentage(
                                starting_indexes.dateindex,
                                self.dateindex_to_supply_even.as_ref().unwrap(),
                                dateindex_to_supply,
                                exit,
                            )
                        },
                    )?;
                self.indexes_to_supply_in_loss_relative_to_circulating_supply
                    .as_mut()
                    .unwrap()
                    .compute_all(
                        indexer,
                        indexes,
                        starting_indexes,
                        exit,
                        |v, _, _, starting_indexes, exit| {
                            v.compute_percentage(
                                starting_indexes.dateindex,
                                self.dateindex_to_supply_in_loss.as_ref().unwrap(),
                                dateindex_to_supply,
                                exit,
                            )
                        },
                    )?;
                self.indexes_to_supply_in_profit_relative_to_circulating_supply
                    .as_mut()
                    .unwrap()
                    .compute_all(
                        indexer,
                        indexes,
                        starting_indexes,
                        exit,
                        |v, _, _, starting_indexes, exit| {
                            v.compute_percentage(
                                starting_indexes.dateindex,
                                self.dateindex_to_supply_in_profit.as_ref().unwrap(),
                                dateindex_to_supply,
                                exit,
                            )
                        },
                    )?;
            }
        }

        Ok(())
    }

    pub fn vecs(&self) -> Vec<&dyn AnyCollectableVec> {
        [
            vec![
                &self.height_to_supply as &dyn AnyCollectableVec,
                &self.height_to_utxo_count,
            ],
            self.height_to_realized_cap
                .as_ref()
                .map_or(vec![], |v| vec![v as &dyn AnyCollectableVec]),
            self.indexes_to_supply.vecs(),
            self.indexes_to_utxo_count.vecs(),
            self.indexes_to_realized_cap
                .as_ref()
                .map_or(vec![], |v| v.vecs()),
            self.indexes_to_realized_price
                .as_ref()
                .map_or(vec![], |v| v.vecs()),
            self.indexes_to_realized_value
                .as_ref()
                .map_or(vec![], |v| v.vecs()),
            self.indexes_to_realized_price_extra
                .as_ref()
                .map_or(vec![], |v| v.vecs()),
            self.height_to_realized_profit
                .as_ref()
                .map_or(vec![], |v| vec![v as &dyn AnyCollectableVec]),
            self.indexes_to_realized_profit
                .as_ref()
                .map_or(vec![], |v| v.vecs()),
            self.height_to_realized_loss
                .as_ref()
                .map_or(vec![], |v| vec![v as &dyn AnyCollectableVec]),
            self.indexes_to_realized_loss
                .as_ref()
                .map_or(vec![], |v| v.vecs()),
            self.indexes_to_negative_realized_loss
                .as_ref()
                .map_or(vec![], |v| v.vecs()),
            self.height_to_value_created
                .as_ref()
                .map_or(vec![], |v| vec![v as &dyn AnyCollectableVec]),
            self.indexes_to_value_created
                .as_ref()
                .map_or(vec![], |v| v.vecs()),
            self.height_to_adjusted_value_created
                .as_ref()
                .map_or(vec![], |v| vec![v as &dyn AnyCollectableVec]),
            self.indexes_to_adjusted_value_created
                .as_ref()
                .map_or(vec![], |v| v.vecs()),
            self.height_to_value_destroyed
                .as_ref()
                .map_or(vec![], |v| vec![v as &dyn AnyCollectableVec]),
            self.dateindex_to_spent_output_profit_ratio
                .as_ref()
                .map_or(vec![], |v| vec![v as &dyn AnyCollectableVec]),
            self.dateindex_to_adjusted_spent_output_profit_ratio
                .as_ref()
                .map_or(vec![], |v| vec![v as &dyn AnyCollectableVec]),
            self.indexes_to_value_destroyed
                .as_ref()
                .map_or(vec![], |v| v.vecs()),
            self.height_to_adjusted_value_destroyed
                .as_ref()
                .map_or(vec![], |v| vec![v as &dyn AnyCollectableVec]),
            self.indexes_to_adjusted_value_destroyed
                .as_ref()
                .map_or(vec![], |v| v.vecs()),
            self.dateindex_to_realized_cap_30d_change
                .as_ref()
                .map_or(vec![], |v| vec![v]),
            self.indexes_to_net_realized_profit_and_loss
                .as_ref()
                .map_or(vec![], |v| v.vecs()),
            self.dateindex_to_sell_side_risk_ratio
                .as_ref()
                .map_or(vec![], |v| vec![v]),
            self.height_to_supply_in_profit
                .as_ref()
                .map_or(vec![], |v| vec![v]),
            self.height_to_supply_in_loss
                .as_ref()
                .map_or(vec![], |v| vec![v]),
            self.height_to_supply_even
                .as_ref()
                .map_or(vec![], |v| vec![v]),
            self.height_to_unrealized_profit
                .as_ref()
                .map_or(vec![], |v| vec![v]),
            self.height_to_unrealized_loss
                .as_ref()
                .map_or(vec![], |v| vec![v]),
            self.dateindex_to_supply_in_profit
                .as_ref()
                .map_or(vec![], |v| vec![v]),
            self.dateindex_to_supply_in_loss
                .as_ref()
                .map_or(vec![], |v| vec![v]),
            self.dateindex_to_supply_even
                .as_ref()
                .map_or(vec![], |v| vec![v]),
            self.dateindex_to_unrealized_profit
                .as_ref()
                .map_or(vec![], |v| vec![v]),
            self.dateindex_to_unrealized_loss
                .as_ref()
                .map_or(vec![], |v| vec![v]),
            self.height_to_min_price_paid
                .as_ref()
                .map_or(vec![], |v| vec![v]),
            self.height_to_max_price_paid
                .as_ref()
                .map_or(vec![], |v| vec![v]),
            self.indexes_to_supply_in_profit
                .as_ref()
                .map_or(vec![], |v| v.vecs()),
            self.indexes_to_supply_in_loss
                .as_ref()
                .map_or(vec![], |v| v.vecs()),
            self.indexes_to_supply_even
                .as_ref()
                .map_or(vec![], |v| v.vecs()),
            self.indexes_to_unrealized_profit
                .as_ref()
                .map_or(vec![], |v| v.vecs()),
            self.indexes_to_unrealized_loss
                .as_ref()
                .map_or(vec![], |v| v.vecs()),
            self.indexes_to_min_price_paid
                .as_ref()
                .map_or(vec![], |v| v.vecs()),
            self.indexes_to_max_price_paid
                .as_ref()
                .map_or(vec![], |v| v.vecs()),
            self.indexes_to_halved_supply.vecs(),
            self.height_to_negative_unrealized_loss
                .as_ref()
                .map_or(vec![], |v| vec![v]),
            self.indexes_to_negative_unrealized_loss
                .as_ref()
                .map_or(vec![], |v| v.vecs()),
            self.height_to_net_unrealized_profit_and_loss
                .as_ref()
                .map_or(vec![], |v| vec![v]),
            self.indexes_to_net_unrealized_profit_and_loss
                .as_ref()
                .map_or(vec![], |v| v.vecs()),
            self.height_to_net_unrealized_profit_and_loss_relative_to_market_cap
                .as_ref()
                .map_or(vec![], |v| vec![v]),
            self.indexes_to_net_unrealized_profit_and_loss_relative_to_market_cap
                .as_ref()
                .map_or(vec![], |v| v.vecs()),
            self.indexes_to_net_realized_profit_and_loss_relative_to_realized_cap
                .as_ref()
                .map_or(vec![], |v| v.vecs()),
            self.height_to_supply_even_value
                .as_ref()
                .map_or(vec![], |v| v.vecs()),
            self.height_to_supply_in_loss_value
                .as_ref()
                .map_or(vec![], |v| v.vecs()),
            self.height_to_supply_in_profit_value
                .as_ref()
                .map_or(vec![], |v| v.vecs()),
            self.height_to_supply_even_relative_to_own_supply
                .as_ref()
                .map_or(vec![], |v| vec![v]),
            self.height_to_supply_in_loss_relative_to_own_supply
                .as_ref()
                .map_or(vec![], |v| vec![v]),
            self.height_to_supply_in_profit_relative_to_own_supply
                .as_ref()
                .map_or(vec![], |v| vec![v]),
            self.indexes_to_supply_even_relative_to_own_supply
                .as_ref()
                .map_or(vec![], |v| v.vecs()),
            self.indexes_to_supply_in_loss_relative_to_own_supply
                .as_ref()
                .map_or(vec![], |v| v.vecs()),
            self.indexes_to_supply_in_profit_relative_to_own_supply
                .as_ref()
                .map_or(vec![], |v| v.vecs()),
            self.indexes_to_supply_relative_to_circulating_supply
                .as_ref()
                .map_or(vec![], |v| v.vecs()),
            self.height_to_supply_even_relative_to_circulating_supply
                .as_ref()
                .map_or(vec![], |v| vec![v]),
            self.height_to_supply_in_loss_relative_to_circulating_supply
                .as_ref()
                .map_or(vec![], |v| vec![v]),
            self.height_to_supply_in_profit_relative_to_circulating_supply
                .as_ref()
                .map_or(vec![], |v| vec![v]),
            self.indexes_to_supply_even_relative_to_circulating_supply
                .as_ref()
                .map_or(vec![], |v| v.vecs()),
            self.indexes_to_supply_in_loss_relative_to_circulating_supply
                .as_ref()
                .map_or(vec![], |v| v.vecs()),
            self.indexes_to_supply_in_profit_relative_to_circulating_supply
                .as_ref()
                .map_or(vec![], |v| v.vecs()),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
    }
}
