/// Exact catalog types. IDs reference other entries in the same type table.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CatalogType {
    Leaf {
        accessor: usize,
        value: String,
    },
    Branch {
        family: usize,
        arguments: Vec<usize>,
    },
}
