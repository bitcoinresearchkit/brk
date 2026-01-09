//! Lazy OHLC component extractors for height + dateindex.

use brk_traversable::Traversable;
use brk_types::{Close, DateIndex, Height, High, Low, Open};
use schemars::JsonSchema;
use serde::Serialize;
use vecdb::{BytesVecValue, Formattable, LazyVecFrom1, VecIndex};

/// Lazy OHLC component extractors for a single index type.
#[derive(Clone, Traversable)]
pub struct LazyOHLC<I, T, SourceT>
where
    I: VecIndex + BytesVecValue + Formattable + Serialize + JsonSchema + 'static,
    T: BytesVecValue + Formattable + Serialize + JsonSchema + 'static,
    SourceT: BytesVecValue + Formattable + Serialize + JsonSchema + 'static,
{
    pub open: LazyVecFrom1<I, Open<T>, I, SourceT>,
    pub high: LazyVecFrom1<I, High<T>, I, SourceT>,
    pub low: LazyVecFrom1<I, Low<T>, I, SourceT>,
    pub close: LazyVecFrom1<I, Close<T>, I, SourceT>,
}

/// Lazy OHLC component extractors for height + dateindex.
#[derive(Clone, Traversable)]
pub struct HeightDateLazyOHLC<T, SourceT>
where
    T: BytesVecValue + Formattable + Serialize + JsonSchema + 'static,
    SourceT: BytesVecValue + Formattable + Serialize + JsonSchema + 'static,
{
    pub height: LazyOHLC<Height, T, SourceT>,
    pub dateindex: LazyOHLC<DateIndex, T, SourceT>,
}
