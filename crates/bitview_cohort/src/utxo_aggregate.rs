use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    AgeRangeId, CohortContext, CohortId, CohortName, LTH_AGE_RANGE_IDS, STH_AGE_RANGE_IDS,
    TERM_NAMES, Term,
};

#[cfg(feature = "storage")]
use bitview_traversable::Traversable;

/// Canonical name for the aggregate cohort containing every UTXO.
pub const UTXO_ALL_NAME: CohortName = CohortName::new("all", "All", "All UTXOs");

/// Canonical names for the aggregate UTXO cohorts.
pub const UTXO_AGGREGATE_NAMES: UTXOAggregate<CohortName> = UTXOAggregate {
    all: UTXO_ALL_NAME,
    sth: TERM_NAMES.short,
    lth: TERM_NAMES.long,
};

#[derive(Debug, Default, Clone, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "storage", derive(Traversable))]
pub struct UTXOAggregate<T> {
    /// Uses all UTXOs.
    pub all: T,
    /// Uses short-term-holder UTXOs younger than 150 days.
    pub sth: T,
    /// Uses long-term-holder UTXOs at least 150 days old.
    pub lth: T,
}

define_cohort_id!(
    UTXOAggregateId for UTXOAggregate {
        All => all,
        Sth => sth,
        Lth => lth,
    }
);

impl UTXOAggregateId {
    pub const fn age_range_ids(self) -> &'static [AgeRangeId] {
        match self {
            Self::All => AgeRangeId::ALL,
            Self::Sth => STH_AGE_RANGE_IDS,
            Self::Lth => LTH_AGE_RANGE_IDS,
        }
    }

    pub fn from_cohort_name(name: &str) -> Option<Self> {
        Self::ALL
            .iter()
            .copied()
            .find(|id| id.cohort_name().id == name)
    }

    pub const fn cohort_name(self) -> CohortName {
        match self {
            Self::All => UTXO_AGGREGATE_NAMES.all,
            Self::Sth => UTXO_AGGREGATE_NAMES.sth,
            Self::Lth => UTXO_AGGREGATE_NAMES.lth,
        }
    }

    pub const fn cohort(self) -> CohortId {
        match self {
            Self::All => CohortId::All,
            Self::Sth => CohortId::Term(Term::Sth),
            Self::Lth => CohortId::Term(Term::Lth),
        }
    }

    pub fn metric_name(self, metric: &str) -> String {
        CohortContext::Utxo.metric_name(self.cohort(), metric)
    }
}

impl<T> UTXOAggregate<T> {
    pub fn map<U>(&self, mut f: impl FnMut(&T) -> U) -> UTXOAggregate<U> {
        UTXOAggregate {
            all: f(&self.all),
            sth: f(&self.sth),
            lth: f(&self.lth),
        }
    }

    pub fn get(&self, id: CohortId) -> Option<&T> {
        match id {
            CohortId::All => Some(&self.all),
            CohortId::Term(Term::Sth) => Some(&self.sth),
            CohortId::Term(Term::Lth) => Some(&self.lth),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aggregate_metric_names_omit_only_the_all_prefix() {
        assert_eq!(
            UTXOAggregateId::All.metric_name("capitalized_price"),
            "capitalized_price"
        );
        assert_eq!(
            UTXOAggregateId::Sth.metric_name("capitalized_price"),
            "sth_capitalized_price"
        );
        assert_eq!(
            UTXOAggregateId::Lth.metric_name("capitalized_price"),
            "lth_capitalized_price"
        );
    }

    #[test]
    fn cohort_names_roundtrip_to_typed_ids() {
        for id in UTXOAggregateId::ALL.iter().copied() {
            assert_eq!(
                UTXOAggregateId::from_cohort_name(id.cohort_name().id),
                Some(id)
            );
        }
        assert_eq!(
            UTXOAggregateId::from_cohort_name("utxos_under_1h_old"),
            None
        );
    }
}
