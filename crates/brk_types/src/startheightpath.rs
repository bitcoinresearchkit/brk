use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Deserialize, JsonSchema)]
pub struct StartHeightPath {
    /// Starting block height (optional, defaults to latest)
    #[schemars(example = 800000)]
    pub start_height: Option<u32>,
}
