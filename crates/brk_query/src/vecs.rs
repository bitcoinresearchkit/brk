use std::{borrow::Cow, collections::BTreeMap};

use brk_computer::Computer;
use brk_indexer::Indexer;
use brk_traversable::{Traversable, TreeNode};
use brk_types::{
    Index, IndexInfo, Limit, PaginatedSeries, Pagination, SeriesCount, SeriesInfo, SeriesName,
};
use derive_more::{Deref, DerefMut};
use quickmatch::{QuickMatch, QuickMatchConfig};
use rustc_hash::{FxHashMap, FxHashSet};
use vecdb::{AnyExportableVec, Ro};

pub struct Vecs<'a> {
    pub series_to_index_to_vec: BTreeMap<&'a str, IndexToVec<'a>>,
    pub index_to_series_to_vec: BTreeMap<Index, SeriesToVec<'a>>,
    pub series: Vec<&'a str>,
    pub indexes: Vec<IndexInfo>,
    pub counts: SeriesCount,
    pub counts_by_db: BTreeMap<String, SeriesCount>,
    catalog: TreeNode,
    matcher: QuickMatch<'a>,
    series_to_indexes: BTreeMap<&'a str, Vec<Index>>,
}

impl<'a> Vecs<'a> {
    pub fn build(indexer: &'a Indexer<Ro>, computer: &'a Computer<Ro>) -> Self {
        Self::build_from(
            indexer.vecs.iter_any_visible(),
            indexer.vecs.to_tree_node(),
            computer.iter_named_visible(),
            computer.to_tree_node(),
        )
    }

    pub fn build_rw(indexer: &'a Indexer, computer: &'a Computer) -> Self {
        Self::build_from(
            indexer.vecs.iter_any_visible(),
            indexer.vecs.to_tree_node(),
            computer.iter_named_visible(),
            computer.to_tree_node(),
        )
    }

    fn build_from(
        indexed_vecs: impl Iterator<Item = &'a dyn AnyExportableVec>,
        indexed_tree: TreeNode,
        computed_vecs: impl Iterator<Item = (&'static str, &'a dyn AnyExportableVec)>,
        computed_tree: TreeNode,
    ) -> Self {
        let mut builder = Builder::default();
        indexed_vecs.for_each(|vec| builder.insert(vec, "indexed"));
        computed_vecs.for_each(|(db, vec)| builder.insert(vec, db));
        builder.counts.distinct_series = builder.series_to_index_to_vec.len();
        let Builder {
            series_to_index_to_vec,
            index_to_series_to_vec,
            counts,
            counts_by_db,
            ..
        } = builder;

        let sort_ids = |ids: &mut Vec<&str>| {
            ids.sort_unstable_by(|a, b| a.len().cmp(&b.len()).then_with(|| a.cmp(b)))
        };

        let mut series = series_to_index_to_vec.keys().copied().collect::<Vec<_>>();
        sort_ids(&mut series);

        let indexes = index_to_series_to_vec
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

        let series_to_indexes = series_to_index_to_vec
            .iter()
            .map(|(id, index_to_vec)| (*id, index_to_vec.keys().copied().collect::<Vec<_>>()))
            .collect();

        let catalog = TreeNode::Branch(
            [
                ("indexed".to_string(), indexed_tree),
                ("computed".to_string(), computed_tree),
            ]
            .into_iter()
            .collect(),
        )
        .merge_branches()
        .expect("indexed/computed catalog merge: same series leaf with incompatible schemas");

        let matcher = QuickMatch::new(&series);

        Self {
            series_to_index_to_vec,
            index_to_series_to_vec,
            series,
            indexes,
            counts,
            counts_by_db,
            catalog,
            matcher,
            series_to_indexes,
        }
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

    pub fn series_to_indexes(&self, series: &SeriesName) -> Option<&Vec<Index>> {
        self.series_to_indexes.get(series.normalize().as_ref())
    }

    pub fn series_info(&self, series: &SeriesName) -> Option<SeriesInfo> {
        let index_to_vec = self
            .series_to_index_to_vec
            .get(series.normalize().as_ref())?;
        let value_type = index_to_vec.values().next()?.value_type_to_string();
        let indexes = index_to_vec.keys().copied().collect();
        Some(SeriesInfo {
            indexes,
            value_type: value_type.into(),
        })
    }

    pub fn catalog(&self) -> &TreeNode {
        &self.catalog
    }

    pub fn matches(&self, series: &SeriesName, limit: Limit) -> Vec<&'_ str> {
        if limit.is_zero() {
            return Vec::new();
        }
        self.matcher
            .matches_with(series, &QuickMatchConfig::new().with_limit(*limit))
    }

    /// Look up a vec by series name and index. `series` is normalized (`-` → `_`, lowercased).
    pub fn get(&self, series: &SeriesName, index: Index) -> Option<&'a dyn AnyExportableVec> {
        self.series_to_index_to_vec
            .get(series.normalize().as_ref())
            .and_then(|index_to_vec| index_to_vec.get(&index).copied())
    }
}

#[derive(Default, Deref, DerefMut)]
pub struct IndexToVec<'a>(BTreeMap<Index, &'a dyn AnyExportableVec>);

#[derive(Default, Deref, DerefMut)]
pub struct SeriesToVec<'a>(BTreeMap<&'a str, &'a dyn AnyExportableVec>);

#[derive(Default)]
struct Builder<'a> {
    series_to_index_to_vec: BTreeMap<&'a str, IndexToVec<'a>>,
    index_to_series_to_vec: BTreeMap<Index, SeriesToVec<'a>>,
    counts: SeriesCount,
    counts_by_db: BTreeMap<String, SeriesCount>,
    seen_by_db: FxHashMap<&'a str, FxHashSet<&'a str>>,
}

impl<'a> Builder<'a> {
    fn insert(&mut self, vec: &'a dyn AnyExportableVec, db: &'a str) {
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

        self.index_to_series_to_vec
            .entry(index)
            .or_default()
            .insert(name, vec);

        let is_lazy = vec.region_names().is_empty();
        let by_db = self.counts_by_db.entry(db.to_string()).or_default();
        self.counts.total_endpoints += 1;
        by_db.total_endpoints += 1;
        if is_lazy {
            self.counts.lazy_endpoints += 1;
            by_db.lazy_endpoints += 1;
        } else {
            self.counts.stored_endpoints += 1;
            by_db.stored_endpoints += 1;
        }
        if self.seen_by_db.entry(db).or_default().insert(name) {
            by_db.distinct_series += 1;
        }
    }
}
