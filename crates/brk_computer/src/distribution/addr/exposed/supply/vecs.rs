use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Sats, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{Database, Rw, StorageMode};

use crate::{
    distribution::addr::WithAddrTypes,
    indexes,
    internal::PerBlock,
};

/// Exposed address supply (sats) — `all` + per-address-type. Tracks the total
/// balance held by addresses currently in the funded exposed set.
#[derive(Deref, DerefMut, Traversable)]
pub struct ExposedAddrSupplyVecs<M: StorageMode = Rw>(
    #[traversable(flatten)] pub WithAddrTypes<PerBlock<Sats, M>>,
);

impl ExposedAddrSupplyVecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self(WithAddrTypes::<PerBlock<Sats>>::forced_import(
            db,
            "exposed_addr_supply",
            version,
            indexes,
        )?))
    }
}
