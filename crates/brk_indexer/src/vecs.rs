use std::path::Path;

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{
    AddressBytes, BlockHash, EmptyOutputIndex, Height, OpReturnIndex, OutPoint, OutputType,
    P2AAddressIndex, P2ABytes, P2MSOutputIndex, P2PK33AddressIndex, P2PK33Bytes,
    P2PK65AddressIndex, P2PK65Bytes, P2PKHAddressIndex, P2PKHBytes, P2SHAddressIndex, P2SHBytes,
    P2TRAddressIndex, P2TRBytes, P2WPKHAddressIndex, P2WPKHBytes, P2WSHAddressIndex, P2WSHBytes,
    RawLockTime, Sats, StoredBool, StoredF64, StoredU32, StoredU64, Timestamp, TxInIndex, TxIndex,
    TxOutIndex, TxVersion, Txid, TypeIndex, UnknownOutputIndex, Version, Weight,
};
use rayon::prelude::*;
use vecdb::{AnyStoredVec, CompressedVec, Database, GenericStoredVec, PAGE_SIZE, RawVec, Stamp};

use crate::Indexes;

#[derive(Clone, Traversable)]
pub struct Vecs {
    db: Database,
    pub emptyoutputindex_to_txindex: CompressedVec<EmptyOutputIndex, TxIndex>,
    pub height_to_blockhash: RawVec<Height, BlockHash>,
    pub height_to_difficulty: CompressedVec<Height, StoredF64>,
    pub height_to_first_emptyoutputindex: CompressedVec<Height, EmptyOutputIndex>,
    pub height_to_first_opreturnindex: CompressedVec<Height, OpReturnIndex>,
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
    pub height_to_first_txinindex: CompressedVec<Height, TxInIndex>,
    pub height_to_first_txoutindex: CompressedVec<Height, TxOutIndex>,
    pub height_to_first_unknownoutputindex: CompressedVec<Height, UnknownOutputIndex>,
    /// Doesn't guarantee continuity due to possible reorgs and more generally the nature of mining
    pub height_to_timestamp: CompressedVec<Height, Timestamp>,
    pub height_to_total_size: CompressedVec<Height, StoredU64>,
    pub height_to_weight: CompressedVec<Height, Weight>,
    pub opreturnindex_to_txindex: CompressedVec<OpReturnIndex, TxIndex>,
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
    pub txindex_to_first_txinindex: CompressedVec<TxIndex, TxInIndex>,
    pub txindex_to_first_txoutindex: CompressedVec<TxIndex, TxOutIndex>,
    pub txindex_to_height: CompressedVec<TxIndex, Height>,
    pub txindex_to_is_explicitly_rbf: CompressedVec<TxIndex, StoredBool>,
    pub txindex_to_rawlocktime: CompressedVec<TxIndex, RawLockTime>,
    pub txindex_to_total_size: CompressedVec<TxIndex, StoredU32>,
    pub txindex_to_txid: RawVec<TxIndex, Txid>,
    pub txindex_to_txversion: CompressedVec<TxIndex, TxVersion>,
    pub txinindex_to_outpoint: CompressedVec<TxInIndex, OutPoint>,
    pub txoutindex_to_outputtype: RawVec<TxOutIndex, OutputType>,
    pub txoutindex_to_txindex: CompressedVec<TxOutIndex, TxIndex>,
    pub txoutindex_to_typeindex: RawVec<TxOutIndex, TypeIndex>,
    pub txoutindex_to_value: RawVec<TxOutIndex, Sats>,
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
            height_to_first_txinindex: CompressedVec::forced_import(
                &db,
                "first_txinindex",
                version,
            )?,
            height_to_first_opreturnindex: CompressedVec::forced_import(
                &db,
                "first_opreturnindex",
                version,
            )?,
            height_to_first_txoutindex: CompressedVec::forced_import(
                &db,
                "first_txoutindex",
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
            opreturnindex_to_txindex: CompressedVec::forced_import(&db, "txindex", version)?,
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
            txindex_to_height: CompressedVec::forced_import(&db, "height", version)?,
            txindex_to_first_txinindex: CompressedVec::forced_import(
                &db,
                "first_txinindex",
                version,
            )?,
            txindex_to_first_txoutindex: CompressedVec::forced_import(
                &db,
                "first_txoutindex",
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
            txinindex_to_outpoint: CompressedVec::forced_import(&db, "outpoint", version)?,
            txoutindex_to_outputtype: RawVec::forced_import(&db, "outputtype", version)?,
            txoutindex_to_txindex: CompressedVec::forced_import(&db, "txindex", version)?,
            txoutindex_to_typeindex: RawVec::forced_import(&db, "typeindex", version)?,
            txoutindex_to_value: RawVec::forced_import(&db, "value", version)?,
            unknownoutputindex_to_txindex: CompressedVec::forced_import(&db, "txindex", version)?,

            db,
        };

        this.db.retain_regions(
            this.iter_any_collectable()
                .flat_map(|v| v.region_names())
                .collect(),
        )?;

        Ok(this)
    }

