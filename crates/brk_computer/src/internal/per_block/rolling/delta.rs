use brk_traversable::Traversable;
use brk_types::{Dollars, Height, StoredF32, Version};
use schemars::JsonSchema;
use vecdb::{DeltaChange, DeltaRate, LazyDeltaVec, LazyVecFrom1, ReadableCloneableVec, VecValue};

use crate::{
    indexes,
    internal::{
        BpsType, CachedWindowStarts, CentsType, DerivedResolutions, LazyPerBlock, NumericValue,
        Resolutions, Windows,
    },
};

/// Generic single-slot lazy delta: a `LazyDeltaVec` + resolution views.
///
/// Used as building block for both change and rate deltas.
/// - Change: `LazyDeltaFromHeight<S, C, DeltaChange>`
/// - Rate BPS: `LazyDeltaFromHeight<S, B, DeltaRate>`
#[derive(Clone, Traversable)]
#[traversable(merge)]
pub struct LazyDeltaFromHeight<S, T, Op: 'static>
where
    S: VecValue,
    T: NumericValue + JsonSchema,
{
    pub height: LazyDeltaVec<Height, S, T, Op>,
    #[traversable(flatten)]
    pub resolutions: Box<Resolutions<T>>,
}

/// Single-slot lazy delta percent: BPS delta + lazy ratio + lazy percent views.
///
/// Mirrors `PercentPerBlock<B>` but with lazy delta for the BPS source.
#[derive(Clone, Traversable)]
pub struct LazyDeltaPercentFromHeight<S, B>
where
    S: VecValue,
    B: BpsType,
{
    pub bps: LazyDeltaFromHeight<S, B, DeltaRate>,
    pub ratio: LazyPerBlock<StoredF32, B>,
    pub percent: LazyPerBlock<StoredF32, B>,
}

/// Lazy rolling deltas for all 4 window durations (24h, 1w, 1m, 1y).
///
/// Tree shape: `absolute._24h/...`, `rate._24h/...` — matches old `RollingDelta`.
///
/// Replaces `RollingDelta`, `RollingDelta1m`, and `RollingDeltaExcept1m` — since
/// there is no storage cost, all 4 windows are always available.
#[derive(Clone, Traversable)]
pub struct LazyRollingDeltasFromHeight<S, C, B>
where
    S: VecValue,
    C: NumericValue + JsonSchema,
    B: BpsType,
{
    pub absolute: Windows<LazyDeltaFromHeight<S, C, DeltaChange>>,
    pub rate: Windows<LazyDeltaPercentFromHeight<S, B>>,
}

