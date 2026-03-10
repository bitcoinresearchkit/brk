mod core;
mod full;

pub use self::core::ActivityCore;
pub use full::ActivityFull;

use brk_error::Result;
use brk_types::{Height, Indexes, Version};
use vecdb::Exit;

use crate::{blocks, distribution::state::{CohortState, RealizedOps}};

pub trait ActivityLike: Send + Sync {
    fn as_core(&self) -> &ActivityCore;
    fn as_core_mut(&mut self) -> &mut ActivityCore;
    fn min_len(&self) -> usize;
    fn truncate_push<R: RealizedOps>(
        &mut self,
        height: Height,
        state: &CohortState<R>,
    ) -> Result<()>;
    fn validate_computed_versions(&mut self, base_version: Version) -> Result<()>;
    fn compute_from_stateful(
        &mut self,
        starting_indexes: &Indexes,
        others: &[&ActivityCore],
        exit: &Exit,
    ) -> Result<()>;
    fn compute_rest_part1(
        &mut self,
        blocks: &blocks::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()>;
}

impl ActivityLike for ActivityCore {
    fn as_core(&self) -> &ActivityCore { self }
    fn as_core_mut(&mut self) -> &mut ActivityCore { self }
    fn min_len(&self) -> usize { self.min_len() }
    fn truncate_push<R: RealizedOps>(&mut self, height: Height, state: &CohortState<R>) -> Result<()> {
        self.truncate_push(height, state)
    }
    fn validate_computed_versions(&mut self, base_version: Version) -> Result<()> {
        self.validate_computed_versions(base_version)
    }
    fn compute_from_stateful(&mut self, starting_indexes: &Indexes, others: &[&ActivityCore], exit: &Exit) -> Result<()> {
        self.compute_from_stateful(starting_indexes, others, exit)
    }
    fn compute_rest_part1(&mut self, blocks: &blocks::Vecs, starting_indexes: &Indexes, exit: &Exit) -> Result<()> {
        self.compute_rest_part1(blocks, starting_indexes, exit)
    }
}

impl ActivityLike for ActivityFull {
    fn as_core(&self) -> &ActivityCore { &self.inner }
    fn as_core_mut(&mut self) -> &mut ActivityCore { &mut self.inner }
    fn min_len(&self) -> usize { self.full_min_len() }
    fn truncate_push<R: RealizedOps>(&mut self, height: Height, state: &CohortState<R>) -> Result<()> {
        self.full_truncate_push(height, state)
    }
    fn validate_computed_versions(&mut self, base_version: Version) -> Result<()> {
        self.inner.validate_computed_versions(base_version)
    }
    fn compute_from_stateful(&mut self, starting_indexes: &Indexes, others: &[&ActivityCore], exit: &Exit) -> Result<()> {
        self.compute_from_stateful(starting_indexes, others, exit)
    }
    fn compute_rest_part1(&mut self, blocks: &blocks::Vecs, starting_indexes: &Indexes, exit: &Exit) -> Result<()> {
        self.compute_rest_part1(blocks, starting_indexes, exit)
    }
}
