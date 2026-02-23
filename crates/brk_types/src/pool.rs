use schemars::JsonSchema;
use serde::Serialize;

use super::PoolSlug;

/// Mining pool information
#[derive(Debug, Serialize, JsonSchema)]
pub struct Pool {
    /// Unique pool identifier
    pub slug: PoolSlug,

    /// Pool name
    pub name: &'static str,

    /// Known payout addresses for pool identification
    #[serde(skip)]
    pub addresses: Box<[&'static str]>,

    /// Coinbase tags used to identify blocks mined by this pool
    #[serde(skip)]
    pub tags: Box<[&'static str]>,

    /// Lowercase coinbase tags for case-insensitive matching
    #[serde(skip)]
    #[schemars(skip)]
    pub tags_lowercase: Box<[String]>,

    /// Pool website URL
    pub link: &'static str,
}

impl Pool {
    /// Get slug of pool
    pub fn slug(&self) -> PoolSlug {
        self.slug
    }

    /// Get the pool's unique numeric ID
    pub fn unique_id(&self) -> u8 {
        self.slug.into()
    }
}

