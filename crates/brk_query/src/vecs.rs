use std::{borrow::Cow, collections::BTreeMap};

use brk_computer::Computer;
use brk_indexer::Indexer;
use brk_traversable::{Traversable, TreeNode};
use brk_types::{
    Index, IndexInfo, Limit, PaginatedSeries, Pagination, PaginationIndex, Series, SeriesCount,
};
use derive_more::{Deref, DerefMut};
use quickmatch::{QuickMatch, QuickMatchConfig};
use vecdb::AnyExportableVec;

#[derive(Default)]
pub struct Vecs<'a> {
    pub series_to_index_to_vec: BTreeMap<&'a str, IndexToVec<'a>>,
    pub index_to_series_to_vec: BTreeMap<Index, SeriesToVec<'a>>,
    pub series: Vec<&'a str>,
    pub indexes: Vec<IndexInfo>,
    pub counts: SeriesCount,
    pub counts_by_db: BTreeMap<String, SeriesCount>,
    catalog: Option<TreeNode>,
    matcher: Option<QuickMatch<'a>>,
    series_to_indexes: BTreeMap<&'a str, Vec<Index>>,
    index_to_series: BTreeMap<Index, Vec<&'a str>>,
}

impl<'a> Vecs<'a> {
    pub fn build(indexer: &'a Indexer<vecdb::Ro>, computer: &'a Computer<vecdb::Ro>) -> Self {
        Self::build_from(
            indexer.vecs.iter_any_exportable(),
            indexer.vecs.to_tree_node(),
            computer.iter_named_exportable(),
            computer.to_tree_node(),
        )
    }

    pub fn build_rw(indexer: &'a Indexer, computer: &'a Computer) -> Self {
        Self::build_from(
            indexer.vecs.iter_any_exportable(),
            indexer.vecs.to_tree_node(),
            computer.iter_named_exportable(),
            computer.to_tree_node(),
        )
    }

    fn build_from(
        indexed_vecs: impl Iterator<Item = &'a dyn AnyExportableVec>,
        indexed_tree: TreeNode,
        computed_vecs: impl Iterator<Item = (&'static str, &'a dyn AnyExportableVec)>,
        computed_tree: TreeNode,
    ) -> Self {
        let mut this = Vecs::default();

        indexed_vecs.for_each(|vec| this.insert(vec, "indexed"));
        computed_vecs.for_each(|(db, vec)| this.insert(vec, db));

        let mut ids = this
            .series_to_index_to_vec
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

        this.series = ids;
        this.counts.distinct_series = this.series_to_index_to_vec.keys().count();
        this.counts.total_endpoints = this
            .index_to_series_to_vec
            .values()
            .map(|tree| tree.len())
            .sum::<usize>();
        this.counts.lazy_endpoints = this
            .index_to_series_to_vec
            .values()
            .flat_map(|tree| tree.values())
            .filter(|vec| vec.region_names().is_empty())
            .count();
        this.counts.stored_endpoints = this.counts.total_endpoints - this.counts.lazy_endpoints;
        this.indexes = this
            .index_to_series_to_vec
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

        this.series_to_indexes = this
            .series_to_index_to_vec
            .iter()
            .map(|(id, index_to_vec)| (*id, index_to_vec.keys().copied().collect::<Vec<_>>()))
            .collect();
        this.index_to_series = this
            .index_to_series_to_vec
            .iter()
            .map(|(index, id_to_vec)| (*index, id_to_vec.keys().cloned().collect::<Vec<_>>()))
            .collect();
        this.index_to_series.values_mut().for_each(sort_ids);
        this.catalog.replace(
            TreeNode::Branch(
                [
                    ("indexed".to_string(), indexed_tree),
                    ("computed".to_string(), computed_tree),
                ]
                .into_iter()
                .collect(),
            )
            .merge_branches()
            .unwrap(),
        );
        this.matcher = Some(QuickMatch::new(&this.series));

        this
    }

    fn insert(&mut self, vec: &'a dyn AnyExportableVec, db: &str) {
        let name = vec.name();
        let serialized_index = vec.index_type_to_string();
        let index = Index::try_from(serialized_index)
            .unwrap_or_else(|_| panic!("Unknown index type: {serialized_index}"));

        let prev = self
            .series_to_index_to_vec
            .entry(name)
            .or_default()
            .insert(index, vec);
        assert!(
            prev.is_none(),
            "Duplicate series: {name} for index {index:?}"
        );

        let prev = self
            .index_to_series_to_vec
            .entry(index)
            .or_default()
            .insert(name, vec);
        assert!(
            prev.is_none(),
            "Duplicate series: {name} for index {index:?}"
        );

        // Track per-db counts
        let is_lazy = vec.region_names().is_empty();
        self.counts_by_db
            .entry(db.to_string())
            .or_default()
            .add_endpoint(name, is_lazy);
    }

    pub fn series(&'static self, pagination: Pagination) -> PaginatedSeries {
        let len = self.series.len();
        let per_page = pagination.per_page();
        let start = pagination.start(len);
        let end = pagination.end(len);
        let max_page = len.div_ceil(per_page).saturating_sub(1);

        PaginatedSeries {
            current_page: pagination.page(),
            max_page,
            total_count: len,
            per_page,
            has_more: pagination.page() < max_page,
            series: self.series[start..end]
                .iter()
                .map(|&s| Cow::Borrowed(s))
                .collect(),
        }
    }

    pub fn series_to_indexes(&self, series: Series) -> Option<&Vec<Index>> {
        self.series_to_indexes
            .get(series.replace("-", "_").as_str())
    }

    pub fn index_to_ids(
        &self,
        PaginationIndex { index, pagination }: PaginationIndex,
    ) -> Option<&[&'a str]> {
        let vec = self.index_to_series.get(&index)?;

        let len = vec.len();
        let start = pagination.start(len);
        let end = pagination.end(len);

        Some(&vec[start..end])
    }

    pub fn catalog(&self) -> &TreeNode {
        self.catalog.as_ref().expect("catalog not initialized")
    }

    pub fn matches(&self, series: &Series, limit: Limit) -> Vec<&'_ str> {
        if limit.is_zero() {
            return Vec::new();
        }
        self.matcher
            .as_ref()
            .expect("matcher not initialized")
            .matches_with(series, &QuickMatchConfig::new().with_limit(*limit))
    }

    /// Look up a vec by series name and index
    pub fn get(&self, series: &Series, index: Index) -> Option<&'a dyn AnyExportableVec> {
        let series_name = series.replace("-", "_");
        self.series_to_index_to_vec
            .get(series_name.as_str())
            .and_then(|index_to_vec| index_to_vec.get(&index).copied())
    }
}

#[derive(Default, Deref, DerefMut)]
pub struct IndexToVec<'a>(BTreeMap<Index, &'a dyn AnyExportableVec>);

#[derive(Default, Deref, DerefMut)]
pub struct SeriesToVec<'a>(BTreeMap<&'a str, &'a dyn AnyExportableVec>);
