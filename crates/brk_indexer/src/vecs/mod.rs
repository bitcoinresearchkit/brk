use std::{fs, path::Path};

use brk_core::{
    AddressBytes, BlockHash, EmptyOutputIndex, Height, InputIndex, OpReturnIndex, OutputIndex,
    OutputType, OutputTypeIndex, P2ABytes, P2AIndex, P2MSIndex, P2PK33Bytes, P2PK33Index,
    P2PK65Bytes, P2PK65Index, P2PKHBytes, P2PKHIndex, P2SHBytes, P2SHIndex, P2TRBytes, P2TRIndex,
    P2WPKHBytes, P2WPKHIndex, P2WSHBytes, P2WSHIndex, RawLockTime, Sats, StoredF64, StoredU32,
    StoredUsize, Timestamp, TxIndex, TxVersion, Txid, UnknownOutputIndex, Weight,
};
use brk_vec::{AnyCollectableVec, AnyIndexedVec, Compressed, IndexedVec, Result, Version};
use rayon::prelude::*;

use crate::Indexes;

#[derive(Clone)]
pub struct Vecs {
    pub emptyoutputindex_to_txindex: IndexedVec<EmptyOutputIndex, TxIndex>,
    pub height_to_blockhash: IndexedVec<Height, BlockHash>,
    pub height_to_difficulty: IndexedVec<Height, StoredF64>,
    pub height_to_first_emptyoutputindex: IndexedVec<Height, EmptyOutputIndex>,
    pub height_to_first_inputindex: IndexedVec<Height, InputIndex>,
    pub height_to_first_opreturnindex: IndexedVec<Height, OpReturnIndex>,
    pub height_to_first_outputindex: IndexedVec<Height, OutputIndex>,
    pub height_to_first_p2aindex: IndexedVec<Height, P2AIndex>,
    pub height_to_first_p2msindex: IndexedVec<Height, P2MSIndex>,
    pub height_to_first_p2pk33index: IndexedVec<Height, P2PK33Index>,
    pub height_to_first_p2pk65index: IndexedVec<Height, P2PK65Index>,
    pub height_to_first_p2pkhindex: IndexedVec<Height, P2PKHIndex>,
    pub height_to_first_p2shindex: IndexedVec<Height, P2SHIndex>,
    pub height_to_first_p2trindex: IndexedVec<Height, P2TRIndex>,
    pub height_to_first_p2wpkhindex: IndexedVec<Height, P2WPKHIndex>,
    pub height_to_first_p2wshindex: IndexedVec<Height, P2WSHIndex>,
    pub height_to_first_txindex: IndexedVec<Height, TxIndex>,
    pub height_to_first_unknownoutputindex: IndexedVec<Height, UnknownOutputIndex>,
    /// Doesn't guarantee continuity due to possible reorgs
    pub height_to_timestamp: IndexedVec<Height, Timestamp>,
    pub height_to_total_size: IndexedVec<Height, StoredUsize>,
    pub height_to_weight: IndexedVec<Height, Weight>,
    /// If outputindex == Outputindex::MAX then it's coinbase
    pub inputindex_to_outputindex: IndexedVec<InputIndex, OutputIndex>,
    pub opreturnindex_to_txindex: IndexedVec<OpReturnIndex, TxIndex>,
    pub outputindex_to_outputtype: IndexedVec<OutputIndex, OutputType>,
    pub outputindex_to_outputtypeindex: IndexedVec<OutputIndex, OutputTypeIndex>,
    pub outputindex_to_value: IndexedVec<OutputIndex, Sats>,
    pub p2aindex_to_p2abytes: IndexedVec<P2AIndex, P2ABytes>,
    pub p2msindex_to_txindex: IndexedVec<P2MSIndex, TxIndex>,
    pub p2pk33index_to_p2pk33bytes: IndexedVec<P2PK33Index, P2PK33Bytes>,
    pub p2pk65index_to_p2pk65bytes: IndexedVec<P2PK65Index, P2PK65Bytes>,
    pub p2pkhindex_to_p2pkhbytes: IndexedVec<P2PKHIndex, P2PKHBytes>,
    pub p2shindex_to_p2shbytes: IndexedVec<P2SHIndex, P2SHBytes>,
    pub p2trindex_to_p2trbytes: IndexedVec<P2TRIndex, P2TRBytes>,
    pub p2wpkhindex_to_p2wpkhbytes: IndexedVec<P2WPKHIndex, P2WPKHBytes>,
    pub p2wshindex_to_p2wshbytes: IndexedVec<P2WSHIndex, P2WSHBytes>,
    pub txindex_to_base_size: IndexedVec<TxIndex, StoredU32>,
    pub txindex_to_first_inputindex: IndexedVec<TxIndex, InputIndex>,
    pub txindex_to_first_outputindex: IndexedVec<TxIndex, OutputIndex>,
    pub txindex_to_is_explicitly_rbf: IndexedVec<TxIndex, bool>,
    pub txindex_to_rawlocktime: IndexedVec<TxIndex, RawLockTime>,
    pub txindex_to_total_size: IndexedVec<TxIndex, StoredU32>,
    pub txindex_to_txid: IndexedVec<TxIndex, Txid>,
    pub txindex_to_txversion: IndexedVec<TxIndex, TxVersion>,
    pub unknownoutputindex_to_txindex: IndexedVec<UnknownOutputIndex, TxIndex>,
}

