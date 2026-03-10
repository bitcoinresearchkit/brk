use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{BasisPoints16, Cents, Height, Version};
use vecdb::{AnyStoredVec, AnyVec, Rw, StorageMode, WritableVec};

use crate::internal::{ComputedPerBlock, PercentPerBlock, PercentilesVecs, Price, PERCENTILES_LEN};

use super::ImportConfig;

/// Cost basis metrics: min/max + percentiles + supply density.
/// Used by all/sth/lth cohorts only.
#[derive(Traversable)]
pub struct CostBasis<M: StorageMode = Rw> {
    pub min: Price<ComputedPerBlock<Cents, M>>,
    pub max: Price<ComputedPerBlock<Cents, M>>,
    pub percentiles: PercentilesVecs<M>,
    pub invested_capital: PercentilesVecs<M>,
    pub supply_density: PercentPerBlock<BasisPoints16, M>,
}

impl CostBasis {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        Ok(Self {
            min: cfg.import("cost_basis_min", Version::ZERO)?,
            max: cfg.import("cost_basis_max", Version::ZERO)?,
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
            supply_density: PercentPerBlock::forced_import(
                cfg.db,
                &cfg.name("supply_density"),
                cfg.version,
                cfg.indexes,
            )?,
        })
    }

    pub(crate) fn min_stateful_height_len(&self) -> usize {
        self.min
            .cents
            .height
            .len()
            .min(self.max.cents.height.len())
            .min(self.supply_density.bps.height.len())
    }

    pub(crate) fn truncate_push_minmax(
        &mut self,
        height: Height,
        min_price: Cents,
        max_price: Cents,
    ) -> Result<()> {
        self.min.cents.height.truncate_push(height, min_price)?;
        self.max.cents.height.truncate_push(height, max_price)?;
        Ok(())
    }

    pub(crate) fn truncate_push_percentiles(
        &mut self,
        height: Height,
        sat_prices: &[Cents; PERCENTILES_LEN],
        usd_prices: &[Cents; PERCENTILES_LEN],
    ) -> Result<()> {
        self.percentiles.truncate_push(height, sat_prices)?;
        self.invested_capital.truncate_push(height, usd_prices)?;
        Ok(())
    }

    pub(crate) fn truncate_push_density(
        &mut self,
        height: Height,
        density_bps: BasisPoints16,
    ) -> Result<()> {
        Ok(self.supply_density.bps.height.truncate_push(height, density_bps)?)
    }

    pub(crate) fn validate_computed_versions(&mut self, base_version: Version) -> Result<()> {
        self.percentiles
            .validate_computed_version_or_reset(base_version)?;
        self.invested_capital
            .validate_computed_version_or_reset(base_version)?;
        Ok(())
    }

    pub(crate) fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        let mut vecs: Vec<&mut dyn AnyStoredVec> = vec![
            &mut self.min.cents.height,
            &mut self.max.cents.height,
            &mut self.supply_density.bps.height,
        ];
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
}
