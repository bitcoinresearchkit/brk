/// A concrete catalog binding. Names are never reconstructed from a template.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CatalogNode {
    pub type_id: usize,
    pub value: CatalogValue,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CatalogValue {
    Leaf(String),
    Branch(Vec<usize>),
}
