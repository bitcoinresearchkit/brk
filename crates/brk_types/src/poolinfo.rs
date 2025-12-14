use schemars::JsonSchema;
use serde::Serialize;

use crate::{Pool, PoolSlug};

/// Basic pool information for listing all pools
#[derive(Debug, Serialize, JsonSchema)]
pub struct PoolInfo {
    /// Pool name
    pub name: &'static str,

    /// URL-friendly pool identifier
    pub slug: PoolSlug,

    /// Unique numeric pool identifier
    pub unique_id: u8,
}

impl From<&'static Pool> for PoolInfo {
    fn from(pool: &'static Pool) -> Self {
        Self {
            name: pool.name,
            slug: pool.slug(),
            unique_id: pool.unique_id(),
        }
    }
}
