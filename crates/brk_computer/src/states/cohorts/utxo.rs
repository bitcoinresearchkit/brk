use std::path::Path;

use brk_error::Result;
use derive_deref::{Deref, DerefMut};

use super::CohortState;

#[derive(Clone, Deref, DerefMut)]
pub struct UTXOCohortState(CohortState);

impl UTXOCohortState {
    pub fn new(path: &Path, name: &str, compute_dollars: bool) -> Self {
        Self(CohortState::new(path, name, compute_dollars))
    }

    pub fn reset_price_to_amount_if_needed(&mut self) -> Result<()> {
        self.0.reset_price_to_amount_if_needed()
    }
}
