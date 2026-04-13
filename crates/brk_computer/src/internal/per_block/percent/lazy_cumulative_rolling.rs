use brk_traversable::Traversable;
use brk_types::Version;
use vecdb::UnaryTransform;

use crate::internal::{
    BpsType, LazyPercentPerBlock, LazyPercentRollingWindows, PercentCumulativeRolling,
};

/// Fully lazy variant of `PercentCumulativeRolling` — no stored vecs.
///
/// Mirrors the flat shape of `PercentCumulativeRolling`: cumulative and
/// rolling window fields are both flattened to the same tree level, so
/// consumers see `{ bps, percent, ratio, _24h, _1w, _1m, _1y }`.
#[derive(Clone, Traversable)]
pub struct LazyPercentCumulativeRolling<B: BpsType> {
    #[traversable(flatten)]
    pub cumulative: LazyPercentPerBlock<B>,
    #[traversable(flatten)]
    pub rolling: LazyPercentRollingWindows<B>,
}

impl<B: BpsType> LazyPercentCumulativeRolling<B> {
    /// Derive from a stored `PercentCumulativeRolling` source via a
    /// BPS-to-BPS unary transform applied to both cumulative and rolling.
    pub(crate) fn from_source<F: UnaryTransform<B, B>>(
        name: &str,
        version: Version,
        source: &PercentCumulativeRolling<B>,
    ) -> Self {
        let cumulative =
            LazyPercentPerBlock::from_percent::<F>(name, version, &source.cumulative);
        let rolling = LazyPercentRollingWindows::from_rolling::<F>(name, version, &source.rolling);
        Self {
            cumulative,
            rolling,
        }
    }
}
