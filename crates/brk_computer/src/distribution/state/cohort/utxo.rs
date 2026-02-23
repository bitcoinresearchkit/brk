use std::path::Path;

use brk_error::Result;
use brk_types::{Sats, SupplyState};
use derive_more::{Deref, DerefMut};

use super::{super::cost_basis::RealizedState, base::CohortState};

#[derive(Deref, DerefMut)]
pub struct UTXOCohortState(CohortState);

impl UTXOCohortState {
    pub(crate) fn new(path: &Path, name: &str) -> Self {
        Self(CohortState::new(path, name))
    }

    pub(crate) fn reset_cost_basis_data_if_needed(&mut self) -> Result<()> {
        self.0.reset_cost_basis_data_if_needed()
    }

    /// Reset state for fresh start.
    pub(crate) fn reset(&mut self) {
        self.0.supply = SupplyState::default();
        self.0.sent = Sats::ZERO;
        self.0.satblocks_destroyed = Sats::ZERO;
        self.0.satdays_destroyed = Sats::ZERO;
        self.0.realized = RealizedState::default();
    }
}
