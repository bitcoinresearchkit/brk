use std::collections::BTreeMap;

use brk_vec::AnyCollectableVec;
use derive_deref::{Deref, DerefMut};

use super::index::Index;

#[derive(Default)]
pub struct VecTrees<'a> {
    pub id_to_index_to_vec: BTreeMap<String, IndexToVec<'a>>,
    pub index_to_id_to_vec: BTreeMap<Index, IdToVec<'a>>,
}

impl<'a> VecTrees<'a> {
    // Not the most performant or type safe but only built once so that's okay
    pub fn insert(&mut self, vec: &'a dyn AnyCollectableVec) {
        let name = vec.name();
        let split = name.split("_to_").collect::<Vec<_>>();
        if split.len() != 2
            && !(split.len() == 3
                && split.get(1).is_some_and(|s| {
                    s == &"up"
                        || s == &"start"
                        || s.ends_with("relative")
                        || s.starts_with("from")
                        || s == &"cumulative_up"
                        || s.starts_with("cumulative_start")
                        || s.starts_with("cumulative_from")
                }))
            && !(split.len() == 4
                && split
                    .get(1)
                    .is_some_and(|s| s == &"up" || s == &"start" || s.starts_with("from"))
                && split.get(2).is_some_and(|s| s.ends_with("relative")))
        {
            dbg!(&name, &split);
            panic!();
        }
        let str = vec
            .index_type_to_string()
            .split("::")
            .last()
            .unwrap()
            .to_lowercase();
        let index = Index::try_from(str.as_str())
            .inspect_err(|_| {
                dbg!(&str);
            })
            .unwrap();
        if split[0] != index.to_string().to_lowercase() {
            dbg!(&name, split[0], index.to_string());
            panic!();
        }
        let key = split[1..].join("_to_").to_string().replace("_", "-");
        let prev = self
            .id_to_index_to_vec
            .entry(key.clone())
            .or_default()
            .insert(index, vec);
        if prev.is_some() {
            dbg!(&key, str, name);
            panic!()
        }
        let prev = self
            .index_to_id_to_vec
            .entry(index)
            .or_default()
            .insert(key.clone(), vec);
        if prev.is_some() {
            dbg!(&key, str, name);
            panic!()
        }
    }

    pub fn serialize_id_to_index_to_vec(&self) -> BTreeMap<String, Vec<String>> {
        self.id_to_index_to_vec
            .iter()
            .map(|(id, index_to_vec)| {
                (
                    id.to_string(),
                    index_to_vec
                        .keys()
                        .map(|i| i.serialize_long())
                        .collect::<Vec<_>>(),
                )
            })
            .collect()
    }

    pub fn serialize_index_to_id_to_vec(&self) -> BTreeMap<String, Vec<String>> {
        self.index_to_id_to_vec
            .iter()
            .map(|(index, id_to_vec)| {
                (
                    index.serialize_long(),
                    id_to_vec
                        .keys()
                        .map(|id| id.to_string())
                        .collect::<Vec<_>>(),
                )
            })
            .collect()
    }
}

#[derive(Default, Deref, DerefMut)]
pub struct IndexToVec<'a>(BTreeMap<Index, &'a dyn AnyCollectableVec>);

#[derive(Default, Deref, DerefMut)]
pub struct IdToVec<'a>(BTreeMap<String, &'a dyn AnyCollectableVec>);
