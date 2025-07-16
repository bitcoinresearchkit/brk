use std::collections::BTreeMap;

use brk_computer::Computer;
use brk_indexer::Indexer;
use brk_vec::AnyCollectableVec;
use derive_deref::{Deref, DerefMut};

use crate::pagination::{PaginatedIndexParam, PaginationParam};

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
    id_to_indexes: BTreeMap<&'a str, Vec<&'static str>>,
    indexes_to_ids: BTreeMap<Index, Vec<&'a str>>,
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

        let sort_ids = |ids: &mut Vec<&str>| {
            ids.sort_unstable_by(|a, b| {
                let len_cmp = a.len().cmp(&b.len());
                if len_cmp == std::cmp::Ordering::Equal {
                    a.cmp(b)
                } else {
                    len_cmp
                }
            })
        };

        sort_ids(&mut ids);

        this.ids = ids;
        this.id_count = this.id_to_index_to_vec.keys().count();
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
        this.id_to_indexes = this
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
        this.indexes_to_ids = this
            .index_to_id_to_vec
            .iter()
            .map(|(index, id_to_vec)| (*index, id_to_vec.keys().cloned().collect::<Vec<_>>()))
            .collect();
        this.indexes_to_ids
            .values_mut()
            .for_each(|ids| sort_ids(ids));

        this
    }

    // Not the most performant or type safe but only built once so that's okay
    fn insert(&mut self, vec: &'a dyn AnyCollectableVec) {
        let name = vec.name();
        let serialized_index = vec.index_type_to_string();
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

    pub fn ids(&self, pagination: PaginationParam) -> &[&'_ str] {
        let len = self.ids.len();
        let start = pagination.start(len);
        let end = pagination.end(len);
        &self.ids[start..end]
    }

    pub fn id_to_indexes(&self, id: String) -> Option<&Vec<&'static str>> {
        self.id_to_indexes.get(id.as_str())
    }

    pub fn index_to_ids(
        &self,
        PaginatedIndexParam { index, pagination }: PaginatedIndexParam,
    ) -> Vec<&'a str> {
        let vec = self.indexes_to_ids.get(&index).unwrap();

        let len = vec.len();
        let start = pagination.start(len);
        let end = pagination.end(len);

        vec.iter().skip(start).take(end).cloned().collect()
    }
}

#[derive(Default, Deref, DerefMut)]
pub struct IndexToVec<'a>(BTreeMap<Index, &'a dyn AnyCollectableVec>);

#[derive(Default, Deref, DerefMut)]
pub struct IdToVec<'a>(BTreeMap<&'a str, &'a dyn AnyCollectableVec>);
