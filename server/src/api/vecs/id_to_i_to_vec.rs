use std::collections::BTreeMap;

use derive_deref::{Deref, DerefMut};
use storable_vec::AnyJsonStorableVec;

use super::index::Index;

#[derive(Default, Deref, DerefMut)]
pub struct VecIdToIndexToVec(BTreeMap<String, IndexToVec>);

impl VecIdToIndexToVec {
    // Not the most performant or type safe but only built once so that's okay
    pub fn insert(&mut self, vec: &'static dyn AnyJsonStorableVec) {
        let file_name = vec.file_name();
        let split = file_name.split("_to_").collect::<Vec<_>>();
        if split.len() != 2 {
            panic!();
        }
        let str = vec.index_type_to_string().split("::").last().unwrap().to_lowercase();
        let index = Index::try_from(str.as_str())
            .inspect_err(|_| {
                dbg!(str);
            })
            .unwrap();
        if split[0] != index.to_string().to_lowercase() {
            dbg!(split[0], index.to_string());
            panic!();
        }
        let key = split[1].to_string().replace("_", "-");
        let prev = self.entry(key).or_default().insert(index, vec);
        if prev.is_some() {
            panic!()
        }
    }
}

#[derive(Default, Deref, DerefMut)]
pub struct IndexToVec(BTreeMap<Index, &'static dyn AnyJsonStorableVec>);
