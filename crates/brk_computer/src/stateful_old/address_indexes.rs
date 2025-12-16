use brk_error::{Error, Result};
use brk_traversable::Traversable;
use brk_types::{
    AnyAddressIndex, EmptyAddressData, EmptyAddressIndex, Height, LoadedAddressData,
    LoadedAddressIndex, OutputType, P2AAddressIndex, P2PK33AddressIndex, P2PK65AddressIndex,
    P2PKHAddressIndex, P2SHAddressIndex, P2TRAddressIndex, P2WPKHAddressIndex, P2WSHAddressIndex,
    TypeIndex,
};
use vecdb::{AnyStoredVec, BytesVec, GenericStoredVec, Reader, Stamp};

#[derive(Clone, Traversable)]
pub struct AnyAddressIndexesVecs {
    pub p2pk33: BytesVec<P2PK33AddressIndex, AnyAddressIndex>,
    pub p2pk65: BytesVec<P2PK65AddressIndex, AnyAddressIndex>,
    pub p2pkh: BytesVec<P2PKHAddressIndex, AnyAddressIndex>,
    pub p2sh: BytesVec<P2SHAddressIndex, AnyAddressIndex>,
    pub p2tr: BytesVec<P2TRAddressIndex, AnyAddressIndex>,
    pub p2wpkh: BytesVec<P2WPKHAddressIndex, AnyAddressIndex>,
    pub p2wsh: BytesVec<P2WSHAddressIndex, AnyAddressIndex>,
    pub p2a: BytesVec<P2AAddressIndex, AnyAddressIndex>,
}

impl AnyAddressIndexesVecs {
    pub fn min_stamped_height(&self) -> Height {
        Height::from(self.p2pk33.stamp())
            .incremented()
            .min(Height::from(self.p2pk65.stamp()).incremented())
            .min(Height::from(self.p2pkh.stamp()).incremented())
            .min(Height::from(self.p2sh.stamp()).incremented())
            .min(Height::from(self.p2tr.stamp()).incremented())
            .min(Height::from(self.p2wpkh.stamp()).incremented())
            .min(Height::from(self.p2wsh.stamp()).incremented())
            .min(Height::from(self.p2a.stamp()).incremented())
    }

    pub fn rollback_before(&mut self, stamp: Stamp) -> Result<[Stamp; 8]> {
        Ok([
            self.p2pk33.rollback_before(stamp)?,
            self.p2pk65.rollback_before(stamp)?,
            self.p2pkh.rollback_before(stamp)?,
            self.p2sh.rollback_before(stamp)?,
            self.p2tr.rollback_before(stamp)?,
            self.p2wpkh.rollback_before(stamp)?,
            self.p2wsh.rollback_before(stamp)?,
            self.p2a.rollback_before(stamp)?,
        ])
    }

    pub fn reset(&mut self) -> Result<()> {
        self.p2pk33.reset()?;
        self.p2pk65.reset()?;
        self.p2pkh.reset()?;
        self.p2sh.reset()?;
        self.p2tr.reset()?;
        self.p2wpkh.reset()?;
        self.p2wsh.reset()?;
        self.p2a.reset()?;
        Ok(())
    }

    pub fn get_anyaddressindex(
        &self,
        address_type: OutputType,
        typeindex: TypeIndex,
        reader: &Reader,
    ) -> AnyAddressIndex {
        match address_type {
            OutputType::P2PK33 => self
                .p2pk33
                .get_pushed_or_read_at_unwrap(typeindex.into(), reader),
            OutputType::P2PK65 => self
                .p2pk65
                .get_pushed_or_read_at_unwrap(typeindex.into(), reader),
            OutputType::P2PKH => self
                .p2pkh
                .get_pushed_or_read_at_unwrap(typeindex.into(), reader),
            OutputType::P2SH => self
                .p2sh
                .get_pushed_or_read_at_unwrap(typeindex.into(), reader),
            OutputType::P2TR => self
                .p2tr
                .get_pushed_or_read_at_unwrap(typeindex.into(), reader),
            OutputType::P2WPKH => self
                .p2wpkh
                .get_pushed_or_read_at_unwrap(typeindex.into(), reader),
            OutputType::P2WSH => self
                .p2wsh
                .get_pushed_or_read_at_unwrap(typeindex.into(), reader),
            OutputType::P2A => self
                .p2a
                .get_pushed_or_read_at_unwrap(typeindex.into(), reader),
            _ => unreachable!(),
        }
    }

