use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Cents, Dollars, Height, Indexes, Sats, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{AnyStoredVec, Exit, ReadableVec, Rw, StorageMode};

use crate::{blocks, prices};

use crate::distribution::metrics::{
    ActivityFull, CohortMetricsBase, ImportConfig, AdjustedSopr,
    RealizedFull, UnrealizedFull,
};

use super::ExtendedCohortMetrics;

/// Cohort metrics with extended + adjusted realized, extended cost basis.
/// Wraps `ExtendedCohortMetrics` and adds adjusted SOPR as a composable add-on.
/// Used by: sth cohort.
#[derive(Deref, DerefMut, Traversable)]
pub struct ExtendedAdjustedCohortMetrics<M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub inner: ExtendedCohortMetrics<M>,
    #[traversable(wrap = "realized/sopr", rename = "adjusted")]
    pub asopr: Box<AdjustedSopr<M>>,
}

impl CohortMetricsBase for ExtendedAdjustedCohortMetrics {
    type ActivityVecs = ActivityFull;
    type RealizedVecs = RealizedFull;
    type UnrealizedVecs = UnrealizedFull;

    impl_cohort_accessors!();

    fn validate_computed_versions(&mut self, base_version: Version) -> Result<()> {
        self.inner.validate_computed_versions(base_version)
    }

    fn min_stateful_len(&self) -> usize {
        self.inner.min_stateful_len()
    }

    fn collect_all_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        self.inner.collect_all_vecs_mut()
    }
}

impl ExtendedAdjustedCohortMetrics {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let inner = ExtendedCohortMetrics::forced_import(cfg)?;
        let asopr = AdjustedSopr::forced_import(cfg)?;
        Ok(Self {
            inner,
            asopr: Box::new(asopr),
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn compute_rest_part2(
        &mut self,
        blocks: &blocks::Vecs,
        prices: &prices::Vecs,
        starting_indexes: &Indexes,
        height_to_market_cap: &impl ReadableVec<Height, Dollars>,
        up_to_1h_value_created: &impl ReadableVec<Height, Cents>,
        up_to_1h_value_destroyed: &impl ReadableVec<Height, Cents>,
        all_supply_sats: &impl ReadableVec<Height, Sats>,
        exit: &Exit,
    ) -> Result<()> {
        self.inner.compute_rest_part2(
            blocks,
            prices,
            starting_indexes,
            height_to_market_cap,
            all_supply_sats,
            exit,
        )?;

        self.asopr.compute_rest_part2(
            blocks,
            starting_indexes,
            &self.inner.realized.minimal.sopr.value_created.raw.height,
            &self.inner.realized.minimal.sopr.value_destroyed.raw.height,
            up_to_1h_value_created,
            up_to_1h_value_destroyed,
            exit,
        )?;

        Ok(())
    }
}
