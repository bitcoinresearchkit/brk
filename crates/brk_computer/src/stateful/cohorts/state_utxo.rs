use std::path::Path;

use brk_error::Result;
use brk_types::Sats;
use derive_deref::{Deref, DerefMut};

use super::CohortState;
use crate::{RealizedState, SupplyState};

#[derive(Clone, Deref, DerefMut)]
pub struct UTXOCohortState(CohortState);

impl UTXOCohortState {
    pub fn new(path: &Path, name: &str, compute_dollars: bool) -> Self {
        Self(CohortState::new(path, name, compute_dollars))
    }

    pub fn reset_price_to_amount_if_needed(&mut self) -> Result<()> {
        self.0.reset_price_to_amount_if_needed()
    }

    /// Reset state for fresh start.
    pub fn reset(&mut self) {
        self.0.supply = SupplyState::default();
        self.0.sent = Sats::ZERO;
        self.0.satblocks_destroyed = Sats::ZERO;
        self.0.satdays_destroyed = Sats::ZERO;
        if let Some(realized) = self.0.realized.as_mut() {
            *realized = RealizedState::NAN;
        }
    }
}
