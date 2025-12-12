use brk_grouper::{ByAddressType, ByAnyAddress};
use brk_indexer::Indexer;
use brk_types::{OutputType, StoredU64, TxIndex};
use vecdb::{BoxedVecIterator, GenericStoredVec, Reader, VecIndex};

use super::Vecs;

pub struct IndexerReaders {
    pub txinindex_to_outpoint: Reader,
    pub txindex_to_first_txoutindex: Reader,
    pub txoutindex_to_value: Reader,
    pub txoutindex_to_outputtype: Reader,
    pub txoutindex_to_typeindex: Reader,
}

impl IndexerReaders {
    pub fn new(indexer: &Indexer) -> Self {
        Self {
            txinindex_to_outpoint: indexer.vecs.txin.txinindex_to_outpoint.create_reader(),
            txindex_to_first_txoutindex: indexer.vecs.tx.txindex_to_first_txoutindex.create_reader(),
            txoutindex_to_value: indexer.vecs.txout.txoutindex_to_value.create_reader(),
            txoutindex_to_outputtype: indexer.vecs.txout.txoutindex_to_outputtype.create_reader(),
            txoutindex_to_typeindex: indexer.vecs.txout.txoutindex_to_typeindex.create_reader(),
        }
    }
}

pub struct VecsReaders {
    pub addresstypeindex_to_anyaddressindex: ByAddressType<Reader>,
    pub anyaddressindex_to_anyaddressdata: ByAnyAddress<Reader>,
}

impl VecsReaders {
    pub fn new(vecs: &Vecs) -> Self {
        Self {
            addresstypeindex_to_anyaddressindex: ByAddressType {
                p2pk33: vecs.any_address_indexes.p2pk33.create_reader(),
                p2pk65: vecs.any_address_indexes.p2pk65.create_reader(),
                p2pkh: vecs.any_address_indexes.p2pkh.create_reader(),
                p2sh: vecs.any_address_indexes.p2sh.create_reader(),
                p2tr: vecs.any_address_indexes.p2tr.create_reader(),
                p2wpkh: vecs.any_address_indexes.p2wpkh.create_reader(),
                p2wsh: vecs.any_address_indexes.p2wsh.create_reader(),
                p2a: vecs.any_address_indexes.p2a.create_reader(),
            },
            anyaddressindex_to_anyaddressdata: ByAnyAddress {
                loaded: vecs.addresses_data.loaded.create_reader(),
                empty: vecs.addresses_data.empty.create_reader(),
            },
        }
    }

    pub fn get_anyaddressindex_reader(&self, address_type: OutputType) -> &Reader {
        self.addresstypeindex_to_anyaddressindex
            .get_unwrap(address_type)
    }
}

pub fn build_txoutindex_to_txindex<'a>(
    block_first_txindex: TxIndex,
    block_tx_count: u64,
    txindex_to_output_count: &mut BoxedVecIterator<'a, TxIndex, StoredU64>,
) -> Vec<TxIndex> {
    let block_first_txindex = block_first_txindex.to_usize();

    let counts: Vec<_> = (0..block_tx_count as usize)
        .map(|tx_offset| {
            let txindex = TxIndex::from(block_first_txindex + tx_offset);
            u64::from(txindex_to_output_count.get_unwrap(txindex))
        })
        .collect();

    let total: u64 = counts.iter().sum();
    let mut vec = Vec::with_capacity(total as usize);

    for (tx_offset, &output_count) in counts.iter().enumerate() {
        let txindex = TxIndex::from(block_first_txindex + tx_offset);
        for _ in 0..output_count {
            vec.push(txindex);
        }
    }

    vec
}

pub fn build_txinindex_to_txindex<'a>(
    block_first_txindex: TxIndex,
    block_tx_count: u64,
    txindex_to_input_count: &mut BoxedVecIterator<'a, TxIndex, StoredU64>,
) -> Vec<TxIndex> {
    let block_first_txindex = block_first_txindex.to_usize();

    let counts: Vec<_> = (0..block_tx_count as usize)
        .map(|tx_offset| {
            let txindex = TxIndex::from(block_first_txindex + tx_offset);
            u64::from(txindex_to_input_count.get_unwrap(txindex))
        })
        .collect();

    let total: u64 = counts.iter().sum();
    let mut vec = Vec::with_capacity(total as usize);

    for (tx_offset, &input_count) in counts.iter().enumerate() {
        let txindex = TxIndex::from(block_first_txindex + tx_offset);
        for _ in 0..input_count {
            vec.push(txindex);
        }
    }

    vec
}
