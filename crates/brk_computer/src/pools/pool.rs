use serde::{Deserialize, Serialize};

use crate::pools::PoolId;

#[derive(Debug, Serialize, Deserialize)]
pub struct Pool {
    pub id: PoolId,
    pub name: &'static str,
    pub addresses: Box<[&'static str]>,
    pub tags: Box<[&'static str]>,
    pub link: &'static str,
}
