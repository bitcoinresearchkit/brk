use bitview_collections::Windows;
use bitview_compute::FixedRatio;
use bitview_traversable::Traversable;
use brk_types::{Height, Version};
use vecdb::{DeltaChange, DeltaRate, LazyDeltaVec, ReadableCloneableVec, VecValue};

use crate::{
    FiatType, IndexSources, LazyDeltaFiatFromHeight, LazyDeltaFromHeight,
    LazyDeltaPercentFromHeight, LazyPerBlock,
};

#[derive(Clone, Traversable)]
pub struct LazyRollingDeltasFiatFromHeight<S, C, B>
where
    S: VecValue,
    C: FiatType,
    B: FixedRatio,
{
    /// Absolute change from the start of a trailing window through the
    /// represented block.
    pub absolute: Windows<LazyDeltaFiatFromHeight<S, C>>,
    /// Relative change from the start of a trailing window through the
    /// represented block, divided by the starting value. Returns zero when the
    /// starting value is zero.
    pub rate: Windows<LazyDeltaPercentFromHeight<S, B>>,
}

impl<S, C, B> LazyRollingDeltasFiatFromHeight<S, C, B>
where
    S: VecValue + Into<f64>,
    C: FiatType + From<f64>,
    B: FixedRatio + From<f64>,
{
    pub fn new(
        name: &str,
        version: Version,
        source: &impl ReadableCloneableVec<Height, S>,
        window_starts: &Windows<&impl ReadableCloneableVec<Height, Height>>,
        indexes: &IndexSources,
    ) -> Self {
        let source = source.read_only_boxed_clone();
        let (absolute, rate) = window_starts
            .map_with_suffix(|suffix, window_start| {
                let name = format!("{name}_{suffix}");
                let cached = window_start.read_only_boxed_clone();
                let starts_version = cached.version();

                let cents_name = format!("{name}_cents");
                let height = LazyDeltaVec::<Height, S, C, DeltaChange>::new(
                    &cents_name,
                    version,
                    source.clone(),
                    starts_version,
                    {
                        let cached = cached.clone();
                        move || cached.snapshot()
                    },
                );
                let cents = LazyDeltaFromHeight::new(&cents_name, version, height, indexes);
                let usd = LazyPerBlock::from_resolutions::<C::ToDollars>(
                    &name,
                    version,
                    &cents.resolutions,
                );
                let absolute = LazyDeltaFiatFromHeight { usd, cents };

                let ppm_name = format!("{name}_rate_{}", B::SUFFIX);
                let height = LazyDeltaVec::<Height, S, B, DeltaRate>::new(
                    &ppm_name,
                    version,
                    source.clone(),
                    starts_version,
                    move || cached.snapshot(),
                );
                let ppm = LazyDeltaFromHeight::new(&ppm_name, version, height, indexes);
                let rate = LazyDeltaPercentFromHeight::from_ppm(&name, version, ppm);

                (absolute, rate)
            })
            .unzip();

        Self { absolute, rate }
    }
}
