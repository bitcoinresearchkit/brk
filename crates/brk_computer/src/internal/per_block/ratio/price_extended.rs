use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{BasisPoints32, Cents, Height, Indexes, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{Database, EagerVec, Exit, PcoVec, Rw, StorageMode};

use crate::internal::{ComputedPerBlock, Price};
use crate::{indexes, prices};

use super::RatioPerBlock;

#[derive(Deref, DerefMut, Traversable)]
pub struct PriceWithRatioPerBlock<M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub inner: RatioPerBlock<BasisPoints32, M>,
    pub price: Price<ComputedPerBlock<Cents, M>>,
}

impl PriceWithRatioPerBlock {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let v = version + Version::TWO;
        Ok(Self {
            inner: RatioPerBlock::forced_import(db, name, version, indexes)?,
            price: Price::forced_import(db, name, v, indexes)?,
        })
    }

    /// Compute price via closure (in cents), then compute ratio.
    pub(crate) fn compute_all<F>(
        &mut self,
        prices: &prices::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
        mut compute_price: F,
    ) -> Result<()>
    where
        F: FnMut(&mut EagerVec<PcoVec<Height, Cents>>) -> Result<()>,
    {
        compute_price(&mut self.price.cents.height)?;
        let close_price = &prices.price.cents.height;
        self.inner
            .compute_ratio(starting_indexes, close_price, &self.price.cents.height, exit)?;
        Ok(())
    }
}
