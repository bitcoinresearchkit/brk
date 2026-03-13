use schemars::JsonSchema;
use serde::Deserialize;

use super::PoolSlug;

#[derive(Deserialize, JsonSchema)]
pub struct PoolSlugParam {
    pub slug: PoolSlug,
}
