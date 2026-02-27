use brk_traversable::Traversable;

#[derive(Clone, Traversable)]
pub struct SplitOhlc<O, H, L, C> {
    pub open: O,
    pub high: H,
    pub low: L,
    pub close: C,
}