impl Vecs {
    pub fn forced_import(path: &Path, compressed: Compressed) -> color_eyre::Result<Self> {
        fs::create_dir_all(path)?;

        Ok(Self {
            emptyoutputindex_to_txindex: IndexedVec::forced_import(
                path,
                "txindex",
                Version::ZERO,
                compressed,
            )?,
            height_to_blockhash: IndexedVec::forced_import(
                path,
                "blockhash",
                Version::ZERO,
                Compressed::NO,
            )?,
            height_to_difficulty: IndexedVec::forced_import(
                path,
                "difficulty",
                Version::ZERO,
                compressed,
            )?,
            height_to_first_emptyoutputindex: IndexedVec::forced_import(
                path,
                "first_emptyoutputindex",
                Version::ZERO,
                compressed,
            )?,
            height_to_first_inputindex: IndexedVec::forced_import(
                path,
                "first_inputindex",
                Version::ZERO,
                compressed,
            )?,
            height_to_first_opreturnindex: IndexedVec::forced_import(
                path,
                "first_opreturnindex",
                Version::ZERO,
                compressed,
            )?,
            height_to_first_outputindex: IndexedVec::forced_import(
                path,
                "first_outputindex",
                Version::ZERO,
                compressed,
            )?,
            height_to_first_p2aindex: IndexedVec::forced_import(
                path,
                "first_p2aindex",
                Version::ZERO,
                compressed,
            )?,
            height_to_first_p2msindex: IndexedVec::forced_import(
                path,
                "first_p2msindex",
                Version::ZERO,
                compressed,
            )?,
            height_to_first_p2pk33index: IndexedVec::forced_import(
                path,
                "first_p2pk33index",
                Version::ZERO,
                compressed,
            )?,
            height_to_first_p2pk65index: IndexedVec::forced_import(
                path,
                "first_p2pk65index",
                Version::ZERO,
                compressed,
            )?,
            height_to_first_p2pkhindex: IndexedVec::forced_import(
                path,
                "first_p2pkhindex",
                Version::ZERO,
                compressed,
            )?,
            height_to_first_p2shindex: IndexedVec::forced_import(
                path,
                "first_p2shindex",
                Version::ZERO,
                compressed,
            )?,
            height_to_first_p2trindex: IndexedVec::forced_import(
                path,
                "first_p2trindex",
                Version::ZERO,
                compressed,
            )?,
            height_to_first_p2wpkhindex: IndexedVec::forced_import(
                path,
                "first_p2wpkhindex",
                Version::ZERO,
                compressed,
            )?,
            height_to_first_p2wshindex: IndexedVec::forced_import(
                path,
                "first_p2wshindex",
                Version::ZERO,
                compressed,
            )?,
            height_to_first_txindex: IndexedVec::forced_import(
                path,
                "first_txindex",
                Version::ZERO,
                compressed,
            )?,
            height_to_first_unknownoutputindex: IndexedVec::forced_import(
                path,
                "first_unknownoutputindex",
                Version::ZERO,
                compressed,
            )?,
            height_to_timestamp: IndexedVec::forced_import(
                path,
                "timestamp",
                Version::ZERO,
                compressed,
            )?,
            height_to_total_size: IndexedVec::forced_import(
                path,
                "total_size",
                Version::ZERO,
                compressed,
            )?,
            height_to_weight: IndexedVec::forced_import(path, "weight", Version::ZERO, compressed)?,
            inputindex_to_outputindex: IndexedVec::forced_import(
                path,
                "outputindex",
                Version::ZERO,
                compressed,
            )?,
            opreturnindex_to_txindex: IndexedVec::forced_import(
                path,
                "txindex",
                Version::ZERO,
                compressed,
            )?,
            outputindex_to_outputtype: IndexedVec::forced_import(
                path,
                "outputtype",
                Version::ZERO,
                compressed,
            )?,
            outputindex_to_outputtypeindex: IndexedVec::forced_import(
                path,
                "outputtypeindex",
                Version::ZERO,
                compressed,
            )?,
            outputindex_to_value: IndexedVec::forced_import(
                path,
                "value",
                Version::ZERO,
                compressed,
            )?,
            p2aindex_to_p2abytes: IndexedVec::forced_import(
                path,
                "p2abytes",
                Version::ZERO,
                Compressed::NO,
            )?,
            p2msindex_to_txindex: IndexedVec::forced_import(
                path,
                "txindex",
                Version::ZERO,
                compressed,
            )?,
            p2pk33index_to_p2pk33bytes: IndexedVec::forced_import(
                path,
                "p2pk33bytes",
                Version::ZERO,
                Compressed::NO,
            )?,
            p2pk65index_to_p2pk65bytes: IndexedVec::forced_import(
                path,
                "p2pk65bytes",
                Version::ZERO,
                Compressed::NO,
            )?,
            p2pkhindex_to_p2pkhbytes: IndexedVec::forced_import(
                path,
                "p2pkhbytes",
                Version::ZERO,
                Compressed::NO,
            )?,
            p2shindex_to_p2shbytes: IndexedVec::forced_import(
                path,
                "p2shbytes",
                Version::ZERO,
                Compressed::NO,
            )?,
            p2trindex_to_p2trbytes: IndexedVec::forced_import(
                path,
                "p2trbytes",
                Version::ZERO,
                Compressed::NO,
            )?,
            p2wpkhindex_to_p2wpkhbytes: IndexedVec::forced_import(
                path,
                "p2wpkhbytes",
                Version::ZERO,
                Compressed::NO,
            )?,
            p2wshindex_to_p2wshbytes: IndexedVec::forced_import(
                path,
                "p2wshbytes",
                Version::ZERO,
                Compressed::NO,
            )?,
            txindex_to_base_size: IndexedVec::forced_import(
                path,
                "base_size",
                Version::ZERO,
                compressed,
            )?,
            txindex_to_first_inputindex: IndexedVec::forced_import(
                path,
                "first_inputindex",
                Version::ZERO,
                compressed,
            )?,
            txindex_to_first_outputindex: IndexedVec::forced_import(
                path,
                "first_outputindex",
                Version::ZERO,
                Compressed::NO,
            )?,
            txindex_to_is_explicitly_rbf: IndexedVec::forced_import(
                path,
                "is_explicitly_rbf",
                Version::ZERO,
                compressed,
            )?,
            txindex_to_rawlocktime: IndexedVec::forced_import(
                path,
                "rawlocktime",
                Version::ZERO,
                compressed,
            )?,
            txindex_to_total_size: IndexedVec::forced_import(
                path,
                "total_size",
                Version::ZERO,
                compressed,
            )?,
            txindex_to_txid: IndexedVec::forced_import(
                path,
                "txid",
                Version::ZERO,
                Compressed::NO,
            )?,
            txindex_to_txversion: IndexedVec::forced_import(
                path,
                "txversion",
                Version::ZERO,
                compressed,
            )?,
            unknownoutputindex_to_txindex: IndexedVec::forced_import(
                path,
                "txindex",
                Version::ZERO,
                compressed,
            )?,
        })
    }

