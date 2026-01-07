use std::{collections::BTreeMap, fmt::Debug};

pub use brk_types::{Index, MetricLeaf, MetricLeafWithSchema, TreeNode};

#[cfg(feature = "derive")]
pub use brk_traversable_derive::Traversable;
use schemars::JsonSchema;
use serde::Serialize;
use vecdb::{
    AnyExportableVec, AnyVec, BytesVec, BytesVecValue, EagerVec, Formattable, LazyVecFrom1,
    LazyVecFrom2, LazyVecFrom3, StoredVec, VecIndex, VecValue,
};

pub trait Traversable {
    fn to_tree_node(&self) -> TreeNode;
    fn iter_any_exportable(&self) -> impl Iterator<Item = &dyn AnyExportableVec>;
}

/// Helper to create a MetricLeafWithSchema from a vec
fn make_leaf<I: VecIndex, T: JsonSchema, V: AnyVec>(vec: &V) -> TreeNode {
    let index_str = I::to_string();
    let index = Index::try_from(index_str).ok();
    let indexes = index.into_iter().collect();

    let leaf = MetricLeaf::new(
        vec.name().to_string(),
        vec.value_type_to_string().to_string(),
        indexes,
    );

    let schema = schemars::SchemaGenerator::default().into_root_schema_for::<T>();
    let schema_json = serde_json::to_value(schema).unwrap_or_default();

    TreeNode::Leaf(MetricLeafWithSchema::new(leaf, schema_json))
}

// BytesVec implementation
impl<I, T> Traversable for BytesVec<I, T>
where
    I: VecIndex,
    T: BytesVecValue + Formattable + Serialize + JsonSchema,
{
    fn iter_any_exportable(&self) -> impl Iterator<Item = &dyn AnyExportableVec> {
        std::iter::once(self as &dyn AnyExportableVec)
    }

    fn to_tree_node(&self) -> TreeNode {
        make_leaf::<I, T, _>(self)
    }
}

// ZeroCopyVec implementation (only if zerocopy feature enabled)
#[cfg(feature = "zerocopy")]
impl<I, T> Traversable for vecdb::ZeroCopyVec<I, T>
where
    I: VecIndex,
    T: vecdb::ZeroCopyVecValue + Formattable + Serialize + JsonSchema,
{
    fn iter_any_exportable(&self) -> impl Iterator<Item = &dyn AnyExportableVec> {
        std::iter::once(self as &dyn AnyExportableVec)
    }

    fn to_tree_node(&self) -> TreeNode {
        make_leaf::<I, T, _>(self)
    }
}

// PcoVec implementation (only if pco feature enabled)
#[cfg(feature = "pco")]
impl<I, T> Traversable for vecdb::PcoVec<I, T>
where
    I: VecIndex,
    T: vecdb::PcoVecValue + Formattable + Serialize + JsonSchema,
{
    fn iter_any_exportable(&self) -> impl Iterator<Item = &dyn AnyExportableVec> {
        std::iter::once(self as &dyn AnyExportableVec)
    }

    fn to_tree_node(&self) -> TreeNode {
        make_leaf::<I, T, _>(self)
    }
}

// LZ4Vec implementation (only if lz4 feature enabled)
#[cfg(feature = "lz4")]
impl<I, T> Traversable for vecdb::LZ4Vec<I, T>
where
    I: VecIndex,
    T: vecdb::LZ4VecValue + Formattable + Serialize + JsonSchema,
{
    fn iter_any_exportable(&self) -> impl Iterator<Item = &dyn AnyExportableVec> {
        std::iter::once(self as &dyn AnyExportableVec)
    }

    fn to_tree_node(&self) -> TreeNode {
        make_leaf::<I, T, _>(self)
    }
}

// ZstdVec implementation (only if zstd feature enabled)
#[cfg(feature = "zstd")]
impl<I, T> Traversable for vecdb::ZstdVec<I, T>
where
    I: VecIndex,
    T: vecdb::ZstdVecValue + Formattable + Serialize + JsonSchema,
{
    fn iter_any_exportable(&self) -> impl Iterator<Item = &dyn AnyExportableVec> {
        std::iter::once(self as &dyn AnyExportableVec)
    }

    fn to_tree_node(&self) -> TreeNode {
        make_leaf::<I, T, _>(self)
    }
}

