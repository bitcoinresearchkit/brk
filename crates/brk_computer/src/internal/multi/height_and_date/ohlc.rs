//! Lazy OHLC component extractors for height + dateindex only.

use brk_traversable::Traversable;
use brk_types::{DateIndex, Height};
use schemars::JsonSchema;
use serde::Serialize;
use vecdb::{BytesVecValue, Formattable};

use crate::internal::LazyOHLC;

/// Lazy OHLC component extractors for height + dateindex.
#[derive(Clone, Traversable)]
#[traversable(merge)]
pub struct LazyHeightAndDateOHLC<T, SourceT>
where
    T: BytesVecValue + Formattable + Serialize + JsonSchema + 'static,
    SourceT: BytesVecValue + Formattable + Serialize + JsonSchema + 'static,
{
    pub height: LazyOHLC<Height, T, SourceT>,
    pub dateindex: LazyOHLC<DateIndex, T, SourceT>,
}
