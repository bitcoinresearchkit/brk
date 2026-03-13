use brk_traversable::Traversable;
use brk_types::Version;
use derive_more::{Deref, DerefMut};
use vecdb::UnaryTransform;

use crate::internal::{BpsType, PercentRollingWindows, Windows};

use super::LazyPercentPerBlock;

/// Fully lazy rolling percent windows — 4 windows (24h, 1w, 1m, 1y),
/// each with lazy BPS + lazy ratio/percent float views.
///
/// No stored vecs. All values derived from a source `PercentRollingWindows`.
#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(transparent)]
pub struct LazyPercentRollingWindows<B: BpsType>(pub Windows<LazyPercentPerBlock<B>>);

impl<B: BpsType> LazyPercentRollingWindows<B> {
    /// Create from a stored `PercentRollingWindows` source via a BPS-to-BPS unary transform.
    pub(crate) fn from_rolling<F: UnaryTransform<B, B>>(
        name: &str,
        version: Version,
        source: &PercentRollingWindows<B>,
    ) -> Self {
        Self(source.0.map_with_suffix(|suffix, source_window| {
            LazyPercentPerBlock::from_percent::<F>(
                &format!("{name}_{suffix}"),
                version,
                source_window,
            )
        }))
    }
}
