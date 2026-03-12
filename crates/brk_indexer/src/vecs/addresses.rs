use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{
    AddressBytes, AddressHash, Height, OutputType, P2AAddressIndex, P2ABytes, P2PK33AddressIndex,
    P2PK33Bytes, P2PK65AddressIndex, P2PK65Bytes, P2PKHAddressIndex, P2PKHBytes, P2SHAddressIndex,
    P2SHBytes, P2TRAddressIndex, P2TRBytes, P2WPKHAddressIndex, P2WPKHBytes, P2WSHAddressIndex,
    P2WSHBytes, TypeIndex, Version,
};
use rayon::prelude::*;
use schemars::JsonSchema;
use serde::Serialize;
use vecdb::{
    AnyStoredVec, BytesVec, BytesVecValue, Database, Formattable, ImportableVec, PcoVec,
    PcoVecValue, ReadableVec, Ro, Rw, Stamp, StorageMode, VecIndex, WritableVec,
};

use crate::parallel_import;
use crate::readers::AddressReaders;

#[derive(Traversable)]
pub struct AddressTypeVecs<I: VecIndex + PcoVecValue + Formattable + Serialize + JsonSchema, B: BytesVecValue + Formattable + Serialize + JsonSchema, M: StorageMode = Rw> {
    pub first_index: M::Stored<PcoVec<Height, I>>,
    pub bytes: M::Stored<BytesVec<I, B>>,
}

#[derive(Traversable)]
pub struct AddressesVecs<M: StorageMode = Rw> {
    pub p2pk65: AddressTypeVecs<P2PK65AddressIndex, P2PK65Bytes, M>,
    pub p2pk33: AddressTypeVecs<P2PK33AddressIndex, P2PK33Bytes, M>,
    pub p2pkh: AddressTypeVecs<P2PKHAddressIndex, P2PKHBytes, M>,
    pub p2sh: AddressTypeVecs<P2SHAddressIndex, P2SHBytes, M>,
    pub p2wpkh: AddressTypeVecs<P2WPKHAddressIndex, P2WPKHBytes, M>,
    pub p2wsh: AddressTypeVecs<P2WSHAddressIndex, P2WSHBytes, M>,
    pub p2tr: AddressTypeVecs<P2TRAddressIndex, P2TRBytes, M>,
    pub p2a: AddressTypeVecs<P2AAddressIndex, P2ABytes, M>,
}

