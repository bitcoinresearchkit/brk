use brk_cohort::ByAddrType;
use brk_types::{Height, StoredU64};
use derive_more::{Deref, DerefMut};
use vecdb::ReadableVec;

use crate::internal::PerBlock;

use super::vecs::ExposedAddrCountAllVecs;

/// Runtime counter for exposed address counts per address type.
#[derive(Debug, Default, Deref, DerefMut)]
pub struct AddrTypeToExposedAddrCount(ByAddrType<u64>);

impl AddrTypeToExposedAddrCount {
    #[inline]
    pub(crate) fn sum(&self) -> u64 {
        self.0.values().sum()
    }
}

impl From<(&ExposedAddrCountAllVecs, Height)> for AddrTypeToExposedAddrCount {
    #[inline]
    fn from((vecs, starting_height): (&ExposedAddrCountAllVecs, Height)) -> Self {
        if let Some(prev_height) = starting_height.decremented() {
            let read = |v: &PerBlock<StoredU64>| -> u64 {
                v.height.collect_one(prev_height).unwrap().into()
            };
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
