use crate::{EmptyAddressIndex, LoadedAddressIndex};

/// Source of address data update (where the data came from).
#[derive(Clone)]
pub enum AddressDataSource<T> {
    /// Brand new address, not in any storage yet.
    New(T),
    /// From empty address storage.
    FromEmpty((EmptyAddressIndex, T)),
    /// From loaded address storage.
    FromLoaded((LoadedAddressIndex, T)),
}
