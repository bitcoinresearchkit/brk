use super::Filter;

/// Context for cohort naming - determines whether a prefix is needed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CohortContext {
    /// UTXO-based cohorts: uses "utxos_" prefix for Time/Amount filters
    Utxo,
    /// Address-based cohorts: uses "addrs_" prefix for Amount filters
    Address,
}

impl CohortContext {
    pub fn prefix(&self) -> &'static str {
        match self {
            CohortContext::Utxo => "utxos",
            CohortContext::Address => "addrs",
        }
    }

    pub fn prefixed(&self, name: &str) -> String {
        format!("{}_{}", self.prefix(), name)
    }

    /// Build full name for a filter, adding prefix only for Time/Amount filters.
    ///
    /// Prefix rules:
    /// - No prefix: `All`, `Term`, `Epoch`, `Year`, `Type`
    /// - Context prefix: `Time`, `Amount`
    pub fn full_name(&self, filter: &Filter, name: &str) -> String {
        match filter {
            Filter::All | Filter::Term(_) | Filter::Epoch(_) | Filter::Year(_) | Filter::Type(_) => {
                name.to_string()
            }
            Filter::Time(_) | Filter::Amount(_) => self.prefixed(name),
        }
    }
}
