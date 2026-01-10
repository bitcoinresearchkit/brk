//! Lazy OHLC component extractors.

use brk_traversable::Traversable;
use brk_types::{Close, High, Low, Open};
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