    pub fn rollback_if_needed(&mut self, starting_indexes: &Indexes) -> brk_vec::Result<()> {
        let saved_height = starting_indexes.height.decremented().unwrap_or_default();

        let &Indexes {
            emptyoutputindex,
            height,
            inputindex,
            opreturnindex,
            outputindex,
            p2aindex,
            p2msindex,
            p2pk33index,
            p2pk65index,
            p2pkhindex,
            p2shindex,
            p2trindex,
            p2wpkhindex,
            p2wshindex,
            txindex,
            unknownoutputindex,
        } = starting_indexes;

        self.emptyoutputindex_to_txindex
            .truncate_if_needed(emptyoutputindex, saved_height)?;
        self.height_to_blockhash
            .truncate_if_needed(height, saved_height)?;
        self.height_to_difficulty
            .truncate_if_needed(height, saved_height)?;
        self.height_to_first_emptyoutputindex
            .truncate_if_needed(height, saved_height)?;
        self.height_to_first_inputindex
            .truncate_if_needed(height, saved_height)?;
        self.height_to_first_opreturnindex
            .truncate_if_needed(height, saved_height)?;
        self.height_to_first_outputindex
            .truncate_if_needed(height, saved_height)?;
        self.height_to_first_p2aindex
            .truncate_if_needed(height, saved_height)?;
        self.height_to_first_p2msindex
            .truncate_if_needed(height, saved_height)?;
        self.height_to_first_p2pk33index
            .truncate_if_needed(height, saved_height)?;
        self.height_to_first_p2pk65index
            .truncate_if_needed(height, saved_height)?;
        self.height_to_first_p2pkhindex
            .truncate_if_needed(height, saved_height)?;
        self.height_to_first_p2shindex
            .truncate_if_needed(height, saved_height)?;
        self.height_to_first_p2trindex
            .truncate_if_needed(height, saved_height)?;
        self.height_to_first_p2wpkhindex
            .truncate_if_needed(height, saved_height)?;
        self.height_to_first_p2wshindex
            .truncate_if_needed(height, saved_height)?;
        self.height_to_first_txindex
            .truncate_if_needed(height, saved_height)?;
        self.height_to_first_unknownoutputindex
            .truncate_if_needed(height, saved_height)?;
        self.height_to_timestamp
            .truncate_if_needed(height, saved_height)?;
        self.height_to_total_size
            .truncate_if_needed(height, saved_height)?;
        self.height_to_weight
            .truncate_if_needed(height, saved_height)?;
        self.inputindex_to_outputindex
            .truncate_if_needed(inputindex, saved_height)?;
        self.opreturnindex_to_txindex
            .truncate_if_needed(opreturnindex, saved_height)?;
        self.outputindex_to_outputtype
            .truncate_if_needed(outputindex, saved_height)?;
        self.outputindex_to_outputtypeindex
            .truncate_if_needed(outputindex, saved_height)?;
        self.outputindex_to_value
            .truncate_if_needed(outputindex, saved_height)?;
        self.p2aindex_to_p2abytes
            .truncate_if_needed(p2aindex, saved_height)?;
        self.p2msindex_to_txindex
            .truncate_if_needed(p2msindex, saved_height)?;
        self.p2pk33index_to_p2pk33bytes
            .truncate_if_needed(p2pk33index, saved_height)?;
        self.p2pk65index_to_p2pk65bytes
            .truncate_if_needed(p2pk65index, saved_height)?;
        self.p2pkhindex_to_p2pkhbytes
            .truncate_if_needed(p2pkhindex, saved_height)?;
        self.p2shindex_to_p2shbytes
            .truncate_if_needed(p2shindex, saved_height)?;
        self.p2trindex_to_p2trbytes
            .truncate_if_needed(p2trindex, saved_height)?;
        self.p2wpkhindex_to_p2wpkhbytes
            .truncate_if_needed(p2wpkhindex, saved_height)?;
        self.p2wshindex_to_p2wshbytes
            .truncate_if_needed(p2wshindex, saved_height)?;
        self.txindex_to_base_size
            .truncate_if_needed(txindex, saved_height)?;
        self.txindex_to_first_inputindex
            .truncate_if_needed(txindex, saved_height)?;
        self.txindex_to_first_outputindex
            .truncate_if_needed(txindex, saved_height)?;
        self.txindex_to_is_explicitly_rbf
            .truncate_if_needed(txindex, saved_height)?;
        self.txindex_to_rawlocktime
            .truncate_if_needed(txindex, saved_height)?;
        self.txindex_to_total_size
            .truncate_if_needed(txindex, saved_height)?;
        self.txindex_to_txid
            .truncate_if_needed(txindex, saved_height)?;
        self.txindex_to_txversion
            .truncate_if_needed(txindex, saved_height)?;
        self.unknownoutputindex_to_txindex
            .truncate_if_needed(unknownoutputindex, saved_height)?;

        Ok(())
    }

