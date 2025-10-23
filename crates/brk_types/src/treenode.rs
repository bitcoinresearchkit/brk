use std::{collections::BTreeMap, sync::LazyLock};

use schemars::JsonSchema;
use serde::Serialize;

use super::Index;

#[derive(Debug, Clone, Serialize, PartialEq, Eq, JsonSchema)]
#[serde(untagged)]
/// Hierarchical tree node for organizing metrics into categories
pub enum TreeNode {
    /// Branch node containing subcategories
    Branch(BTreeMap<String, TreeNode>),
    /// Leaf node containing the metric name
    #[schemars(example = &"price_close", example = &"market_cap", example = &"realized_price")]
    Leaf(String),
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
                Self::Leaf(value) => {
                    Self::merge_node(&mut merged, BASE, &Self::Leaf(value.clone()))?;
                }
                Self::Branch(inner) => {
                    for (key, inner_node) in inner {
                        Self::merge_node(&mut merged, key, inner_node)?;
                    }
                }
            }
        }

        let result = Self::Branch(merged);

        // Check if all leaves have the same value
        if let Some(common_value) = result.all_leaves_same() {
            Some(Self::Leaf(common_value))
        } else {
            Some(result)
        }
    }

    /// Checks if all leaves in the tree have the same value.
    /// Returns Some(value) if all leaves are identical, None otherwise.
    fn all_leaves_same(&self) -> Option<String> {
        match self {
            Self::Leaf(value) => Some(value.clone()),
            Self::Branch(map) => {
                let mut common_value: Option<String> = None;

                for node in map.values() {
                    let node_value = node.all_leaves_same()?;

                    match &common_value {
                        None => common_value = Some(node_value),
                        Some(existing) if existing != &node_value => return None,
                        _ => {}
                    }
                }

                common_value
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
            Some(existing) => match (&existing, node) {
                // Same leaf values: ok
                (Self::Leaf(a), Self::Leaf(b)) if a == b => Some(()),
                // Different leaf values: conflict
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
                (Self::Branch(_), Self::Leaf(leaf)) => {
                    Self::merge_node(existing.as_mut_branch(), BASE, &Self::Leaf(leaf.clone()))?;
                    Some(())
                }
                // Both branches: merge recursively
                (Self::Branch(_), Self::Branch(new_inner)) => {
                    for (k, v) in new_inner {
                        Self::merge_node(existing.as_mut_branch(), k, v)?;
                    }
                    Some(())
                }
            },
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
