use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{StoredU64, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{Database, Rw, StorageMode};

use crate::{
    indexes,
    internal::{PerBlock, WithAddrTypes},
};

/// Reused address count (`all` + per-type) for a single variant (funded or total).
#[derive(Deref, DerefMut, Traversable)]
pub struct ReusedAddrCountAllVecs<M: StorageMode = Rw>(
    #[traversable(flatten)] pub WithAddrTypes<PerBlock<StoredU64, M>>,
);

impl ReusedAddrCountAllVecs {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self(WithAddrTypes::<PerBlock<StoredU64>>::forced_import(
            db, name, version, indexes,
        )?))
    }
}
