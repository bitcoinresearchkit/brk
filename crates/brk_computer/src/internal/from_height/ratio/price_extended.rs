use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Cents, Height, Indexes, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{Database, EagerVec, Exit, PcoVec, Rw, StorageMode};

use crate::internal::{ComputedFromHeight, Price};
use crate::{blocks, indexes, prices};

use super::ComputedFromHeightRatioExtended;

#[derive(Deref, DerefMut, Traversable)]
pub struct ComputedFromHeightPriceWithRatioExtended<M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub inner: ComputedFromHeightRatioExtended<M>,
    pub price: Price<ComputedFromHeight<Cents, M>>,
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

    /// Compute price via closure (in cents), then compute ratio + extended metrics.
    pub(crate) fn compute_all<F>(
        &mut self,
        blocks: &blocks::Vecs,
        prices: &prices::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
        mut compute_price: F,
    ) -> Result<()>
    where
        F: FnMut(&mut EagerVec<PcoVec<Height, Cents>>) -> Result<()>,
    {
        compute_price(&mut self.price.cents.height)?;
        self.inner.compute_rest(
            blocks,
            prices,
            starting_indexes,
            exit,
            &self.price.cents.height,
        )?;
        Ok(())
    }
}
