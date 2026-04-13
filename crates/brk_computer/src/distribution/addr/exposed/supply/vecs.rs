use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::Version;
use derive_more::{Deref, DerefMut};
use vecdb::{Database, Rw, StorageMode};

use crate::{
    indexes,
    internal::{AmountPerBlock, WithAddrTypes},
};

/// Exposed address supply (sats/btc/cents/usd) — `all` + per-address-type.
/// Tracks the total balance held by addresses currently in the funded
/// exposed set. Sats are pushed stateful per block; cents/usd are derived
/// post-hoc from sats × spot price.
#[derive(Deref, DerefMut, Traversable)]
pub struct ExposedAddrSupplyVecs<M: StorageMode = Rw>(
    #[traversable(flatten)] pub WithAddrTypes<AmountPerBlock<M>>,
);

impl ExposedAddrSupplyVecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self(WithAddrTypes::<AmountPerBlock>::forced_import(
            db,
            "exposed_addr_supply",
            version,
            indexes,
        )?))
    }
}
