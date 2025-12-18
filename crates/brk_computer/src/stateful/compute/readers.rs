//! Cached readers for efficient data access during computation.
//!
//! Readers provide mmap-based access to indexed data without repeated syscalls.

use brk_grouper::{ByAddressType, ByAnyAddress};
use brk_indexer::Indexer;
use brk_types::{OutPoint, OutputType, Sats, StoredU64, TxInIndex, TxIndex, TxOutIndex, TypeIndex};
use vecdb::{
    BoxedVecIterator, BytesVecIterator, GenericStoredVec, PcodecVecIterator, Reader, VecIndex,
    VecIterator,
};

use crate::stateful::address::{AddressesDataVecs, AnyAddressIndexesVecs};

/// Cached readers for indexer vectors.
pub struct IndexerReaders {
    pub txindex_to_first_txoutindex: Reader,
    pub txoutindex_to_value: Reader,
    pub txoutindex_to_outputtype: Reader,
    pub txoutindex_to_typeindex: Reader,
}

impl IndexerReaders {
    pub fn new(indexer: &Indexer) -> Self {
        Self {
            txindex_to_first_txoutindex: indexer
                .vecs
                .tx
                .txindex_to_first_txoutindex
                .create_reader(),
            txoutindex_to_value: indexer.vecs.txout.txoutindex_to_value.create_reader(),
            txoutindex_to_outputtype: indexer.vecs.txout.txoutindex_to_outputtype.create_reader(),
            txoutindex_to_typeindex: indexer.vecs.txout.txoutindex_to_typeindex.create_reader(),
        }
    }
}

/// Reusable iterators for txout vectors (16KB buffered reads).
///
/// Iterators are created once and re-positioned each block to avoid
/// creating new file handles repeatedly.
pub struct TxOutIterators<'a> {
    value_iter: BytesVecIterator<'a, TxOutIndex, Sats>,
    outputtype_iter: BytesVecIterator<'a, TxOutIndex, OutputType>,
    typeindex_iter: BytesVecIterator<'a, TxOutIndex, TypeIndex>,
}

impl<'a> TxOutIterators<'a> {
    pub fn new(indexer: &'a Indexer) -> Self {
        Self {
            value_iter: indexer.vecs.txout.txoutindex_to_value.into_iter(),
            outputtype_iter: indexer.vecs.txout.txoutindex_to_outputtype.into_iter(),
            typeindex_iter: indexer.vecs.txout.txoutindex_to_typeindex.into_iter(),
        }
    }

    /// Collect output data for a block range using buffered iteration.
    pub fn collect_block_outputs(
        &mut self,
        first_txoutindex: usize,
        output_count: usize,
    ) -> (Vec<Sats>, Vec<OutputType>, Vec<TypeIndex>) {
        let mut values = Vec::with_capacity(output_count);
        let mut output_types = Vec::with_capacity(output_count);
        let mut type_indexes = Vec::with_capacity(output_count);

        for i in first_txoutindex..first_txoutindex + output_count {
            values.push(self.value_iter.get_at_unwrap(i));
            output_types.push(self.outputtype_iter.get_at_unwrap(i));
            type_indexes.push(self.typeindex_iter.get_at_unwrap(i));
        }

        (values, output_types, type_indexes)
    }
}

/// Reusable iterator for txin outpoints (PcoVec - avoids repeated page decompression).
pub struct TxInIterators<'a> {
    outpoint_iter: PcodecVecIterator<'a, TxInIndex, OutPoint>,
}

impl<'a> TxInIterators<'a> {
    pub fn new(indexer: &'a Indexer) -> Self {
        Self {
            outpoint_iter: indexer.vecs.txin.txinindex_to_outpoint.into_iter(),
        }
    }

    /// Collect outpoints for a block range using buffered iteration.
    /// This avoids repeated PcoVec page decompression (~1000x speedup).
    pub fn collect_block_outpoints(
        &mut self,
        first_txinindex: usize,
        input_count: usize,
    ) -> Vec<OutPoint> {
        (first_txinindex..first_txinindex + input_count)
            .map(|i| self.outpoint_iter.get_at_unwrap(i))
            .collect()
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
    txindex_to_count: &mut BoxedVecIterator<'a, TxIndex, StoredU64>,
) -> Vec<TxIndex> {
    build_index_to_txindex(block_first_txindex, block_tx_count, txindex_to_count)
}

/// Build txinindex -> txindex mapping for a block.
pub fn build_txinindex_to_txindex<'a>(
    block_first_txindex: TxIndex,
    block_tx_count: u64,
    txindex_to_count: &mut BoxedVecIterator<'a, TxIndex, StoredU64>,
) -> Vec<TxIndex> {
    build_index_to_txindex(block_first_txindex, block_tx_count, txindex_to_count)
}

/// Build index -> txindex mapping for a block (shared implementation).
fn build_index_to_txindex<'a>(
    block_first_txindex: TxIndex,
    block_tx_count: u64,
    txindex_to_count: &mut BoxedVecIterator<'a, TxIndex, StoredU64>,
) -> Vec<TxIndex> {
    let first = block_first_txindex.to_usize();

    let counts: Vec<u64> = (0..block_tx_count as usize)
        .map(|offset| {
            let txindex = TxIndex::from(first + offset);
            u64::from(txindex_to_count.get_unwrap(txindex))
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
