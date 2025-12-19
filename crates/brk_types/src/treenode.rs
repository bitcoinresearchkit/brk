use std::{
    collections::{BTreeMap, BTreeSet},
    sync::LazyLock,
};

use schemars::JsonSchema;
use serde::Serialize;

use super::Index;

/// Leaf node containing metric metadata
#[derive(Debug, Clone, Serialize, PartialEq, Eq, JsonSchema)]
pub struct MetricLeaf {
    /// The metric name/identifier
    pub name: String,
    /// The value type (e.g., "Sats", "StoredF64")
    pub value_type: String,
    /// Available indexes for this metric
    pub indexes: BTreeSet<Index>,
}

impl MetricLeaf {
    pub fn new(name: String, value_type: String, indexes: BTreeSet<Index>) -> Self {
        Self {
            name,
            value_type,
            indexes,
        }
    }

    /// Merge another leaf's indexes into this one (union)
    pub fn merge_indexes(&mut self, other: &MetricLeaf) {
        self.indexes.extend(other.indexes.iter().copied());
    }
}

/// MetricLeaf with JSON Schema for client generation
#[derive(Debug, Clone, Serialize)]
pub struct MetricLeafWithSchema {
    /// The core metric metadata
    #[serde(flatten)]
    pub leaf: MetricLeaf,
    /// JSON Schema for the value type
    #[serde(skip)]
    pub schema: serde_json::Value,
}

impl MetricLeafWithSchema {
    pub fn new(leaf: MetricLeaf, schema: serde_json::Value) -> Self {
        Self { leaf, schema }
    }

    /// The metric name/identifier
    pub fn name(&self) -> &str {
        &self.leaf.name
    }

    /// The value type (e.g., "Sats", "StoredF64")
    pub fn value_type(&self) -> &str {
        &self.leaf.value_type
    }

    /// Available indexes for this metric
    pub fn indexes(&self) -> &BTreeSet<Index> {
        &self.leaf.indexes
    }

    /// Check if this leaf refers to the same metric as another
    pub fn is_same_metric(&self, other: &MetricLeafWithSchema) -> bool {
        self.leaf.name == other.leaf.name
    }

    /// Merge another leaf's indexes into this one (union)
    pub fn merge_indexes(&mut self, other: &MetricLeafWithSchema) {
        self.leaf.merge_indexes(&other.leaf);
    }
}

impl PartialEq for MetricLeafWithSchema {
    fn eq(&self, other: &Self) -> bool {
        self.leaf == other.leaf
    }
}

impl Eq for MetricLeafWithSchema {}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(untagged)]
/// Hierarchical tree node for organizing metrics into categories
pub enum TreeNode {
    /// Branch node containing subcategories
    Branch(BTreeMap<String, TreeNode>),
    /// Leaf node containing metric metadata with schema
    Leaf(MetricLeafWithSchema),
}

const BASE: &str = "base";

/// List of prefixes to remove during simplification
static PREFIXES: LazyLock<Vec<String>> = LazyLock::new(|| {
    ["indexes", "timeindexes", "chainindexes"]
        .into_iter()
        .chain(Index::all().iter().map(|i| i.serialize_long()))
        .map(|s| format!("{s}_to_"))
        .collect()
});

impl TreeNode {
    pub fn is_empty(&self) -> bool {
        if let Self::Branch(tree) = self {
            tree.is_empty()
        } else {
            false
        }
    }

    pub fn as_mut_branch(&mut self) -> &mut BTreeMap<String, TreeNode> {
        match self {
            Self::Branch(b) => b,
            _ => panic!(),
        }
    }

