use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Cents, Height, Indexes, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{Database, Exit, ReadableVec, Rw, StorageMode};

use crate::{blocks, indexes, prices};

use super::{ComputedFromHeightRatio, ComputedFromHeightRatioExtension};

#[derive(Deref, DerefMut, Traversable)]
pub struct ComputedFromHeightRatioExtended<M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub base: ComputedFromHeightRatio<M>,
    #[traversable(flatten)]
    pub extended: ComputedFromHeightRatioExtension<M>,
}

impl ComputedFromHeightRatioExtended {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self {
            base: ComputedFromHeightRatio::forced_import(db, name, version, indexes)?,
            extended: ComputedFromHeightRatioExtension::forced_import(db, name, version, indexes)?,
        })
    }

    /// Compute ratio and all extended metrics from an externally-provided metric price (in cents).
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
        self.extended
            .compute_rest(blocks, starting_indexes, exit, &self.base.ratio.height)?;
        self.extended
            .compute_cents_bands(starting_indexes, metric_price, exit)?;
        Ok(())
    }
}
