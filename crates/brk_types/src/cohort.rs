use std::{fmt, ops::Deref, path::Path};

use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize};

/// URPD cohort identifier. Use `GET /api/urpd` to list available cohorts.
///
/// Validated at construction: non-empty, ASCII `[a-z0-9_]+`. Matches the
/// schemars enum value set; the type therefore proves "this is a valid
/// cohort name" wherever a `Cohort` is held.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, JsonSchema)]
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

impl Cohort {
    /// Returns `Some(Cohort)` iff `s` is non-empty ASCII `[a-z0-9_]+`.
    pub fn new(s: impl Into<String>) -> Option<Self> {
        let s = s.into();
        if s.is_empty()
            || !s
                .bytes()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_')
        {
            return None;
        }
        Some(Self(s))
    }
}

impl fmt::Display for Cohort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl Deref for Cohort {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<str> for Cohort {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl AsRef<Path> for Cohort {
    fn as_ref(&self) -> &Path {
        Path::new(&self.0)
    }
}

impl<'de> Deserialize<'de> for Cohort {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        Self::new(s).ok_or_else(|| {
            serde::de::Error::custom("invalid cohort: expected non-empty [a-z0-9_]+")
        })
    }
}
