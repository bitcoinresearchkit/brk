use std::path::Path;

use brk_error::Result;
use brk_structs::Height;
use derive_deref::{Deref, DerefMut};

use super::CohortState;

#[derive(Clone, Deref, DerefMut)]
pub struct UTXOCohortState(CohortState);

impl UTXOCohortState {
    pub fn default_and_import(path: &Path, name: &str, compute_dollars: bool) -> Result<Self> {
        Ok(Self(CohortState::default_and_import(
            path,
            name,
            compute_dollars,
        )?))
    }

    pub fn height(&self) -> Option<Height> {
        self.0.height()
    }

    pub fn reset_price_to_amount(&mut self) -> Result<()> {
        self.0.reset_price_to_amount()
    }
}
