use brk_cohort::ByAddressType;
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
            txindex_to_first_txoutindex: vecs.transactions.first_txoutindex.create_reader(),
            txoutindex_to_outputtype: vecs.outputs.outputtype.create_reader(),
            txoutindex_to_typeindex: vecs.outputs.typeindex.create_reader(),
            addressbytes: ByAddressType {
                p2pk65: vecs
                    .addresses
                    .p2pk65bytes
                    .create_reader(),
                p2pk33: vecs
                    .addresses
                    .p2pk33bytes
                    .create_reader(),
                p2pkh: vecs.addresses.p2pkhbytes.create_reader(),
                p2sh: vecs.addresses.p2shbytes.create_reader(),
                p2wpkh: vecs
                    .addresses
                    .p2wpkhbytes
                    .create_reader(),
                p2wsh: vecs.addresses.p2wshbytes.create_reader(),
                p2tr: vecs.addresses.p2trbytes.create_reader(),
                p2a: vecs.addresses.p2abytes.create_reader(),
            },
        }
    }
}
