use bitview_traversable::Traversable;
use brk_types::Height;
use brk_types::{Dollars, Version};
use vecdb::ReadableCloneableVec;

use crate::{FiatType, Identity, IndexSources, LazyPerBlock, NumericValue};

/// Lazy fiat: both cents and usd are lazy views of a stored source.
/// Zero extra stored vecs.
#[derive(Clone, Traversable)]
pub struct LazyFiatPerBlock<C: FiatType> {
    /// Reported in US dollars.
    pub usd: LazyPerBlock<Dollars, C>,
    /// Reported in US cents; 100 cents equal one US dollar.
    pub cents: LazyPerBlock<C, C>,
}

impl<C: FiatType> LazyFiatPerBlock<C> {
    pub fn from_lazy(name: &str, version: Version, source: &LazyPerBlock<C>) -> Self
    where
        C: NumericValue,
    {
        let cents =
            LazyPerBlock::from_lazy::<Identity<C>, C>(&format!("{name}_cents"), version, source);
        let usd = LazyPerBlock::from_lazy::<C::ToDollars, C>(name, version, source);
        Self { usd, cents }
    }

    pub fn from_cents_source(
        name: &str,
        version: Version,
        source: &(impl ReadableCloneableVec<Height, C> + ?Sized),
        indexes: &IndexSources,
    ) -> Self
    where
        C: NumericValue,
    {
        let source = LazyPerBlock::from_height_source::<Identity<C>>(
            &format!("{name}_cents"),
            version,
            source,
            indexes,
        );
        Self::from_lazy(name, version, &source)
    }
}
