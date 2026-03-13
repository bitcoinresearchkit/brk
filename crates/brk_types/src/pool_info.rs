use std::borrow::Cow;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{Pool, PoolSlug};

/// Basic pool information for listing all pools
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PoolInfo {
    /// Pool name
    pub name: Cow<'static, str>,

    /// URL-friendly pool identifier
    pub slug: PoolSlug,

    /// Unique numeric pool identifier
    pub unique_id: u8,
}

impl From<&'static Pool> for PoolInfo {
    fn from(pool: &'static Pool) -> Self {
        Self {
            name: Cow::Borrowed(pool.name),
            slug: pool.slug(),
            unique_id: pool.unique_id(),
        }
    }
}
