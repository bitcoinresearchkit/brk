use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{DateIndex, Dollars, Height, Version};
use rayon::prelude::*;
use vecdb::{
    AnyStoredVec, AnyVec, EagerVec, Exit, GenericStoredVec, ImportableVec, IterableCloneableVec,
    PcoVec,
};

use crate::{
    ComputeIndexes,
    distribution::state::CohortState,
    internal::{ComputedVecsFromHeight, CostBasisPercentiles, Source, VecBuilderOptions},
};

use super::ImportConfig;

/// Cost basis metrics.
#[derive(Clone, Traversable)]
pub struct CostBasisMetrics {
    /// Minimum cost basis for any UTXO at this height
    pub height_to_min_cost_basis: EagerVec<PcoVec<Height, Dollars>>,
    pub indexes_to_min_cost_basis: ComputedVecsFromHeight<Dollars>,

    /// Maximum cost basis for any UTXO at this height
    pub height_to_max_cost_basis: EagerVec<PcoVec<Height, Dollars>>,
    pub indexes_to_max_cost_basis: ComputedVecsFromHeight<Dollars>,

    /// Cost basis distribution percentiles (median, quartiles, etc.)
    pub percentiles: Option<CostBasisPercentiles>,
}

impl CostBasisMetrics {
    /// Import cost basis metrics from database.
    pub fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let extended = cfg.extended();
        let last = VecBuilderOptions::default().add_last();

        let height_to_min_cost_basis =
            EagerVec::forced_import(cfg.db, &cfg.name("min_cost_basis"), cfg.version)?;

        let height_to_max_cost_basis =
            EagerVec::forced_import(cfg.db, &cfg.name("max_cost_basis"), cfg.version)?;

        Ok(Self {
            indexes_to_min_cost_basis: ComputedVecsFromHeight::forced_import(
                cfg.db,
                &cfg.name("min_cost_basis"),
                Source::Vec(height_to_min_cost_basis.boxed_clone()),
                cfg.version,
                cfg.indexes,
                last,
            )?,
            indexes_to_max_cost_basis: ComputedVecsFromHeight::forced_import(
                cfg.db,
                &cfg.name("max_cost_basis"),
                Source::Vec(height_to_max_cost_basis.boxed_clone()),
                cfg.version,
                cfg.indexes,
                last,
            )?,
            height_to_min_cost_basis,
            height_to_max_cost_basis,
            percentiles: extended
                .then(|| {
                    CostBasisPercentiles::forced_import(
                        cfg.db,
                        &cfg.name(""),
                        cfg.version,
                        cfg.indexes,
                        true,
                    )
                })
                .transpose()?,
        })
    }

    /// Get minimum length across height-indexed vectors written in block loop.
    pub fn min_stateful_height_len(&self) -> usize {
        self.height_to_min_cost_basis
            .len()
            .min(self.height_to_max_cost_basis.len())
    }

    /// Get minimum length across dateindex-indexed vectors written in block loop.
    pub fn min_stateful_dateindex_len(&self) -> usize {
        self.percentiles
            .as_ref()
            .map(|p| p.min_stateful_dateindex_len())
            .unwrap_or(usize::MAX)
    }

    /// Push min/max cost basis from state.
    pub fn truncate_push_minmax(&mut self, height: Height, state: &CohortState) -> Result<()> {
        self.height_to_min_cost_basis.truncate_push(
            height,
            state
                .price_to_amount_first_key_value()
                .map(|(dollars, _)| dollars)
                .unwrap_or(Dollars::NAN),
        )?;
        self.height_to_max_cost_basis.truncate_push(
            height,
            state
                .price_to_amount_last_key_value()
                .map(|(dollars, _)| dollars)
                .unwrap_or(Dollars::NAN),
        )?;
        Ok(())
    }

    /// Push cost basis percentiles from state at date boundary.
    /// Only called when at the last height of a day.
    pub fn truncate_push_percentiles(
        &mut self,
        dateindex: DateIndex,
        state: &CohortState,
    ) -> Result<()> {
        if let Some(percentiles) = self.percentiles.as_mut() {
            let percentile_prices = state.compute_percentile_prices();
            percentiles.truncate_push(dateindex, &percentile_prices)?;
        }
        Ok(())
    }

    /// Write height-indexed vectors to disk.
    pub fn write(&mut self) -> Result<()> {
        self.height_to_min_cost_basis.write()?;
        self.height_to_max_cost_basis.write()?;
        if let Some(percentiles) = self.percentiles.as_mut() {
            percentiles.write()?;
        }
        Ok(())
    }

    /// Returns a parallel iterator over all vecs for parallel writing.
    pub fn par_iter_mut(&mut self) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        let mut vecs: Vec<&mut dyn AnyStoredVec> = vec![
            &mut self.height_to_min_cost_basis,
            &mut self.height_to_max_cost_basis,
        ];
        if let Some(percentiles) = self.percentiles.as_mut() {
            vecs.extend(
                percentiles
                    .vecs
                    .iter_mut()
                    .flatten()
                    .filter_map(|v| v.dateindex.as_mut())
                    .map(|v| v as &mut dyn AnyStoredVec),
            );
        }
        vecs.into_par_iter()
    }

    /// Validate computed versions or reset if mismatched.
    pub fn validate_computed_versions(&mut self, base_version: Version) -> Result<()> {
        if let Some(percentiles) = self.percentiles.as_mut() {
            percentiles.validate_computed_version_or_reset(base_version)?;
        }
        Ok(())
    }

    /// Compute aggregate values from separate cohorts.
    pub fn compute_from_stateful(
        &mut self,
        starting_indexes: &ComputeIndexes,
        others: &[&Self],
        exit: &Exit,
    ) -> Result<()> {
        self.height_to_min_cost_basis.compute_min_of_others(
            starting_indexes.height,
            &others
                .iter()
                .map(|v| &v.height_to_min_cost_basis)
                .collect::<Vec<_>>(),
            exit,
        )?;
        self.height_to_max_cost_basis.compute_max_of_others(
            starting_indexes.height,
            &others
                .iter()
                .map(|v| &v.height_to_max_cost_basis)
                .collect::<Vec<_>>(),
            exit,
        )?;
        Ok(())
    }

    /// First phase of computed metrics (indexes from height).
    pub fn compute_rest_part1(
        &mut self,
        indexes: &crate::indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.indexes_to_min_cost_basis.compute_rest(
            indexes,
            starting_indexes,
            exit,
            Some(&self.height_to_min_cost_basis),
        )?;

        self.indexes_to_max_cost_basis.compute_rest(
            indexes,
            starting_indexes,
            exit,
            Some(&self.height_to_max_cost_basis),
        )?;

        Ok(())
    }
}
