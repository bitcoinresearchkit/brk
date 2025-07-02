use std::path::Path;

use brk_core::Result;
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
}
