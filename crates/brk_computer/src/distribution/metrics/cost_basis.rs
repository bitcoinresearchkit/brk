use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Dollars, Height, StoredF32, Version};
use rayon::prelude::*;
use vecdb::{AnyStoredVec, AnyVec, Exit, Rw, StorageMode, WritableVec};

use crate::{
    ComputeIndexes,
    distribution::state::CohortState,
    internal::{
        ComputedFromHeightLast, PERCENTILES_LEN, Price, PriceFromHeight, PercentilesVecs,
        compute_spot_percentile_rank,
    },
};

use super::ImportConfig;

/// Cost basis metrics.
#[derive(Traversable)]
pub struct CostBasisMetrics<M: StorageMode = Rw> {
    /// Minimum cost basis for any UTXO at this height
    pub min: Price<ComputedFromHeightLast<Dollars, M>>,

    /// Maximum cost basis for any UTXO at this height
    pub max: Price<ComputedFromHeightLast<Dollars, M>>,

    /// Cost basis percentiles (sat-weighted)
    pub percentiles: Option<PercentilesVecs<M>>,

    /// Invested capital percentiles (USD-weighted)
    pub invested_capital: Option<PercentilesVecs<M>>,

    /// What percentile of cost basis is below spot (sat-weighted)
    pub spot_cost_basis_percentile: Option<ComputedFromHeightLast<StoredF32, M>>,

    /// What percentile of invested capital is below spot (USD-weighted)
    pub spot_invested_capital_percentile: Option<ComputedFromHeightLast<StoredF32, M>>,
}

impl CostBasisMetrics {
    /// Import cost basis metrics from database.
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let extended = cfg.extended();

