use schemars::JsonSchema;
use serde::Deserialize;

use crate::Height;

#[derive(Deserialize, JsonSchema)]
pub struct StartHeightParam {
    /// Starting block height (optional, defaults to latest)
    #[schemars(example = 800000)]
    pub start_height: Option<Height>,
}
