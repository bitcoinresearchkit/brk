use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Cents, Height, Indexes, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{Database, Exit, ReadableVec, Rw, StorageMode};

use crate::{blocks, indexes, prices};

use super::{ComputedFromHeightRatioExtended, ComputedFromHeightRatioStdDevBands};

#[derive(Deref, DerefMut, Traversable)]
pub struct ComputedFromHeightRatioFull<M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub base: ComputedFromHeightRatioExtended<M>,
    #[traversable(flatten)]
    pub std_dev: ComputedFromHeightRatioStdDevBands<M>,
}

impl ComputedFromHeightRatioFull {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self {
            base: ComputedFromHeightRatioExtended::forced_import(db, name, version, indexes)?,
            std_dev: ComputedFromHeightRatioStdDevBands::forced_import(
                db, name, version, indexes,
            )?,
        })
    }

    /// Compute ratio, percentiles, and all stddev bands from an externally-provided metric price (in cents).
    pub(crate) fn compute_rest(
        &mut self,
        blocks: &blocks::Vecs,
        prices: &prices::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
        metric_price: &impl ReadableVec<Height, Cents>,
    ) -> Result<()> {
        self.base
            .compute_rest(blocks, prices, starting_indexes, exit, metric_price)?;
        self.std_dev
            .compute(blocks, starting_indexes, exit, &self.base.base.ratio.height, metric_price)?;
        Ok(())
    }
}
