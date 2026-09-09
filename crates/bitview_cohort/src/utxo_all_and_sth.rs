use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{Filter, Term};

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

define_column_id!(
    UTXOAllAndSthId for UTXOAllAndSth, version = 1 {
        All => all,
        Sth => sth,
    }
);

impl<T> UTXOAllAndSth<T> {
    pub fn get(&self, filter: &Filter) -> Option<&T> {
        match filter {
            Filter::All => Some(&self.all),
            Filter::Term(Term::Sth) => Some(&self.sth),
            _ => None,
        }
    }
}
