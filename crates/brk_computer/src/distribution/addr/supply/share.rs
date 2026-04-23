use brk_cohort::ByAddrType;
use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{BasisPoints16, Height, Sats, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{Database, Exit, ReadableVec, Rw, StorageMode};

use crate::{
    indexes,
    internal::{PercentPerBlock, RatioSatsBp16, WithAddrTypes},
};

use super::vecs::AddrSupplyVecs;

/// Share of a predicate-based supply category relative to total supply.
///
/// - `all`: category supply / circulating supply
/// - Per-type: type's category supply / type's total supply
#[derive(Deref, DerefMut, Traversable)]
pub struct AddrSupplyShareVecs<M: StorageMode = Rw>(
    #[traversable(flatten)] pub WithAddrTypes<PercentPerBlock<BasisPoints16, M>>,
);

impl AddrSupplyShareVecs {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self(
            WithAddrTypes::<PercentPerBlock<BasisPoints16>>::forced_import(
                db,
                &format!("{name}_addr_supply_share"),
                version,
                indexes,
            )?,
        ))
    }

    pub(crate) fn compute_rest(
        &mut self,
        max_from: Height,
        supply: &AddrSupplyVecs,
        all_supply_sats: &impl ReadableVec<Height, Sats>,
        type_supply_sats: &ByAddrType<&impl ReadableVec<Height, Sats>>,
        exit: &Exit,
    ) -> Result<()> {
        self.all.compute_binary::<Sats, Sats, RatioSatsBp16>(
            max_from,
            &supply.all.sats.height,
            all_supply_sats,
            exit,
        )?;
        for ((_, share), ((_, cat), (_, denom))) in self
            .by_addr_type
            .iter_mut()
            .zip(supply.by_addr_type.iter().zip(type_supply_sats.iter()))
        {
            share.compute_binary::<Sats, Sats, RatioSatsBp16>(
                max_from,
                &cat.sats.height,
                *denom,
                exit,
            )?;
        }
        Ok(())
    }
}
