use schemars::JsonSchema;
use serde::Deserialize;

use super::{Height, PoolSlug};

#[derive(Deserialize, JsonSchema)]
pub struct PoolSlugParam {
    pub slug: PoolSlug,
}

#[derive(Deserialize, JsonSchema)]
pub struct PoolSlugAndHeightParam {
    pub slug: PoolSlug,
    pub height: Height,
}
