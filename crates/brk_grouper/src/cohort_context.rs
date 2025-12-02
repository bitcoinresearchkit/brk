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
}
