use std::{collections::BTreeSet, path::PathBuf};

use crate::{io::Serialization, structs::AnyMap};

use super::Kind;

#[derive(Debug, Clone)]
pub struct Route {
    pub type_name: String,
    pub list: BTreeSet<Kind>,
    pub path: PathBuf,
    pub serialization: Serialization,
}

impl Route {
    pub fn update(&mut self, map: &(dyn AnyMap + Send + Sync)) {
        self.list.append(&mut BTreeSet::from(map));
        if self.serialization != map.serialization() {
            panic!("route.upate() different serialization")
        }
    }
}

impl From<&(dyn AnyMap + Send + Sync)> for Route {
    fn from(map: &(dyn AnyMap + Send + Sync)) -> Self {
        Self {
            list: BTreeSet::from(map),
            path: map.path_parent().to_owned(),
            type_name: map.type_name().split("::").last().unwrap().to_owned(),
            serialization: map.serialization(),
        }
    }
}
