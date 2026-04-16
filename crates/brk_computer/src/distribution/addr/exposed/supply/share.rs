use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{BasisPoints16, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{Database, Rw, StorageMode};

use crate::{
    indexes,
    internal::{PercentPerBlock, WithAddrTypes},
};

/// Share of exposed supply relative to total supply.
///
/// - `all`: exposed_supply / circulating_supply
/// - Per-type: type's exposed_supply / type's total supply
#[derive(Deref, DerefMut, Traversable)]
pub struct ExposedSupplyShareVecs<M: StorageMode = Rw>(
    #[traversable(flatten)] pub WithAddrTypes<PercentPerBlock<BasisPoints16, M>>,
);

impl ExposedSupplyShareVecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self(
            WithAddrTypes::<PercentPerBlock<BasisPoints16>>::forced_import(
                db,
                "exposed_supply_share",
                version,
                indexes,
            )?,
        ))
    }
}
