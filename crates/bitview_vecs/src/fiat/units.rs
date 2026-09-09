use bitview_traversable::Traversable;

/// A monetary value in its integer cents and dollar representations.
#[derive(Clone, Traversable)]
pub struct Fiat<C, U> {
    /// Reported in US dollars.
    pub usd: U,
    /// Reported in US cents; 100 cents equal one US dollar.
    pub cents: C,
}
