use bitview_collections::ByLookbackPeriod;
use bitview_traversable::Traversable;
use bitview_vecs::{LazyPerBlock, Price};
use brk_types::Cents;

#[derive(Clone, Traversable)]
pub struct Vecs {
    /// Bitcoin spot price at the first block in a trailing monotonic-time
    /// window.
    #[traversable(flatten)]
    pub price_past: ByLookbackPeriod<Price<LazyPerBlock<Cents>>>,
}