impl<S, C, B> LazyRollingDeltasFromHeight<S, C, B>
where
    S: VecValue + Into<f64>,
    C: NumericValue + JsonSchema + From<f64>,
    B: BpsType + From<f64>,
{
    pub fn new(
        name: &str,
        version: Version,
        source: &(impl ReadableCloneableVec<Height, S> + 'static),
        cached_starts: &CachedWindowStarts,
        indexes: &indexes::Vecs,
    ) -> Self {
        let src = source.read_only_boxed_clone();

        let make_slot = |suffix: &str, cached_start: &vecdb::CachedVec<Height, Height>| {
            let full_name = format!("{name}_{suffix}");
            let cached = cached_start.clone();
            let starts_version = cached.version();

            // Change: source[h] - source[ago] as C (via f64)
            let change_vec = LazyDeltaVec::<Height, S, C, DeltaChange>::new(
                &format!("{full_name}_change"),
                version,
                src.clone(),
                starts_version,
                {
                    let cached = cached.clone();
                    move || cached.get()
                },
            );
            let change_resolutions = Resolutions::forced_import(
                &format!("{full_name}_change"),
                change_vec.read_only_boxed_clone(),
                version,
                indexes,
            );
            let absolute = LazyDeltaFromHeight {
                height: change_vec,
                resolutions: Box::new(change_resolutions),
            };

            // Rate BPS: (source[h] - source[ago]) / source[ago] as B (via f64)
            let rate_vec = LazyDeltaVec::<Height, S, B, DeltaRate>::new(
                &format!("{full_name}_rate_bps"),
                version,
                src.clone(),
                starts_version,
                move || cached.get(),
            );
            let rate_resolutions = Resolutions::forced_import(
                &format!("{full_name}_rate_bps"),
                rate_vec.read_only_boxed_clone(),
                version,
                indexes,
            );
            let bps = LazyDeltaFromHeight {
                height: rate_vec,
                resolutions: Box::new(rate_resolutions),
            };

            // Ratio: bps / 10000
            let ratio = LazyPerBlock {
                height: LazyVecFrom1::transformed::<B::ToRatio>(
                    &format!("{full_name}_rate_ratio"),
                    version,
                    bps.height.read_only_boxed_clone(),
                ),
                resolutions: Box::new(DerivedResolutions::from_derived_computed::<B::ToRatio>(
                    &format!("{full_name}_rate_ratio"),
                    version,
                    &bps.resolutions,
                )),
            };

            // Percent: bps / 100
            let percent = LazyPerBlock {
                height: LazyVecFrom1::transformed::<B::ToPercent>(
                    &format!("{full_name}_rate"),
                    version,
                    bps.height.read_only_boxed_clone(),
                ),
                resolutions: Box::new(DerivedResolutions::from_derived_computed::<B::ToPercent>(
                    &format!("{full_name}_rate"),
                    version,
                    &bps.resolutions,
                )),
            };

            let rate = LazyDeltaPercentFromHeight {
                bps,
                ratio,
                percent,
            };

            (absolute, rate)
        };

        let (absolute, rate) = cached_starts.0.map_with_suffix(make_slot).unzip();

        Self { absolute, rate }
    }
}

// ---------------------------------------------------------------------------
// Fiat delta types (cents change + lazy USD + rate)
// ---------------------------------------------------------------------------

/// Single-slot fiat delta change: cents delta + lazy USD.
#[derive(Clone, Traversable)]
pub struct LazyDeltaFiatFromHeight<S, C>
where
    S: VecValue,
    C: CentsType,
{
    pub usd: LazyPerBlock<Dollars, C>,
    pub cents: LazyDeltaFromHeight<S, C, DeltaChange>,
}

/// Lazy fiat rolling deltas for all 4 windows.
///
/// Tree shape: `absolute._24h.{cents,usd}/...`, `rate._24h/...` — matches old `FiatRollingDelta`.
///
/// Replaces `FiatRollingDelta`, `FiatRollingDelta1m`, and `FiatRollingDeltaExcept1m`.
#[derive(Clone, Traversable)]
pub struct LazyRollingDeltasFiatFromHeight<S, C, B>
where
    S: VecValue,
    C: CentsType,
    B: BpsType,
{
    pub absolute: Windows<LazyDeltaFiatFromHeight<S, C>>,
    pub rate: Windows<LazyDeltaPercentFromHeight<S, B>>,
}

impl<S, C, B> LazyRollingDeltasFiatFromHeight<S, C, B>
where
    S: VecValue + Into<f64>,
    C: CentsType + From<f64>,
    B: BpsType + From<f64>,
{
    pub fn new(
        name: &str,
        version: Version,
        source: &(impl ReadableCloneableVec<Height, S> + 'static),
        cached_starts: &CachedWindowStarts,
        indexes: &indexes::Vecs,
    ) -> Self {
        let src = source.read_only_boxed_clone();

        let make_slot = |suffix: &str, cached_start: &vecdb::CachedVec<Height, Height>| {
            let full_name = format!("{name}_{suffix}");
            let cached = cached_start.clone();
            let starts_version = cached.version();

            // Change cents: source[h] - source[ago] as C (via f64)
            let change_vec = LazyDeltaVec::<Height, S, C, DeltaChange>::new(
                &format!("{full_name}_change"),
                version,
                src.clone(),
                starts_version,
                {
                    let cached = cached.clone();
                    move || cached.get()
                },
            );
            let change_resolutions = Resolutions::forced_import(
                &format!("{full_name}_change"),
                change_vec.read_only_boxed_clone(),
                version,
                indexes,
            );
            let cents = LazyDeltaFromHeight {
                height: change_vec,
                resolutions: Box::new(change_resolutions),
            };

            // Change USD: lazy from cents delta
            let usd = LazyPerBlock {
                height: LazyVecFrom1::transformed::<C::ToDollars>(
                    &format!("{full_name}_change_usd"),
                    version,
                    cents.height.read_only_boxed_clone(),
                ),
                resolutions: Box::new(DerivedResolutions::from_derived_computed::<C::ToDollars>(
                    &format!("{full_name}_change_usd"),
                    version,
                    &cents.resolutions,
                )),
            };

            let absolute = LazyDeltaFiatFromHeight { usd, cents };

            // Rate BPS: (source[h] - source[ago]) / source[ago] as B (via f64)
            let rate_vec = LazyDeltaVec::<Height, S, B, DeltaRate>::new(
                &format!("{full_name}_rate_bps"),
                version,
                src.clone(),
                starts_version,
                move || cached.get(),
            );
            let rate_resolutions = Resolutions::forced_import(
                &format!("{full_name}_rate_bps"),
                rate_vec.read_only_boxed_clone(),
                version,
                indexes,
            );
            let bps = LazyDeltaFromHeight {
                height: rate_vec,
                resolutions: Box::new(rate_resolutions),
            };

            let ratio = LazyPerBlock {
                height: LazyVecFrom1::transformed::<B::ToRatio>(
                    &format!("{full_name}_rate_ratio"),
                    version,
                    bps.height.read_only_boxed_clone(),
                ),
                resolutions: Box::new(DerivedResolutions::from_derived_computed::<B::ToRatio>(
                    &format!("{full_name}_rate_ratio"),
                    version,
                    &bps.resolutions,
                )),
            };

            let percent = LazyPerBlock {
                height: LazyVecFrom1::transformed::<B::ToPercent>(
                    &format!("{full_name}_rate"),
                    version,
                    bps.height.read_only_boxed_clone(),
                ),
                resolutions: Box::new(DerivedResolutions::from_derived_computed::<B::ToPercent>(
                    &format!("{full_name}_rate"),
                    version,
                    &bps.resolutions,
                )),
            };

            let rate = LazyDeltaPercentFromHeight {
                bps,
                ratio,
                percent,
            };

            (absolute, rate)
        };

        let (absolute, rate) = cached_starts.0.map_with_suffix(make_slot).unzip();

        Self { absolute, rate }
    }
}
