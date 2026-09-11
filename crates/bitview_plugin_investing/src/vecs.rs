mod import;

use bitview_plugin::{Plugin, PluginStorage};
use bitview_traversable::Traversable;
use brk_types::{Height, Sats};

use super::{STORAGE, class_vecs::ClassVecs, period_vecs::PeriodVecs};
use bitview_vecs::LazyPreviousDeltaVec;

#[derive(Clone, Traversable)]
pub struct Vecs {
    /// Satoshis purchased by investing 100 USD at each UTC daily close newly
    /// crossed at this block. It is zero within a day, includes every
    /// intervening daily purchase when block time skips days, and treats a
    /// missing or zero daily close as a zero purchase.
    pub sats_per_day: LazyPreviousDeltaVec<Height, Sats>,
    pub period: PeriodVecs,
    pub class: ClassVecs,
}

impl Plugin for Vecs {
    fn storage(&self) -> PluginStorage {
        STORAGE
    }
}
