use brk_traversable::Traversable;
use brk_types::StoredU64;
use vecdb::{Rw, StorageMode};

use crate::internal::PerBlockCumulativeWithSums;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub p2a: PerBlockCumulativeWithSums<StoredU64, StoredU64, M>,
    pub p2ms: PerBlockCumulativeWithSums<StoredU64, StoredU64, M>,
    pub p2pk33: PerBlockCumulativeWithSums<StoredU64, StoredU64, M>,
    pub p2pk65: PerBlockCumulativeWithSums<StoredU64, StoredU64, M>,
    pub p2pkh: PerBlockCumulativeWithSums<StoredU64, StoredU64, M>,
    pub p2sh: PerBlockCumulativeWithSums<StoredU64, StoredU64, M>,
    pub p2tr: PerBlockCumulativeWithSums<StoredU64, StoredU64, M>,
    pub p2wpkh: PerBlockCumulativeWithSums<StoredU64, StoredU64, M>,
    pub p2wsh: PerBlockCumulativeWithSums<StoredU64, StoredU64, M>,
    pub opreturn: PerBlockCumulativeWithSums<StoredU64, StoredU64, M>,
    pub emptyoutput: PerBlockCumulativeWithSums<StoredU64, StoredU64, M>,
    pub unknownoutput: PerBlockCumulativeWithSums<StoredU64, StoredU64, M>,

    pub segwit: PerBlockCumulativeWithSums<StoredU64, StoredU64, M>,
}
