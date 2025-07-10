use std::path::Path;

use brk_core::{Bitcoin, CheckedSub, Dollars, StoredF64, Version};
use brk_exit::Exit;
use brk_indexer::Indexer;
use brk_vec::{AnyCollectableVec, Computation, Format, VecIterator};

use crate::vecs::{
    fetched,
    grouped::{ComputedRatioVecsFromDateIndex, ComputedValueVecsFromHeight, Source},
    stateful, transactions,
};

use super::{
    Indexes,
    grouped::{ComputedVecsFromHeight, EagerVecBuilderOptions},
    indexes,
};

const VERSION: Version = Version::ZERO;

#[derive(Clone)]
pub struct Vecs {
    pub indexes_to_coinblocks_created: ComputedVecsFromHeight<StoredF64>,
    pub indexes_to_coinblocks_stored: ComputedVecsFromHeight<StoredF64>,
    pub indexes_to_liveliness: ComputedVecsFromHeight<StoredF64>,
    pub indexes_to_vaultedness: ComputedVecsFromHeight<StoredF64>,
    pub indexes_to_activity_to_vaultedness_ratio: ComputedVecsFromHeight<StoredF64>,
    pub indexes_to_vaulted_supply: ComputedValueVecsFromHeight,
    pub indexes_to_active_supply: ComputedValueVecsFromHeight,
    pub indexes_to_thermo_cap: ComputedVecsFromHeight<Dollars>,
    pub indexes_to_investor_cap: ComputedVecsFromHeight<Dollars>,
    pub indexes_to_vaulted_cap: ComputedVecsFromHeight<Dollars>,
    pub indexes_to_active_cap: ComputedVecsFromHeight<Dollars>,
    pub indexes_to_vaulted_price: ComputedVecsFromHeight<Dollars>,
    pub indexes_to_vaulted_price_ratio: ComputedRatioVecsFromDateIndex,
    pub indexes_to_active_price: ComputedVecsFromHeight<Dollars>,
    pub indexes_to_active_price_ratio: ComputedRatioVecsFromDateIndex,
    pub indexes_to_true_market_mean: ComputedVecsFromHeight<Dollars>,
    pub indexes_to_true_market_mean_ratio: ComputedRatioVecsFromDateIndex,
    pub indexes_to_cointime_value_destroyed: ComputedVecsFromHeight<StoredF64>,
    pub indexes_to_cointime_value_created: ComputedVecsFromHeight<StoredF64>,
    pub indexes_to_cointime_value_stored: ComputedVecsFromHeight<StoredF64>,
    pub indexes_to_cointime_price: ComputedVecsFromHeight<Dollars>,
    pub indexes_to_cointime_cap: ComputedVecsFromHeight<Dollars>,
    pub indexes_to_cointime_price_ratio: ComputedRatioVecsFromDateIndex,
    // pub indexes_to_thermo_cap_relative_to_investor_cap: ComputedValueVecsFromHeight,
}

