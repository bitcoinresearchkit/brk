use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, Sats, StoredU64, SupplyState, Version};
use rayon::prelude::*;
use vecdb::{
    AnyStoredVec, AnyVec, EagerVec, Exit, GenericStoredVec, ImportableVec, IterableCloneableVec,
    PcoVec, TypedVecIterator,
};

use crate::{
    Indexes,
    grouped::{
        ComputedHeightValueVecs, ComputedValueVecsFromDateIndex, ComputedVecsFromHeight,
        HalfClosePriceTimesSats, HalveDollars, HalveSats, HalveSatsToBitcoin, LazyHeightValueVecs,
        LazyValueVecsFromDateIndex, Source, VecBuilderOptions,
    },
    indexes, price,
};

use super::ImportConfig;

/// Supply and UTXO count metrics for a cohort.
#[derive(Clone, Traversable)]
pub struct SupplyMetrics {
    /// Total supply at each height
    pub height_to_supply: EagerVec<PcoVec<Height, Sats>>,

    /// Supply value in BTC and USD (computed from height_to_supply)
    pub height_to_supply_value: ComputedHeightValueVecs,

    /// Supply indexed by date
    pub indexes_to_supply: ComputedValueVecsFromDateIndex,

    /// UTXO count at each height
    pub height_to_utxo_count: EagerVec<PcoVec<Height, StoredU64>>,

    /// UTXO count indexed by various dimensions
    pub indexes_to_utxo_count: ComputedVecsFromHeight<StoredU64>,

    /// Half of supply value (used for computing median) - lazy from supply_value
    pub height_to_supply_half_value: LazyHeightValueVecs,

    /// Half of supply indexed by date - lazy from indexes_to_supply
    pub indexes_to_supply_half: LazyValueVecsFromDateIndex,
}

impl SupplyMetrics {
    /// Import supply metrics from database.
    pub fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v0 = Version::ZERO;
        let v1 = Version::ONE;
        let compute_dollars = cfg.compute_dollars();
        let last = VecBuilderOptions::default().add_last();

        let height_to_supply: EagerVec<PcoVec<Height, Sats>> =
            EagerVec::forced_import(cfg.db, &cfg.name("supply"), cfg.version + v0)?;

        let price_source = cfg
            .price
            .map(|p| p.chainindexes_to_price_close.height.boxed_clone());

        let height_to_supply_value = ComputedHeightValueVecs::forced_import(
            cfg.db,
            &cfg.name("supply"),
            Source::Vec(height_to_supply.boxed_clone()),
            cfg.version + v0,
            price_source.clone(),
        )?;

        let indexes_to_supply = ComputedValueVecsFromDateIndex::forced_import(
            cfg.db,
            &cfg.name("supply"),
            Source::Compute,
            cfg.version + v1,
            last,
            compute_dollars,
            cfg.indexes,
        )?;

        // Create lazy supply_half from supply sources
        let height_to_supply_half_value = LazyHeightValueVecs::from_sources::<
            HalveSats,
            HalveSatsToBitcoin,
            HalfClosePriceTimesSats,
        >(
            &cfg.name("supply_half"),
            height_to_supply.boxed_clone(),
            price_source,
            cfg.version + v0,
        );

        let indexes_to_supply_half =
            LazyValueVecsFromDateIndex::from_source::<HalveSats, HalveSatsToBitcoin, HalveDollars>(
                &cfg.name("supply_half"),
                &indexes_to_supply,
                cfg.version + v0,
            );

        let height_to_utxo_count =
            EagerVec::forced_import(cfg.db, &cfg.name("utxo_count"), cfg.version + v0)?;

        Ok(Self {
            indexes_to_utxo_count: ComputedVecsFromHeight::forced_import(
                cfg.db,
                &cfg.name("utxo_count"),
                Source::Vec(height_to_utxo_count.boxed_clone()),
                cfg.version + v0,
                cfg.indexes,
                last,
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
        starting_indexes: &Indexes,
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
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
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

        Ok(())
    }
}
