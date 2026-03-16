use schemars::JsonSchema;
use serde::Deserialize;

use crate::Series;

#[derive(Deserialize, JsonSchema)]
pub struct SeriesParam {
    pub series: Series,
}
