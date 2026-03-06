mod base;
mod extended;
mod with_extended;

pub use base::CostBasisBase;
pub use extended::CostBasisExtended;
pub use with_extended::CostBasisWithExtended;

use brk_error::Result;
use brk_types::{Height, Version};

use crate::distribution::state::{CohortState, RealizedState};

/// Polymorphic dispatch for cost basis metric types.
///
/// `CostBasisBase` has no version validation or percentiles (no-op defaults).
/// `CostBasisWithExtended` validates versions and pushes percentiles.
pub trait CostBasisLike: Send + Sync {
    fn as_base(&self) -> &CostBasisBase;
    fn as_base_mut(&mut self) -> &mut CostBasisBase;
    fn validate_computed_versions(&mut self, _base_version: Version) -> Result<()> { Ok(()) }
    fn truncate_push_percentiles(
        &mut self,
        _height: Height,
        _state: &mut CohortState<RealizedState>,
        _is_day_boundary: bool,
    ) -> Result<()> {
        Ok(())
    }
}

impl CostBasisLike for CostBasisBase {
    fn as_base(&self) -> &CostBasisBase { self }
    fn as_base_mut(&mut self) -> &mut CostBasisBase { self }
}

impl CostBasisLike for CostBasisWithExtended {
    fn as_base(&self) -> &CostBasisBase { &self.base }
    fn as_base_mut(&mut self) -> &mut CostBasisBase { &mut self.base }
    fn validate_computed_versions(&mut self, base_version: Version) -> Result<()> {
        self.extended.validate_computed_versions(base_version)
    }
    fn truncate_push_percentiles(
        &mut self,
        height: Height,
        state: &mut CohortState<RealizedState>,
        is_day_boundary: bool,
    ) -> Result<()> {
        self.extended.truncate_push_percentiles(height, state, is_day_boundary)
    }
}
