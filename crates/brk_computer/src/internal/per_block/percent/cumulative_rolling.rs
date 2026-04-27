//! PercentCumulativeRolling - cumulative percent + 4 rolling window percents.
//!
//! Mirrors `PerBlockCumulativeRolling` but for percentages derived from ratios
//! of cumulative values and rolling sums.

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{BasisPoints16, Height, StoredU64, Version};
use vecdb::{BinaryTransform, Database, Exit, ReadableVec, Rw, StorageMode, VecValue};

use crate::{
    indexes,
    internal::{
        BpsType, PerBlockCumulativeRolling, PercentPerBlock, PercentRollingWindows, RatioU64Bp16,
    },
};

#[derive(Traversable)]
pub struct PercentCumulativeRolling<B: BpsType, M: StorageMode = Rw> {
    #[traversable(flatten)]
    pub cumulative: PercentPerBlock<B, M>,
    #[traversable(flatten)]
    pub rolling: PercentRollingWindows<B, M>,
}

impl<B: BpsType> PercentCumulativeRolling<B> {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let cumulative = PercentPerBlock::forced_import(db, name, version, indexes)?;
        let rolling = PercentRollingWindows::forced_import(db, name, version, indexes)?;
        Ok(Self {
            cumulative,
            rolling,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn compute_binary<S1T, S2T, F, Rc1, Rc2, Rw1, Rw2>(
        &mut self,
        max_from: Height,
        cumulative_numerator: &Rc1,
        cumulative_denominator: &Rc2,
        rolling_numerators: [&Rw1; 4],
        rolling_denominators: [&Rw2; 4],
        exit: &Exit,
    ) -> Result<()>
    where
        S1T: VecValue,
        S2T: VecValue,
        Rc1: ReadableVec<Height, S1T>,
        Rc2: ReadableVec<Height, S2T>,
        Rw1: ReadableVec<Height, S1T>,
        Rw2: ReadableVec<Height, S2T>,
        F: BinaryTransform<S1T, S2T, B>,
    {
        self.cumulative.compute_binary::<S1T, S2T, F>(
            max_from,
            cumulative_numerator,
            cumulative_denominator,
            exit,
        )?;
        self.rolling.compute_binary::<S1T, S2T, F, Rw1, Rw2>(
            max_from,
            rolling_numerators,
            rolling_denominators,
            exit,
        )?;
        Ok(())
    }
}

impl PercentCumulativeRolling<BasisPoints16> {
    /// Derive a percent from two `PerBlockCumulativeRolling<StoredU64>`
    /// sources (numerator and denominator). Both sources must already have
    /// their cumulative and rolling sums computed.
    #[inline]
    pub(crate) fn compute_count_ratio(
        &mut self,
        numerator: &PerBlockCumulativeRolling<StoredU64, StoredU64>,
        denominator: &PerBlockCumulativeRolling<StoredU64, StoredU64>,
        starting_height: Height,
        exit: &Exit,
    ) -> Result<()> {
        self.compute_binary::<StoredU64, StoredU64, RatioU64Bp16, _, _, _, _>(
            starting_height,
            &numerator.cumulative.height,
            &denominator.cumulative.height,
            numerator.sum.as_array().map(|w| &w.height),
            denominator.sum.as_array().map(|w| &w.height),
            exit,
        )
    }
}
