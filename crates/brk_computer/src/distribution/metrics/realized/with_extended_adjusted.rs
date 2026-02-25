use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Bitcoin, Dollars, Height};
use derive_more::{Deref, DerefMut};
use vecdb::{Exit, ReadableVec, Rw, StorageMode};

use crate::{ComputeIndexes, blocks, prices};

use crate::distribution::metrics::ImportConfig;

use super::{RealizedAdjusted, RealizedBase, RealizedExtended};

/// Realized metrics with guaranteed extended AND adjusted (no Options).
#[derive(Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct RealizedWithExtendedAdjusted<M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub base: RealizedBase<M>,
    #[traversable(flatten)]
    pub extended: RealizedExtended<M>,
    #[traversable(flatten)]
    pub adjusted: RealizedAdjusted<M>,
}

impl RealizedWithExtendedAdjusted {
    pub(crate) fn forced_import(cfg: &ImportConfig, up_to_1h: &RealizedBase) -> Result<Self> {
        let base = RealizedBase::forced_import(cfg)?;
        let extended = RealizedExtended::forced_import(cfg)?;
        let adjusted = RealizedAdjusted::forced_import(cfg, &base, up_to_1h)?;
        Ok(Self {
            base,
            extended,
            adjusted,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn compute_rest_part2(
        &mut self,
        blocks: &blocks::Vecs,
        prices: &prices::Vecs,
        starting_indexes: &ComputeIndexes,
        height_to_supply: &impl ReadableVec<Height, Bitcoin>,
        height_to_market_cap: &impl ReadableVec<Height, Dollars>,
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

        self.extended.compute_rest_part2_ext(
            &self.base,
            blocks,
            starting_indexes,
            height_to_market_cap,
            exit,
        )?;

        self.adjusted.compute_rest_part2_adj(
            blocks,
            starting_indexes,
            exit,
        )?;

        Ok(())
    }
}