    pub fn push_bytes_if_needed(
        &mut self,
        index: OutputTypeIndex,
        bytes: AddressBytes,
    ) -> brk_vec::Result<()> {
        match bytes {
            AddressBytes::P2PK65(bytes) => self
                .p2pk65index_to_p2pk65bytes
                .push_if_needed(index.into(), bytes),
            AddressBytes::P2PK33(bytes) => self
                .p2pk33index_to_p2pk33bytes
                .push_if_needed(index.into(), bytes),
            AddressBytes::P2PKH(bytes) => self
                .p2pkhindex_to_p2pkhbytes
                .push_if_needed(index.into(), bytes),
            AddressBytes::P2SH(bytes) => self
                .p2shindex_to_p2shbytes
                .push_if_needed(index.into(), bytes),
            AddressBytes::P2WPKH(bytes) => self
                .p2wpkhindex_to_p2wpkhbytes
                .push_if_needed(index.into(), bytes),
            AddressBytes::P2WSH(bytes) => self
                .p2wshindex_to_p2wshbytes
                .push_if_needed(index.into(), bytes),
            AddressBytes::P2TR(bytes) => self
                .p2trindex_to_p2trbytes
                .push_if_needed(index.into(), bytes),
            AddressBytes::P2A(bytes) => self
                .p2aindex_to_p2abytes
                .push_if_needed(index.into(), bytes),
        }
    }

