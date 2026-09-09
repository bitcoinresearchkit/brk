use bitview_traversable::Traversable;
use bitview_vecs::StoredSeries;
use brk_types::{StoredBool, TxIndex};
use derive_more::{Deref, DerefMut};
use vecdb::{Rw, StorageMode};

use super::{CountVecs, Flags};

#[derive(Deref, DerefMut, Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub count: CountVecs<M>,
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub flags: Flags<StoredSeries<TxIndex, StoredBool, M>>,
}