    pub fn rollback_if_needed(&mut self, starting_indexes: &Indexes) -> Result<()> {
        let saved_height = starting_indexes.height.decremented().unwrap_or_default();

        let &Indexes {
            emptyoutputindex,
            height,
            txinindex,
            opreturnindex,
            txoutindex,
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
        self.height_to_first_txinindex
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.height_to_first_opreturnindex
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.height_to_first_txoutindex
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
        self.txinindex_to_outpoint
            .truncate_if_needed_with_stamp(txinindex, stamp)?;
        self.opreturnindex_to_txindex
            .truncate_if_needed_with_stamp(opreturnindex, stamp)?;
        self.txoutindex_to_outputtype
            .truncate_if_needed_with_stamp(txoutindex, stamp)?;
        self.txoutindex_to_typeindex
            .truncate_if_needed_with_stamp(txoutindex, stamp)?;
        self.txoutindex_to_value
            .truncate_if_needed_with_stamp(txoutindex, stamp)?;
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
        self.txindex_to_first_txinindex
            .truncate_if_needed_with_stamp(txindex, stamp)?;
        self.txindex_to_first_txoutindex
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
                .push_if_needed(index.into(), *bytes)?,
            AddressBytes::P2PK33(bytes) => self
                .p2pk33addressindex_to_p2pk33bytes
                .push_if_needed(index.into(), *bytes)?,
            AddressBytes::P2PKH(bytes) => self
                .p2pkhaddressindex_to_p2pkhbytes
                .push_if_needed(index.into(), *bytes)?,
            AddressBytes::P2SH(bytes) => self
                .p2shaddressindex_to_p2shbytes
                .push_if_needed(index.into(), *bytes)?,
            AddressBytes::P2WPKH(bytes) => self
                .p2wpkhaddressindex_to_p2wpkhbytes
                .push_if_needed(index.into(), *bytes)?,
            AddressBytes::P2WSH(bytes) => self
                .p2wshaddressindex_to_p2wshbytes
                .push_if_needed(index.into(), *bytes)?,
            AddressBytes::P2TR(bytes) => self
                .p2traddressindex_to_p2trbytes
                .push_if_needed(index.into(), *bytes)?,
            AddressBytes::P2A(bytes) => self
                .p2aaddressindex_to_p2abytes
                .push_if_needed(index.into(), *bytes)?,
        };
        Ok(())
    }

    pub fn flush(&mut self, height: Height) -> Result<()> {
        self.iter_mut_any_stored_vec()
            .par_bridge()
            .try_for_each(|vec| vec.stamped_flush(Stamp::from(height)))?;
        self.db.flush()?;
        Ok(())
    }

    pub fn starting_height(&mut self) -> Height {
        self.iter_mut_any_stored_vec()
            .map(|vec| {
                let h = Height::from(vec.stamp());
                if h > Height::ZERO { h.incremented() } else { h }
            })
            .min()
            .unwrap()
    }

    pub fn compact(&self) -> Result<()> {
        self.db.compact()?;
        Ok(())
    }

    fn iter_mut_any_stored_vec(&mut self) -> impl Iterator<Item = &mut dyn AnyStoredVec> {
        [
            &mut self.emptyoutputindex_to_txindex as &mut dyn AnyStoredVec,
            &mut self.height_to_blockhash,
            &mut self.height_to_difficulty,
            &mut self.height_to_first_emptyoutputindex,
            &mut self.height_to_first_opreturnindex,
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
            &mut self.height_to_first_txinindex,
            &mut self.height_to_first_txoutindex,
            &mut self.height_to_first_unknownoutputindex,
            &mut self.height_to_timestamp,
            &mut self.height_to_total_size,
            &mut self.height_to_weight,
            &mut self.opreturnindex_to_txindex,
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
            &mut self.txindex_to_first_txinindex,
            &mut self.txindex_to_first_txoutindex,
            &mut self.txindex_to_height,
            &mut self.txindex_to_is_explicitly_rbf,
            &mut self.txindex_to_rawlocktime,
            &mut self.txindex_to_total_size,
            &mut self.txindex_to_txid,
            &mut self.txindex_to_txversion,
            &mut self.txinindex_to_outpoint,
            &mut self.txoutindex_to_outputtype,
            &mut self.txoutindex_to_txindex,
            &mut self.txoutindex_to_typeindex,
            &mut self.txoutindex_to_value,
            &mut self.unknownoutputindex_to_txindex,
        ]
        .into_iter()
    }
}
