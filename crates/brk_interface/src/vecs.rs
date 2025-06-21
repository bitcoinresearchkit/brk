use std::collections::BTreeMap;

use brk_computer::Computer;
use brk_indexer::Indexer;
use brk_vec::AnyCollectableVec;
use derive_deref::{Deref, DerefMut};

use crate::params::Pagination;

use super::index::Index;

#[derive(Default)]
pub struct Vecs<'a> {
    pub id_to_index_to_vec: BTreeMap<&'a str, IndexToVec<'a>>,
    pub index_to_id_to_vec: BTreeMap<Index, IdToVec<'a>>,
    pub ids: Vec<&'a str>,
    pub indexes: Vec<&'static str>,
    pub accepted_indexes: BTreeMap<&'static str, &'static [&'static str]>,
    pub index_count: usize,
    pub id_count: usize,
    pub vec_count: usize,
    serialized_id_to_indexes: BTreeMap<&'a str, Vec<&'static str>>,
    serialized_indexes_to_ids: BTreeMap<&'static str, Vec<&'a str>>,
}

impl<'a> Vecs<'a> {
    pub fn build(indexer: &'a Indexer, computer: &'a Computer) -> Self {
        let mut this = Vecs::default();

        indexer
            .vecs
            .vecs()
            .into_iter()
            .for_each(|vec| this.insert(vec));

        computer
            .vecs
            .vecs()
            .into_iter()
            .for_each(|vec| this.insert(vec));

        let mut ids = this.id_to_index_to_vec.keys().cloned().collect::<Vec<_>>();
        ids.sort_unstable_by(|a, b| {
            let len_cmp = a.len().cmp(&b.len());
            if len_cmp == std::cmp::Ordering::Equal {
                a.cmp(b)
            } else {
                len_cmp
            }
        });

        this.ids = ids;
        this.id_count = this.index_to_id_to_vec.keys().count();
        this.index_count = this.index_to_id_to_vec.keys().count();
        this.vec_count = this
            .index_to_id_to_vec
            .values()
            .map(|tree| tree.len())
            .sum::<usize>();
        this.indexes = this
            .index_to_id_to_vec
            .keys()
            .map(|i| i.serialize_long())
            .collect::<Vec<_>>();
        this.accepted_indexes = this
            .index_to_id_to_vec
            .keys()
            .map(|i| (i.serialize_long(), i.possible_values()))
            .collect::<BTreeMap<_, _>>();
        this.serialized_id_to_indexes = this
            .id_to_index_to_vec
            .iter()
            .map(|(id, index_to_vec)| {
                (
                    *id,
                    index_to_vec
                        .keys()
                        .map(|i| i.serialize_long())
                        .collect::<Vec<_>>(),
                )
            })
            .collect();
        this.serialized_indexes_to_ids = this
            .index_to_id_to_vec
            .iter()
            .map(|(index, id_to_vec)| {
                (
                    index.serialize_long(),
                    id_to_vec.keys().cloned().collect::<Vec<_>>(),
                )
            })
            .collect();

        this
    }

    // Not the most performant or type safe but only built once so that's okay
    fn insert(&mut self, vec: &'a dyn AnyCollectableVec) {
        let name = vec.name();
        let serialized_index = vec.index_type_to_string();
        let split = name.split("_to_").collect::<Vec<_>>();
        if split.len() != 1
            && !(split.len() == 2
                && split.first().is_some_and(|s| {
                    s == &"up"
                        || s == &"start"
                        || s.ends_with("relative")
                        || s.starts_with("from")
                        || s == &"cumulative_up"
                        || s.starts_with("cumulative_start")
                        || s.starts_with("cumulative_from")
                        || s == &"activity"
                }))
            && !(split.len() == 3
                && split.first().is_some_and(|s| {
                    s == &"up"
                        || s == &"start"
                        || s.starts_with("from")
                        || s == &"cumulative_up"
                        || s == &"cumulative_start"
                        || s.starts_with("cumulative_from")
                })
                && split.get(1).is_some_and(|s| s.ends_with("relative")))
        {
            dbg!((&serialized_index, &name, &split));
            unreachable!();
        }
        let index = Index::try_from(serialized_index)
            .inspect_err(|_| {
                dbg!(&serialized_index);
            })
            .unwrap();
        let prev = self
            .id_to_index_to_vec
            .entry(name)
            .or_default()
            .insert(index, vec);
        if prev.is_some() {
            dbg!(serialized_index, name);
            panic!()
        }
        let prev = self
            .index_to_id_to_vec
            .entry(index)
            .or_default()
            .insert(name, vec);
        if prev.is_some() {
            dbg!(serialized_index, name);
            panic!()
        }
    }

    pub fn ids(&self, pagination: Pagination) -> &[&'_ str] {
        let len = self.ids.len();
        let start = pagination.start(len);
        let end = pagination.end(len);
        &self.ids[start..end]
    }

    pub fn ids_to_indexes(&self, pagination: Pagination) -> BTreeMap<&'_ str, Vec<&'static str>> {
        let len = self.serialized_id_to_indexes.len();
        let start = pagination.start(len);
        let end = pagination.end(len);
        self.serialized_id_to_indexes
            .iter()
            .skip(start)
            .take(end)
            .map(|(ids, indexes)| (*ids, indexes.clone()))
            .collect()
    }

    pub fn indexes_to_ids(&self, pagination: Pagination) -> BTreeMap<&'static str, Vec<&'a str>> {
        let len = self.serialized_indexes_to_ids.len();
        let start = pagination.start(len);
        let end = pagination.end(len);
        self.serialized_indexes_to_ids
            .iter()
            .skip(start)
            .take(end)
            .map(|(index, ids)| (*index, ids.clone()))
            .collect()
    }
}

#[derive(Default, Deref, DerefMut)]
pub struct IndexToVec<'a>(BTreeMap<Index, &'a dyn AnyCollectableVec>);

#[derive(Default, Deref, DerefMut)]
pub struct IdToVec<'a>(BTreeMap<&'a str, &'a dyn AnyCollectableVec>);
