use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{Bitcoin, Dollars};

/// A single bucket in a URPD snapshot.
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct UrpdBucket {
    /// Inclusive lower bound of the bucket, in USD.
    pub price_floor: Dollars,
    /// Supply held with a last-move price inside this bucket, in BTC.
    pub supply: Bitcoin,
    /// Realized cap contribution in USD: `price_floor * supply`.
    pub realized_cap: Dollars,
    /// Unrealized P&L in USD against the close on the snapshot date: `(close - price_floor) * supply`. Can be negative.
    pub unrealized_pnl: Dollars,
}
