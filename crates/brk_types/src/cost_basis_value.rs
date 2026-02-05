use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::Display;

/// Value type for cost basis distribution.
/// Options: supply (BTC), realized (USD, price × supply), unrealized (USD, spot × supply).
#[derive(Debug, Display, Clone, Copy, Default, PartialEq, Eq, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum CostBasisValue {
    #[default]
    Supply,
    Realized,
    Unrealized,
}
