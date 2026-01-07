use std::{borrow::Cow, collections::BTreeMap};

use brk_computer::Computer;
use brk_indexer::Indexer;
use brk_traversable::{Traversable, TreeNode};
use brk_types::{
    Index, IndexInfo, Limit, Metric, MetricCount, PaginatedMetrics, Pagination, PaginationIndex,
};
use derive_more::{Deref, DerefMut};
use quickmatch::{QuickMatch, QuickMatchConfig};
use vecdb::AnyExportableVec;

#[derive(Default)]
pub struct Vecs<'a> {
    pub metric_to_index_to_vec: BTreeMap<&'a str, IndexToVec<'a>>,
    pub index_to_metric_to_vec: BTreeMap<Index, MetricToVec<'a>>,
    pub metrics: Vec<&'a str>,
    pub indexes: Vec<IndexInfo>,
    pub counts: MetricCount,
    pub counts_by_db: BTreeMap<String, MetricCount>,
    catalog: Option<TreeNode>,
    matcher: Option<QuickMatch<'a>>,
    metric_to_indexes: BTreeMap<&'a str, Vec<Index>>,
    index_to_metrics: BTreeMap<Index, Vec<&'a str>>,
}

impl<'a> Vecs<'a> {
    pub fn build(indexer: &'a Indexer, computer: &'a Computer) -> Self {
        let mut this = Vecs::default();

        indexer
            .vecs
            .iter_any_exportable()
            .for_each(|vec| this.insert(vec, "indexed"));

        computer
            .iter_named_exportable()
            .for_each(|(db, vec)| this.insert(vec, db));

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
        this.counts.distinct_metrics = this.metric_to_index_to_vec.keys().count();
        this.counts.total_endpoints = this
            .index_to_metric_to_vec
            .values()
            .map(|tree| tree.len())
            .sum::<usize>();
        this.counts.lazy_endpoints = this
            .index_to_metric_to_vec
            .values()
            .flat_map(|tree| tree.values())
            .filter(|vec| vec.region_names().is_empty())
            .count();
        this.counts.stored_endpoints = this.counts.total_endpoints - this.counts.lazy_endpoints;
        this.indexes = this
            .index_to_metric_to_vec
            .keys()
            .map(|i| IndexInfo {
                index: *i,
                aliases: i
                    .possible_values()
                    .iter()
                    .map(|v| Cow::Borrowed(*v))
                    .collect(),
            })
            .collect();

        this.metric_to_indexes = this
            .metric_to_index_to_vec
            .iter()
            .map(|(id, index_to_vec)| (*id, index_to_vec.keys().copied().collect::<Vec<_>>()))
            .collect();
        this.index_to_metrics = this
            .index_to_metric_to_vec
            .iter()
            .map(|(index, id_to_vec)| (*index, id_to_vec.keys().cloned().collect::<Vec<_>>()))
            .collect();
        this.index_to_metrics.values_mut().for_each(sort_ids);
        this.catalog.replace(
            TreeNode::Branch(
                [
                    ("indexed".to_string(), indexer.vecs.to_tree_node()),
                    ("computed".to_string(), computer.to_tree_node()),
                ]
                .into_iter()
                .collect(),
            )
            .simplify()
            .unwrap(),
        );
        this.matcher = Some(QuickMatch::new(&this.metrics));

        this
    }

    fn insert(&mut self, vec: &'a dyn AnyExportableVec, db: &str) {
        let name = vec.name();
        let serialized_index = vec.index_type_to_string();
        let index = Index::try_from(serialized_index)
            .unwrap_or_else(|_| panic!("Unknown index type: {serialized_index}"));

        let prev = self
            .metric_to_index_to_vec
            .entry(name)
            .or_default()
            .insert(index, vec);
        assert!(
            prev.is_none(),
            "Duplicate metric: {name} for index {index:?}"
        );

        let prev = self
            .index_to_metric_to_vec
            .entry(index)
            .or_default()
            .insert(name, vec);
        assert!(
            prev.is_none(),
            "Duplicate metric: {name} for index {index:?}"
        );

        // Track per-db counts
        let is_lazy = vec.region_names().is_empty();
        self.counts_by_db
            .entry(db.to_string())
            .or_default()
            .add_endpoint(is_lazy);
    }

    pub fn metrics(&'static self, pagination: Pagination) -> PaginatedMetrics {
        let len = self.metrics.len();
        let start = pagination.start(len);
        let end = pagination.end(len);

        PaginatedMetrics {
            current_page: pagination.page(),
            max_page: len.div_ceil(Pagination::PER_PAGE).saturating_sub(1),
            metrics: self.metrics[start..end]
                .iter()
                .map(|&s| Cow::Borrowed(s))
                .collect(),
        }
    }

    pub fn metric_to_indexes(&self, metric: Metric) -> Option<&Vec<Index>> {
        self.metric_to_indexes
            .get(metric.replace("-", "_").as_str())
    }

    pub fn index_to_ids(
        &self,
        PaginationIndex { index, pagination }: PaginationIndex,
    ) -> Option<&[&'a str]> {
        let vec = self.index_to_metrics.get(&index)?;

        let len = vec.len();
        let start = pagination.start(len);
        let end = pagination.end(len);

        Some(&vec[start..end])
    }

    pub fn catalog(&self) -> &TreeNode {
        self.catalog.as_ref().expect("catalog not initialized")
    }

    pub fn matches(&self, metric: &Metric, limit: Limit) -> Vec<&'_ str> {
        self.matcher
            .as_ref()
            .expect("matcher not initialized")
            .matches_with(metric, &QuickMatchConfig::new().with_limit(*limit))
    }

    /// Look up a vec by metric name and index
    pub fn get(&self, metric: &Metric, index: Index) -> Option<&'a dyn AnyExportableVec> {
        let metric_name = metric.replace("-", "_");
        self.metric_to_index_to_vec
            .get(metric_name.as_str())
            .and_then(|index_to_vec| index_to_vec.get(&index).copied())
    }
}

#[derive(Default, Deref, DerefMut)]
pub struct IndexToVec<'a>(BTreeMap<Index, &'a dyn AnyExportableVec>);

#[derive(Default, Deref, DerefMut)]
pub struct MetricToVec<'a>(BTreeMap<&'a str, &'a dyn AnyExportableVec>);
