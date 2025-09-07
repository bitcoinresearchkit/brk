use std::path::Path;

use brk_error::Result;
use brk_structs::{
    AddressBytes, BlockHash, EmptyOutputIndex, Height, InputIndex, OpReturnIndex, OutputIndex,
    OutputType, P2AAddressIndex, P2ABytes, P2MSOutputIndex, P2PK33AddressIndex, P2PK33Bytes,
    P2PK65AddressIndex, P2PK65Bytes, P2PKHAddressIndex, P2PKHBytes, P2SHAddressIndex, P2SHBytes,
    P2TRAddressIndex, P2TRBytes, P2WPKHAddressIndex, P2WPKHBytes, P2WSHAddressIndex, P2WSHBytes,
    RawLockTime, Sats, StoredBool, StoredF64, StoredU32, StoredU64, Timestamp, TxIndex, TxVersion,
    Txid, TypeIndex, UnknownOutputIndex, Version, Weight,
};
use rayon::prelude::*;
use vecdb::{
    AnyCollectableVec, AnyStoredVec, CompressedVec, Database, GenericStoredVec, PAGE_SIZE, RawVec,
    Stamp,
};

use crate::Indexes;

#[derive(Clone)]
pub struct Vecs {
    db: Database,

    pub emptyoutputindex_to_txindex: CompressedVec<EmptyOutputIndex, TxIndex>,
    pub height_to_blockhash: RawVec<Height, BlockHash>,
    pub height_to_difficulty: CompressedVec<Height, StoredF64>,
    pub height_to_first_emptyoutputindex: CompressedVec<Height, EmptyOutputIndex>,
    pub height_to_first_inputindex: CompressedVec<Height, InputIndex>,
    pub height_to_first_opreturnindex: CompressedVec<Height, OpReturnIndex>,
    pub height_to_first_outputindex: CompressedVec<Height, OutputIndex>,
    pub height_to_first_p2aaddressindex: CompressedVec<Height, P2AAddressIndex>,
    pub height_to_first_p2msoutputindex: CompressedVec<Height, P2MSOutputIndex>,
    pub height_to_first_p2pk33addressindex: CompressedVec<Height, P2PK33AddressIndex>,
    pub height_to_first_p2pk65addressindex: CompressedVec<Height, P2PK65AddressIndex>,
    pub height_to_first_p2pkhaddressindex: CompressedVec<Height, P2PKHAddressIndex>,
    pub height_to_first_p2shaddressindex: CompressedVec<Height, P2SHAddressIndex>,
    pub height_to_first_p2traddressindex: CompressedVec<Height, P2TRAddressIndex>,
    pub height_to_first_p2wpkhaddressindex: CompressedVec<Height, P2WPKHAddressIndex>,
    pub height_to_first_p2wshaddressindex: CompressedVec<Height, P2WSHAddressIndex>,
    pub height_to_first_txindex: CompressedVec<Height, TxIndex>,
    pub height_to_first_unknownoutputindex: CompressedVec<Height, UnknownOutputIndex>,
    /// Doesn't guarantee continuity due to possible reorgs
    pub height_to_timestamp: CompressedVec<Height, Timestamp>,
    pub height_to_total_size: CompressedVec<Height, StoredU64>,
    pub height_to_weight: CompressedVec<Height, Weight>,
    /// If outputindex == Outputindex::MAX then it's coinbase
    pub inputindex_to_outputindex: RawVec<InputIndex, OutputIndex>,
    pub opreturnindex_to_txindex: CompressedVec<OpReturnIndex, TxIndex>,
    pub outputindex_to_outputtype: RawVec<OutputIndex, OutputType>,
    pub outputindex_to_typeindex: RawVec<OutputIndex, TypeIndex>,
    pub outputindex_to_value: RawVec<OutputIndex, Sats>,
    pub p2aaddressindex_to_p2abytes: RawVec<P2AAddressIndex, P2ABytes>,
    pub p2msoutputindex_to_txindex: CompressedVec<P2MSOutputIndex, TxIndex>,
    pub p2pk33addressindex_to_p2pk33bytes: RawVec<P2PK33AddressIndex, P2PK33Bytes>,
    pub p2pk65addressindex_to_p2pk65bytes: RawVec<P2PK65AddressIndex, P2PK65Bytes>,
    pub p2pkhaddressindex_to_p2pkhbytes: RawVec<P2PKHAddressIndex, P2PKHBytes>,
    pub p2shaddressindex_to_p2shbytes: RawVec<P2SHAddressIndex, P2SHBytes>,
    pub p2traddressindex_to_p2trbytes: RawVec<P2TRAddressIndex, P2TRBytes>,
    pub p2wpkhaddressindex_to_p2wpkhbytes: RawVec<P2WPKHAddressIndex, P2WPKHBytes>,
    pub p2wshaddressindex_to_p2wshbytes: RawVec<P2WSHAddressIndex, P2WSHBytes>,
    pub txindex_to_base_size: CompressedVec<TxIndex, StoredU32>,
    pub txindex_to_first_inputindex: CompressedVec<TxIndex, InputIndex>,
    pub txindex_to_first_outputindex: CompressedVec<TxIndex, OutputIndex>,
    pub txindex_to_is_explicitly_rbf: CompressedVec<TxIndex, StoredBool>,
    pub txindex_to_rawlocktime: CompressedVec<TxIndex, RawLockTime>,
    pub txindex_to_total_size: CompressedVec<TxIndex, StoredU32>,
    pub txindex_to_txid: RawVec<TxIndex, Txid>,
    pub txindex_to_txversion: CompressedVec<TxIndex, TxVersion>,
    pub unknownoutputindex_to_txindex: CompressedVec<UnknownOutputIndex, TxIndex>,
}

