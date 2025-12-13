use std::str::FromStr;

use brk_error::{Error, Result};
use brk_types::{Address, AddressBytes, AddressHash, OutputType, TypeIndex};

use crate::Query;

/// Resolve an address string to its output type and type_index
pub fn resolve_address(address: &Address, query: &Query) -> Result<(OutputType, TypeIndex)> {
    let stores = &query.indexer().stores;

    let bytes = AddressBytes::from_str(&address.address)?;
    let outputtype = OutputType::from(&bytes);
    let hash = AddressHash::from(&bytes);

    let Ok(Some(type_index)) = stores
        .addresstype_to_addresshash_to_addressindex
        .get(outputtype)
        .unwrap()
        .get(&hash)
        .map(|opt| opt.map(|cow| cow.into_owned()))
    else {
        return Err(Error::UnknownAddress);
    };

    Ok((outputtype, type_index))
}
