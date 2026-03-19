mod base;
mod basic;
mod core;
mod full;
mod minimal;

pub use self::core::UnrealizedCore;
pub use base::UnrealizedBase;
pub use basic::UnrealizedBasic;
pub use full::UnrealizedFull;
pub use minimal::UnrealizedMinimal;

use brk_error::Result;
use brk_types::{Height, Indexes, Sats};
use vecdb::{Exit, ReadableVec};

use crate::{distribution::state::UnrealizedState, prices};

pub trait UnrealizedLike: Send + Sync {
    fn as_base(&self) -> &UnrealizedBase;
    fn as_base_mut(&mut self) -> &mut UnrealizedBase;
    fn min_stateful_len(&self) -> usize;
    fn push_state(&mut self, state: &UnrealizedState);
    fn compute_rest(
        &mut self,
        prices: &prices::Vecs,
        starting_indexes: &Indexes,
        supply_in_profit_sats: &(impl ReadableVec<Height, Sats> + Sync),
        supply_in_loss_sats: &(impl ReadableVec<Height, Sats> + Sync),
        exit: &Exit,
    ) -> Result<()>;
}

impl UnrealizedLike for UnrealizedBase {
    fn as_base(&self) -> &UnrealizedBase {
        self
    }
    fn as_base_mut(&mut self) -> &mut UnrealizedBase {
        self
    }
    fn min_stateful_len(&self) -> usize {
        self.min_stateful_len()
    }
    #[inline(always)]
    fn push_state(&mut self, state: &UnrealizedState) {
        self.push_state(state);
    }
    fn compute_rest(
        &mut self,
        _prices: &prices::Vecs,
        starting_indexes: &Indexes,
        _supply_in_profit_sats: &(impl ReadableVec<Height, Sats> + Sync),
        _supply_in_loss_sats: &(impl ReadableVec<Height, Sats> + Sync),
        exit: &Exit,
    ) -> Result<()> {
        self.compute_rest(starting_indexes, exit)
    }
}

impl UnrealizedLike for UnrealizedFull {
    fn as_base(&self) -> &UnrealizedBase {
        &self.inner
    }
    fn as_base_mut(&mut self) -> &mut UnrealizedBase {
        &mut self.inner
    }
    fn min_stateful_len(&self) -> usize {
        self.inner.min_stateful_len()
    }
    #[inline(always)]
    fn push_state(&mut self, state: &UnrealizedState) {
        self.push_state_all(state);
    }
    fn compute_rest(
        &mut self,
        prices: &prices::Vecs,
        starting_indexes: &Indexes,
        supply_in_profit_sats: &(impl ReadableVec<Height, Sats> + Sync),
        supply_in_loss_sats: &(impl ReadableVec<Height, Sats> + Sync),
        exit: &Exit,
    ) -> Result<()> {
        self.compute_rest_all(prices, starting_indexes, supply_in_profit_sats, supply_in_loss_sats, exit)
    }
}