impl Vecs {
    pub fn forced_import(parent: &Path, version: Version) -> Result<Self> {
        let db = Database::open(&parent.join("vecs"))?;

        db.set_min_len(PAGE_SIZE * 50_000_000)?;

        let this = Self {
            emptyoutputindex_to_txindex: CompressedVec::forced_import(&db, "txindex", version)?,
            height_to_blockhash: RawVec::forced_import(&db, "blockhash", version)?,
            height_to_difficulty: CompressedVec::forced_import(&db, "difficulty", version)?,
            height_to_first_emptyoutputindex: CompressedVec::forced_import(
                &db,
                "first_emptyoutputindex",
                version,
            )?,
            height_to_first_inputindex: CompressedVec::forced_import(
                &db,
                "first_inputindex",
                version,
            )?,
            height_to_first_opreturnindex: CompressedVec::forced_import(
                &db,
                "first_opreturnindex",
                version,
            )?,
            height_to_first_outputindex: CompressedVec::forced_import(
                &db,
                "first_outputindex",
                version,
            )?,
            height_to_first_p2aaddressindex: CompressedVec::forced_import(
                &db,
                "first_p2aaddressindex",
                version,
            )?,
            height_to_first_p2msoutputindex: CompressedVec::forced_import(
                &db,
                "first_p2msoutputindex",
                version,
            )?,
            height_to_first_p2pk33addressindex: CompressedVec::forced_import(
                &db,
                "first_p2pk33addressindex",
                version,
            )?,
            height_to_first_p2pk65addressindex: CompressedVec::forced_import(
                &db,
                "first_p2pk65addressindex",
                version,
            )?,
            height_to_first_p2pkhaddressindex: CompressedVec::forced_import(
                &db,
                "first_p2pkhaddressindex",
                version,
            )?,
            height_to_first_p2shaddressindex: CompressedVec::forced_import(
                &db,
                "first_p2shaddressindex",
                version,
            )?,
            height_to_first_p2traddressindex: CompressedVec::forced_import(
                &db,
                "first_p2traddressindex",
                version,
            )?,
            height_to_first_p2wpkhaddressindex: CompressedVec::forced_import(
                &db,
                "first_p2wpkhaddressindex",
                version,
            )?,
            height_to_first_p2wshaddressindex: CompressedVec::forced_import(
                &db,
                "first_p2wshaddressindex",
                version,
            )?,
            height_to_first_txindex: CompressedVec::forced_import(&db, "first_txindex", version)?,
            height_to_first_unknownoutputindex: CompressedVec::forced_import(
                &db,
                "first_unknownoutputindex",
                version,
            )?,
            height_to_timestamp: CompressedVec::forced_import(&db, "timestamp", version)?,
            height_to_total_size: CompressedVec::forced_import(&db, "total_size", version)?,
            height_to_weight: CompressedVec::forced_import(&db, "weight", version)?,
            inputindex_to_outputindex: RawVec::forced_import(&db, "outputindex", version)?,
            opreturnindex_to_txindex: CompressedVec::forced_import(&db, "txindex", version)?,
            outputindex_to_outputtype: RawVec::forced_import(&db, "outputtype", version)?,
            outputindex_to_typeindex: RawVec::forced_import(&db, "typeindex", version)?,
            outputindex_to_value: RawVec::forced_import(&db, "value", version)?,
            p2aaddressindex_to_p2abytes: RawVec::forced_import(&db, "p2abytes", version)?,
            p2msoutputindex_to_txindex: CompressedVec::forced_import(&db, "txindex", version)?,
            p2pk33addressindex_to_p2pk33bytes: RawVec::forced_import(&db, "p2pk33bytes", version)?,
            p2pk65addressindex_to_p2pk65bytes: RawVec::forced_import(&db, "p2pk65bytes", version)?,
            p2pkhaddressindex_to_p2pkhbytes: RawVec::forced_import(&db, "p2pkhbytes", version)?,
            p2shaddressindex_to_p2shbytes: RawVec::forced_import(&db, "p2shbytes", version)?,
            p2traddressindex_to_p2trbytes: RawVec::forced_import(&db, "p2trbytes", version)?,
            p2wpkhaddressindex_to_p2wpkhbytes: RawVec::forced_import(&db, "p2wpkhbytes", version)?,
            p2wshaddressindex_to_p2wshbytes: RawVec::forced_import(&db, "p2wshbytes", version)?,
            txindex_to_base_size: CompressedVec::forced_import(&db, "base_size", version)?,
            txindex_to_first_inputindex: CompressedVec::forced_import(
                &db,
                "first_inputindex",
                version,
            )?,
            txindex_to_first_outputindex: CompressedVec::forced_import(
                &db,
                "first_outputindex",
                version,
            )?,
            txindex_to_is_explicitly_rbf: CompressedVec::forced_import(
                &db,
                "is_explicitly_rbf",
                version,
            )?,
            txindex_to_rawlocktime: CompressedVec::forced_import(&db, "rawlocktime", version)?,
            txindex_to_total_size: CompressedVec::forced_import(&db, "total_size", version)?,
            txindex_to_txid: RawVec::forced_import(&db, "txid", version)?,
            txindex_to_txversion: CompressedVec::forced_import(&db, "txversion", version)?,
            unknownoutputindex_to_txindex: CompressedVec::forced_import(&db, "txindex", version)?,

            db,
        };

        // self.db.retain_regions(
        //     this.vecs()
        //         .into_iter()
        //         .flat_map(|v| v.region_names())
        //         .collect(),
        // )?;

        Ok(this)
    }

