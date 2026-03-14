use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{BasisPoints32, Cents, Dollars, Height, Indexes, SatsFract, StoredF32, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{Database, EagerVec, Exit, PcoVec, ReadableVec, Rw, StorageMode};

use crate::internal::{PerBlock, LazyPerBlock, Price};
use crate::{indexes, prices};

use super::{RatioPerBlock, RatioPerBlockPercentiles};

#[derive(Traversable)]
pub struct PriceWithRatioPerBlock<M: StorageMode = Rw> {
    pub usd: LazyPerBlock<Dollars, Cents>,
    pub cents: PerBlock<Cents, M>,
    pub sats: LazyPerBlock<SatsFract, Dollars>,
    pub bps: PerBlock<BasisPoints32, M>,
    pub ratio: LazyPerBlock<StoredF32, BasisPoints32>,
}

impl PriceWithRatioPerBlock {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let price = Price::forced_import(db, name, version, indexes)?;
        let ratio = RatioPerBlock::forced_import(db, name, version, indexes)?;
        Ok(Self {
            usd: price.usd,
            cents: price.cents,
            sats: price.sats,
            bps: ratio.bps,
            ratio: ratio.ratio,
        })
    }

    /// Compute ratio from close price and this metric's price.
    pub(crate) fn compute_ratio(
        &mut self,
        starting_indexes: &Indexes,
        close_price: &impl ReadableVec<Height, Cents>,
        exit: &Exit,
    ) -> Result<()> {
        self.bps.height.compute_transform2(
            starting_indexes.height,
            close_price,
            &self.cents.height,
            |(i, close, price, ..)| {
                if price == Cents::ZERO {
                    (i, BasisPoints32::from(1.0))
                } else {
                    (i, BasisPoints32::from(f64::from(close) / f64::from(price)))
                }
            },
            exit,
        )?;
        Ok(())
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
        compute_price(&mut self.cents.height)?;
        self.compute_ratio(starting_indexes, &prices.spot.cents.height, exit)
    }
}

#[derive(Deref, DerefMut, Traversable)]
pub struct PriceWithRatioExtendedPerBlock<M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub base: PriceWithRatioPerBlock<M>,
    pub percentiles: RatioPerBlockPercentiles<M>,
}

impl PriceWithRatioExtendedPerBlock {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self {
            base: PriceWithRatioPerBlock::forced_import(db, name, version, indexes)?,
            percentiles: RatioPerBlockPercentiles::forced_import(db, name, version, indexes)?,
        })
    }

    /// Compute ratio and percentiles from already-computed price cents.
    pub(crate) fn compute_rest(
        &mut self,
        prices: &prices::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        let close_price = &prices.spot.cents.height;
        self.base.compute_ratio(starting_indexes, close_price, exit)?;
        self.percentiles.compute(
            starting_indexes,
            exit,
            &self.base.ratio.height,
            &self.base.cents.height,
        )?;
        Ok(())
    }

    /// Compute price via closure (in cents), then compute ratio and percentiles.
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
        compute_price(&mut self.base.cents.height)?;
        self.compute_rest(prices, starting_indexes, exit)
    }
}
