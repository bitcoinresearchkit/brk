use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::Version;
use derive_more::{Deref, DerefMut};
use vecdb::{Database, Rw, StorageMode};

use crate::{
    indexes,
    internal::{ValuePerBlock, WithAddrTypes},
};

use super::AddrTypeToSupply;

/// Per-addr-type running supply (sats/btc/cents/usd) with an aggregated `all`.
/// Shared across predicate-based supply categories (exposed, reused, respent).
/// Sats are pushed stateful per block; cents/usd are derived post-hoc from
/// sats × spot price.
#[derive(Deref, DerefMut, Traversable)]
pub struct AddrSupplyVecs<M: StorageMode = Rw>(
    #[traversable(flatten)] pub WithAddrTypes<ValuePerBlock<M>>,
);

impl AddrSupplyVecs {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self(WithAddrTypes::<ValuePerBlock>::forced_import(
            db,
            &format!("{name}_addr_supply"),
            version,
            indexes,
        )?))
    }

    #[inline(always)]
    pub(crate) fn push_supply(&mut self, supply: &AddrTypeToSupply) {
        self.push_height(supply.sum(), supply.values().copied());
    }
}
