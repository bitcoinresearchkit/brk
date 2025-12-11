//! Address handling with macro-generated code for 8 address types.
//!
//! This module provides:
//! - `AnyAddressIndexesVecs` for storing address indexes by type
//! - `AddressesDataVecs` for storing address data (loaded/empty)
//! - `AddressTypeToTypeIndexMap` for per-type hashmaps
//! - `AddressTypeToVec` for per-type vectors
//! - `HeightToAddressTypeToVec` for height-keyed per-type vectors
//! - `AddressTypeToAddressCount` for runtime address counts
//! - `AddressTypeToHeightToAddressCount` for height-indexed address counts
//! - `AddressTypeToIndexesToAddressCount` for computed address counts

mod address_count;
mod any_address_indexes;
mod data;
mod height_type_vec;
mod type_index_map;
mod type_vec;

pub use address_count::{
    AddressTypeToAddressCount, AddressTypeToHeightToAddressCount,
    AddressTypeToIndexesToAddressCount,
};
pub use any_address_indexes::AnyAddressIndexesVecs;
pub use data::AddressesDataVecs;
pub use height_type_vec::HeightToAddressTypeToVec;
pub use type_index_map::AddressTypeToTypeIndexMap;
pub use type_vec::AddressTypeToVec;
