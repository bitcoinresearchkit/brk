use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Bitcoin, Dollars, Height};
use derive_more::{Deref, DerefMut};
use vecdb::{Exit, ReadableVec, Rw, StorageMode};

use crate::{ComputeIndexes, blocks, prices};

use crate::distribution::metrics::ImportConfig;

use super::{RealizedAdjusted, RealizedBase};

/// Realized metrics with guaranteed adjusted (no Option).
#[derive(Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct RealizedWithAdjusted<M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub base: RealizedBase<M>,
    #[traversable(flatten)]
    pub adjusted: RealizedAdjusted<M>,
}

impl RealizedWithAdjusted {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let base = RealizedBase::forced_import(cfg)?;
        let adjusted = RealizedAdjusted::forced_import(cfg)?;
        Ok(Self { base, adjusted })
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn compute_rest_part2(
        &mut self,
        blocks: &blocks::Vecs,
        prices: &prices::Vecs,
        starting_indexes: &ComputeIndexes,
        height_to_supply: &impl ReadableVec<Height, Bitcoin>,
        height_to_market_cap: &impl ReadableVec<Height, Dollars>,
        up_to_1h_value_created: &impl ReadableVec<Height, Dollars>,
        up_to_1h_value_destroyed: &impl ReadableVec<Height, Dollars>,
        exit: &Exit,
    ) -> Result<()> {
        self.base.compute_rest_part2_base(
            blocks,
            prices,
            starting_indexes,
            height_to_supply,
            height_to_market_cap,
            exit,
        )?;

        self.adjusted.compute_rest_part2_adj(
            blocks,
            starting_indexes,
            &self.base.value_created.height,
            &self.base.value_destroyed.height,
            up_to_1h_value_created,
            up_to_1h_value_destroyed,
            exit,
        )?;

        Ok(())
    }
}