impl AddressesVecs {
    pub fn forced_import(db: &Database, version: Version) -> Result<Self> {
        let (
            first_p2pk65addressindex,
            first_p2pk33addressindex,
            first_p2pkhaddressindex,
            first_p2shaddressindex,
            first_p2wpkhaddressindex,
            first_p2wshaddressindex,
            first_p2traddressindex,
            first_p2aaddressindex,
            p2pk65bytes,
            p2pk33bytes,
            p2pkhbytes,
            p2shbytes,
            p2wpkhbytes,
            p2wshbytes,
            p2trbytes,
            p2abytes,
        ) = parallel_import! {
            first_p2pk65addressindex = PcoVec::forced_import(db, "first_p2pk65addressindex", version),
            first_p2pk33addressindex = PcoVec::forced_import(db, "first_p2pk33addressindex", version),
            first_p2pkhaddressindex = PcoVec::forced_import(db, "first_p2pkhaddressindex", version),
            first_p2shaddressindex = PcoVec::forced_import(db, "first_p2shaddressindex", version),
            first_p2wpkhaddressindex = PcoVec::forced_import(db, "first_p2wpkhaddressindex", version),
            first_p2wshaddressindex = PcoVec::forced_import(db, "first_p2wshaddressindex", version),
            first_p2traddressindex = PcoVec::forced_import(db, "first_p2traddressindex", version),
            first_p2aaddressindex = PcoVec::forced_import(db, "first_p2aaddressindex", version),
            p2pk65bytes = BytesVec::forced_import(db, "p2pk65bytes", version),
            p2pk33bytes = BytesVec::forced_import(db, "p2pk33bytes", version),
            p2pkhbytes = BytesVec::forced_import(db, "p2pkhbytes", version),
            p2shbytes = BytesVec::forced_import(db, "p2shbytes", version),
            p2wpkhbytes = BytesVec::forced_import(db, "p2wpkhbytes", version),
            p2wshbytes = BytesVec::forced_import(db, "p2wshbytes", version),
            p2trbytes = BytesVec::forced_import(db, "p2trbytes", version),
            p2abytes = BytesVec::forced_import(db, "p2abytes", version),
        };
        Ok(Self {
            p2pk65: AddressTypeVecs { first_index: first_p2pk65addressindex, bytes: p2pk65bytes },
            p2pk33: AddressTypeVecs { first_index: first_p2pk33addressindex, bytes: p2pk33bytes },
            p2pkh: AddressTypeVecs { first_index: first_p2pkhaddressindex, bytes: p2pkhbytes },
            p2sh: AddressTypeVecs { first_index: first_p2shaddressindex, bytes: p2shbytes },
            p2wpkh: AddressTypeVecs { first_index: first_p2wpkhaddressindex, bytes: p2wpkhbytes },
            p2wsh: AddressTypeVecs { first_index: first_p2wshaddressindex, bytes: p2wshbytes },
            p2tr: AddressTypeVecs { first_index: first_p2traddressindex, bytes: p2trbytes },
            p2a: AddressTypeVecs { first_index: first_p2aaddressindex, bytes: p2abytes },
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn truncate(
        &mut self,
        height: Height,
        p2pk65addressindex: P2PK65AddressIndex,
        p2pk33addressindex: P2PK33AddressIndex,
        p2pkhaddressindex: P2PKHAddressIndex,
        p2shaddressindex: P2SHAddressIndex,
        p2wpkhaddressindex: P2WPKHAddressIndex,
        p2wshaddressindex: P2WSHAddressIndex,
        p2traddressindex: P2TRAddressIndex,
        p2aaddressindex: P2AAddressIndex,
        stamp: Stamp,
    ) -> Result<()> {
        self.p2pk65.first_index
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.p2pk33.first_index
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.p2pkh.first_index
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.p2sh.first_index
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.p2wpkh.first_index
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.p2wsh.first_index
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.p2tr.first_index
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.p2a.first_index
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.p2pk65.bytes
            .truncate_if_needed_with_stamp(p2pk65addressindex, stamp)?;
        self.p2pk33.bytes
            .truncate_if_needed_with_stamp(p2pk33addressindex, stamp)?;
        self.p2pkh.bytes
            .truncate_if_needed_with_stamp(p2pkhaddressindex, stamp)?;
        self.p2sh.bytes
            .truncate_if_needed_with_stamp(p2shaddressindex, stamp)?;
        self.p2wpkh.bytes
            .truncate_if_needed_with_stamp(p2wpkhaddressindex, stamp)?;
        self.p2wsh.bytes
            .truncate_if_needed_with_stamp(p2wshaddressindex, stamp)?;
        self.p2tr.bytes
            .truncate_if_needed_with_stamp(p2traddressindex, stamp)?;
        self.p2a.bytes
            .truncate_if_needed_with_stamp(p2aaddressindex, stamp)?;
        Ok(())
    }

    pub fn par_iter_mut_any(&mut self) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        [
            &mut self.p2pk65.first_index as &mut dyn AnyStoredVec,
            &mut self.p2pk33.first_index,
            &mut self.p2pkh.first_index,
            &mut self.p2sh.first_index,
            &mut self.p2wpkh.first_index,
            &mut self.p2wsh.first_index,
            &mut self.p2tr.first_index,
            &mut self.p2a.first_index,
            &mut self.p2pk65.bytes,
            &mut self.p2pk33.bytes,
            &mut self.p2pkh.bytes,
            &mut self.p2sh.bytes,
            &mut self.p2wpkh.bytes,
            &mut self.p2wsh.bytes,
            &mut self.p2tr.bytes,
            &mut self.p2a.bytes,
        ]
        .into_par_iter()
    }

    /// Get address bytes by output type, using the cached VecReader for the specific address type.
    /// Returns None if the index doesn't exist yet.
    pub fn get_bytes_by_type(
        &self,
        addresstype: OutputType,
        typeindex: TypeIndex,
        readers: &AddressReaders,
    ) -> Option<AddressBytes> {
        match addresstype {
            OutputType::P2PK65 => self
                .p2pk65.bytes
                .get_pushed_or_read(typeindex.into(), &readers.p2pk65)
                .map(AddressBytes::from),
            OutputType::P2PK33 => self
                .p2pk33.bytes
                .get_pushed_or_read(typeindex.into(), &readers.p2pk33)
                .map(AddressBytes::from),
            OutputType::P2PKH => self
                .p2pkh.bytes
                .get_pushed_or_read(typeindex.into(), &readers.p2pkh)
                .map(AddressBytes::from),
            OutputType::P2SH => self
                .p2sh.bytes
                .get_pushed_or_read(typeindex.into(), &readers.p2sh)
                .map(AddressBytes::from),
            OutputType::P2WPKH => self
                .p2wpkh.bytes
                .get_pushed_or_read(typeindex.into(), &readers.p2wpkh)
                .map(AddressBytes::from),
            OutputType::P2WSH => self
                .p2wsh.bytes
                .get_pushed_or_read(typeindex.into(), &readers.p2wsh)
                .map(AddressBytes::from),
            OutputType::P2TR => self
                .p2tr.bytes
                .get_pushed_or_read(typeindex.into(), &readers.p2tr)
                .map(AddressBytes::from),
            OutputType::P2A => self
                .p2a.bytes
                .get_pushed_or_read(typeindex.into(), &readers.p2a)
                .map(AddressBytes::from),
            _ => unreachable!("get_bytes_by_type called with non-address type"),
        }
    }

    pub fn push_bytes_if_needed(&mut self, index: TypeIndex, bytes: AddressBytes) -> Result<()> {
        match bytes {
            AddressBytes::P2PK65(bytes) => self.p2pk65.bytes.checked_push(index.into(), bytes)?,
            AddressBytes::P2PK33(bytes) => self.p2pk33.bytes.checked_push(index.into(), bytes)?,
            AddressBytes::P2PKH(bytes) => self.p2pkh.bytes.checked_push(index.into(), bytes)?,
            AddressBytes::P2SH(bytes) => self.p2sh.bytes.checked_push(index.into(), bytes)?,
            AddressBytes::P2WPKH(bytes) => self.p2wpkh.bytes.checked_push(index.into(), bytes)?,
            AddressBytes::P2WSH(bytes) => self.p2wsh.bytes.checked_push(index.into(), bytes)?,
            AddressBytes::P2TR(bytes) => self.p2tr.bytes.checked_push(index.into(), bytes)?,
            AddressBytes::P2A(bytes) => self.p2a.bytes.checked_push(index.into(), bytes)?,
        };
        Ok(())
    }

    /// Iterate address hashes starting from a given height (for rollback).
    /// Returns an iterator of AddressHash values for all addresses of the given type
    /// that were added at or after the given height.
    pub fn iter_hashes_from(
        &self,
        address_type: OutputType,
        height: Height,
    ) -> Result<Box<dyn Iterator<Item = AddressHash> + '_>> {
        macro_rules! make_iter {
            ($addr:expr) => {{
                match $addr.first_index.collect_one(height) {
                    Some(mut index) => {
                        let reader = $addr.bytes.reader();
                        Ok(Box::new(std::iter::from_fn(move || {
                            reader.try_get(index.to_usize()).map(|typedbytes| {
                                let bytes = AddressBytes::from(typedbytes);
                                index.increment();
                                AddressHash::from(&bytes)
                            })
                        }))
                            as Box<dyn Iterator<Item = AddressHash> + '_>)
                    }
                    None => {
                        Ok(Box::new(std::iter::empty())
                            as Box<dyn Iterator<Item = AddressHash> + '_>)
                    }
                }
            }};
        }

        match address_type {
            OutputType::P2PK65 => make_iter!(self.p2pk65),
            OutputType::P2PK33 => make_iter!(self.p2pk33),
            OutputType::P2PKH => make_iter!(self.p2pkh),
            OutputType::P2SH => make_iter!(self.p2sh),
            OutputType::P2WPKH => make_iter!(self.p2wpkh),
            OutputType::P2WSH => make_iter!(self.p2wsh),
            OutputType::P2TR => make_iter!(self.p2tr),
            OutputType::P2A => make_iter!(self.p2a),
            _ => Ok(Box::new(std::iter::empty())),
        }
    }
}

macro_rules! impl_address_readers {
    ($mode:ty) => {
        impl AddressesVecs<$mode> {
            pub fn address_readers(&self) -> AddressReaders {
                AddressReaders {
                    p2pk65: self.p2pk65.bytes.reader(),
                    p2pk33: self.p2pk33.bytes.reader(),
                    p2pkh: self.p2pkh.bytes.reader(),
                    p2sh: self.p2sh.bytes.reader(),
                    p2wpkh: self.p2wpkh.bytes.reader(),
                    p2wsh: self.p2wsh.bytes.reader(),
                    p2tr: self.p2tr.bytes.reader(),
                    p2a: self.p2a.bytes.reader(),
                }
            }
        }
    };
}

impl_address_readers!(Rw);
impl_address_readers!(Ro);
