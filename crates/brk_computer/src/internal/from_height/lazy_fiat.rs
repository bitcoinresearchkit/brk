use brk_traversable::Traversable;
use brk_types::{Dollars, Version};
use vecdb::ReadableCloneableVec;

use super::{ComputedFromHeight, LazyFromHeight};
use crate::internal::{Identity, NumericValue};

use super::fiat::CentsType;

/// Lazy fiat: both cents and usd are lazy views of a stored source.
/// Zero extra stored vecs.
#[derive(Clone, Traversable)]
pub struct LazyFiatFromHeight<C: CentsType> {
    pub cents: LazyFromHeight<C, C>,
    pub usd: LazyFromHeight<Dollars, C>,
}

impl<C: CentsType> LazyFiatFromHeight<C> {
    pub(crate) fn from_computed(
        name: &str,
        version: Version,
        source: &ComputedFromHeight<C>,
    ) -> Self
    where
        C: NumericValue,
    {
        let cents = LazyFromHeight::from_computed::<Identity<C>>(
            &format!("{name}_cents"),
            version,
            source.height.read_only_boxed_clone(),
            source,
        );
        let usd = LazyFromHeight::from_computed::<C::ToDollars>(
            &format!("{name}_usd"),
            version,
            source.height.read_only_boxed_clone(),
            source,
        );
        Self { cents, usd }
    }
}
