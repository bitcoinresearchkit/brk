//! Address data lookup and source tracking.
//!
//! Handles looking up existing address data from storage and tracking
//! whether addresses are new, from storage, or previously empty.

use brk_types::{EmptyAddressData, LoadedAddressData, OutputType, TypeIndex};

use super::super::address::AddressTypeToTypeIndexMap;
pub use super::WithAddressDataSource;

/// Loaded address data with source tracking for flush operations.
pub type LoadedAddressDataWithSource = WithAddressDataSource<LoadedAddressData>;

/// Empty address data with source tracking for flush operations.
pub type EmptyAddressDataWithSource = WithAddressDataSource<EmptyAddressData>;

/// Context for looking up and storing address data during block processing.
///
/// Uses the same pattern as the original stateful module:
/// - `loaded`: addresses with non-zero balance (wrapped with source info)
/// - `empty`: addresses that became empty this block (wrapped with source info)
pub struct AddressLookup<'a, F> {
    /// Function to get existing address data from storage
    pub get_address_data: F,
    /// Loaded addresses touched in current block
    pub loaded: &'a mut AddressTypeToTypeIndexMap<LoadedAddressDataWithSource>,
    /// Empty addresses touched in current block
    pub empty: &'a mut AddressTypeToTypeIndexMap<EmptyAddressDataWithSource>,
}

impl<'a, F> AddressLookup<'a, F>
where
    F: FnMut(OutputType, TypeIndex) -> Option<LoadedAddressDataWithSource>,
{
    /// Get or create address data for a receive operation.
    ///
    /// Returns (address_data, is_new, from_empty)
    pub fn get_or_create_for_receive(
        &mut self,
        output_type: OutputType,
        type_index: TypeIndex,
    ) -> (&mut LoadedAddressDataWithSource, bool, bool) {
        let mut is_new = false;
        let mut from_empty = false;

        let addr_data = self
            .loaded
            .get_mut(output_type)
            .unwrap()
            .entry(type_index)
            .or_insert_with(|| {
                // Check if it was in empty set
                if let Some(empty_data) = self.empty.get_mut(output_type).unwrap().remove(&type_index) {
                    from_empty = true;
                    return empty_data.into();
                }

                // Look up from storage or create new
                match (self.get_address_data)(output_type, type_index) {
                    Some(data) => {
                        is_new = data.is_new();
                        from_empty = data.is_from_emptyaddressdata();
                        data
                    }
                    None => {
                        is_new = true;
                        WithAddressDataSource::New(LoadedAddressData::default())
                    }
                }
            });

        (addr_data, is_new, from_empty)
    }

    /// Get address data for a send operation (must exist).
    pub fn get_for_send(
        &mut self,
        output_type: OutputType,
        type_index: TypeIndex,
    ) -> &mut LoadedAddressDataWithSource {
        self.loaded
            .get_mut(output_type)
            .unwrap()
            .entry(type_index)
            .or_insert_with(|| {
                (self.get_address_data)(output_type, type_index)
                    .expect("Address must exist for send")
            })
    }

    /// Move address from loaded to empty set.
    pub fn move_to_empty(&mut self, output_type: OutputType, type_index: TypeIndex) {
        let data = self
            .loaded
            .get_mut(output_type)
            .unwrap()
            .remove(&type_index)
            .unwrap();

        self.empty
            .get_mut(output_type)
            .unwrap()
            .insert(type_index, data.into());
    }
}