        Ok(Self {
            min: PriceFromHeight::forced_import(
                cfg.db,
                &cfg.name("min_cost_basis"),
                cfg.version,
                cfg.indexes,
            )?,
            max: PriceFromHeight::forced_import(
                cfg.db,
                &cfg.name("max_cost_basis"),
                cfg.version,
                cfg.indexes,
            )?,
            percentiles: extended
                .then(|| {
                    PercentilesVecs::forced_import(
                        cfg.db,
                        &cfg.name("cost_basis"),
                        cfg.version,
                        cfg.indexes,
                        true,
                    )
                })
                .transpose()?,
            invested_capital: extended
                .then(|| {
                    PercentilesVecs::forced_import(
                        cfg.db,
                        &cfg.name("invested_capital"),
                        cfg.version,
                        cfg.indexes,
                        true,
                    )
                })
                .transpose()?,
            spot_cost_basis_percentile: extended
                .then(|| {
                    ComputedFromHeightLast::forced_import(
                        cfg.db,
                        &cfg.name("spot_cost_basis_percentile"),
                        cfg.version,
                        cfg.indexes,
                    )
                })
                .transpose()?,
            spot_invested_capital_percentile: extended
                .then(|| {
                    ComputedFromHeightLast::forced_import(
                        cfg.db,
                        &cfg.name("spot_invested_capital_percentile"),
                        cfg.version,
                        cfg.indexes,
                    )
                })
                .transpose()?,
        })
    }

    /// Get minimum length across height-indexed vectors written in block loop.
    pub(crate) fn min_stateful_height_len(&self) -> usize {
        let mut min = self.min.height.len().min(self.max.height.len());
        if let Some(v) = &self.spot_cost_basis_percentile {
            min = min.min(v.height.len());
        }
        if let Some(v) = &self.spot_invested_capital_percentile {
            min = min.min(v.height.len());
        }
        if let Some(p) = &self.percentiles {
            min = min.min(p.min_stateful_height_len());
        }
        if let Some(p) = &self.invested_capital {
            min = min.min(p.min_stateful_height_len());
        }
        min
    }

    /// Push min/max cost basis from state.
    pub(crate) fn truncate_push_minmax(&mut self, height: Height, state: &CohortState) -> Result<()> {
        self.min.height.truncate_push(
            height,
            state
                .cost_basis_data_first_key_value()
                .map(|(cents, _)| cents.into())
                .unwrap_or(Dollars::NAN),
        )?;
        self.max.height.truncate_push(
            height,
            state
                .cost_basis_data_last_key_value()
                .map(|(cents, _)| cents.into())
                .unwrap_or(Dollars::NAN),
        )?;
        Ok(())
    }

    /// Push cost basis percentiles and spot ranks at every height.
    pub(crate) fn truncate_push_percentiles(
        &mut self,
        height: Height,
        state: &mut CohortState,
        spot: Dollars,
    ) -> Result<()> {
        let computed = state.compute_percentiles();

        // Sat-weighted percentiles and spot rank
        let sat_prices = computed
            .as_ref()
            .map(|p| p.sat_weighted.map(|c| c.to_dollars()))
            .unwrap_or([Dollars::NAN; PERCENTILES_LEN]);

        if let Some(percentiles) = self.percentiles.as_mut() {
            percentiles.truncate_push(height, &sat_prices)?;
        }
        if let Some(spot_pct) = self.spot_cost_basis_percentile.as_mut() {
            let rank = compute_spot_percentile_rank(&sat_prices, spot);
            spot_pct.height.truncate_push(height, rank)?;
        }

        // USD-weighted percentiles and spot rank
        let usd_prices = computed
            .as_ref()
            .map(|p| p.usd_weighted.map(|c| c.to_dollars()))
            .unwrap_or([Dollars::NAN; PERCENTILES_LEN]);

        if let Some(invested_capital) = self.invested_capital.as_mut() {
            invested_capital.truncate_push(height, &usd_prices)?;
        }
        if let Some(spot_pct) = self.spot_invested_capital_percentile.as_mut() {
            let rank = compute_spot_percentile_rank(&usd_prices, spot);
            spot_pct.height.truncate_push(height, rank)?;
        }

        Ok(())
    }

    /// Returns a parallel iterator over all vecs for parallel writing.
    pub(crate) fn par_iter_mut(&mut self) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        let mut vecs: Vec<&mut dyn AnyStoredVec> = vec![&mut self.min.height, &mut self.max.height];
        if let Some(percentiles) = self.percentiles.as_mut() {
            vecs.extend(
                percentiles
                    .vecs
                    .iter_mut()
                    .flatten()
                    .map(|v| &mut v.height as &mut dyn AnyStoredVec),
            );
        }
        if let Some(invested_capital) = self.invested_capital.as_mut() {
            vecs.extend(
                invested_capital
                    .vecs
                    .iter_mut()
                    .flatten()
                    .map(|v| &mut v.height as &mut dyn AnyStoredVec),
            );
        }
        if let Some(v) = self.spot_cost_basis_percentile.as_mut() {
            vecs.push(&mut v.height);
        }
        if let Some(v) = self.spot_invested_capital_percentile.as_mut() {
            vecs.push(&mut v.height);
        }
        vecs.into_par_iter()
    }

    /// Validate computed versions or reset if mismatched.
    pub(crate) fn validate_computed_versions(&mut self, base_version: Version) -> Result<()> {
        if let Some(percentiles) = self.percentiles.as_mut() {
            percentiles.validate_computed_version_or_reset(base_version)?;
        }
        if let Some(invested_capital) = self.invested_capital.as_mut() {
            invested_capital.validate_computed_version_or_reset(base_version)?;
        }
        if let Some(v) = self.spot_cost_basis_percentile.as_mut() {
            v.height
                .validate_computed_version_or_reset(base_version)?;
        }
        if let Some(v) = self.spot_invested_capital_percentile.as_mut() {
            v.height
                .validate_computed_version_or_reset(base_version)?;
        }
        Ok(())
    }

    /// Compute aggregate values from separate cohorts.
    pub(crate) fn compute_from_stateful(
        &mut self,
        starting_indexes: &ComputeIndexes,
        others: &[&Self],
        exit: &Exit,
    ) -> Result<()> {
        self.min.height.compute_min_of_others(
            starting_indexes.height,
            &others.iter().map(|v| &v.min.height).collect::<Vec<_>>(),
            exit,
        )?;
        self.max.height.compute_max_of_others(
            starting_indexes.height,
            &others.iter().map(|v| &v.max.height).collect::<Vec<_>>(),
            exit,
        )?;
        Ok(())
    }
}
