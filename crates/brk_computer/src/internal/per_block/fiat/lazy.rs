use brk_traversable::Traversable;
use brk_types::{Dollars, Version};
use vecdb::ReadableCloneableVec;

use crate::internal::{CentsType, Identity, LazyPerBlock, NumericValue, PerBlock};

/// Lazy fiat: both cents and usd are lazy views of a stored source.
/// Zero extra stored vecs.
#[derive(Clone, Traversable)]
pub struct LazyFiatPerBlock<C: CentsType> {
    pub usd: LazyPerBlock<Dollars, C>,
    pub cents: LazyPerBlock<C, C>,
}

impl<C: CentsType> LazyFiatPerBlock<C> {
    pub(crate) fn from_computed(name: &str, version: Version, source: &PerBlock<C>) -> Self
    where
        C: NumericValue,
    {
        let cents = LazyPerBlock::from_computed::<Identity<C>>(
            &format!("{name}_cents"),
            version,
            source.height.read_only_boxed_clone(),
            source,
        );
        let usd = LazyPerBlock::from_computed::<C::ToDollars>(
            name,
            version,
            source.height.read_only_boxed_clone(),
            source,
        );
        Self { usd, cents }
    }
}
