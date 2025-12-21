//! Storage for address indexes by type.

use brk_error::{Error, Result};
use brk_traversable::Traversable;
use brk_types::{
    AnyAddressIndex, Height, OutputType, P2AAddressIndex, P2PK33AddressIndex, P2PK65AddressIndex,
    P2PKHAddressIndex, P2SHAddressIndex, P2TRAddressIndex, P2WPKHAddressIndex, P2WSHAddressIndex,
    TypeIndex, Version,
};
use rayon::prelude::*;
use vecdb::{
    AnyStoredVec, BytesVec, Database, GenericStoredVec, ImportOptions, ImportableVec, Reader, Stamp,
};

const SAVED_STAMPED_CHANGES: u16 = 10;

/// Macro to define AnyAddressIndexesVecs and its methods.
macro_rules! define_any_address_indexes_vecs {
    ($(($field:ident, $variant:ident, $index:ty)),* $(,)?) => {
        #[derive(Clone, Traversable)]
        pub struct AnyAddressIndexesVecs {
            $(pub $field: BytesVec<$index, AnyAddressIndex>,)*
        }

        impl AnyAddressIndexesVecs {
            /// Import from database.
            pub fn forced_import(db: &Database, version: Version) -> Result<Self> {
                Ok(Self {
                    $($field: BytesVec::forced_import_with(
                        ImportOptions::new(db, "anyaddressindex", version)
                            .with_saved_stamped_changes(SAVED_STAMPED_CHANGES),
                    )?,)*
                })
            }

            /// Get minimum stamped height across all address types.
            pub fn min_stamped_height(&self) -> Height {
                [$(Height::from(self.$field.stamp()).incremented()),*]
                    .into_iter()
                    .min()
                    .unwrap_or_default()
            }

            /// Rollback all address types to before the given stamp.
            pub fn rollback_before(&mut self, stamp: Stamp) -> Result<Vec<Stamp>> {
                Ok(vec![$(self.$field.rollback_before(stamp)?),*])
            }

            /// Reset all address types.
            pub fn reset(&mut self) -> Result<()> {
                $(self.$field.reset()?;)*
                Ok(())
            }

            /// Get address index for a given type and typeindex.
            pub fn get(&self, address_type: OutputType, typeindex: TypeIndex, reader: &Reader) -> AnyAddressIndex {
                match address_type {
                    $(OutputType::$variant => self.$field.get_pushed_or_read_at_unwrap(typeindex.into(), reader),)*
                    _ => unreachable!("Invalid address type: {:?}", address_type),
                }
            }

            /// Get address index with single read (no caching).
            pub fn get_once(&self, address_type: OutputType, typeindex: TypeIndex) -> Result<AnyAddressIndex> {
                match address_type {
                    $(OutputType::$variant => self.$field.read_at_once(typeindex.into()).map_err(Into::into),)*
                    _ => Err(Error::UnsupportedType(address_type.to_string())),
                }
            }

            /// Update or push address index for a given type.
            pub fn update_or_push(&mut self, address_type: OutputType, typeindex: TypeIndex, index: AnyAddressIndex) -> Result<()> {
                match address_type {
                    $(OutputType::$variant => self.$field.update_or_push(typeindex.into(), index)?,)*
                    _ => unreachable!("Invalid address type: {:?}", address_type),
                }
                Ok(())
            }

            /// Write all address types with stamp.
            pub fn write(&mut self, stamp: Stamp, with_changes: bool) -> Result<()> {
                $(self.$field.stamped_write_maybe_with_changes(stamp, with_changes)?;)*
                Ok(())
            }

            /// Returns a parallel iterator over all vecs for parallel writing.
            pub fn par_iter_mut(&mut self) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
                vec![$(&mut self.$field as &mut dyn AnyStoredVec),*].into_par_iter()
            }
        }
    };
}

// Generate the struct and methods
define_any_address_indexes_vecs!(
    (p2a, P2A, P2AAddressIndex),
    (p2pk33, P2PK33, P2PK33AddressIndex),
    (p2pk65, P2PK65, P2PK65AddressIndex),
    (p2pkh, P2PKH, P2PKHAddressIndex),
    (p2sh, P2SH, P2SHAddressIndex),
    (p2tr, P2TR, P2TRAddressIndex),
    (p2wpkh, P2WPKH, P2WPKHAddressIndex),
    (p2wsh, P2WSH, P2WSHAddressIndex),
);
