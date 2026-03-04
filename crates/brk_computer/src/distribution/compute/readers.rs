use brk_cohort::{ByAddressType, ByAnyAddress};
use brk_indexer::Indexer;
use brk_types::{Height, OutPoint, OutputType, Sats, StoredU64, TxIndex, TypeIndex};
use vecdb::{ReadableVec, Reader, VecIndex};

use crate::{
    distribution::{
        RangeMap,
        address::{AddressesDataVecs, AnyAddressIndexesVecs},
    },
    inputs,
};

/// Output data collected from separate vecs.
#[derive(Debug, Clone, Copy)]
pub struct TxOutData {
    pub value: Sats,
    pub outputtype: OutputType,
    pub typeindex: TypeIndex,
}

/// Readers for txout vectors. Reuses internal buffers across blocks.
pub struct TxOutReaders<'a> {
    indexer: &'a Indexer,
    values_buf: Vec<Sats>,
    outputtypes_buf: Vec<OutputType>,
    typeindexes_buf: Vec<TypeIndex>,
    txoutdata_buf: Vec<TxOutData>,
}

impl<'a> TxOutReaders<'a> {
    pub(crate) fn new(indexer: &'a Indexer) -> Self {
        Self {
            indexer,
            values_buf: Vec::new(),
            outputtypes_buf: Vec::new(),
            typeindexes_buf: Vec::new(),
            txoutdata_buf: Vec::new(),
        }
    }

    /// Collect output data for a block range using bulk reads with buffer reuse.
    pub(crate) fn collect_block_outputs(
        &mut self,
        first_txoutindex: usize,
        output_count: usize,
    ) -> &[TxOutData] {
        let end = first_txoutindex + output_count;
        self.indexer.vecs.outputs.value.collect_range_into_at(
            first_txoutindex,
            end,
            &mut self.values_buf,
        );
        self.indexer.vecs.outputs.outputtype.collect_range_into_at(
            first_txoutindex,
            end,
            &mut self.outputtypes_buf,
        );
        self.indexer.vecs.outputs.typeindex.collect_range_into_at(
            first_txoutindex,
            end,
            &mut self.typeindexes_buf,
        );

        self.txoutdata_buf.clear();
        self.txoutdata_buf.extend(
            self.values_buf
                .iter()
                .zip(&self.outputtypes_buf)
                .zip(&self.typeindexes_buf)
                .map(|((&value, &outputtype), &typeindex)| TxOutData {
                    value,
                    outputtype,
                    typeindex,
                }),
        );
        &self.txoutdata_buf
    }
}

/// Readers for txin vectors. Reuses all buffers across blocks.
pub struct TxInReaders<'a> {
    indexer: &'a Indexer,
    txins: &'a inputs::Vecs,
    txindex_to_height: &'a mut RangeMap<TxIndex, Height>,
    outpoints_buf: Vec<OutPoint>,
    values_buf: Vec<Sats>,
    prev_heights_buf: Vec<Height>,
    outputtypes_buf: Vec<OutputType>,
    typeindexes_buf: Vec<TypeIndex>,
}

impl<'a> TxInReaders<'a> {
    pub(crate) fn new(
        indexer: &'a Indexer,
        txins: &'a inputs::Vecs,
        txindex_to_height: &'a mut RangeMap<TxIndex, Height>,
    ) -> Self {
        Self {
            indexer,
            txins,
            txindex_to_height,
            outpoints_buf: Vec::new(),
            values_buf: Vec::new(),
            prev_heights_buf: Vec::new(),
            outputtypes_buf: Vec::new(),
            typeindexes_buf: Vec::new(),
        }
    }

    /// Collect input data for a block range using bulk reads with buffer reuse.
    pub(crate) fn collect_block_inputs(
        &mut self,
        first_txinindex: usize,
        input_count: usize,
        current_height: Height,
    ) -> (&[Sats], &[Height], &[OutputType], &[TypeIndex]) {
        let end = first_txinindex + input_count;
        self.txins.spent.value.collect_range_into_at(
            first_txinindex,
            end,
            &mut self.values_buf,
        );
        self.indexer.vecs.inputs.outpoint.collect_range_into_at(
            first_txinindex,
            end,
            &mut self.outpoints_buf,
        );
        self.indexer.vecs.inputs.outputtype.collect_range_into_at(
            first_txinindex,
            end,
            &mut self.outputtypes_buf,
        );
        self.indexer.vecs.inputs.typeindex.collect_range_into_at(
            first_txinindex,
            end,
            &mut self.typeindexes_buf,
        );

        self.prev_heights_buf.clear();
        self.prev_heights_buf.extend(
            self.outpoints_buf.iter().map(|outpoint| {
                if outpoint.is_coinbase() {
                    current_height
                } else {
                    self.txindex_to_height
                        .get(outpoint.txindex())
                        .unwrap_or(current_height)
                }
            }),
        );

        (
            &self.values_buf,
            &self.prev_heights_buf,
            &self.outputtypes_buf,
            &self.typeindexes_buf,
        )
    }
}

/// Cached readers for stateful vectors.
pub struct VecsReaders {
    pub addresstypeindex_to_anyaddressindex: ByAddressType<Reader>,
    pub anyaddressindex_to_anyaddressdata: ByAnyAddress<Reader>,
}

impl VecsReaders {
    pub(crate) fn new(
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
                funded: addresses_data.funded.create_reader(),
                empty: addresses_data.empty.create_reader(),
            },
        }
    }

    /// Get reader for specific address type.
    pub(crate) fn address_reader(&self, address_type: OutputType) -> &Reader {
        self.addresstypeindex_to_anyaddressindex
            .get(address_type)
            .unwrap()
    }
}

/// Reusable buffers for per-block txindex mapping construction.
pub(crate) struct IndexToTxIndexBuf {
    counts: Vec<StoredU64>,
    result: Vec<TxIndex>,
}

impl IndexToTxIndexBuf {
    pub(crate) fn new() -> Self {
        Self {
            counts: Vec::new(),
            result: Vec::new(),
        }
    }

    /// Build index -> txindex mapping for a block, reusing internal buffers.
    pub(crate) fn build(
        &mut self,
        block_first_txindex: TxIndex,
        block_tx_count: u64,
        txindex_to_count: &impl ReadableVec<TxIndex, StoredU64>,
    ) -> &[TxIndex] {
        let first = block_first_txindex.to_usize();
        txindex_to_count.collect_range_into_at(
            first,
            first + block_tx_count as usize,
            &mut self.counts,
        );

        let total: u64 = self.counts.iter().map(|c| u64::from(*c)).sum();
        self.result.clear();
        self.result.reserve(total as usize);

        for (offset, count) in self.counts.iter().enumerate() {
            let txindex = TxIndex::from(first + offset);
            self.result
                .extend(std::iter::repeat_n(txindex, u64::from(*count) as usize));
        }

        &self.result
    }
}
