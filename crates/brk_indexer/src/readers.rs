use brk_grouper::ByAddressType;
use vecdb::{GenericStoredVec, Reader};

use crate::Vecs;

/// Readers for vectors that need to be accessed during block processing.
/// These provide consistent snapshots for reading while the main vectors are being modified.
pub struct Readers {
    pub txindex_to_first_txoutindex: Reader,
    pub txoutindex_to_outputtype: Reader,
    pub txoutindex_to_typeindex: Reader,
    pub addressbytes: ByAddressType<Reader>,
}

impl Readers {
    pub fn new(vecs: &Vecs) -> Self {
        Self {
            txindex_to_first_txoutindex: vecs.txindex_to_first_txoutindex.create_reader(),
            txoutindex_to_outputtype: vecs.txoutindex_to_outputtype.create_reader(),
            txoutindex_to_typeindex: vecs.txoutindex_to_typeindex.create_reader(),
            addressbytes: ByAddressType {
                p2pk65: vecs.p2pk65addressindex_to_p2pk65bytes.create_reader(),
                p2pk33: vecs.p2pk33addressindex_to_p2pk33bytes.create_reader(),
                p2pkh: vecs.p2pkhaddressindex_to_p2pkhbytes.create_reader(),
                p2sh: vecs.p2shaddressindex_to_p2shbytes.create_reader(),
                p2wpkh: vecs.p2wpkhaddressindex_to_p2wpkhbytes.create_reader(),
                p2wsh: vecs.p2wshaddressindex_to_p2wshbytes.create_reader(),
                p2tr: vecs.p2traddressindex_to_p2trbytes.create_reader(),
                p2a: vecs.p2aaddressindex_to_p2abytes.create_reader(),
            },
        }
    }
}