    pub fn flush(&mut self, height: Height) -> Result<()> {
        self.mut_vecs()
            .into_par_iter()
            .try_for_each(|vec| vec.flush(height))
    }

    pub fn starting_height(&mut self) -> Height {
        self.mut_vecs()
            .into_iter()
            .map(|vec| vec.height().map(Height::incremented).unwrap_or_default())
            .min()
            .unwrap()
    }

    pub fn vecs(&self) -> Vec<&dyn AnyCollectableVec> {
        vec![
            &self.emptyoutputindex_to_txindex,
            &self.height_to_blockhash,
            &self.height_to_difficulty,
            &self.height_to_first_emptyoutputindex,
            &self.height_to_first_inputindex,
            &self.height_to_first_opreturnindex,
            &self.height_to_first_outputindex,
            &self.height_to_first_p2aindex,
            &self.height_to_first_p2msindex,
            &self.height_to_first_p2pk33index,
            &self.height_to_first_p2pk65index,
            &self.height_to_first_p2pkhindex,
            &self.height_to_first_p2shindex,
            &self.height_to_first_p2trindex,
            &self.height_to_first_p2wpkhindex,
            &self.height_to_first_p2wshindex,
            &self.height_to_first_txindex,
            &self.height_to_first_unknownoutputindex,
            &self.height_to_timestamp,
            &self.height_to_total_size,
            &self.height_to_weight,
            &self.inputindex_to_outputindex,
            &self.opreturnindex_to_txindex,
            &self.outputindex_to_outputtype,
            &self.outputindex_to_outputtypeindex,
            &self.outputindex_to_value,
            &self.p2aindex_to_p2abytes,
            &self.p2msindex_to_txindex,
            &self.p2pk33index_to_p2pk33bytes,
            &self.p2pk65index_to_p2pk65bytes,
            &self.p2pkhindex_to_p2pkhbytes,
            &self.p2shindex_to_p2shbytes,
            &self.p2trindex_to_p2trbytes,
            &self.p2wpkhindex_to_p2wpkhbytes,
            &self.p2wshindex_to_p2wshbytes,
            &self.txindex_to_base_size,
            &self.txindex_to_first_inputindex,
            &self.txindex_to_first_outputindex,
            &self.txindex_to_is_explicitly_rbf,
            &self.txindex_to_rawlocktime,
            &self.txindex_to_total_size,
            &self.txindex_to_txid,
            &self.txindex_to_txversion,
            &self.unknownoutputindex_to_txindex,
        ]
    }

