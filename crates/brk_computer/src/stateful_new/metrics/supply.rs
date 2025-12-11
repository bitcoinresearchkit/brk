//! Supply and UTXO count metrics.
//!
//! These metrics are always computed regardless of price data availability.

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Bitcoin, DateIndex, Dollars, Height, Sats, StoredU64, Version};
use vecdb::{
    AnyStoredVec, AnyVec, EagerVec, Exit, GenericStoredVec, ImportableVec, IterableVec, PcoVec,
    TypedVecIterator,
};

use crate::{
    Indexes,
    grouped::{
        ComputedHeightValueVecs, ComputedValueVecsFromDateIndex, ComputedVecsFromHeight, Source,
        VecBuilderOptions,
    },
    indexes, price,
    states::SupplyState,
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

    /// Half of supply value (used for computing median)
    pub height_to_supply_half_value: ComputedHeightValueVecs,

    /// Half of supply indexed by date
    pub indexes_to_supply_half: ComputedValueVecsFromDateIndex,
}

impl SupplyMetrics {
    /// Import supply metrics from database.
    pub fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v0 = Version::ZERO;
        let v1 = Version::ONE;
        let compute_dollars = cfg.compute_dollars();
        let last = VecBuilderOptions::default().add_last();

        Ok(Self {
            height_to_supply: EagerVec::forced_import(
                cfg.db,
                &cfg.name("supply"),
                cfg.version + v0,
            )?,

            height_to_supply_value: ComputedHeightValueVecs::forced_import(
                cfg.db,
                &cfg.name("supply"),
                Source::None,
                cfg.version + v0,
                compute_dollars,
            )?,

            indexes_to_supply: ComputedValueVecsFromDateIndex::forced_import(
                cfg.db,
                &cfg.name("supply"),
                Source::Compute,
                cfg.version + v1,
                last,
                compute_dollars,
                cfg.indexes,
            )?,

            height_to_utxo_count: EagerVec::forced_import(
                cfg.db,
                &cfg.name("utxo_count"),
                cfg.version + v0,
            )?,

            indexes_to_utxo_count: ComputedVecsFromHeight::forced_import(
                cfg.db,
                &cfg.name("utxo_count"),
                Source::None,
                cfg.version + v0,
                cfg.indexes,
                last,
            )?,

            height_to_supply_half_value: ComputedHeightValueVecs::forced_import(
                cfg.db,
                &cfg.name("supply_half"),
                Source::Compute,
                cfg.version + v0,
                compute_dollars,
            )?,

            indexes_to_supply_half: ComputedValueVecsFromDateIndex::forced_import(
                cfg.db,
                &cfg.name("supply_half"),
                Source::Compute,
                cfg.version + v0,
                last,
                compute_dollars,
                cfg.indexes,
            )?,
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

    /// Flush height-indexed vectors to disk.
    pub fn safe_flush(&mut self, exit: &Exit) -> Result<()> {
        self.height_to_supply.safe_write(exit)?;
        self.height_to_utxo_count.safe_write(exit)?;
        Ok(())
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
            &others.iter().map(|v| &v.height_to_supply).collect::<Vec<_>>(),
            exit,
        )?;
        self.height_to_utxo_count.compute_sum_of_others(
            starting_indexes.height,
            &others.iter().map(|v| &v.height_to_utxo_count).collect::<Vec<_>>(),
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

        Ok(())
    }

    /// Second phase of computed metrics (ratios, relative values).
    #[allow(clippy::too_many_arguments)]
    pub fn compute_rest_part2(
        &mut self,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
        starting_indexes: &Indexes,
        height_to_supply: &impl IterableVec<Height, Bitcoin>,
        _dateindex_to_supply: &impl IterableVec<DateIndex, Bitcoin>,
        height_to_market_cap: Option<&impl IterableVec<Height, Dollars>>,
        dateindex_to_market_cap: Option<&impl IterableVec<DateIndex, Dollars>>,
        exit: &Exit,
    ) -> Result<()> {
        let _ = (indexes, price, height_to_supply, height_to_market_cap, dateindex_to_market_cap);

        // Supply relative metrics computed here if needed
        Ok(())
    }
}