    pub fn rollback_if_needed(&mut self, starting_indexes: &Indexes) -> Result<()> {
        let saved_height = starting_indexes.height.decremented().unwrap_or_default();

        let &Indexes {
            emptyoutputindex,
            height,
            inputindex,
            opreturnindex,
            outputindex,
            p2aaddressindex,
            p2msoutputindex,
            p2pk33addressindex,
            p2pk65addressindex,
            p2pkhaddressindex,
            p2shaddressindex,
            p2traddressindex,
            p2wpkhaddressindex,
            p2wshaddressindex,
            txindex,
            unknownoutputindex,
        } = starting_indexes;

        let stamp = u64::from(saved_height).into();

        self.emptyoutputindex_to_txindex
            .truncate_if_needed_with_stamp(emptyoutputindex, stamp)?;
        self.height_to_blockhash
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.height_to_difficulty
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.height_to_first_emptyoutputindex
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.height_to_first_inputindex
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.height_to_first_opreturnindex
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.height_to_first_outputindex
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.height_to_first_p2aaddressindex
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.height_to_first_p2msoutputindex
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.height_to_first_p2pk33addressindex
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.height_to_first_p2pk65addressindex
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.height_to_first_p2pkhaddressindex
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.height_to_first_p2shaddressindex
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.height_to_first_p2traddressindex
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.height_to_first_p2wpkhaddressindex
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.height_to_first_p2wshaddressindex
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.height_to_first_txindex
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.height_to_first_unknownoutputindex
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.height_to_timestamp
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.height_to_total_size
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.height_to_weight
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.inputindex_to_outputindex
            .truncate_if_needed_with_stamp(inputindex, stamp)?;
        self.opreturnindex_to_txindex
            .truncate_if_needed_with_stamp(opreturnindex, stamp)?;
        self.outputindex_to_outputtype
            .truncate_if_needed_with_stamp(outputindex, stamp)?;
        self.outputindex_to_typeindex
            .truncate_if_needed_with_stamp(outputindex, stamp)?;
        self.outputindex_to_value
            .truncate_if_needed_with_stamp(outputindex, stamp)?;
        self.p2aaddressindex_to_p2abytes
            .truncate_if_needed_with_stamp(p2aaddressindex, stamp)?;
        self.p2msoutputindex_to_txindex
            .truncate_if_needed_with_stamp(p2msoutputindex, stamp)?;
        self.p2pk33addressindex_to_p2pk33bytes
            .truncate_if_needed_with_stamp(p2pk33addressindex, stamp)?;
        self.p2pk65addressindex_to_p2pk65bytes
            .truncate_if_needed_with_stamp(p2pk65addressindex, stamp)?;
        self.p2pkhaddressindex_to_p2pkhbytes
            .truncate_if_needed_with_stamp(p2pkhaddressindex, stamp)?;
        self.p2shaddressindex_to_p2shbytes
            .truncate_if_needed_with_stamp(p2shaddressindex, stamp)?;
        self.p2traddressindex_to_p2trbytes
            .truncate_if_needed_with_stamp(p2traddressindex, stamp)?;
        self.p2wpkhaddressindex_to_p2wpkhbytes
            .truncate_if_needed_with_stamp(p2wpkhaddressindex, stamp)?;
        self.p2wshaddressindex_to_p2wshbytes
            .truncate_if_needed_with_stamp(p2wshaddressindex, stamp)?;
        self.txindex_to_base_size
            .truncate_if_needed_with_stamp(txindex, stamp)?;
        self.txindex_to_first_inputindex
            .truncate_if_needed_with_stamp(txindex, stamp)?;
        self.txindex_to_first_outputindex
            .truncate_if_needed_with_stamp(txindex, stamp)?;
        self.txindex_to_is_explicitly_rbf
            .truncate_if_needed_with_stamp(txindex, stamp)?;
        self.txindex_to_rawlocktime
            .truncate_if_needed_with_stamp(txindex, stamp)?;
        self.txindex_to_total_size
            .truncate_if_needed_with_stamp(txindex, stamp)?;
        self.txindex_to_txid
            .truncate_if_needed_with_stamp(txindex, stamp)?;
        self.txindex_to_txversion
            .truncate_if_needed_with_stamp(txindex, stamp)?;
        self.unknownoutputindex_to_txindex
            .truncate_if_needed_with_stamp(unknownoutputindex, stamp)?;

        Ok(())
    }

