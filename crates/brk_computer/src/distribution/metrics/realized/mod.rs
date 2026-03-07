mod adjusted;
mod base;
mod core;
mod full;
mod minimal;

pub use adjusted::RealizedAdjusted;
pub use base::RealizedBase;
pub use self::core::RealizedCore;
pub use full::RealizedFull;
pub use minimal::RealizedMinimal;

use brk_error::Result;
use brk_types::{Height, Indexes};
use vecdb::Exit;

use crate::{blocks, distribution::state::RealizedState};

/// Polymorphic dispatch for realized metric types.
///
/// Both `RealizedBase` and `RealizedFull` have the same inherent methods
/// but with different behavior (Full checks/pushes more fields).
/// This trait enables `CohortMetricsBase` to dispatch correctly via associated type.
pub trait RealizedLike: Send + Sync {
    fn as_base(&self) -> &RealizedBase;
    fn as_base_mut(&mut self) -> &mut RealizedBase;
    fn min_stateful_height_len(&self) -> usize;
    fn truncate_push(&mut self, height: Height, state: &RealizedState) -> Result<()>;
    fn compute_rest_part1(&mut self, blocks: &blocks::Vecs, starting_indexes: &Indexes, exit: &Exit) -> Result<()>;
    fn compute_from_stateful(
        &mut self,
        starting_indexes: &Indexes,
        others: &[&RealizedBase],
        exit: &Exit,
    ) -> Result<()>;
}

impl RealizedLike for RealizedBase {
    fn as_base(&self) -> &RealizedBase { self }
    fn as_base_mut(&mut self) -> &mut RealizedBase { self }
    fn min_stateful_height_len(&self) -> usize { self.min_stateful_height_len() }
    fn truncate_push(&mut self, height: Height, state: &RealizedState) -> Result<()> {
        self.truncate_push(height, state)
    }
    fn compute_rest_part1(&mut self, blocks: &blocks::Vecs, starting_indexes: &Indexes, exit: &Exit) -> Result<()> {
        self.compute_rest_part1(blocks, starting_indexes, exit)
    }
    fn compute_from_stateful(&mut self, starting_indexes: &Indexes, others: &[&RealizedBase], exit: &Exit) -> Result<()> {
        self.compute_from_stateful(starting_indexes, others, exit)
    }
}

impl RealizedLike for RealizedFull {
    fn as_base(&self) -> &RealizedBase { &self.base }
    fn as_base_mut(&mut self) -> &mut RealizedBase { &mut self.base }
    fn min_stateful_height_len(&self) -> usize { self.min_stateful_height_len() }
    fn truncate_push(&mut self, height: Height, state: &RealizedState) -> Result<()> {
        self.truncate_push(height, state)
    }
    fn compute_rest_part1(&mut self, blocks: &blocks::Vecs, starting_indexes: &Indexes, exit: &Exit) -> Result<()> {
        self.compute_rest_part1(blocks, starting_indexes, exit)
    }
    fn compute_from_stateful(&mut self, starting_indexes: &Indexes, others: &[&RealizedBase], exit: &Exit) -> Result<()> {
        self.compute_from_stateful(starting_indexes, others, exit)
    }
}