impl Vecs {
    pub fn forced_import(
        path: &Path,
        version: Version,
        computation: Computation,
        format: Format,
        fetched: Option<&fetched::Vecs>,
    ) -> color_eyre::Result<Self> {
        let compute_dollars = fetched.is_some();

        Ok(Self {
            indexes_to_coinblocks_created: ComputedVecsFromHeight::forced_import(
                path,
                "coinblocks_created",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                EagerVecBuilderOptions::default().add_sum().add_cumulative(),
            )?,
            indexes_to_coinblocks_stored: ComputedVecsFromHeight::forced_import(
                path,
                "coinblocks_stored",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                EagerVecBuilderOptions::default().add_sum().add_cumulative(),
            )?,
            indexes_to_liveliness: ComputedVecsFromHeight::forced_import(
                path,
                "liveliness",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                EagerVecBuilderOptions::default().add_last(),
            )?,
            indexes_to_vaultedness: ComputedVecsFromHeight::forced_import(
                path,
                "vaultedness",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                EagerVecBuilderOptions::default().add_last(),
            )?,
            indexes_to_activity_to_vaultedness_ratio: ComputedVecsFromHeight::forced_import(
                path,
                "activity_to_vaultedness_ratio",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                EagerVecBuilderOptions::default().add_last(),
            )?,
            indexes_to_vaulted_supply: ComputedValueVecsFromHeight::forced_import(
                path,
                "vaulted_supply",
                Source::Compute,
                version + VERSION + Version::ONE,
                format,
                computation,
                EagerVecBuilderOptions::default().add_last(),
                compute_dollars,
            )?,
            indexes_to_active_supply: ComputedValueVecsFromHeight::forced_import(
                path,
                "active_supply",
                Source::Compute,
                version + VERSION + Version::ONE,
                format,
                computation,
                EagerVecBuilderOptions::default().add_last(),
                compute_dollars,
            )?,
            indexes_to_thermo_cap: ComputedVecsFromHeight::forced_import(
                path,
                "thermo_cap",
                Source::Compute,
                version + VERSION + Version::ONE,
                format,
                computation,
                EagerVecBuilderOptions::default().add_last(),
            )?,
            indexes_to_investor_cap: ComputedVecsFromHeight::forced_import(
                path,
                "investor_cap",
                Source::Compute,
                version + VERSION + Version::ONE,
                format,
                computation,
                EagerVecBuilderOptions::default().add_last(),
            )?,
            indexes_to_vaulted_cap: ComputedVecsFromHeight::forced_import(
                path,
                "vaulted_cap",
                Source::Compute,
                version + VERSION + Version::ONE,
                format,
                computation,
                EagerVecBuilderOptions::default().add_last(),
            )?,
            indexes_to_active_cap: ComputedVecsFromHeight::forced_import(
                path,
                "active_cap",
                Source::Compute,
                version + VERSION + Version::ONE,
                format,
                computation,
                EagerVecBuilderOptions::default().add_last(),
            )?,
            indexes_to_vaulted_price: ComputedVecsFromHeight::forced_import(
                path,
                "vaulted_price",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                EagerVecBuilderOptions::default().add_last(),
            )?,
            indexes_to_vaulted_price_ratio: ComputedRatioVecsFromDateIndex::forced_import(
                path,
                "vaulted_price",
                Source::None,
                version + VERSION + Version::ZERO,
                format,
                computation,
            )?,
            indexes_to_active_price: ComputedVecsFromHeight::forced_import(
                path,
                "active_price",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                EagerVecBuilderOptions::default().add_last(),
            )?,
            indexes_to_active_price_ratio: ComputedRatioVecsFromDateIndex::forced_import(
                path,
                "active_price",
                Source::None,
                version + VERSION + Version::ZERO,
                format,
                computation,
            )?,
            indexes_to_true_market_mean: ComputedVecsFromHeight::forced_import(
                path,
                "true_market_mean",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                EagerVecBuilderOptions::default().add_last(),
            )?,
            indexes_to_true_market_mean_ratio: ComputedRatioVecsFromDateIndex::forced_import(
                path,
                "true_market_mean",
                Source::None,
                version + VERSION + Version::ZERO,
                format,
                computation,
            )?,
            indexes_to_cointime_value_destroyed: ComputedVecsFromHeight::forced_import(
                path,
                "cointime_value_destroyed",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                EagerVecBuilderOptions::default().add_sum().add_cumulative(),
            )?,
            indexes_to_cointime_value_created: ComputedVecsFromHeight::forced_import(
                path,
                "cointime_value_created",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                EagerVecBuilderOptions::default().add_sum().add_cumulative(),
            )?,
            indexes_to_cointime_value_stored: ComputedVecsFromHeight::forced_import(
                path,
                "cointime_value_stored",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                EagerVecBuilderOptions::default().add_sum().add_cumulative(),
            )?,
            indexes_to_cointime_price: ComputedVecsFromHeight::forced_import(
                path,
                "cointime_price",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                EagerVecBuilderOptions::default().add_last(),
            )?,
            indexes_to_cointime_cap: ComputedVecsFromHeight::forced_import(
                path,
                "cointime_cap",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                EagerVecBuilderOptions::default().add_last(),
            )?,
            indexes_to_cointime_price_ratio: ComputedRatioVecsFromDateIndex::forced_import(
                path,
                "cointime_price",
                Source::None,
                version + VERSION + Version::ZERO,
                format,
                computation,
            )?,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &Indexes,
        fetched: Option<&fetched::Vecs>,
        transactions: &transactions::Vecs,
        stateful: &stateful::Vecs,
        exit: &Exit,
    ) -> color_eyre::Result<()> {
        let circulating_supply = &stateful.utxo_vecs.all.1.height_to_supply;

        self.indexes_to_coinblocks_created.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, _, _, starting_indexes, exit| {
                vec.compute_transform(
                    starting_indexes.height,
                    circulating_supply,
                    |(i, v, ..)| (i, StoredF64::from(Bitcoin::from(v))),
                    exit,
                )
            },
        )?;

        let indexes_to_coinblocks_destroyed =
            &stateful.utxo_vecs.all.1.indexes_to_coinblocks_destroyed;

        self.indexes_to_coinblocks_stored.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, _, _, starting_indexes, exit| {
                let mut coinblocks_destroyed_iter = indexes_to_coinblocks_destroyed
                    .height
                    .as_ref()
                    .unwrap()
                    .into_iter();
                vec.compute_transform(
                    starting_indexes.height,
                    self.indexes_to_coinblocks_created.height.as_ref().unwrap(),
                    |(i, created, ..)| {
                        let destroyed = coinblocks_destroyed_iter.unwrap_get_inner(i);
                        (i, created.checked_sub(destroyed).unwrap())
                    },
                    exit,
                )
            },
        )?;

        self.indexes_to_liveliness.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, _, _, starting_indexes, exit| {
                vec.compute_divide(
                    starting_indexes.height,
                    indexes_to_coinblocks_destroyed
                        .height_extra
                        .unwrap_cumulative(),
                    self.indexes_to_coinblocks_created
                        .height_extra
                        .unwrap_cumulative(),
                    exit,
                )
            },
        )?;
        let liveliness = &self.indexes_to_liveliness;

