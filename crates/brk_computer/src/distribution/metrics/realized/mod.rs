mod adjusted;
mod core;
mod full;
mod minimal;

pub use adjusted::AdjustedSopr;
pub use self::core::RealizedCore;
pub use full::{RealizedFull, RealizedFullAccum};
pub use minimal::RealizedMinimal;

use brk_error::Result;
use brk_types::{Height, Indexes};
use vecdb::Exit;

use crate::distribution::state::{WithCapital, CohortState, CostBasisData, RealizedState};

pub trait RealizedLike: Send + Sync {
    fn as_core(&self) -> &RealizedCore;
    fn as_core_mut(&mut self) -> &mut RealizedCore;
    fn min_stateful_len(&self) -> usize;
    fn truncate_push(&mut self, height: Height, state: &CohortState<RealizedState, CostBasisData<WithCapital>>) -> Result<()>;
    fn compute_rest_part1(&mut self, starting_indexes: &Indexes, exit: &Exit) -> Result<()>;
    fn compute_from_stateful(
        &mut self,
        starting_indexes: &Indexes,
        others: &[&RealizedCore],
        exit: &Exit,
    ) -> Result<()>;
}

impl RealizedLike for RealizedCore {
    fn as_core(&self) -> &RealizedCore { self }
    fn as_core_mut(&mut self) -> &mut RealizedCore { self }
    fn min_stateful_len(&self) -> usize { self.min_stateful_len() }
    fn truncate_push(&mut self, height: Height, state: &CohortState<RealizedState, CostBasisData<WithCapital>>) -> Result<()> {
        self.truncate_push(height, state)
    }
    fn compute_rest_part1(&mut self, starting_indexes: &Indexes, exit: &Exit) -> Result<()> {
        self.compute_rest_part1(starting_indexes, exit)
    }
    fn compute_from_stateful(&mut self, starting_indexes: &Indexes, others: &[&RealizedCore], exit: &Exit) -> Result<()> {
        self.compute_from_stateful(starting_indexes, others, exit)
    }
}

impl RealizedLike for RealizedFull {
    fn as_core(&self) -> &RealizedCore { &self.core }
    fn as_core_mut(&mut self) -> &mut RealizedCore { &mut self.core }
    fn min_stateful_len(&self) -> usize { self.min_stateful_len() }
    fn truncate_push(&mut self, height: Height, state: &CohortState<RealizedState, CostBasisData<WithCapital>>) -> Result<()> {
        self.truncate_push(height, state)
    }
    fn compute_rest_part1(&mut self, starting_indexes: &Indexes, exit: &Exit) -> Result<()> {
        self.compute_rest_part1(starting_indexes, exit)
    }
    fn compute_from_stateful(&mut self, starting_indexes: &Indexes, others: &[&RealizedCore], exit: &Exit) -> Result<()> {
        self.compute_from_stateful(starting_indexes, others, exit)
    }
}
