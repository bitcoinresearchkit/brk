use std::{fmt, ops::Deref};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// URPD cohort identifier. Use `GET /api/urpd` to list available cohorts.
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
#[schemars(extend("enum" = [
    "all", "sth", "lth",
    "utxos_under_1h_old", "utxos_1h_to_1d_old", "utxos_1d_to_1w_old", "utxos_1w_to_1m_old",
    "utxos_1m_to_2m_old", "utxos_2m_to_3m_old", "utxos_3m_to_4m_old", "utxos_4m_to_5m_old",
    "utxos_5m_to_6m_old", "utxos_6m_to_1y_old", "utxos_1y_to_2y_old", "utxos_2y_to_3y_old",
    "utxos_3y_to_4y_old", "utxos_4y_to_5y_old", "utxos_5y_to_6y_old", "utxos_6y_to_7y_old",
    "utxos_7y_to_8y_old", "utxos_8y_to_10y_old", "utxos_10y_to_12y_old", "utxos_12y_to_15y_old",
    "utxos_over_15y_old",
]))]
pub struct Cohort(String);

impl fmt::Display for Cohort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl<T: Into<String>> From<T> for Cohort {
    fn from(s: T) -> Self {
        Self(s.into())
    }
}

impl Deref for Cohort {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
