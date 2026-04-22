use std::{fmt, ops::Deref};

use schemars::JsonSchema;
use serde::Deserialize;

/// Cohort identifier for cost basis distribution.
#[derive(Deserialize, JsonSchema)]
#[schemars(extend("enum" = [
    "all", "sth", "lth",
    "under_1h_old", "1h_to_1d_old", "1d_to_1w_old", "1w_to_1m_old",
    "1m_to_2m_old", "2m_to_3m_old", "3m_to_4m_old", "4m_to_5m_old", "5m_to_6m_old",
    "6m_to_1y_old", "1y_to_2y_old", "2y_to_3y_old", "3y_to_4y_old", "4y_to_5y_old",
    "5y_to_6y_old", "6y_to_7y_old", "7y_to_8y_old", "8y_to_10y_old",
    "10y_to_12y_old", "12y_to_15y_old", "over_15y_old",
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
