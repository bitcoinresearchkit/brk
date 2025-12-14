use schemars::JsonSchema;
use serde::Serialize;

use super::PoolId;

/// Mining pool information
#[derive(Debug, Serialize, JsonSchema)]
pub struct Pool {
    /// Unique pool identifier
    pub id: PoolId,

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
    pub fn serialized_id(&self) -> String {
        self.id.to_string()
    }
}

impl From<(usize, JSONPool)> for Pool {
    #[inline]
    fn from((index, pool): (usize, JSONPool)) -> Self {
        Self {
            id: (index as u8).into(),
            name: pool.name,
            addresses: pool.addresses,
            tags_lowercase: pool
                .tags
                .iter()
                .map(|t| t.to_lowercase())
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            tags: pool.tags,
            link: pool.link,
        }
    }
}

#[derive(Debug)]
pub struct JSONPool {
    pub name: &'static str,
    pub addresses: Box<[&'static str]>,
    pub tags: Box<[&'static str]>,
    pub link: &'static str,
}
