use std::collections::{BTreeMap, BTreeSet};

use bitview_catalog::TreeNode;

use super::{CatalogFamily, CatalogNode, CatalogType, CatalogValue};
use crate::{IndexSetPattern, to_pascal_case};

/// Compiled, lossless catalog bindings and source-defined record families.
#[derive(Debug, Clone, Default)]
pub struct CatalogTree {
    pub families: Vec<CatalogFamily>,
    pub types: Vec<CatalogType>,
    pub nodes: Vec<CatalogNode>,
    pub root: usize,
}

#[derive(Default)]
struct Compiler {
    tree: CatalogTree,
    families: BTreeMap<(String, Vec<(String, usize)>), usize>,
    types: BTreeMap<CatalogType, usize>,
    nodes: BTreeMap<CatalogNode, usize>,
}

impl CatalogTree {
    pub fn from_catalog(catalog: &TreeNode, indexes: &[IndexSetPattern]) -> Self {
        let mut compiler = Compiler::default();
        compiler.tree.root = compiler.compile(catalog, "Root", indexes);
        compiler.name_families();
        compiler.tree
    }
}

impl Compiler {
    fn compile(&mut self, node: &TreeNode, path: &str, indexes: &[IndexSetPattern]) -> usize {
        let (ty, value) = match node {
            TreeNode::Leaf(leaf) => (
                CatalogType::Leaf {
                    accessor: indexes
                        .iter()
                        .position(|pattern| &pattern.indexes == leaf.indexes())
                        .expect("Every catalog index set must have an accessor"),
                    value: leaf.kind().to_string(),
                },
                CatalogValue::Leaf(leaf.name().to_string()),
            ),
            TreeNode::Branch(branch) => {
                let mut fields = Vec::new();
                let mut children = Vec::new();
                let mut arguments = Vec::new();
                let mut slots = BTreeMap::new();
                for (key, child) in branch {
                    let child_id = self.compile(child, &format!("{path}::{key}"), indexes);
                    let child_type = self.tree.nodes[child_id].type_id;
                    // A repeated declared field type shares a parameter only when its
                    // public projection is also identical (optional fields may differ).
                    let declaration = branch.field_types.get(key).copied().unwrap_or(key);
                    let parameter = *slots.entry((declaration, child_type)).or_insert_with(|| {
                        let slot = arguments.len();
                        arguments.push(child_type);
                        slot
                    });
                    fields.push((key.clone(), parameter));
                    children.push(child_id);
                }
                let source = branch
                    .source
                    .map(str::to_owned)
                    .unwrap_or_else(|| format!("catalog::{path}"));
                let family = *self
                    .families
                    .entry((source.clone(), fields.clone()))
                    .or_insert_with(|| {
                        let id = self.tree.families.len();
                        self.tree.families.push(CatalogFamily {
                            name: String::new(),
                            source,
                            fields,
                            parameters: arguments.len(),
                        });
                        id
                    });
                (
                    CatalogType::Branch { family, arguments },
                    CatalogValue::Branch(children),
                )
            }
        };
        let type_id = *self.types.entry(ty.clone()).or_insert_with(|| {
            let id = self.tree.types.len();
            self.tree.types.push(ty);
            id
        });
        let node = CatalogNode { type_id, value };
        *self.nodes.entry(node.clone()).or_insert_with(|| {
            let id = self.tree.nodes.len();
            self.tree.nodes.push(node);
            id
        })
    }

    fn name_families(&mut self) {
        let mut origins: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
        for family in &self.tree.families {
            origins
                .entry(family.source.rsplit("::").next().unwrap())
                .or_default()
                .insert(&family.source);
        }
        let bases: Vec<_> = self
            .tree
            .families
            .iter()
            .map(|family| {
                let short = family.source.rsplit("::").next().unwrap();
                let source = if origins[short].len() == 1 {
                    short
                } else {
                    &family.source
                };
                let words: String = source
                    .chars()
                    .map(|c| if c.is_alphanumeric() { c } else { '_' })
                    .collect();
                format!("Catalog{}", to_pascal_case(&words))
            })
            .collect();
        let mut counts = BTreeMap::new();
        for (family, base) in self.tree.families.iter_mut().zip(bases) {
            let count = counts.entry(base.clone()).or_insert(0);
            *count += 1;
            family.name = if *count == 1 {
                base
            } else {
                format!("{base}_{count}")
            };
        }
    }
}