        self.indexes_to_vaultedness.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, _, _, starting_indexes, exit| {
                vec.compute_transform(
                    starting_indexes.height,
                    liveliness.height.as_ref().unwrap(),
                    |(i, v, ..)| (i, StoredF64::from(1.0).checked_sub(v).unwrap()),
                    exit,
                )
            },
        )?;
        let vaultedness = &self.indexes_to_vaultedness;

        self.indexes_to_activity_to_vaultedness_ratio.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, _, _, starting_indexes, exit| {
                vec.compute_divide(
                    starting_indexes.height,
                    liveliness.height.as_ref().unwrap(),
                    vaultedness.height.as_ref().unwrap(),
                    exit,
                )
            },
        )?;

        self.indexes_to_vaulted_supply.compute_all(
            indexer,
            indexes,
            fetched,
            starting_indexes,
            exit,
            |vec, _, _, starting_indexes, exit| {
                vec.compute_multiply(
                    starting_indexes.height,
                    circulating_supply,
                    vaultedness.height.as_ref().unwrap(),
                    exit,
                )
            },
        )?;

        self.indexes_to_active_supply.compute_all(
            indexer,
            indexes,
            fetched,
            starting_indexes,
            exit,
            |vec, _, _, starting_indexes, exit| {
                vec.compute_multiply(
                    starting_indexes.height,
                    circulating_supply,
                    liveliness.height.as_ref().unwrap(),
                    exit,
                )
            },
        )?;

        if let Some(fetched) = fetched {
            let realized_cap = stateful
                .utxo_vecs
                .all
                .1
                .height_to_realized_cap
                .as_ref()
                .unwrap();

            let realized_price = stateful
                .utxo_vecs
                .all
                .1
                .indexes_to_realized_price
                .as_ref()
                .unwrap()
                .height
                .as_ref()
                .unwrap();

            self.indexes_to_thermo_cap.compute_all(
                indexer,
                indexes,
                starting_indexes,
                exit,
                |vec, _, _, starting_indexes, exit| {
                    vec.compute_transform(
                        starting_indexes.height,
                        transactions
                            .indexes_to_subsidy
                            .dollars
                            .as_ref()
                            .unwrap()
                            .height_extra
                            .unwrap_cumulative(),
                        |(i, v, ..)| (i, v),
                        exit,
                    )
                },
            )?;

            self.indexes_to_investor_cap.compute_all(
                indexer,
                indexes,
                starting_indexes,
                exit,
                |vec, _, _, starting_indexes, exit| {
                    vec.compute_subtract(
                        starting_indexes.height,
                        realized_cap,
                        self.indexes_to_thermo_cap.height.as_ref().unwrap(),
                        exit,
                    )
                },
            )?;

            self.indexes_to_vaulted_cap.compute_all(
                indexer,
                indexes,
                starting_indexes,
                exit,
                |vec, _, _, starting_indexes, exit| {
                    vec.compute_divide(
                        starting_indexes.height,
                        realized_cap,
                        self.indexes_to_vaultedness.height.as_ref().unwrap(),
                        exit,
                    )
                },
            )?;

            self.indexes_to_active_cap.compute_all(
                indexer,
                indexes,
                starting_indexes,
                exit,
                |vec, _, _, starting_indexes, exit| {
                    vec.compute_multiply(
                        starting_indexes.height,
                        realized_cap,
                        self.indexes_to_liveliness.height.as_ref().unwrap(),
                        exit,
                    )
                },
            )?;

            self.indexes_to_vaulted_price.compute_all(
                indexer,
                indexes,
                starting_indexes,
                exit,
                |vec, _, _, starting_indexes, exit| {
                    vec.compute_divide(
                        starting_indexes.height,
                        realized_price,
                        self.indexes_to_vaultedness.height.as_ref().unwrap(),
                        exit,
                    )
                },
            )?;

            self.indexes_to_vaulted_price_ratio.compute_rest(
                indexer,
                indexes,
                fetched,
                starting_indexes,
                exit,
                Some(self.indexes_to_vaulted_price.dateindex.unwrap_last()),
            )?;

            self.indexes_to_active_price.compute_all(
                indexer,
                indexes,
                starting_indexes,
                exit,
                |vec, _, _, starting_indexes, exit| {
                    vec.compute_multiply(
                        starting_indexes.height,
                        realized_price,
                        self.indexes_to_liveliness.height.as_ref().unwrap(),
                        exit,
                    )
                },
            )?;

            self.indexes_to_active_price_ratio.compute_rest(
                indexer,
                indexes,
                fetched,
                starting_indexes,
                exit,
                Some(self.indexes_to_active_price.dateindex.unwrap_last()),
            )?;

            self.indexes_to_true_market_mean.compute_all(
                indexer,
                indexes,
                starting_indexes,
                exit,
                |vec, _, _, starting_indexes, exit| {
                    vec.compute_divide(
                        starting_indexes.height,
                        self.indexes_to_investor_cap.height.as_ref().unwrap(),
                        self.indexes_to_active_supply
                            .bitcoin
                            .height
                            .as_ref()
                            .unwrap(),
                        exit,
                    )
                },
            )?;

            self.indexes_to_true_market_mean_ratio.compute_rest(
                indexer,
                indexes,
                fetched,
                starting_indexes,
                exit,
                Some(self.indexes_to_true_market_mean.dateindex.unwrap_last()),
            )?;

            self.indexes_to_cointime_value_destroyed.compute_all(
                indexer,
                indexes,
                starting_indexes,
                exit,
                |vec, _, _, starting_indexes, exit| {
                    // TODO: Another example when the callback should be applied to each index, instead of to base then merging from more granular to less
                    // The price taken won't be correct for time based indexes
                    vec.compute_multiply(
                        starting_indexes.height,
                        &fetched.chainindexes_to_close.height,
                        indexes_to_coinblocks_destroyed.height.as_ref().unwrap(),
                        exit,
                    )
                },
            )?;

            self.indexes_to_cointime_value_created.compute_all(
                indexer,
                indexes,
                starting_indexes,
                exit,
                |vec, _, _, starting_indexes, exit| {
                    vec.compute_multiply(
                        starting_indexes.height,
                        &fetched.chainindexes_to_close.height,
                        self.indexes_to_coinblocks_created.height.as_ref().unwrap(),
                        exit,
                    )
                },
            )?;

            self.indexes_to_cointime_value_stored.compute_all(
                indexer,
                indexes,
                starting_indexes,
                exit,
                |vec, _, _, starting_indexes, exit| {
                    vec.compute_multiply(
                        starting_indexes.height,
                        &fetched.chainindexes_to_close.height,
                        self.indexes_to_coinblocks_stored.height.as_ref().unwrap(),
                        exit,
                    )
                },
            )?;

            self.indexes_to_cointime_price.compute_all(
                indexer,
                indexes,
                starting_indexes,
                exit,
                |vec, _, _, starting_indexes, exit| {
                    vec.compute_divide(
                        starting_indexes.height,
                        self.indexes_to_cointime_value_destroyed
                            .height_extra
                            .unwrap_cumulative(),
                        self.indexes_to_coinblocks_stored
                            .height_extra
                            .unwrap_cumulative(),
                        exit,
                    )
                },
            )?;

            self.indexes_to_cointime_cap.compute_all(
                indexer,
                indexes,
                starting_indexes,
                exit,
                |vec, _, _, starting_indexes, exit| {
                    vec.compute_multiply(
                        starting_indexes.height,
                        self.indexes_to_cointime_price.height.as_ref().unwrap(),
                        circulating_supply,
                        exit,
                    )
                },
            )?;

            self.indexes_to_cointime_price_ratio.compute_rest(
                indexer,
                indexes,
                fetched,
                starting_indexes,
                exit,
                Some(self.indexes_to_cointime_price.dateindex.unwrap_last()),
            )?;
        }

        Ok(())
    }

    pub fn vecs(&self) -> Vec<&dyn AnyCollectableVec> {
        [
            self.indexes_to_coinblocks_created.vecs(),
            self.indexes_to_coinblocks_stored.vecs(),
            self.indexes_to_liveliness.vecs(),
            self.indexes_to_vaultedness.vecs(),
            self.indexes_to_activity_to_vaultedness_ratio.vecs(),
            self.indexes_to_vaulted_supply.vecs(),
            self.indexes_to_active_supply.vecs(),
            self.indexes_to_thermo_cap.vecs(),
            self.indexes_to_investor_cap.vecs(),
            self.indexes_to_vaulted_cap.vecs(),
            self.indexes_to_active_cap.vecs(),
            self.indexes_to_vaulted_price.vecs(),
            self.indexes_to_vaulted_price_ratio.vecs(),
            self.indexes_to_active_price.vecs(),
            self.indexes_to_active_price_ratio.vecs(),
            self.indexes_to_true_market_mean.vecs(),
            self.indexes_to_true_market_mean_ratio.vecs(),
            self.indexes_to_cointime_price.vecs(),
            self.indexes_to_cointime_cap.vecs(),
            self.indexes_to_cointime_price_ratio.vecs(),
            self.indexes_to_cointime_value_destroyed.vecs(),
            self.indexes_to_cointime_value_created.vecs(),
            self.indexes_to_cointime_value_stored.vecs(),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
    }
}
