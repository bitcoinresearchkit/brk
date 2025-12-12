//! Cached readers for efficient data access during computation.
//!
//! Readers provide mmap-based access to indexed data without repeated syscalls.

use brk_grouper::{ByAddressType, ByAnyAddress};
use brk_indexer::Indexer;
use brk_types::{OutputType, StoredU64, TxIndex};
use vecdb::{BoxedVecIterator, GenericStoredVec, Reader, VecIndex};

use crate::stateful_new::address::{AddressesDataVecs, AnyAddressIndexesVecs};

/// Cached readers for indexer vectors.
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

/// Cached readers for stateful vectors.
pub struct VecsReaders {
    pub addresstypeindex_to_anyaddressindex: ByAddressType<Reader>,
    pub anyaddressindex_to_anyaddressdata: ByAnyAddress<Reader>,
}

impl VecsReaders {
    pub fn new(
        any_address_indexes: &AnyAddressIndexesVecs,
        addresses_data: &AddressesDataVecs,
    ) -> Self {
        Self {
            addresstypeindex_to_anyaddressindex: ByAddressType {
                p2a: any_address_indexes.p2a.create_reader(),
                p2pk33: any_address_indexes.p2pk33.create_reader(),
                p2pk65: any_address_indexes.p2pk65.create_reader(),
                p2pkh: any_address_indexes.p2pkh.create_reader(),
                p2sh: any_address_indexes.p2sh.create_reader(),
                p2tr: any_address_indexes.p2tr.create_reader(),
                p2wpkh: any_address_indexes.p2wpkh.create_reader(),
                p2wsh: any_address_indexes.p2wsh.create_reader(),
            },
            anyaddressindex_to_anyaddressdata: ByAnyAddress {
                loaded: addresses_data.loaded.create_reader(),
                empty: addresses_data.empty.create_reader(),
            },
        }
    }

    /// Get reader for specific address type.
    pub fn address_reader(&self, address_type: OutputType) -> &Reader {
        self.addresstypeindex_to_anyaddressindex
            .get_unwrap(address_type)
    }
}

/// Build txoutindex -> txindex mapping for a block.
pub fn build_txoutindex_to_txindex<'a>(
    block_first_txindex: TxIndex,
    block_tx_count: u64,
    txindex_to_output_count: &mut BoxedVecIterator<'a, TxIndex, StoredU64>,
) -> Vec<TxIndex> {
    let first = block_first_txindex.to_usize();

    let counts: Vec<u64> = (0..block_tx_count as usize)
        .map(|offset| {
            let txindex = TxIndex::from(first + offset);
            u64::from(txindex_to_output_count.get_unwrap(txindex))
        })
        .collect();

    let total: u64 = counts.iter().sum();
    let mut result = Vec::with_capacity(total as usize);

    for (offset, &count) in counts.iter().enumerate() {
        let txindex = TxIndex::from(first + offset);
        result.extend(std::iter::repeat(txindex).take(count as usize));
    }

    result
}

/// Build txinindex -> txindex mapping for a block.
pub fn build_txinindex_to_txindex<'a>(
    block_first_txindex: TxIndex,
    block_tx_count: u64,
    txindex_to_input_count: &mut BoxedVecIterator<'a, TxIndex, StoredU64>,
) -> Vec<TxIndex> {
    let first = block_first_txindex.to_usize();

    let counts: Vec<u64> = (0..block_tx_count as usize)
        .map(|offset| {
            let txindex = TxIndex::from(first + offset);
            u64::from(txindex_to_input_count.get_unwrap(txindex))
        })
        .collect();

    let total: u64 = counts.iter().sum();
    let mut result = Vec::with_capacity(total as usize);

    for (offset, &count) in counts.iter().enumerate() {
        let txindex = TxIndex::from(first + offset);
        result.extend(std::iter::repeat_n(txindex, count as usize));
    }

    result
}
