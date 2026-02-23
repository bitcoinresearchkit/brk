use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{
    AddressBytes, AddressHash, Height, OutputType, P2AAddressIndex, P2ABytes, P2PK33AddressIndex,
    P2PK33Bytes, P2PK65AddressIndex, P2PK65Bytes, P2PKHAddressIndex, P2PKHBytes, P2SHAddressIndex,
    P2SHBytes, P2TRAddressIndex, P2TRBytes, P2WPKHAddressIndex, P2WPKHBytes, P2WSHAddressIndex,
    P2WSHBytes, TypeIndex, Version,
};
use rayon::prelude::*;
use vecdb::{
    AnyStoredVec, BytesVec, Database, WritableVec, ImportableVec, PcoVec, ReadableVec,
    Rw, Stamp, StorageMode, VecIndex,
};

use crate::readers::AddressReaders;
use crate::parallel_import;

#[derive(Traversable)]
pub struct AddressesVecs<M: StorageMode = Rw> {
    // Height to first address index (per address type)
    pub first_p2pk65addressindex: M::Stored<PcoVec<Height, P2PK65AddressIndex>>,
    pub first_p2pk33addressindex: M::Stored<PcoVec<Height, P2PK33AddressIndex>>,
    pub first_p2pkhaddressindex: M::Stored<PcoVec<Height, P2PKHAddressIndex>>,
    pub first_p2shaddressindex: M::Stored<PcoVec<Height, P2SHAddressIndex>>,
    pub first_p2wpkhaddressindex: M::Stored<PcoVec<Height, P2WPKHAddressIndex>>,
    pub first_p2wshaddressindex: M::Stored<PcoVec<Height, P2WSHAddressIndex>>,
    pub first_p2traddressindex: M::Stored<PcoVec<Height, P2TRAddressIndex>>,
    pub first_p2aaddressindex: M::Stored<PcoVec<Height, P2AAddressIndex>>,
    // Address index to bytes (per address type)
    pub p2pk65bytes: M::Stored<BytesVec<P2PK65AddressIndex, P2PK65Bytes>>,
    pub p2pk33bytes: M::Stored<BytesVec<P2PK33AddressIndex, P2PK33Bytes>>,
    pub p2pkhbytes: M::Stored<BytesVec<P2PKHAddressIndex, P2PKHBytes>>,
    pub p2shbytes: M::Stored<BytesVec<P2SHAddressIndex, P2SHBytes>>,
    pub p2wpkhbytes: M::Stored<BytesVec<P2WPKHAddressIndex, P2WPKHBytes>>,
    pub p2wshbytes: M::Stored<BytesVec<P2WSHAddressIndex, P2WSHBytes>>,
    pub p2trbytes: M::Stored<BytesVec<P2TRAddressIndex, P2TRBytes>>,
    pub p2abytes: M::Stored<BytesVec<P2AAddressIndex, P2ABytes>>,
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
        self.first_p2pk65addressindex
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.first_p2pk33addressindex
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.first_p2pkhaddressindex
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.first_p2shaddressindex
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.first_p2wpkhaddressindex
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.first_p2wshaddressindex
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.first_p2traddressindex
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.first_p2aaddressindex
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.p2pk65bytes
            .truncate_if_needed_with_stamp(p2pk65addressindex, stamp)?;
        self.p2pk33bytes
            .truncate_if_needed_with_stamp(p2pk33addressindex, stamp)?;
        self.p2pkhbytes
            .truncate_if_needed_with_stamp(p2pkhaddressindex, stamp)?;
        self.p2shbytes
            .truncate_if_needed_with_stamp(p2shaddressindex, stamp)?;
        self.p2wpkhbytes
            .truncate_if_needed_with_stamp(p2wpkhaddressindex, stamp)?;
        self.p2wshbytes
            .truncate_if_needed_with_stamp(p2wshaddressindex, stamp)?;
        self.p2trbytes
            .truncate_if_needed_with_stamp(p2traddressindex, stamp)?;
        self.p2abytes
            .truncate_if_needed_with_stamp(p2aaddressindex, stamp)?;
        Ok(())
    }

    pub fn par_iter_mut_any(&mut self) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        [
            &mut self.first_p2pk65addressindex as &mut dyn AnyStoredVec,
            &mut self.first_p2pk33addressindex,
            &mut self.first_p2pkhaddressindex,
            &mut self.first_p2shaddressindex,
            &mut self.first_p2wpkhaddressindex,
            &mut self.first_p2wshaddressindex,
            &mut self.first_p2traddressindex,
            &mut self.first_p2aaddressindex,
            &mut self.p2pk65bytes,
            &mut self.p2pk33bytes,
            &mut self.p2pkhbytes,
            &mut self.p2shbytes,
            &mut self.p2wpkhbytes,
            &mut self.p2wshbytes,
            &mut self.p2trbytes,
            &mut self.p2abytes,
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
                .p2pk65bytes
                .get_pushed_or_read(typeindex.into(), &readers.p2pk65)
                .map(AddressBytes::from),
            OutputType::P2PK33 => self
                .p2pk33bytes
                .get_pushed_or_read(typeindex.into(), &readers.p2pk33)
                .map(AddressBytes::from),
            OutputType::P2PKH => self
                .p2pkhbytes
                .get_pushed_or_read(typeindex.into(), &readers.p2pkh)
                .map(AddressBytes::from),
            OutputType::P2SH => self
                .p2shbytes
                .get_pushed_or_read(typeindex.into(), &readers.p2sh)
                .map(AddressBytes::from),
            OutputType::P2WPKH => self
                .p2wpkhbytes
                .get_pushed_or_read(typeindex.into(), &readers.p2wpkh)
                .map(AddressBytes::from),
            OutputType::P2WSH => self
                .p2wshbytes
                .get_pushed_or_read(typeindex.into(), &readers.p2wsh)
                .map(AddressBytes::from),
            OutputType::P2TR => self
                .p2trbytes
                .get_pushed_or_read(typeindex.into(), &readers.p2tr)
                .map(AddressBytes::from),
            OutputType::P2A => self
                .p2abytes
                .get_pushed_or_read(typeindex.into(), &readers.p2a)
                .map(AddressBytes::from),
            _ => unreachable!("get_bytes_by_type called with non-address type"),
        }
    }

    pub fn push_bytes_if_needed(&mut self, index: TypeIndex, bytes: AddressBytes) -> Result<()> {
        match bytes {
            AddressBytes::P2PK65(bytes) => self
                .p2pk65bytes
                .checked_push(index.into(), bytes)?,
            AddressBytes::P2PK33(bytes) => self
                .p2pk33bytes
                .checked_push(index.into(), bytes)?,
            AddressBytes::P2PKH(bytes) => self
                .p2pkhbytes
                .checked_push(index.into(), bytes)?,
            AddressBytes::P2SH(bytes) => self
                .p2shbytes
                .checked_push(index.into(), bytes)?,
            AddressBytes::P2WPKH(bytes) => self
                .p2wpkhbytes
                .checked_push(index.into(), bytes)?,
            AddressBytes::P2WSH(bytes) => self
                .p2wshbytes
                .checked_push(index.into(), bytes)?,
            AddressBytes::P2TR(bytes) => self
                .p2trbytes
                .checked_push(index.into(), bytes)?,
            AddressBytes::P2A(bytes) => self
                .p2abytes
                .checked_push(index.into(), bytes)?,
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
            ($height_vec:expr, $bytes_vec:expr) => {{
                match $height_vec.collect_one(height) {
                    Some(mut index) => {
                        let reader = $bytes_vec.reader();
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
            OutputType::P2PK65 => make_iter!(
                self.first_p2pk65addressindex,
                self.p2pk65bytes
            ),
            OutputType::P2PK33 => make_iter!(
                self.first_p2pk33addressindex,
                self.p2pk33bytes
            ),
            OutputType::P2PKH => make_iter!(
                self.first_p2pkhaddressindex,
                self.p2pkhbytes
            ),
            OutputType::P2SH => make_iter!(
                self.first_p2shaddressindex,
                self.p2shbytes
            ),
            OutputType::P2WPKH => make_iter!(
                self.first_p2wpkhaddressindex,
                self.p2wpkhbytes
            ),
            OutputType::P2WSH => make_iter!(
                self.first_p2wshaddressindex,
                self.p2wshbytes
            ),
            OutputType::P2TR => make_iter!(
                self.first_p2traddressindex,
                self.p2trbytes
            ),
            OutputType::P2A => make_iter!(
                self.first_p2aaddressindex,
                self.p2abytes
            ),
            _ => Ok(Box::new(std::iter::empty())),
        }
    }
}
