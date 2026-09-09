use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{CohortId, Term};

#[cfg(feature = "storage")]
use bitview_traversable::Traversable;

#[derive(Debug, Default, Clone, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "storage", derive(Traversable))]
pub struct UTXOAllAndSth<T> {
    /// Uses all UTXOs.
    pub all: T,
    /// Uses short-term-holder UTXOs younger than 150 days.
    pub sth: T,
}

define_cohort_id!(
    UTXOAllAndSthId for UTXOAllAndSth {
        All => all,
        Sth => sth,
    }
);

impl<T> UTXOAllAndSth<T> {
    pub fn get(&self, id: CohortId) -> Option<&T> {
        match id {
            CohortId::All => Some(&self.all),
            CohortId::Term(Term::Sth) => Some(&self.sth),
            _ => None,
        }
    }
}
