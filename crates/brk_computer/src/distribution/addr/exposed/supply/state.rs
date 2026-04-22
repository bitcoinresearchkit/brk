use brk_cohort::ByAddrType;
use brk_types::{Height, Sats};
use derive_more::{Deref, DerefMut};
use vecdb::ReadableVec;

use crate::internal::ValuePerBlock;

use super::vecs::ExposedAddrSupplyVecs;

/// Runtime running counter for the total balance (sats) held by funded
/// exposed addresses, per address type.
#[derive(Debug, Default, Deref, DerefMut)]
pub struct AddrTypeToExposedSupply(ByAddrType<Sats>);

impl AddrTypeToExposedSupply {
    #[inline]
    pub(crate) fn sum(&self) -> Sats {
        self.0.values().copied().sum()
    }
}

impl From<(&ExposedAddrSupplyVecs, Height)> for AddrTypeToExposedSupply {
    #[inline]
    fn from((vecs, starting_height): (&ExposedAddrSupplyVecs, Height)) -> Self {
        if let Some(prev_height) = starting_height.decremented() {
            let read =
                |v: &ValuePerBlock| -> Sats { v.sats.height.collect_one(prev_height).unwrap() };
            Self(ByAddrType {
                p2pk65: read(&vecs.by_addr_type.p2pk65),
                p2pk33: read(&vecs.by_addr_type.p2pk33),
                p2pkh: read(&vecs.by_addr_type.p2pkh),
                p2sh: read(&vecs.by_addr_type.p2sh),
                p2wpkh: read(&vecs.by_addr_type.p2wpkh),
                p2wsh: read(&vecs.by_addr_type.p2wsh),
                p2tr: read(&vecs.by_addr_type.p2tr),
                p2a: read(&vecs.by_addr_type.p2a),
            })
        } else {
            Default::default()
        }
    }
}
