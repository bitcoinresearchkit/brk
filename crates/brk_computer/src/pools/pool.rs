use allocative::Allocative;

use crate::pools::PoolId;

#[derive(Debug, Allocative)]
pub struct Pool {
    pub id: PoolId,
    pub name: &'static str,
    pub addresses: Box<[&'static str]>,
    pub tags: Box<[&'static str]>,
    pub tags_lowercase: Box<[String]>,
    pub link: &'static str,
}

impl Pool {
    pub fn serialized_id(&self) -> String {
        let value = serde_json::to_value(self.id).unwrap();
        value.as_str().unwrap().to_string()
    }
}

impl From<(usize, JSONPool)> for Pool {
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
