use brk_traversable::Traversable;
use brk_types::StoredU64;
use vecdb::{Rw, StorageMode};

use crate::internal::ComputedPerBlockCumulativeWithSums;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub p2a: ComputedPerBlockCumulativeWithSums<StoredU64, StoredU64, M>,
    pub p2ms: ComputedPerBlockCumulativeWithSums<StoredU64, StoredU64, M>,
    pub p2pk33: ComputedPerBlockCumulativeWithSums<StoredU64, StoredU64, M>,
    pub p2pk65: ComputedPerBlockCumulativeWithSums<StoredU64, StoredU64, M>,
    pub p2pkh: ComputedPerBlockCumulativeWithSums<StoredU64, StoredU64, M>,
    pub p2sh: ComputedPerBlockCumulativeWithSums<StoredU64, StoredU64, M>,
    pub p2tr: ComputedPerBlockCumulativeWithSums<StoredU64, StoredU64, M>,
    pub p2wpkh: ComputedPerBlockCumulativeWithSums<StoredU64, StoredU64, M>,
    pub p2wsh: ComputedPerBlockCumulativeWithSums<StoredU64, StoredU64, M>,
    pub opreturn: ComputedPerBlockCumulativeWithSums<StoredU64, StoredU64, M>,
    pub emptyoutput: ComputedPerBlockCumulativeWithSums<StoredU64, StoredU64, M>,
    pub unknownoutput: ComputedPerBlockCumulativeWithSums<StoredU64, StoredU64, M>,

    pub segwit: ComputedPerBlockCumulativeWithSums<StoredU64, StoredU64, M>,
}
