#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddedKind {
    /// First time we've seen this txid.
    Fresh,
    /// Re-entered the pool after a prior removal still in the graveyard.
    Revived,
}
