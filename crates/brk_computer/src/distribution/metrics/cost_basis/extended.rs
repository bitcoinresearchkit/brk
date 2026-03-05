use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Cents, Height, Version};
use vecdb::{AnyStoredVec, Rw, StorageMode};

use crate::{
    distribution::state::CohortState,
    internal::{PERCENTILES_LEN, PercentilesVecs},
};

use crate::distribution::metrics::ImportConfig;

/// Extended cost basis metrics (only for extended cohorts).
#[derive(Traversable)]
pub struct CostBasisExtended<M: StorageMode = Rw> {
    /// Cost basis percentiles (sat-weighted)
    pub percentiles: PercentilesVecs<M>,

    /// Invested capital percentiles (USD-weighted)
    pub invested_capital: PercentilesVecs<M>,
}

impl CostBasisExtended {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        Ok(Self {
            percentiles: PercentilesVecs::forced_import(
                cfg.db,
                &cfg.name("cost_basis"),
                cfg.version,
                cfg.indexes,
            )?,
            invested_capital: PercentilesVecs::forced_import(
                cfg.db,
                &cfg.name("invested_capital"),
                cfg.version,
                cfg.indexes,
            )?,
        })
    }

    pub(crate) fn truncate_push_percentiles(
        &mut self,
        height: Height,
        state: &mut CohortState,
        is_day_boundary: bool,
    ) -> Result<()> {
        let computed = if is_day_boundary {
            state.compute_percentiles()
        } else {
            state.cached_percentiles()
        };

        let sat_prices = computed
            .as_ref()
            .map(|p| p.sat_weighted)
            .unwrap_or([Cents::ZERO; PERCENTILES_LEN]);
        let usd_prices = computed
            .as_ref()
            .map(|p| p.usd_weighted)
            .unwrap_or([Cents::ZERO; PERCENTILES_LEN]);

        self.push_arrays(height, &sat_prices, &usd_prices)
    }

    /// Push pre-computed percentile arrays.
    /// Shared by both individual cohort and aggregate (K-way merge) paths.
    pub(crate) fn push_arrays(
        &mut self,
        height: Height,
        sat_prices: &[Cents; PERCENTILES_LEN],
        usd_prices: &[Cents; PERCENTILES_LEN],
    ) -> Result<()> {
        self.percentiles.truncate_push(height, sat_prices)?;
        self.invested_capital.truncate_push(height, usd_prices)?;
        Ok(())
    }

    pub(crate) fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        let mut vecs: Vec<&mut dyn AnyStoredVec> = Vec::new();
        vecs.extend(
            self.percentiles
                .vecs
                .iter_mut()
                .map(|v| &mut v.cents.height as &mut dyn AnyStoredVec),
        );
        vecs.extend(
            self.invested_capital
                .vecs
                .iter_mut()
                .map(|v| &mut v.cents.height as &mut dyn AnyStoredVec),
        );
        vecs
    }

    pub(crate) fn validate_computed_versions(&mut self, base_version: Version) -> Result<()> {
        self.percentiles
            .validate_computed_version_or_reset(base_version)?;
        self.invested_capital
            .validate_computed_version_or_reset(base_version)?;
        Ok(())
    }
}
