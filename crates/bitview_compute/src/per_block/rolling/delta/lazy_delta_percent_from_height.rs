use bitview_traversable::Traversable;
use brk_types::{StoredF32, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{DeltaRate, ReadableCloneableVec, VecValue};

use crate::{FixedRatio, LazyPerBlock, Percent};

use super::LazyDeltaFromHeight;

#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(transparent)]
pub struct LazyDeltaPercentFromHeight<S, B>(
    pub Percent<LazyDeltaFromHeight<S, B, DeltaRate>, LazyPerBlock<StoredF32, B>>,
)
where
    S: VecValue,
    B: FixedRatio;

impl<S, B> LazyDeltaPercentFromHeight<S, B>
where
    S: VecValue + Into<f64>,
    B: FixedRatio + From<f64>,
{
    pub fn from_ppm(
        name: &str,
        version: Version,
        ppm: LazyDeltaFromHeight<S, B, DeltaRate>,
    ) -> Self {
        let ratio_name = format!("{name}_rate_ratio");
        let ratio = LazyPerBlock::from_resolutions::<B::ToRatio>(
            &ratio_name,
            version,
            ppm.height.read_only_boxed_clone(),
            &ppm.resolutions,
        );

        let percent_name = format!("{name}_rate");
        let percent = LazyPerBlock::from_resolutions::<B::ToPercent>(
            &percent_name,
            version,
            ppm.height.read_only_boxed_clone(),
            &ppm.resolutions,
        );

        Self(Percent {
            ppm,
            ratio,
            percent,
        })
    }
}
