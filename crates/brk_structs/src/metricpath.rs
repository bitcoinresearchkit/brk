use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Deserialize, JsonSchema)]
pub struct MetricPath {
    /// Metric name
    #[schemars(example = &"price_close", example = &"market_cap", example = &"realized_price")]
    pub metric: String,
}
