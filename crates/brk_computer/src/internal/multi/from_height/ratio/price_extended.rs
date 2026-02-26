use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Dollars, Height, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{Database, EagerVec, Exit, PcoVec, Rw, StorageMode};

use crate::internal::{ComputedFromHeightLast, Price};
use crate::{ComputeIndexes, blocks, indexes, prices};

use super::ComputedFromHeightRatioExtended;

#[derive(Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct ComputedFromHeightPriceWithRatioExtended<M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub inner: ComputedFromHeightRatioExtended<M>,
    pub price: Price<ComputedFromHeightLast<Dollars, M>>,
}

impl ComputedFromHeightPriceWithRatioExtended {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let v = version + Version::TWO;
        Ok(Self {
            inner: ComputedFromHeightRatioExtended::forced_import(db, name, version, indexes)?,
            price: Price::forced_import(db, name, v, indexes)?,
        })
    }

    /// Compute price via closure, then compute ratio + extended metrics.
    pub(crate) fn compute_all<F>(
        &mut self,
        blocks: &blocks::Vecs,
        prices: &prices::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
        mut compute_price: F,
    ) -> Result<()>
    where
        F: FnMut(&mut EagerVec<PcoVec<Height, Dollars>>) -> Result<()>,
    {
        compute_price(&mut self.price.usd.height)?;
        self.inner.compute_rest(
            blocks,
            prices,
            starting_indexes,
            exit,
            &self.price.usd.height,
        )?;
        Ok(())
    }
}
