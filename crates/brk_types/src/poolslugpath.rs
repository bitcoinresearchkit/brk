use schemars::JsonSchema;
use serde::Deserialize;

use super::PoolSlug;

/// Path parameter for pool detail endpoint
#[derive(Deserialize, JsonSchema)]
pub struct PoolSlugPath {
    /// Pool slug (e.g., "foundryusa", "f2pool", "antpool")
    pub slug: PoolSlug,
}
