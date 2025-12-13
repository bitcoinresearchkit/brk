use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Deserialize, JsonSchema)]
pub struct HeightPath {
    /// Bitcoin block height
    #[schemars(example = 0)]
    pub height: u32,
}
