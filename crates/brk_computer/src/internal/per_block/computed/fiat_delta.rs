//! Fiat delta variants — same as RollingDelta* but change is FiatPerBlock<C>
//! (stored cents + lazy USD) instead of ComputedPerBlock<C> (stored cents only).

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{BasisPointsSigned32, Height, Version};
use schemars::JsonSchema;
use vecdb::{Database, Exit, ReadableVec, Rw, StorageMode};

use crate::{
    indexes,
    internal::{
        CentsType, FiatPerBlock, NumericValue, PercentPerBlock, PercentRollingWindows,
        WindowStarts,
    },
};

use super::delta::compute_delta_window;

/// Fiat 1m-only delta: fiat change (cents + usd) + rate for the 1-month window.
#[derive(Traversable)]
pub struct FiatRollingDelta1m<S, C, M: StorageMode = Rw>
where
    S: NumericValue + JsonSchema,
    C: CentsType,
{
    pub change_1m: FiatPerBlock<C, M>,
    pub rate_1m: PercentPerBlock<BasisPointsSigned32, M>,
    _phantom: std::marker::PhantomData<S>,
}

impl<S, C> FiatRollingDelta1m<S, C>
where
    S: NumericValue + JsonSchema,
    C: CentsType,
{
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self {
            change_1m: FiatPerBlock::forced_import(
                db,
                &format!("{name}_change_1m"),
                version,
                indexes,
            )?,
            rate_1m: PercentPerBlock::forced_import(
                db,
                &format!("{name}_rate_1m"),
                version,
                indexes,
            )?,
            _phantom: std::marker::PhantomData,
        })
    }

    pub(crate) fn compute(
        &mut self,
        max_from: Height,
        height_1m_ago: &impl ReadableVec<Height, Height>,
        source: &impl ReadableVec<Height, S>,
        exit: &Exit,
    ) -> Result<()> {
        compute_delta_window(
            &mut self.change_1m.cents.height,
            &mut self.rate_1m.bps.height,
            max_from,
            height_1m_ago,
            source,
            exit,
        )
    }
}

/// Fiat extended delta: 24h + 1w + 1y windows, fiat change (cents + usd) + rate.
#[derive(Traversable)]
pub struct FiatRollingDeltaExcept1m<S, C, M: StorageMode = Rw>
where
    S: NumericValue + JsonSchema,
    C: CentsType,
{
    #[traversable(rename = "24h")]
    pub change_24h: FiatPerBlock<C, M>,
    pub change_1w: FiatPerBlock<C, M>,
    pub change_1y: FiatPerBlock<C, M>,
    #[traversable(rename = "24h")]
    pub rate_24h: PercentPerBlock<BasisPointsSigned32, M>,
    pub rate_1w: PercentPerBlock<BasisPointsSigned32, M>,
    pub rate_1y: PercentPerBlock<BasisPointsSigned32, M>,
    _phantom: std::marker::PhantomData<S>,
}

impl<S, C> FiatRollingDeltaExcept1m<S, C>
where
    S: NumericValue + JsonSchema,
    C: CentsType,
{
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self {
            change_24h: FiatPerBlock::forced_import(
                db,
                &format!("{name}_change_24h"),
                version,
                indexes,
            )?,
            change_1w: FiatPerBlock::forced_import(
                db,
                &format!("{name}_change_1w"),
                version,
                indexes,
            )?,
            change_1y: FiatPerBlock::forced_import(
                db,
                &format!("{name}_change_1y"),
                version,
                indexes,
            )?,
            rate_24h: PercentPerBlock::forced_import(
                db,
                &format!("{name}_rate_24h"),
                version,
                indexes,
            )?,
            rate_1w: PercentPerBlock::forced_import(
                db,
                &format!("{name}_rate_1w"),
                version,
                indexes,
            )?,
            rate_1y: PercentPerBlock::forced_import(
                db,
                &format!("{name}_rate_1y"),
                version,
                indexes,
            )?,
            _phantom: std::marker::PhantomData,
        })
    }

    pub(crate) fn compute(
        &mut self,
        max_from: Height,
        windows: &WindowStarts<'_>,
        source: &impl ReadableVec<Height, S>,
        exit: &Exit,
    ) -> Result<()> {
        let changes: [&mut FiatPerBlock<C>; 3] = [
            &mut self.change_24h,
            &mut self.change_1w,
            &mut self.change_1y,
        ];
        let rates = [&mut self.rate_24h, &mut self.rate_1w, &mut self.rate_1y];
        let starts = [windows._24h, windows._1w, windows._1y];

        for ((change_w, rate_w), starts) in changes.into_iter().zip(rates).zip(starts) {
            compute_delta_window(
                &mut change_w.cents.height,
                &mut rate_w.bps.height,
                max_from,
                starts,
                source,
                exit,
            )?;
        }
        Ok(())
    }
}

/// Fiat rolling delta: all 4 windows, fiat change (cents + usd) + rate.
#[derive(Traversable)]
pub struct FiatRollingDelta<S, C, M: StorageMode = Rw>
where
    S: NumericValue + JsonSchema,
    C: CentsType,
{
    pub change_24h: FiatPerBlock<C, M>,
    pub change_1w: FiatPerBlock<C, M>,
    pub change_1m: FiatPerBlock<C, M>,
    pub change_1y: FiatPerBlock<C, M>,
    pub rate: PercentRollingWindows<BasisPointsSigned32, M>,
    _phantom: std::marker::PhantomData<S>,
}

impl<S, C> FiatRollingDelta<S, C>
where
    S: NumericValue + JsonSchema,
    C: CentsType,
{
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self {
            change_24h: FiatPerBlock::forced_import(
                db,
                &format!("{name}_change_24h"),
                version,
                indexes,
            )?,
            change_1w: FiatPerBlock::forced_import(
                db,
                &format!("{name}_change_1w"),
                version,
                indexes,
            )?,
            change_1m: FiatPerBlock::forced_import(
                db,
                &format!("{name}_change_1m"),
                version,
                indexes,
            )?,
            change_1y: FiatPerBlock::forced_import(
                db,
                &format!("{name}_change_1y"),
                version,
                indexes,
            )?,
            rate: PercentRollingWindows::forced_import(
                db,
                &format!("{name}_rate"),
                version,
                indexes,
            )?,
            _phantom: std::marker::PhantomData,
        })
    }

    pub(crate) fn compute(
        &mut self,
        max_from: Height,
        windows: &WindowStarts<'_>,
        source: &impl ReadableVec<Height, S>,
        exit: &Exit,
    ) -> Result<()> {
        let changes: [&mut FiatPerBlock<C>; 4] = [
            &mut self.change_24h,
            &mut self.change_1w,
            &mut self.change_1m,
            &mut self.change_1y,
        ];
        let rates = self.rate.0.as_mut_array();
        let starts = windows.as_array();

        for ((change_w, rate_w), starts) in changes.into_iter().zip(rates).zip(starts) {
            compute_delta_window(
                &mut change_w.cents.height,
                &mut rate_w.bps.height,
                max_from,
                *starts,
                source,
                exit,
            )?;
        }
        Ok(())
    }
}