    pub fn push_bytes_if_needed(&mut self, index: TypeIndex, bytes: AddressBytes) -> Result<()> {
        match bytes {
            AddressBytes::P2PK65(bytes) => self
                .p2pk65addressindex_to_p2pk65bytes
                .push_if_needed(index.into(), bytes)?,
            AddressBytes::P2PK33(bytes) => self
                .p2pk33addressindex_to_p2pk33bytes
                .push_if_needed(index.into(), bytes)?,
            AddressBytes::P2PKH(bytes) => self
                .p2pkhaddressindex_to_p2pkhbytes
                .push_if_needed(index.into(), bytes)?,
            AddressBytes::P2SH(bytes) => self
                .p2shaddressindex_to_p2shbytes
                .push_if_needed(index.into(), bytes)?,
            AddressBytes::P2WPKH(bytes) => self
                .p2wpkhaddressindex_to_p2wpkhbytes
                .push_if_needed(index.into(), bytes)?,
            AddressBytes::P2WSH(bytes) => self
                .p2wshaddressindex_to_p2wshbytes
                .push_if_needed(index.into(), bytes)?,
            AddressBytes::P2TR(bytes) => self
                .p2traddressindex_to_p2trbytes
                .push_if_needed(index.into(), bytes)?,
            AddressBytes::P2A(bytes) => self
                .p2aaddressindex_to_p2abytes
                .push_if_needed(index.into(), bytes)?,
        };
        Ok(())
    }

