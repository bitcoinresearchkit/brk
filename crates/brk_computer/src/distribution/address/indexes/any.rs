use std::thread;

use brk_cohort::ByAddressType;
use brk_error::{Error, Result};
use brk_traversable::Traversable;
use brk_types::{
    AnyAddressIndex, Height, OutputType, P2AAddressIndex, P2PK33AddressIndex, P2PK65AddressIndex,
    P2PKHAddressIndex, P2SHAddressIndex, P2TRAddressIndex, P2WPKHAddressIndex, P2WSHAddressIndex,
    TypeIndex, Version,
};
use rayon::prelude::*;
use rustc_hash::FxHashMap;
use vecdb::{
    AnyStoredVec, AnyVec, BytesVec, Database, GenericStoredVec, ImportOptions, ImportableVec,
    Reader, Stamp,
};

use super::super::AddressTypeToTypeIndexMap;

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
            /// Uses get_any_or_read_at_unwrap to check updated layer (needed after rollback).
            pub fn get(&self, address_type: OutputType, typeindex: TypeIndex, reader: &Reader) -> AnyAddressIndex {
                match address_type {
                    $(OutputType::$variant => self.$field.get_any_or_read_at_unwrap(typeindex.into(), reader),)*
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

            /// Get length for a given address type.
            pub fn len_of(&self, address_type: OutputType) -> usize {
                match address_type {
                    $(OutputType::$variant => self.$field.len(),)*
                    _ => unreachable!("Invalid address type: {:?}", address_type),
                }
            }

            /// Update existing entry (must be within bounds).
            pub fn update(&mut self, address_type: OutputType, typeindex: TypeIndex, index: AnyAddressIndex) -> Result<()> {
                match address_type {
                    $(OutputType::$variant => self.$field.update(typeindex.into(), index)?,)*
                    _ => unreachable!("Invalid address type: {:?}", address_type),
                }
                Ok(())
            }

            /// Push new entry (must be at exactly len position).
            pub fn push(&mut self, address_type: OutputType, index: AnyAddressIndex) {
                match address_type {
                    $(OutputType::$variant => self.$field.push(index),)*
                    _ => unreachable!("Invalid address type: {:?}", address_type),
                }
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

impl AnyAddressIndexesVecs {
    /// Process index updates in parallel by address type.
    /// Accepts two maps (e.g. from empty and loaded processing) and merges per-thread.
    /// Updates existing entries and pushes new ones (sorted).
    /// Returns (update_count, push_count).
    pub fn par_batch_update(
        &mut self,
        updates1: AddressTypeToTypeIndexMap<AnyAddressIndex>,
        updates2: AddressTypeToTypeIndexMap<AnyAddressIndex>,
    ) -> Result<(usize, usize)> {
        let ByAddressType {
            p2a: u1_p2a,
            p2pk33: u1_p2pk33,
            p2pk65: u1_p2pk65,
            p2pkh: u1_p2pkh,
            p2sh: u1_p2sh,
            p2tr: u1_p2tr,
            p2wpkh: u1_p2wpkh,
            p2wsh: u1_p2wsh,
        } = updates1.into_inner();

        let ByAddressType {
            p2a: u2_p2a,
            p2pk33: u2_p2pk33,
            p2pk65: u2_p2pk65,
            p2pkh: u2_p2pkh,
            p2sh: u2_p2sh,
            p2tr: u2_p2tr,
            p2wpkh: u2_p2wpkh,
            p2wsh: u2_p2wsh,
        } = updates2.into_inner();

        let Self {
            p2a,
            p2pk33,
            p2pk65,
            p2pkh,
            p2sh,
            p2tr,
            p2wpkh,
            p2wsh,
        } = self;

        thread::scope(|s| {
            let h_p2a = s.spawn(|| process_single_type_merged(p2a, u1_p2a, u2_p2a));
            let h_p2pk33 = s.spawn(|| process_single_type_merged(p2pk33, u1_p2pk33, u2_p2pk33));
            let h_p2pk65 = s.spawn(|| process_single_type_merged(p2pk65, u1_p2pk65, u2_p2pk65));
            let h_p2pkh = s.spawn(|| process_single_type_merged(p2pkh, u1_p2pkh, u2_p2pkh));
            let h_p2sh = s.spawn(|| process_single_type_merged(p2sh, u1_p2sh, u2_p2sh));
            let h_p2tr = s.spawn(|| process_single_type_merged(p2tr, u1_p2tr, u2_p2tr));
            let h_p2wpkh = s.spawn(|| process_single_type_merged(p2wpkh, u1_p2wpkh, u2_p2wpkh));
            let h_p2wsh = s.spawn(|| process_single_type_merged(p2wsh, u1_p2wsh, u2_p2wsh));

            let mut total_updates = 0usize;
            let mut total_pushes = 0usize;

            for h in [
                h_p2a, h_p2pk33, h_p2pk65, h_p2pkh, h_p2sh, h_p2tr, h_p2wpkh, h_p2wsh,
            ] {
                let (updates, pushes) = h.join().unwrap()?;
                total_updates += updates;
                total_pushes += pushes;
            }

            Ok((total_updates, total_pushes))
        })
    }
}

/// Process updates for a single address type's BytesVec, merging two maps.
fn process_single_type_merged<I: vecdb::VecIndex>(
    vec: &mut BytesVec<I, AnyAddressIndex>,
    map1: FxHashMap<TypeIndex, AnyAddressIndex>,
    map2: FxHashMap<TypeIndex, AnyAddressIndex>,
) -> Result<(usize, usize)> {
    let current_len = vec.len();
    let mut pushes = Vec::with_capacity(map1.len() + map2.len());
    let mut update_count = 0usize;

    for (typeindex, any_index) in map1.into_iter().chain(map2) {
        if usize::from(typeindex) < current_len {
            vec.update(I::from(usize::from(typeindex)), any_index)?;
            update_count += 1;
        } else {
            pushes.push((typeindex, any_index));
        }
    }

    let push_count = pushes.len();
    if !pushes.is_empty() {
        pushes.sort_unstable_by_key(|(typeindex, _)| *typeindex);
        for (_, any_index) in pushes {
            vec.push(any_index);
        }
    }

    Ok((update_count, push_count))
}
