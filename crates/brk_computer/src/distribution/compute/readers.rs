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
    pub output_type: OutputType,
    pub type_index: TypeIndex,
}

/// Readers for txout vectors. Reuses internal buffers across blocks.
pub struct TxOutReaders<'a> {
    indexer: &'a Indexer,
    values_buf: Vec<Sats>,
    output_types_buf: Vec<OutputType>,
    type_indexes_buf: Vec<TypeIndex>,
    txout_data_buf: Vec<TxOutData>,
}

impl<'a> TxOutReaders<'a> {
    pub(crate) fn new(indexer: &'a Indexer) -> Self {
        Self {
            indexer,
            values_buf: Vec::new(),
            output_types_buf: Vec::new(),
            type_indexes_buf: Vec::new(),
            txout_data_buf: Vec::new(),
        }
    }

    /// Collect output data for a block range using bulk reads with buffer reuse.
    pub(crate) fn collect_block_outputs(
        &mut self,
        first_txout_index: usize,
        output_count: usize,
    ) -> &[TxOutData] {
        let end = first_txout_index + output_count;
        self.indexer.vecs.outputs.value.collect_range_into_at(
            first_txout_index,
            end,
            &mut self.values_buf,
        );
        self.indexer.vecs.outputs.output_type.collect_range_into_at(
            first_txout_index,
            end,
            &mut self.output_types_buf,
        );
        self.indexer.vecs.outputs.type_index.collect_range_into_at(
            first_txout_index,
            end,
            &mut self.type_indexes_buf,
        );

        self.txout_data_buf.clear();
        self.txout_data_buf.extend(
            self.values_buf
                .iter()
                .zip(&self.output_types_buf)
                .zip(&self.type_indexes_buf)
                .map(|((&value, &output_type), &type_index)| TxOutData {
                    value,
                    output_type,
                    type_index,
                }),
        );
        &self.txout_data_buf
    }
}

/// Readers for txin vectors. Reuses all buffers across blocks.
pub struct TxInReaders<'a> {
    indexer: &'a Indexer,
    txins: &'a inputs::Vecs,
    tx_index_to_height: &'a mut RangeMap<TxIndex, Height>,
    outpoints_buf: Vec<OutPoint>,
    values_buf: Vec<Sats>,
    prev_heights_buf: Vec<Height>,
    output_types_buf: Vec<OutputType>,
    type_indexes_buf: Vec<TypeIndex>,
}

impl<'a> TxInReaders<'a> {
    pub(crate) fn new(
        indexer: &'a Indexer,
        txins: &'a inputs::Vecs,
        tx_index_to_height: &'a mut RangeMap<TxIndex, Height>,
    ) -> Self {
        Self {
            indexer,
            txins,
            tx_index_to_height,
            outpoints_buf: Vec::new(),
            values_buf: Vec::new(),
            prev_heights_buf: Vec::new(),
            output_types_buf: Vec::new(),
            type_indexes_buf: Vec::new(),
        }
    }

    /// Collect input data for a block range using bulk reads with buffer reuse.
    pub(crate) fn collect_block_inputs(
        &mut self,
        first_txin_index: usize,
        input_count: usize,
        current_height: Height,
    ) -> (&[Sats], &[Height], &[OutputType], &[TypeIndex]) {
        let end = first_txin_index + input_count;
        self.txins.spent.value.collect_range_into_at(
            first_txin_index,
            end,
            &mut self.values_buf,
        );
        self.indexer.vecs.inputs.outpoint.collect_range_into_at(
            first_txin_index,
            end,
            &mut self.outpoints_buf,
        );
        self.indexer.vecs.inputs.output_type.collect_range_into_at(
            first_txin_index,
            end,
            &mut self.output_types_buf,
        );
        self.indexer.vecs.inputs.type_index.collect_range_into_at(
            first_txin_index,
            end,
            &mut self.type_indexes_buf,
        );

        self.prev_heights_buf.clear();
        self.prev_heights_buf.extend(
            self.outpoints_buf.iter().map(|outpoint| {
                if outpoint.is_coinbase() {
                    current_height
                } else {
                    self.tx_index_to_height
                        .get(outpoint.tx_index())
                        .unwrap_or(current_height)
                }
            }),
        );

        (
            &self.values_buf,
            &self.prev_heights_buf,
            &self.output_types_buf,
            &self.type_indexes_buf,
        )
    }
}

/// Cached readers for stateful vectors.
pub struct VecsReaders {
    pub address_type_index_to_any_address_index: ByAddressType<Reader>,
    pub any_address_index_to_any_address_data: ByAnyAddress<Reader>,
}

impl VecsReaders {
    pub(crate) fn new(
        any_address_indexes: &AnyAddressIndexesVecs,
        addresses_data: &AddressesDataVecs,
    ) -> Self {
        Self {
            address_type_index_to_any_address_index: ByAddressType {
                p2a: any_address_indexes.p2a.create_reader(),
                p2pk33: any_address_indexes.p2pk33.create_reader(),
                p2pk65: any_address_indexes.p2pk65.create_reader(),
                p2pkh: any_address_indexes.p2pkh.create_reader(),
                p2sh: any_address_indexes.p2sh.create_reader(),
                p2tr: any_address_indexes.p2tr.create_reader(),
                p2wpkh: any_address_indexes.p2wpkh.create_reader(),
                p2wsh: any_address_indexes.p2wsh.create_reader(),
            },
            any_address_index_to_any_address_data: ByAnyAddress {
                funded: addresses_data.funded.create_reader(),
                empty: addresses_data.empty.create_reader(),
            },
        }
    }

    /// Get reader for specific address type.
    pub(crate) fn address_reader(&self, address_type: OutputType) -> &Reader {
        self.address_type_index_to_any_address_index
            .get(address_type)
            .unwrap()
    }
}

/// Reusable buffers for per-block tx_index mapping construction.
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

    /// Build index -> tx_index mapping for a block, reusing internal buffers.
    pub(crate) fn build(
        &mut self,
        block_first_tx_index: TxIndex,
        block_tx_count: u64,
        tx_index_to_count: &impl ReadableVec<TxIndex, StoredU64>,
    ) -> &[TxIndex] {
        let first = block_first_tx_index.to_usize();
        tx_index_to_count.collect_range_into_at(
            first,
            first + block_tx_count as usize,
            &mut self.counts,
        );

        let total: u64 = self.counts.iter().map(|c| u64::from(*c)).sum();
        self.result.clear();
        self.result.reserve(total as usize);

        for (offset, count) in self.counts.iter().enumerate() {
            let tx_index = TxIndex::from(first + offset);
            self.result
                .extend(std::iter::repeat_n(tx_index, u64::from(*count) as usize));
        }

        &self.result
    }
}
