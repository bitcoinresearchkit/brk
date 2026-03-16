use schemars::JsonSchema;
use serde::Deserialize;

use crate::SeriesName;

#[derive(Deserialize, JsonSchema)]
pub struct SeriesParam {
    pub series: SeriesName,
}
