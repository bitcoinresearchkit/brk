//! RollingDelta - raw change + growth rate (%) across 4 time windows.
//!
//! For a monotonic source (e.g., cumulative address count):
//! - `change._24h` = count_now - count_24h_ago
//! - `rate._24h` = (count_now - count_24h_ago) / count_24h_ago in BPS

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{BasisPoints16, Height, Version};
use schemars::JsonSchema;
use vecdb::{Database, Exit, ReadableVec, Rw, StorageMode};

use crate::{
    indexes,
    internal::{NumericValue, PercentRollingWindows, RollingWindows, WindowStarts},
};

#[derive(Traversable)]
pub struct RollingDelta<S, C = S, M: StorageMode = Rw>
where
    S: NumericValue + JsonSchema,
    C: NumericValue + JsonSchema,
{
    pub change: RollingWindows<C, M>,
    pub rate: PercentRollingWindows<BasisPoints16, M>,
    _phantom: std::marker::PhantomData<S>,
}

impl<S, C> RollingDelta<S, C>
where
    S: NumericValue + JsonSchema,
    C: NumericValue + JsonSchema,
{
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self {
            change: RollingWindows::forced_import(
                db,
                &format!("{name}_change"),
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
    ) -> Result<()>
    where
        S: Default,
    {
        // Step 1: change = current - ago
        for (change_w, starts) in self.change.0.as_mut_array().into_iter().zip(windows.as_array())
        {
            change_w.height.compute_transform(
                max_from,
                *starts,
                |(h, ago_h, ..)| {
                    let current: f64 = source.collect_one(h).unwrap_or_default().into();
                    let ago: f64 = source.collect_one(ago_h).unwrap_or_default().into();
                    (h, C::from(current - ago))
                },
                exit,
            )?;
        }

        // Step 2: rate = change / ago = change / (current - change)
        for (growth_w, change_w) in self
            .rate
            .0
            .as_mut_array()
            .into_iter()
            .zip(self.change.0.as_array())
        {
            growth_w.bps.height.compute_transform2(
                max_from,
                source,
                &change_w.height,
                |(h, current, change, ..)| {
                    let current_f: f64 = current.into();
                    let change_f: f64 = change.into();
                    let ago = current_f - change_f;
                    let rate = if ago == 0.0 { 0.0 } else { change_f / ago };
                    (h, BasisPoints16::from(rate))
                },
                exit,
            )?;
        }

        Ok(())
    }
}
