use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, Sats, StoredU64, SupplyState, Version};
use rayon::prelude::*;
use vecdb::{
    AnyStoredVec, AnyVec, EagerVec, Exit, GenericStoredVec, ImportableVec, IterableCloneableVec,
    PcoVec, TypedVecIterator,
};

use crate::{
    ComputeIndexes, indexes,
    internal::{
        DerivedComputedBlockLast, HalfClosePriceTimesSats, HalveDollars, HalveSats,
        HalveSatsToBitcoin, LazyBlockValue, LazyDerivedBlockValue, LazyValueDateLast, ValueDateLast,
    },
    price,
};

use super::ImportConfig;

/// Supply and UTXO count metrics for a cohort.
#[derive(Clone, Traversable)]
pub struct SupplyMetrics {
    pub height_to_supply: EagerVec<PcoVec<Height, Sats>>,
    pub height_to_supply_value: LazyDerivedBlockValue,
    pub indexes_to_supply: ValueDateLast,
    pub height_to_utxo_count: EagerVec<PcoVec<Height, StoredU64>>,
    pub indexes_to_utxo_count: DerivedComputedBlockLast<StoredU64>,
    pub height_to_supply_half_value: LazyBlockValue,
    pub indexes_to_supply_half: LazyValueDateLast,
}

impl SupplyMetrics {
    /// Import supply metrics from database.
    pub fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v1 = Version::ONE;
        let compute_dollars = cfg.compute_dollars();

        let height_to_supply: EagerVec<PcoVec<Height, Sats>> =
            EagerVec::forced_import(cfg.db, &cfg.name("supply"), cfg.version)?;

        let price_source = cfg
            .price
            .map(|p| p.usd.chainindexes_to_price_close.height.boxed_clone());

        let height_to_supply_value = LazyDerivedBlockValue::from_source(
            &cfg.name("supply"),
            height_to_supply.boxed_clone(),
            cfg.version,
            price_source.clone(),
        );

        let indexes_to_supply = ValueDateLast::forced_import(
            cfg.db,
            &cfg.name("supply"),
            cfg.version + v1,
            compute_dollars,
            cfg.indexes,
        )?;

        // Create lazy supply_half from supply sources
        let height_to_supply_half_value = LazyBlockValue::from_sources::<
            HalveSats,
            HalveSatsToBitcoin,
            HalfClosePriceTimesSats,
        >(
            &cfg.name("supply_half"),
            height_to_supply.boxed_clone(),
            price_source,
            cfg.version,
        );

        let indexes_to_supply_half =
            LazyValueDateLast::from_source::<HalveSats, HalveSatsToBitcoin, HalveDollars>(
                &cfg.name("supply_half"),
                &indexes_to_supply,
                cfg.version,
            );

        let height_to_utxo_count =
            EagerVec::forced_import(cfg.db, &cfg.name("utxo_count"), cfg.version)?;

        Ok(Self {
            indexes_to_utxo_count: DerivedComputedBlockLast::forced_import(
                cfg.db,
                &cfg.name("utxo_count"),
                height_to_utxo_count.boxed_clone(),
                cfg.version,
                cfg.indexes,
            )?,
            height_to_supply,
            height_to_supply_value,
            indexes_to_supply,
            height_to_utxo_count,
            height_to_supply_half_value,
            indexes_to_supply_half,
        })
    }

    /// Get minimum length across height-indexed vectors.
    pub fn min_len(&self) -> usize {
        self.height_to_supply
            .len()
            .min(self.height_to_utxo_count.len())
    }

    /// Push supply state values to height-indexed vectors.
    pub fn truncate_push(&mut self, height: Height, state: &SupplyState) -> Result<()> {
        self.height_to_supply.truncate_push(height, state.value)?;
        self.height_to_utxo_count
            .truncate_push(height, StoredU64::from(state.utxo_count))?;
        Ok(())
    }

    /// Write height-indexed vectors to disk.
    pub fn write(&mut self) -> Result<()> {
        self.height_to_supply.write()?;
        self.height_to_utxo_count.write()?;
        Ok(())
    }

    /// Returns a parallel iterator over all vecs for parallel writing.
    pub fn par_iter_mut(&mut self) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        vec![
            &mut self.height_to_supply as &mut dyn AnyStoredVec,
            &mut self.height_to_utxo_count as &mut dyn AnyStoredVec,
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
        self.height_to_supply.compute_sum_of_others(
            starting_indexes.height,
            &others
                .iter()
                .map(|v| &v.height_to_supply)
                .collect::<Vec<_>>(),
            exit,
        )?;
        self.height_to_utxo_count.compute_sum_of_others(
            starting_indexes.height,
            &others
                .iter()
                .map(|v| &v.height_to_utxo_count)
                .collect::<Vec<_>>(),
            exit,
        )?;
        Ok(())
    }

    /// First phase of computed metrics (indexes from height).
    pub fn compute_rest_part1(
        &mut self,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.indexes_to_supply
            .compute_all(price, starting_indexes, exit, |v| {
                let mut dateindex_to_height_count_iter =
                    indexes.time.dateindex_to_height_count.into_iter();
                let mut height_to_supply_iter = self.height_to_supply.into_iter();
                v.compute_transform(
                    starting_indexes.dateindex,
                    &indexes.time.dateindex_to_first_height,
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

        self.indexes_to_utxo_count.derive_from(
            indexes,
            starting_indexes,
            &self.height_to_utxo_count,
            exit,
        )?;

        Ok(())
    }
}