// EagerVec implementation (wraps any stored vector)
impl<V> Traversable for EagerVec<V>
where
    V: StoredVec,
    V::T: Formattable + Serialize + JsonSchema,
{
    fn iter_any_exportable(&self) -> impl Iterator<Item = &dyn AnyExportableVec> {
        std::iter::once(self as &dyn AnyExportableVec)
    }

    fn to_tree_node(&self) -> TreeNode {
        make_leaf::<V::I, V::T, _>(self)
    }
}

impl<I, T, S1I, S1T> Traversable for LazyVecFrom1<I, T, S1I, S1T>
where
    I: VecIndex,
    T: VecValue + Formattable + Serialize + JsonSchema,
    S1I: VecIndex,
    S1T: VecValue,
{
    fn iter_any_exportable(&self) -> impl Iterator<Item = &dyn AnyExportableVec> {
        std::iter::once(self as &dyn AnyExportableVec)
    }

    fn to_tree_node(&self) -> TreeNode {
        make_leaf::<I, T, _>(self)
    }
}

impl<I, T, S1I, S1T, S2I, S2T> Traversable for LazyVecFrom2<I, T, S1I, S1T, S2I, S2T>
where
    I: VecIndex,
    T: VecValue + Formattable + Serialize + JsonSchema,
    S1I: VecIndex,
    S1T: VecValue,
    S2I: VecIndex,
    S2T: VecValue,
{
    fn iter_any_exportable(&self) -> impl Iterator<Item = &dyn AnyExportableVec> {
        std::iter::once(self as &dyn AnyExportableVec)
    }

    fn to_tree_node(&self) -> TreeNode {
        make_leaf::<I, T, _>(self)
    }
}

impl<I, T, S1I, S1T, S2I, S2T, S3I, S3T> Traversable
    for LazyVecFrom3<I, T, S1I, S1T, S2I, S2T, S3I, S3T>
where
    I: VecIndex,
    T: VecValue + Formattable + Serialize + JsonSchema,
    S1I: VecIndex,
    S1T: VecValue,
    S2I: VecIndex,
    S2T: VecValue,
    S3I: VecIndex,
    S3T: VecValue,
{
    fn iter_any_exportable(&self) -> impl Iterator<Item = &dyn AnyExportableVec> {
        std::iter::once(self as &dyn AnyExportableVec)
    }

    fn to_tree_node(&self) -> TreeNode {
        make_leaf::<I, T, _>(self)
    }
}

impl<T: Traversable + ?Sized> Traversable for Box<T> {
    fn to_tree_node(&self) -> TreeNode {
        (**self).to_tree_node()
    }

    fn iter_any_exportable(&self) -> impl Iterator<Item = &dyn AnyExportableVec> {
        (**self).iter_any_exportable()
    }
}

impl<T: Traversable> Traversable for Option<T> {
    fn to_tree_node(&self) -> TreeNode {
        match self {
            Some(inner) => inner.to_tree_node(),
            None => TreeNode::Branch(BTreeMap::new()),
        }
    }

    fn iter_any_exportable(&self) -> impl Iterator<Item = &dyn AnyExportableVec> {
        match self {
            Some(inner) => Box::new(inner.iter_any_exportable())
                as Box<dyn Iterator<Item = &dyn AnyExportableVec>>,
            None => Box::new(std::iter::empty()),
        }
    }
}

impl<K: Debug, V: Traversable> Traversable for BTreeMap<K, V> {
    fn to_tree_node(&self) -> TreeNode {
        let children = self
            .iter()
            .map(|(k, v)| (format!("{:?}", k), v.to_tree_node()))
            .collect();
        TreeNode::Branch(children)
    }

    fn iter_any_exportable(&self) -> impl Iterator<Item = &dyn AnyExportableVec> {
        let mut iter: Box<dyn Iterator<Item = &dyn AnyExportableVec>> =
            Box::new(std::iter::empty());
        for v in self.values() {
            iter = Box::new(iter.chain(v.iter_any_exportable()));
        }
        iter
    }
}

/// Unit type implementation - used as ZST placeholder for disabled features
/// (e.g., Unpriced variants where dollar fields are not needed)
impl Traversable for () {
    fn to_tree_node(&self) -> TreeNode {
        TreeNode::Branch(BTreeMap::new())
    }

    fn iter_any_exportable(&self) -> impl Iterator<Item = &dyn AnyExportableVec> {
        std::iter::empty()
    }
}
