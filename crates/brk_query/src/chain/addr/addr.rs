use std::str::FromStr;

use bitcoin::{Network, PublicKey, ScriptBuf};
use brk_error::{Error, Result};
use brk_types::{
    Address, AddressBytes, AddressChainStats, AddressHash, AddressStats, AnyAddressDataIndexEnum,
    OutputType,
};
use vecdb::TypedVecIterator;

use crate::Query;

pub fn get_address(Address { address }: Address, query: &Query) -> Result<AddressStats> {
    let indexer = query.indexer();
    let computer = query.computer();
    let stores = &indexer.stores;

    let script = if let Ok(address) = bitcoin::Address::from_str(&address) {
        if !address.is_valid_for_network(Network::Bitcoin) {
            return Err(Error::InvalidNetwork);
        }
        let address = address.assume_checked();
        address.script_pubkey()
    } else if let Ok(pubkey) = PublicKey::from_str(&address) {
        ScriptBuf::new_p2pk(&pubkey)
    } else {
        return Err(Error::InvalidAddress);
    };

    let outputtype = OutputType::from(&script);
    let Ok(bytes) = AddressBytes::try_from((&script, outputtype)) else {
        return Err(Error::Str("Failed to convert the address to bytes"));
    };
    let addresstype = outputtype;
    let hash = AddressHash::from(&bytes);

    let Ok(Some(type_index)) = stores
        .addresstype_to_addresshash_to_addressindex
        .get(addresstype)
        .unwrap()
        .get(&hash)
        .map(|opt| opt.map(|cow| cow.into_owned()))
    else {
        return Err(Error::UnknownAddress);
    };

    let any_address_index = computer
        .stateful
        .any_address_indexes
        .get_anyaddressindex_once(outputtype, type_index)?;

    let address_data = match any_address_index.to_enum() {
        AnyAddressDataIndexEnum::Loaded(index) => computer
            .stateful
            .addresses_data
            .loaded
            .iter()?
            .get_unwrap(index),
        AnyAddressDataIndexEnum::Empty(index) => computer
            .stateful
            .addresses_data
            .empty
            .iter()?
            .get_unwrap(index)
            .into(),
    };

    Ok(AddressStats {
        address: address.into(),
        chain_stats: AddressChainStats {
            type_index,
            funded_txo_count: address_data.funded_txo_count,
            funded_txo_sum: address_data.received,
            spent_txo_count: address_data.spent_txo_count,
            spent_txo_sum: address_data.sent,
            tx_count: address_data.tx_count,
        },
        mempool_stats: query.mempool().map(|mempool| {
            mempool
                .get_addresses()
                .get(&bytes)
                .map(|(stats, _)| stats)
                .cloned()
                .unwrap_or_default()
        }),
    })
}