    pub fn get_anyaddressindex_once(
        &self,
        address_type: OutputType,
        typeindex: TypeIndex,
    ) -> Result<AnyAddressIndex> {
        match address_type {
            OutputType::P2PK33 => self
                .p2pk33
                .read_at_once(typeindex.into())
                .map_err(|e| e.into()),
            OutputType::P2PK65 => self
                .p2pk65
                .read_at_once(typeindex.into())
                .map_err(|e| e.into()),
            OutputType::P2PKH => self
                .p2pkh
                .read_at_once(typeindex.into())
                .map_err(|e| e.into()),
            OutputType::P2SH => self
                .p2sh
                .read_at_once(typeindex.into())
                .map_err(|e| e.into()),
            OutputType::P2TR => self
                .p2tr
                .read_at_once(typeindex.into())
                .map_err(|e| e.into()),
            OutputType::P2WPKH => self
                .p2wpkh
                .read_at_once(typeindex.into())
                .map_err(|e| e.into()),
            OutputType::P2WSH => self
                .p2wsh
                .read_at_once(typeindex.into())
                .map_err(|e| e.into()),
            OutputType::P2A => self
                .p2a
                .read_at_once(typeindex.into())
                .map_err(|e| e.into()),
            _ => Err(Error::UnsupportedType(address_type.to_string())),
        }
    }

    pub fn update_or_push(
        &mut self,
        address_type: OutputType,
        typeindex: TypeIndex,
        anyaddressindex: AnyAddressIndex,
    ) -> Result<()> {
        (match address_type {
            OutputType::P2PK33 => self
                .p2pk33
                .update_or_push(typeindex.into(), anyaddressindex),
            OutputType::P2PK65 => self
                .p2pk65
                .update_or_push(typeindex.into(), anyaddressindex),
            OutputType::P2PKH => self.p2pkh.update_or_push(typeindex.into(), anyaddressindex),
            OutputType::P2SH => self.p2sh.update_or_push(typeindex.into(), anyaddressindex),
            OutputType::P2TR => self.p2tr.update_or_push(typeindex.into(), anyaddressindex),
            OutputType::P2WPKH => self
                .p2wpkh
                .update_or_push(typeindex.into(), anyaddressindex),
            OutputType::P2WSH => self.p2wsh.update_or_push(typeindex.into(), anyaddressindex),
            OutputType::P2A => self.p2a.update_or_push(typeindex.into(), anyaddressindex),
            _ => unreachable!(),
        })?;
        Ok(())
    }

    pub fn stamped_flush_maybe_with_changes(
        &mut self,
        stamp: Stamp,
        with_changes: bool,
    ) -> Result<()> {
        self.p2pk33
            .stamped_flush_maybe_with_changes(stamp, with_changes)?;
        self.p2pk65
            .stamped_flush_maybe_with_changes(stamp, with_changes)?;
        self.p2pkh
            .stamped_flush_maybe_with_changes(stamp, with_changes)?;
        self.p2sh
            .stamped_flush_maybe_with_changes(stamp, with_changes)?;
        self.p2tr
            .stamped_flush_maybe_with_changes(stamp, with_changes)?;
        self.p2wpkh
            .stamped_flush_maybe_with_changes(stamp, with_changes)?;
        self.p2wsh
            .stamped_flush_maybe_with_changes(stamp, with_changes)?;
        self.p2a
            .stamped_flush_maybe_with_changes(stamp, with_changes)?;
        Ok(())
    }
}

#[derive(Clone, Traversable)]
pub struct AddressesDataVecs {
    pub loaded: BytesVec<LoadedAddressIndex, LoadedAddressData>,
    pub empty: BytesVec<EmptyAddressIndex, EmptyAddressData>,
}

impl AddressesDataVecs {
    pub fn min_stamped_height(&self) -> Height {
        Height::from(self.loaded.stamp())
            .incremented()
            .min(Height::from(self.empty.stamp()).incremented())
    }

    pub fn rollback_before(&mut self, stamp: Stamp) -> Result<[Stamp; 2]> {
        Ok([
            self.loaded.rollback_before(stamp)?,
            self.empty.rollback_before(stamp)?,
        ])
    }

    pub fn reset(&mut self) -> Result<()> {
        self.loaded.reset()?;
        self.empty.reset()?;
        Ok(())
    }

    pub fn stamped_flush_maybe_with_changes(
        &mut self,
        stamp: Stamp,
        with_changes: bool,
    ) -> Result<()> {
        self.loaded
            .stamped_flush_maybe_with_changes(stamp, with_changes)?;
        self.empty
            .stamped_flush_maybe_with_changes(stamp, with_changes)?;
        Ok(())
    }
}
