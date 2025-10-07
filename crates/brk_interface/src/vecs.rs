use std::collections::BTreeMap;

use brk_computer::Computer;
use brk_indexer::Indexer;
use brk_traversable::{Traversable, TreeNode};
use derive_deref::{Deref, DerefMut};
use schemars::JsonSchema;
use serde::Serialize;
use vecdb::AnyCollectableVec;

use crate::{
    index::Indexes,
    pagination::{PaginatedIndexParam, PaginationParam},
};

use super::index::Index;

#[derive(Default)]
pub struct Vecs<'a> {
    pub metric_to_index_to_vec: BTreeMap<&'a str, IndexToVec<'a>>,
    pub index_to_metric_to_vec: BTreeMap<Index, MetricToVec<'a>>,
    pub metrics: Vec<&'a str>,
    pub indexes: Indexes,
    pub distinct_metric_count: usize,
    pub total_metric_count: usize,
    pub catalog: Option<TreeNode>,
    metric_to_indexes: BTreeMap<&'a str, Vec<&'static str>>,
    index_to_metrics: BTreeMap<Index, Vec<&'a str>>,
}

impl<'a> Vecs<'a> {
    pub fn build(indexer: &'a Indexer, computer: &'a Computer) -> Self {
        let mut this = Vecs::default();

        indexer
            .vecs
            .iter_any_collectable()
            .for_each(|vec| this.insert(vec));

        computer
            .iter_any_collectable()
            .for_each(|vec| this.insert(vec));

        let mut ids = this
            .metric_to_index_to_vec
            .keys()
            .cloned()
            .collect::<Vec<_>>();

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

        this.metrics = ids;
        this.distinct_metric_count = this.metric_to_index_to_vec.keys().count();
        this.total_metric_count = this
            .index_to_metric_to_vec
            .values()
            .map(|tree| tree.len())
            .sum::<usize>();
        this.indexes = Indexes::new(
            this.index_to_metric_to_vec
                .keys()
                .map(|i| (*i, i.possible_values()))
                .collect::<BTreeMap<_, _>>(),
        );
        this.metric_to_indexes = this
            .metric_to_index_to_vec
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
        this.index_to_metrics = this
            .index_to_metric_to_vec
            .iter()
            .map(|(index, id_to_vec)| (*index, id_to_vec.keys().cloned().collect::<Vec<_>>()))
            .collect();
        this.index_to_metrics
            .values_mut()
            .for_each(|ids| sort_ids(ids));
        this.catalog.replace(TreeNode::Branch(
            [
                ("indexer".to_string(), indexer.vecs.to_tree_node()),
                ("computer".to_string(), computer.to_tree_node()),
            ]
            .into_iter()
            .collect(),
        ));

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
            .metric_to_index_to_vec
            .entry(name)
            .or_default()
            .insert(index, vec);
        if prev.is_some() {
            dbg!(serialized_index, name);
            panic!()
        }
        let prev = self
            .index_to_metric_to_vec
            .entry(index)
            .or_default()
            .insert(name, vec);
        if prev.is_some() {
            dbg!(serialized_index, name);
            panic!()
        }
    }

    pub fn metrics(&'static self, pagination: PaginationParam) -> PaginatedMetrics {
        let len = self.metrics.len();
        let start = pagination.start(len);
        let end = pagination.end(len);

        PaginatedMetrics {
            current_page: pagination.page.unwrap_or_default(),
            total_pages: len / PaginationParam::PER_PAGE,
            metrics: &self.metrics[start..end],
        }
    }

    pub fn metric_to_indexes(&self, metric: String) -> Option<&Vec<&'static str>> {
        self.metric_to_indexes
            .get(metric.replace("-", "_").as_str())
    }

    pub fn index_to_ids(
        &self,
        PaginatedIndexParam { index, pagination }: PaginatedIndexParam,
    ) -> Vec<&'a str> {
        let vec = self.index_to_metrics.get(&index).unwrap();

        let len = vec.len();
        let start = pagination.start(len);
        let end = pagination.end(len);

        vec.iter().skip(start).take(end).cloned().collect()
    }
}

#[derive(Default, Deref, DerefMut)]
pub struct IndexToVec<'a>(BTreeMap<Index, &'a dyn AnyCollectableVec>);

#[derive(Default, Deref, DerefMut)]
pub struct MetricToVec<'a>(BTreeMap<&'a str, &'a dyn AnyCollectableVec>);

/// A paginated list of available metric names (1000 per page)
#[derive(Debug, Serialize, JsonSchema)]
pub struct PaginatedMetrics {
    /// Current page number (0-indexed)
    #[schemars(example = 0)]
    current_page: usize,
    /// Total number of pages available
    #[schemars(example = 21000)]
    total_pages: usize,
    /// List of metric names (max 1000 per page)
    #[schemars(example = ["price_open", "price_close", "realized_price", "..."])]
    metrics: &'static [&'static str],
}
