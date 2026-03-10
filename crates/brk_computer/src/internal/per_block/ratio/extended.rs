use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{BasisPoints32, Cents, Height, Indexes, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{Database, Exit, ReadableVec, Rw, StorageMode};

use crate::{blocks, indexes, prices};

use super::{RatioPerBlock, RatioPerBlockPercentiles};

#[derive(Deref, DerefMut, Traversable)]
pub struct RatioPerBlockExtended<M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub base: RatioPerBlock<BasisPoints32, M>,
    #[traversable(flatten)]
    pub percentiles: RatioPerBlockPercentiles<M>,
}

impl RatioPerBlockExtended {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self {
            base: RatioPerBlock::forced_import(db, name, version, indexes)?,
            percentiles: RatioPerBlockPercentiles::forced_import(
                db, name, version, indexes,
            )?,
        })
    }

    /// Compute ratio and all percentile metrics from an externally-provided metric price (in cents).
    pub(crate) fn compute_rest(
        &mut self,
        blocks: &blocks::Vecs,
        prices: &prices::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
        metric_price: &impl ReadableVec<Height, Cents>,
    ) -> Result<()> {
        let close_price = &prices.price.cents.height;
        self.base
            .compute_ratio(starting_indexes, close_price, metric_price, exit)?;
        self.percentiles
            .compute(blocks, starting_indexes, exit, &self.base.ratio.height, metric_price)?;
        Ok(())
    }
}
