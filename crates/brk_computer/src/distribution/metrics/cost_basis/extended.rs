use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{BasisPoints16, Cents, Height, Version};
use vecdb::{AnyStoredVec, Rw, StorageMode, WritableVec};

use crate::{
    distribution::state::CohortState,
    internal::{
        PERCENTILES_LEN, PercentFromHeight, PercentilesVecs,
        compute_spot_percentile_rank,
    },
};

use crate::distribution::metrics::ImportConfig;

/// Extended cost basis metrics (only for extended cohorts).
#[derive(Traversable)]
pub struct CostBasisExtended<M: StorageMode = Rw> {
    /// Cost basis percentiles (sat-weighted)
    pub percentiles: PercentilesVecs<M>,

    /// Invested capital percentiles (USD-weighted)
    pub invested_capital: PercentilesVecs<M>,

    /// What percentile of cost basis is below spot (sat-weighted)
    pub spot_cost_basis_percentile: PercentFromHeight<BasisPoints16, M>,

    /// What percentile of invested capital is below spot (USD-weighted)
    pub spot_invested_capital_percentile: PercentFromHeight<BasisPoints16, M>,
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
            spot_cost_basis_percentile: cfg.import_percent_bp16("spot_cost_basis_percentile", Version::ZERO)?,
            spot_invested_capital_percentile: cfg.import_percent_bp16("spot_invested_capital_percentile", Version::ZERO)?,
        })
    }

    pub(crate) fn truncate_push_percentiles(
        &mut self,
        height: Height,
        state: &mut CohortState,
        spot: Cents,
    ) -> Result<()> {
        let computed = state.compute_percentiles();

        let sat_prices = computed
            .as_ref()
            .map(|p| p.sat_weighted)
            .unwrap_or([Cents::ZERO; PERCENTILES_LEN]);

        self.percentiles.truncate_push(height, &sat_prices)?;
        let rank = compute_spot_percentile_rank(&sat_prices, spot);
        self.spot_cost_basis_percentile
            .bps
            .height
            .truncate_push(height, rank)?;

        let usd_prices = computed
            .as_ref()
            .map(|p| p.usd_weighted)
            .unwrap_or([Cents::ZERO; PERCENTILES_LEN]);

        self.invested_capital.truncate_push(height, &usd_prices)?;
        let rank = compute_spot_percentile_rank(&usd_prices, spot);
        self.spot_invested_capital_percentile
            .bps
            .height
            .truncate_push(height, rank)?;

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
        vecs.push(&mut self.spot_cost_basis_percentile.bps.height);
        vecs.push(&mut self.spot_invested_capital_percentile.bps.height);
        vecs
    }

    pub(crate) fn validate_computed_versions(&mut self, base_version: Version) -> Result<()> {
        self.percentiles
            .validate_computed_version_or_reset(base_version)?;
        self.invested_capital
            .validate_computed_version_or_reset(base_version)?;
        self.spot_cost_basis_percentile
            .bps
            .height
            .validate_computed_version_or_reset(base_version)?;
        self.spot_invested_capital_percentile
            .bps
            .height
            .validate_computed_version_or_reset(base_version)?;
        Ok(())
    }
}
