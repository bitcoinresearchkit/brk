use bitview_transforms::price_ratio;
use bitview_traversable::Traversable;
use bitview_vecs::{IndexSources, PerBlock, Price, RatioPerBlock};
use brk_error::Result;
use brk_exit::Exit;
use brk_types::{Cents, Height, PriceRatio, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{Database, ReadableVec, Rw, StorageMode};

/// A stored reference price and its stored spot/reference ratio. Unit and
/// resolution views remain single-source derivations of those two histories.
#[derive(Deref, DerefMut, Traversable)]
pub struct ReferencePrice<M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub price: Price<PerBlock<Cents, M>>,
    #[traversable(flatten)]
    pub relative: RatioPerBlock<PriceRatio, M>,
}

impl ReferencePrice {
    pub fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &IndexSources,
    ) -> Result<Self> {
        Ok(Self {
            price: Price::forced_import(db, name, version, indexes)?,
            relative: RatioPerBlock::forced_import(db, name, version, indexes)?,
        })
    }

    pub fn compute_ratio(
        &mut self,
        starting_height: Height,
        spot: &impl ReadableVec<Height, Cents>,
        exit: &Exit,
    ) -> Result<()> {
        let price = &self.price.cents.height;
        let ratio = &mut self.relative.ppm.height;
        ratio.compute_transform2(
            starting_height,
            spot,
            price,
            |(height, spot, price, _)| (height, price_ratio(spot, price)),
            exit,
        )?;
        Ok(())
    }
}
