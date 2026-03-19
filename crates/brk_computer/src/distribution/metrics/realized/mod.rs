mod adjusted;
mod core;
mod full;
mod minimal;

pub use adjusted::AdjustedSopr;
pub use self::core::RealizedCore;
pub use full::{RealizedFull, RealizedFullAccum};
pub use minimal::RealizedMinimal;

use brk_error::Result;
use brk_types::Indexes;
use vecdb::Exit;

use crate::{distribution::state::{WithCapital, CohortState, CostBasisData, RealizedState}, prices};

pub trait RealizedLike: Send + Sync {
    fn as_core(&self) -> &RealizedCore;
    fn as_core_mut(&mut self) -> &mut RealizedCore;
    fn min_stateful_len(&self) -> usize;
    fn push_state(&mut self, state: &CohortState<RealizedState, CostBasisData<WithCapital>>);
    fn compute_rest_part1(&mut self, prices: &prices::Vecs, starting_indexes: &Indexes, exit: &Exit) -> Result<()>;
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
    #[inline(always)]
    fn push_state(&mut self, state: &CohortState<RealizedState, CostBasisData<WithCapital>>) {
        self.push_state(state)
    }
    fn compute_rest_part1(&mut self, prices: &prices::Vecs, starting_indexes: &Indexes, exit: &Exit) -> Result<()> {
        self.compute_rest_part1(prices, starting_indexes, exit)
    }
    fn compute_from_stateful(&mut self, starting_indexes: &Indexes, others: &[&RealizedCore], exit: &Exit) -> Result<()> {
        self.compute_from_stateful(starting_indexes, others, exit)
    }
}

impl RealizedLike for RealizedFull {
    fn as_core(&self) -> &RealizedCore { &self.core }
    fn as_core_mut(&mut self) -> &mut RealizedCore { &mut self.core }
    fn min_stateful_len(&self) -> usize { self.min_stateful_len() }
    #[inline(always)]
    fn push_state(&mut self, state: &CohortState<RealizedState, CostBasisData<WithCapital>>) {
        self.push_state(state)
    }
    fn compute_rest_part1(&mut self, prices: &prices::Vecs, starting_indexes: &Indexes, exit: &Exit) -> Result<()> {
        self.compute_rest_part1(prices, starting_indexes, exit)
    }
    fn compute_from_stateful(&mut self, starting_indexes: &Indexes, others: &[&RealizedCore], exit: &Exit) -> Result<()> {
        self.compute_from_stateful(starting_indexes, others, exit)
    }
}