    pub fn flush(&mut self, height: Height) -> Result<()> {
        self.mut_vecs()
            .into_par_iter()
            .try_for_each(|vec| vec.stamped_flush(Stamp::from(height)))?;
        self.db.flush()?;
        Ok(())
    }

    pub fn starting_height(&mut self) -> Height {
        self.mut_vecs()
            .into_iter()
            .map(|vec| {
                let h = Height::from(vec.stamp());
                if h > Height::ZERO { h.incremented() } else { h }
            })
            .min()
            .unwrap()
    }

    pub fn punch_holes(&self) -> Result<()> {
        self.db.punch_holes()?;
        Ok(())
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
            &self.height_to_first_p2aaddressindex,
            &self.height_to_first_p2msoutputindex,
            &self.height_to_first_p2pk33addressindex,
            &self.height_to_first_p2pk65addressindex,
            &self.height_to_first_p2pkhaddressindex,
            &self.height_to_first_p2shaddressindex,
            &self.height_to_first_p2traddressindex,
            &self.height_to_first_p2wpkhaddressindex,
            &self.height_to_first_p2wshaddressindex,
            &self.height_to_first_txindex,
            &self.height_to_first_unknownoutputindex,
            &self.height_to_timestamp,
            &self.height_to_total_size,
            &self.height_to_weight,
            &self.inputindex_to_outputindex,
            &self.opreturnindex_to_txindex,
            &self.outputindex_to_outputtype,
            &self.outputindex_to_typeindex,
            &self.outputindex_to_value,
            &self.p2aaddressindex_to_p2abytes,
            &self.p2msoutputindex_to_txindex,
            &self.p2pk33addressindex_to_p2pk33bytes,
            &self.p2pk65addressindex_to_p2pk65bytes,
            &self.p2pkhaddressindex_to_p2pkhbytes,
            &self.p2shaddressindex_to_p2shbytes,
            &self.p2traddressindex_to_p2trbytes,
            &self.p2wpkhaddressindex_to_p2wpkhbytes,
            &self.p2wshaddressindex_to_p2wshbytes,
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

    fn mut_vecs(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        vec![
            &mut self.emptyoutputindex_to_txindex,
            &mut self.height_to_blockhash,
            &mut self.height_to_difficulty,
            &mut self.height_to_first_emptyoutputindex,
            &mut self.height_to_first_inputindex,
            &mut self.height_to_first_opreturnindex,
            &mut self.height_to_first_outputindex,
            &mut self.height_to_first_p2aaddressindex,
            &mut self.height_to_first_p2msoutputindex,
            &mut self.height_to_first_p2pk33addressindex,
            &mut self.height_to_first_p2pk65addressindex,
            &mut self.height_to_first_p2pkhaddressindex,
            &mut self.height_to_first_p2shaddressindex,
            &mut self.height_to_first_p2traddressindex,
            &mut self.height_to_first_p2wpkhaddressindex,
            &mut self.height_to_first_p2wshaddressindex,
            &mut self.height_to_first_txindex,
            &mut self.height_to_first_unknownoutputindex,
            &mut self.height_to_timestamp,
            &mut self.height_to_total_size,
            &mut self.height_to_weight,
            &mut self.inputindex_to_outputindex,
            &mut self.opreturnindex_to_txindex,
            &mut self.outputindex_to_outputtype,
            &mut self.outputindex_to_typeindex,
            &mut self.outputindex_to_value,
            &mut self.p2aaddressindex_to_p2abytes,
            &mut self.p2msoutputindex_to_txindex,
            &mut self.p2pk33addressindex_to_p2pk33bytes,
            &mut self.p2pk65addressindex_to_p2pk65bytes,
            &mut self.p2pkhaddressindex_to_p2pkhbytes,
            &mut self.p2shaddressindex_to_p2shbytes,
            &mut self.p2traddressindex_to_p2trbytes,
            &mut self.p2wpkhaddressindex_to_p2wpkhbytes,
            &mut self.p2wshaddressindex_to_p2wshbytes,
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
