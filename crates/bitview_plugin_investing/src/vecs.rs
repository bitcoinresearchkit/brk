mod compute;
mod import;

use bitview_plugin::{Plugin, PluginStorage};
use bitview_traversable::Traversable;
use brk_types::{Day1, Height, Sats};
use vecdb::{Database, Rw, StorageMode};

use super::{STORAGE, class_vecs::ClassVecs, period_vecs::PeriodVecs};
use bitview_vecs::{CachedSeries, LazyPreviousDeltaVec};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    #[traversable(skip)]
    db: Database,
    /// Shared cumulative daily purchases, including the represented day's close.
    #[traversable(hidden)]
    pub sats_cumulative: CachedSeries<Day1, Sats, M>,
    /// Satoshis purchased by investing 100 USD at each UTC daily close newly
    /// crossed at this block. It is zero within a day, includes every
    /// intervening daily purchase when block time skips days, and treats a
    /// missing or zero daily close as a zero purchase.
    pub sats_per_day: LazyPreviousDeltaVec<Height, Sats>,
    pub period: PeriodVecs,
    pub class: ClassVecs,
}

impl<M: StorageMode> Plugin for Vecs<M>
where
    Self: Traversable + Send + Sync,
{
    fn storage(&self) -> PluginStorage {
        STORAGE
    }
}