    /// Merges all first-level branches into a single flattened structure.
    /// Root-level leaves are placed under BASE key.
    /// Returns None if conflicts are found (same key with incompatible values).
    pub fn merge_branches(&self) -> Option<Self> {
        let Self::Branch(tree) = self else {
            return Some(self.clone());
        };

        let mut merged: BTreeMap<String, TreeNode> = BTreeMap::new();

        for node in tree.values() {
            match node {
                Self::Leaf(leaf) => {
                    Self::merge_node(&mut merged, BASE, &Self::Leaf(leaf.clone()))?;
                }
                Self::Branch(inner) => {
                    for (key, inner_node) in inner {
                        Self::merge_node(&mut merged, key, inner_node)?;
                    }
                }
            }
        }

        let result = Self::Branch(merged);

        // Check if all leaves have the same name (can be collapsed)
        if let Some(common_leaf) = result.all_leaves_same() {
            Some(Self::Leaf(common_leaf))
        } else {
            Some(result)
        }
    }

    /// Checks if all leaves in the tree have the same metric name.
    /// Returns Some(merged_leaf) if all leaves have the same name, None otherwise.
    /// When merging, indexes are unioned together.
    fn all_leaves_same(&self) -> Option<MetricLeafWithSchema> {
        match self {
            Self::Leaf(leaf) => Some(leaf.clone()),
            Self::Branch(map) => {
                let mut common_leaf: Option<MetricLeafWithSchema> = None;

                for node in map.values() {
                    let node_leaf = node.all_leaves_same()?;

                    match &mut common_leaf {
                        None => common_leaf = Some(node_leaf),
                        Some(existing) if existing.is_same_metric(&node_leaf) => {
                            existing.merge_indexes(&node_leaf);
                        }
                        Some(_) => return None,
                    }
                }

                common_leaf
            }
        }
    }

    /// Merges a node into the target map at the given key.
    /// Returns None if there's a conflict.
    fn merge_node(
        target: &mut BTreeMap<String, TreeNode>,
        key: &str,
        node: &TreeNode,
    ) -> Option<()> {
        match target.get_mut(key) {
            None => {
                target.insert(key.to_string(), node.clone());
                Some(())
            }
            Some(existing) => {
                match (&mut *existing, node) {
                    (Self::Leaf(a), Self::Leaf(b)) if a.is_same_metric(b) => {
                        a.merge_indexes(b);
                        Some(())
                    }
                    (Self::Leaf(a), Self::Leaf(b)) => {
                        eprintln!("Conflict: Different leaf values for key '{key}'");
                        eprintln!("  Existing: {a:?}");
                        eprintln!("  New: {b:?}");
                        None
                    }
                    (Self::Leaf(leaf), Self::Branch(branch)) => {
                        let mut new_branch = BTreeMap::new();
                        new_branch.insert(BASE.to_string(), Self::Leaf(leaf.clone()));

                        for (k, v) in branch {
                            Self::merge_node(&mut new_branch, k, v)?;
                        }

                        *existing = Self::Branch(new_branch);
                        Some(())
                    }
                    (Self::Branch(existing_branch), Self::Leaf(leaf)) => {
                        Self::merge_node(existing_branch, BASE, &Self::Leaf(leaf.clone()))?;
                        Some(())
                    }
                    // Both branches: merge recursively
                    (Self::Branch(existing_branch), Self::Branch(new_inner)) => {
                        for (k, v) in new_inner {
                            Self::merge_node(existing_branch, k, v)?;
                        }
                        Some(())
                    }
                }
            }
        }
    }

    /// Recursively simplifies the tree by removing known prefixes from keys.
    /// If multiple keys map to the same simplified name, checks for conflicts.
    /// Returns None if there are conflicts (same simplified key, different values).
    pub fn simplify(&self) -> Option<Self> {
        match self {
            Self::Leaf(value) => Some(Self::Leaf(value.clone())),
            Self::Branch(map) => {
                let mut simplified: BTreeMap<String, TreeNode> = BTreeMap::new();

                for (key, node) in map {
                    // Recursively simplify the child node first
                    let simplified_node = node.simplify()?;

                    // Remove prefixes from the key
                    let simplified_key = PREFIXES
                        .iter()
                        .find_map(|prefix| key.strip_prefix(prefix))
                        .map(String::from)
                        .unwrap_or_else(|| key.clone());

                    // Try to merge into the result
                    Self::merge_node(&mut simplified, &simplified_key, &simplified_node)?;
                }

                Some(Self::Branch(simplified))
            }
        }
    }
}
