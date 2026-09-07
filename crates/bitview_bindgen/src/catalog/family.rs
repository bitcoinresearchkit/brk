/// A projected Rust declaration, parameterized by its child types.
#[derive(Debug, Clone)]
pub struct CatalogFamily {
    pub name: String,
    pub source: String,
    /// Catalog field key and its declared type-parameter slot.
    pub fields: Vec<(String, usize)>,
    pub parameters: usize,
}
