use std::path::Path;

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Bitcoin, CheckedSub, Dollars, StoredF32, StoredF64, Version};
use vecdb::{Database, Exit, PAGE_SIZE, VecIteratorExtended};

use crate::grouped::ComputedVecsFromDateIndex;

use super::{
    Indexes, chain,
    grouped::{
        ComputedRatioVecsFromDateIndex, ComputedValueVecsFromHeight, ComputedVecsFromHeight,
        Source, VecBuilderOptions,
    },
    indexes, price, stateful,
};

#[derive(Clone, Traversable)]
pub struct Vecs {
    db: Database,

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
    pub indexes_to_cointime_adj_inflation_rate: ComputedVecsFromDateIndex<StoredF32>,
    pub indexes_to_cointime_adj_tx_btc_velocity: ComputedVecsFromDateIndex<StoredF64>,
    pub indexes_to_cointime_adj_tx_usd_velocity: ComputedVecsFromDateIndex<StoredF64>,
    // pub indexes_to_thermo_cap_rel_to_investor_cap: ComputedValueVecsFromHeight,
}

impl Vecs {
    pub fn forced_import(
        parent_path: &Path,
        parent_version: Version,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
    ) -> Result<Self> {
        let db = Database::open(&parent_path.join("cointime"))?;
        db.set_min_len(PAGE_SIZE * 1_000_000)?;

        let compute_dollars = price.is_some();

        let version = parent_version + Version::ZERO;

        let this = Self {
            indexes_to_coinblocks_created: ComputedVecsFromHeight::forced_import(
                &db,
                "coinblocks_created",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_sum().add_cumulative(),
            )?,
            indexes_to_coinblocks_stored: ComputedVecsFromHeight::forced_import(
                &db,
                "coinblocks_stored",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_sum().add_cumulative(),
            )?,
            indexes_to_liveliness: ComputedVecsFromHeight::forced_import(
                &db,
                "liveliness",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            indexes_to_vaultedness: ComputedVecsFromHeight::forced_import(
                &db,
                "vaultedness",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            indexes_to_activity_to_vaultedness_ratio: ComputedVecsFromHeight::forced_import(
                &db,
                "activity_to_vaultedness_ratio",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            indexes_to_vaulted_supply: ComputedValueVecsFromHeight::forced_import(
                &db,
                "vaulted_supply",
                Source::Compute,
                version + Version::ONE,
                VecBuilderOptions::default().add_last(),
                compute_dollars,
                indexes,
            )?,
            indexes_to_active_supply: ComputedValueVecsFromHeight::forced_import(
                &db,
                "active_supply",
                Source::Compute,
                version + Version::ONE,
                VecBuilderOptions::default().add_last(),
                compute_dollars,
                indexes,
            )?,
            indexes_to_thermo_cap: ComputedVecsFromHeight::forced_import(
                &db,
                "thermo_cap",
                Source::Compute,
                version + Version::ONE,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            indexes_to_investor_cap: ComputedVecsFromHeight::forced_import(
                &db,
                "investor_cap",
                Source::Compute,
                version + Version::ONE,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            indexes_to_vaulted_cap: ComputedVecsFromHeight::forced_import(
                &db,
                "vaulted_cap",
                Source::Compute,
                version + Version::ONE,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            indexes_to_active_cap: ComputedVecsFromHeight::forced_import(
                &db,
                "active_cap",
                Source::Compute,
                version + Version::ONE,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            indexes_to_vaulted_price: ComputedVecsFromHeight::forced_import(
                &db,
                "vaulted_price",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            indexes_to_vaulted_price_ratio: ComputedRatioVecsFromDateIndex::forced_import(
                &db,
                "vaulted_price",
                Source::None,
                version + Version::ZERO,
                indexes,
                true,
            )?,
            indexes_to_active_price: ComputedVecsFromHeight::forced_import(
                &db,
                "active_price",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            indexes_to_active_price_ratio: ComputedRatioVecsFromDateIndex::forced_import(
                &db,
                "active_price",
                Source::None,
                version + Version::ZERO,
                indexes,
                true,
            )?,
            indexes_to_true_market_mean: ComputedVecsFromHeight::forced_import(
                &db,
                "true_market_mean",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            indexes_to_true_market_mean_ratio: ComputedRatioVecsFromDateIndex::forced_import(
                &db,
                "true_market_mean",
                Source::None,
                version + Version::ZERO,
                indexes,
                true,
            )?,
            indexes_to_cointime_value_destroyed: ComputedVecsFromHeight::forced_import(
                &db,
                "cointime_value_destroyed",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_sum().add_cumulative(),
            )?,
            indexes_to_cointime_value_created: ComputedVecsFromHeight::forced_import(
                &db,
                "cointime_value_created",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_sum().add_cumulative(),
            )?,
            indexes_to_cointime_value_stored: ComputedVecsFromHeight::forced_import(
                &db,
                "cointime_value_stored",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_sum().add_cumulative(),
            )?,
            indexes_to_cointime_price: ComputedVecsFromHeight::forced_import(
                &db,
                "cointime_price",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            indexes_to_cointime_cap: ComputedVecsFromHeight::forced_import(
                &db,
                "cointime_cap",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            indexes_to_cointime_price_ratio: ComputedRatioVecsFromDateIndex::forced_import(
                &db,
                "cointime_price",
                Source::None,
                version + Version::ZERO,
                indexes,
                true,
            )?,
            indexes_to_cointime_adj_inflation_rate: ComputedVecsFromDateIndex::forced_import(
                &db,
                "cointime_adj_inflation_rate",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            indexes_to_cointime_adj_tx_btc_velocity: ComputedVecsFromDateIndex::forced_import(
                &db,
                "cointime_adj_tx_btc_velocity",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            indexes_to_cointime_adj_tx_usd_velocity: ComputedVecsFromDateIndex::forced_import(
                &db,
                "cointime_adj_tx_usd_velocity",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,

            db,
        };

        this.db.retain_regions(
            this.iter_any_collectable()
                .flat_map(|v| v.region_names())
                .collect(),
        )?;

        Ok(this)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn compute(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &Indexes,
        price: Option<&price::Vecs>,
        chain: &chain::Vecs,
        stateful: &stateful::Vecs,
        exit: &Exit,
    ) -> Result<()> {
        self.compute_(indexes, starting_indexes, price, chain, stateful, exit)?;
        self.db.compact()?;
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn compute_(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &Indexes,
        price: Option<&price::Vecs>,
        chain: &chain::Vecs,
        stateful: &stateful::Vecs,
        exit: &Exit,
    ) -> Result<()> {
        let circulating_supply = &stateful.utxo_cohorts.all.1.height_to_supply;

        self.indexes_to_coinblocks_created
            .compute_all(indexes, starting_indexes, exit, |vec| {
                vec.compute_transform(
                    starting_indexes.height,
                    circulating_supply,
                    |(i, v, ..)| (i, StoredF64::from(Bitcoin::from(v))),
                    exit,
                )?;
                Ok(())
            })?;

        let indexes_to_coinblocks_destroyed =
            &stateful.utxo_cohorts.all.1.indexes_to_coinblocks_destroyed;

        self.indexes_to_coinblocks_stored
            .compute_all(indexes, starting_indexes, exit, |vec| {
                let mut coinblocks_destroyed_iter = indexes_to_coinblocks_destroyed
                    .height
                    .as_ref()
                    .unwrap()
                    .into_iter();
                vec.compute_transform(
                    starting_indexes.height,
                    self.indexes_to_coinblocks_created.height.as_ref().unwrap(),
                    |(i, created, ..)| {
                        let destroyed = coinblocks_destroyed_iter.get_unwrap(i);
                        (i, created.checked_sub(destroyed).unwrap())
                    },
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_liveliness
            .compute_all(indexes, starting_indexes, exit, |vec| {
                vec.compute_divide(
                    starting_indexes.height,
                    indexes_to_coinblocks_destroyed
                        .height_extra
                        .unwrap_cumulative(),
                    self.indexes_to_coinblocks_created
                        .height_extra
                        .unwrap_cumulative(),
                    exit,
                )?;
                Ok(())
            })?;
        let liveliness = &self.indexes_to_liveliness;

        self.indexes_to_vaultedness
            .compute_all(indexes, starting_indexes, exit, |vec| {
                vec.compute_transform(
                    starting_indexes.height,
                    liveliness.height.as_ref().unwrap(),
                    |(i, v, ..)| (i, StoredF64::from(1.0).checked_sub(v).unwrap()),
                    exit,
                )?;
                Ok(())
            })?;
        let vaultedness = &self.indexes_to_vaultedness;

        self.indexes_to_activity_to_vaultedness_ratio.compute_all(
            indexes,
            starting_indexes,
            exit,
            |vec| {
                vec.compute_divide(
                    starting_indexes.height,
                    liveliness.height.as_ref().unwrap(),
                    vaultedness.height.as_ref().unwrap(),
                    exit,
                )?;
                Ok(())
            },
        )?;

        self.indexes_to_vaulted_supply.compute_all(
            indexes,
            price,
            starting_indexes,
            exit,
            |vec| {
                vec.compute_multiply(
                    starting_indexes.height,
                    circulating_supply,
                    vaultedness.height.as_ref().unwrap(),
                    exit,
                )?;
                Ok(())
            },
        )?;

        self.indexes_to_active_supply.compute_all(
            indexes,
            price,
            starting_indexes,
            exit,
            |vec| {
                vec.compute_multiply(
                    starting_indexes.height,
                    circulating_supply,
                    liveliness.height.as_ref().unwrap(),
                    exit,
                )?;
                Ok(())
            },
        )?;

        self.indexes_to_cointime_adj_inflation_rate
            .compute_all(starting_indexes, exit, |v| {
                v.compute_multiply(
                    starting_indexes.dateindex,
                    self.indexes_to_activity_to_vaultedness_ratio
                        .dateindex
                        .unwrap_last(),
                    chain.indexes_to_inflation_rate.dateindex.as_ref().unwrap(),
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_cointime_adj_tx_btc_velocity
            .compute_all(starting_indexes, exit, |v| {
                v.compute_multiply(
                    starting_indexes.dateindex,
                    self.indexes_to_activity_to_vaultedness_ratio
                        .dateindex
                        .unwrap_last(),
                    chain.indexes_to_tx_btc_velocity.dateindex.as_ref().unwrap(),
                    exit,
                )?;
                Ok(())
            })?;

        if let Some(price) = price {
            let realized_cap = stateful
                .utxo_cohorts
                .all
                .1
                .height_to_realized_cap
                .as_ref()
                .unwrap();

            let realized_price = stateful
                .utxo_cohorts
                .all
                .1
                .indexes_to_realized_price
                .as_ref()
                .unwrap()
                .height
                .as_ref()
                .unwrap();

            self.indexes_to_thermo_cap
                .compute_all(indexes, starting_indexes, exit, |vec| {
                    vec.compute_transform(
                        starting_indexes.height,
                        chain
                            .indexes_to_subsidy
                            .dollars
                            .as_ref()
                            .unwrap()
                            .height_extra
                            .unwrap_cumulative(),
                        |(i, v, ..)| (i, v),
                        exit,
                    )?;
                    Ok(())
                })?;

            self.indexes_to_investor_cap
                .compute_all(indexes, starting_indexes, exit, |vec| {
                    vec.compute_subtract(
                        starting_indexes.height,
                        realized_cap,
                        self.indexes_to_thermo_cap.height.as_ref().unwrap(),
                        exit,
                    )?;
                    Ok(())
                })?;

            self.indexes_to_vaulted_cap
                .compute_all(indexes, starting_indexes, exit, |vec| {
                    vec.compute_divide(
                        starting_indexes.height,
                        realized_cap,
                        self.indexes_to_vaultedness.height.as_ref().unwrap(),
                        exit,
                    )?;
                    Ok(())
                })?;

            self.indexes_to_active_cap
                .compute_all(indexes, starting_indexes, exit, |vec| {
                    vec.compute_multiply(
                        starting_indexes.height,
                        realized_cap,
                        self.indexes_to_liveliness.height.as_ref().unwrap(),
                        exit,
                    )?;
                    Ok(())
                })?;

            self.indexes_to_vaulted_price
                .compute_all(indexes, starting_indexes, exit, |vec| {
                    vec.compute_divide(
                        starting_indexes.height,
                        realized_price,
                        self.indexes_to_vaultedness.height.as_ref().unwrap(),
                        exit,
                    )?;
                    Ok(())
                })?;

            self.indexes_to_vaulted_price_ratio.compute_rest(
                price,
                starting_indexes,
                exit,
                Some(self.indexes_to_vaulted_price.dateindex.unwrap_last()),
            )?;

            self.indexes_to_active_price
                .compute_all(indexes, starting_indexes, exit, |vec| {
                    vec.compute_multiply(
                        starting_indexes.height,
                        realized_price,
                        self.indexes_to_liveliness.height.as_ref().unwrap(),
                        exit,
                    )?;
                    Ok(())
                })?;

            self.indexes_to_active_price_ratio.compute_rest(
                price,
                starting_indexes,
                exit,
                Some(self.indexes_to_active_price.dateindex.unwrap_last()),
            )?;

            self.indexes_to_true_market_mean.compute_all(
                indexes,
                starting_indexes,
                exit,
                |vec| {
                    vec.compute_divide(
                        starting_indexes.height,
                        self.indexes_to_investor_cap.height.as_ref().unwrap(),
                        self.indexes_to_active_supply
                            .bitcoin
                            .height
                            .as_ref()
                            .unwrap(),
                        exit,
                    )?;
                    Ok(())
                },
            )?;

            self.indexes_to_true_market_mean_ratio.compute_rest(
                price,
                starting_indexes,
                exit,
                Some(self.indexes_to_true_market_mean.dateindex.unwrap_last()),
            )?;

            self.indexes_to_cointime_value_destroyed.compute_all(
                indexes,
                starting_indexes,
                exit,
                |vec| {
                    // TODO: Another example when the callback should be applied to each index, instead of to base then merging from more granular to less
                    // The price taken won't be correct for time based indexes
                    vec.compute_multiply(
                        starting_indexes.height,
                        &price.chainindexes_to_price_close.height,
                        indexes_to_coinblocks_destroyed.height.as_ref().unwrap(),
                        exit,
                    )?;
                    Ok(())
                },
            )?;

            self.indexes_to_cointime_value_created.compute_all(
                indexes,
                starting_indexes,
                exit,
                |vec| {
                    vec.compute_multiply(
                        starting_indexes.height,
                        &price.chainindexes_to_price_close.height,
                        self.indexes_to_coinblocks_created.height.as_ref().unwrap(),
                        exit,
                    )?;
                    Ok(())
                },
            )?;

            self.indexes_to_cointime_value_stored.compute_all(
                indexes,
                starting_indexes,
                exit,
                |vec| {
                    vec.compute_multiply(
                        starting_indexes.height,
                        &price.chainindexes_to_price_close.height,
                        self.indexes_to_coinblocks_stored.height.as_ref().unwrap(),
                        exit,
                    )?;
                    Ok(())
                },
            )?;

            self.indexes_to_cointime_price
                .compute_all(indexes, starting_indexes, exit, |vec| {
                    vec.compute_divide(
                        starting_indexes.height,
                        self.indexes_to_cointime_value_destroyed
                            .height_extra
                            .unwrap_cumulative(),
                        self.indexes_to_coinblocks_stored
                            .height_extra
                            .unwrap_cumulative(),
                        exit,
                    )?;
                    Ok(())
                })?;

            self.indexes_to_cointime_cap
                .compute_all(indexes, starting_indexes, exit, |vec| {
                    vec.compute_multiply(
                        starting_indexes.height,
                        self.indexes_to_cointime_price.height.as_ref().unwrap(),
                        circulating_supply,
                        exit,
                    )?;
                    Ok(())
                })?;

            self.indexes_to_cointime_price_ratio.compute_rest(
                price,
                starting_indexes,
                exit,
                Some(self.indexes_to_cointime_price.dateindex.unwrap_last()),
            )?;

            self.indexes_to_cointime_adj_tx_usd_velocity.compute_all(
                starting_indexes,
                exit,
                |v| {
                    v.compute_multiply(
                        starting_indexes.dateindex,
                        self.indexes_to_activity_to_vaultedness_ratio
                            .dateindex
                            .unwrap_last(),
                        chain.indexes_to_tx_usd_velocity.dateindex.as_ref().unwrap(),
                        exit,
                    )?;
                    Ok(())
                },
            )?;
        }

        Ok(())
    }
}