    fn mut_vecs(&mut self) -> Vec<&mut dyn AnyIndexedVec> {
        vec![
            &mut self.emptyoutputindex_to_txindex,
            &mut self.height_to_blockhash,
            &mut self.height_to_difficulty,
            &mut self.height_to_first_emptyoutputindex,
            &mut self.height_to_first_inputindex,
            &mut self.height_to_first_opreturnindex,
            &mut self.height_to_first_outputindex,
            &mut self.height_to_first_p2aindex,
            &mut self.height_to_first_p2msindex,
            &mut self.height_to_first_p2pk33index,
            &mut self.height_to_first_p2pk65index,
            &mut self.height_to_first_p2pkhindex,
            &mut self.height_to_first_p2shindex,
            &mut self.height_to_first_p2trindex,
            &mut self.height_to_first_p2wpkhindex,
            &mut self.height_to_first_p2wshindex,
            &mut self.height_to_first_txindex,
            &mut self.height_to_first_unknownoutputindex,
            &mut self.height_to_timestamp,
            &mut self.height_to_total_size,
            &mut self.height_to_weight,
            &mut self.inputindex_to_outputindex,
            &mut self.opreturnindex_to_txindex,
            &mut self.outputindex_to_outputtype,
            &mut self.outputindex_to_outputtypeindex,
            &mut self.outputindex_to_value,
            &mut self.p2aindex_to_p2abytes,
            &mut self.p2msindex_to_txindex,
            &mut self.p2pk33index_to_p2pk33bytes,
            &mut self.p2pk65index_to_p2pk65bytes,
            &mut self.p2pkhindex_to_p2pkhbytes,
            &mut self.p2shindex_to_p2shbytes,
            &mut self.p2trindex_to_p2trbytes,
            &mut self.p2wpkhindex_to_p2wpkhbytes,
            &mut self.p2wshindex_to_p2wshbytes,
            &mut self.txindex_to_base_size,
            &mut self.txindex_to_first_inputindex,
            &mut self.txindex_to_first_outputindex,
            &mut self.txindex_to_is_explicitly_rbf,
            &mut self.txindex_to_rawlocktime,
            &mut self.txindex_to_total_size,
            &mut self.txindex_to_txid,
            &mut self.txindex_to_txversion,
            &mut self.unknownoutputindex_to_txindex,
        ]
    }
}
