use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::Dollars;

/// Historical price response
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct HistoricalPrice {
    pub prices: Vec<HistoricalPriceEntry>,
    #[serde(rename = "exchangeRates")]
    pub exchange_rates: ExchangeRates,
}

/// A single price data point
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct HistoricalPriceEntry {
    pub time: u64,
    #[serde(rename = "USD")]
    pub usd: Dollars,
}

/// Exchange rates (USD base, on-chain only — no fiat pairs available)
#[derive(Debug, Default, Serialize, Deserialize, JsonSchema)]
pub struct ExchangeRates {}
