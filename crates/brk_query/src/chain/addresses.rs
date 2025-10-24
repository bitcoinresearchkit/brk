use std::str::FromStr;

use bitcoin::{Network, PublicKey, ScriptBuf};
use brk_error::{Error, Result};
use brk_types::{
    Address, AddressBytes, AddressBytesHash, AddressChainStats, AddressMempoolStats, AddressStats,
    AnyAddressDataIndexEnum, OutputType,
};
use vecdb::{AnyIterableVec, VecIterator};

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

    let type_ = OutputType::from(&script);
    let Ok(bytes) = AddressBytes::try_from((&script, type_)) else {
        return Err(Error::Str("Failed to convert the address to bytes"));
    };
    let hash = AddressBytesHash::from(&bytes);

    let Ok(Some(type_index)) = stores
        .addressbyteshash_to_typeindex
        .get(&hash)
        .map(|opt| opt.map(|cow| cow.into_owned()))
    else {
        return Err(Error::UnknownAddress);
    };

    let stateful = &computer.stateful;
    let price = computer.price.as_ref().map(|v| {
        *v.timeindexes_to_price_close
            .dateindex
            .as_ref()
            .unwrap()
            .iter()
            .last()
            .unwrap()
            .1
            .into_owned()
    });

    let any_address_index = match type_ {
        OutputType::P2PK33 => stateful
            .p2pk33addressindex_to_anyaddressindex
            .iter()
            .unwrap_get_inner(type_index.into()),
        OutputType::P2PK65 => stateful
            .p2pk65addressindex_to_anyaddressindex
            .iter()
            .unwrap_get_inner(type_index.into()),
        OutputType::P2PKH => stateful
            .p2pkhaddressindex_to_anyaddressindex
            .iter()
            .unwrap_get_inner(type_index.into()),
        OutputType::P2SH => stateful
            .p2shaddressindex_to_anyaddressindex
            .iter()
            .unwrap_get_inner(type_index.into()),
        OutputType::P2TR => stateful
            .p2traddressindex_to_anyaddressindex
            .iter()
            .unwrap_get_inner(type_index.into()),
        OutputType::P2WPKH => stateful
            .p2wpkhaddressindex_to_anyaddressindex
            .iter()
            .unwrap_get_inner(type_index.into()),
        OutputType::P2WSH => stateful
            .p2wshaddressindex_to_anyaddressindex
            .iter()
            .unwrap_get_inner(type_index.into()),
        OutputType::P2A => stateful
            .p2aaddressindex_to_anyaddressindex
            .iter()
            .unwrap_get_inner(type_index.into()),
        t => {
            return Err(Error::UnsupportedType(t.to_string()));
        }
    };

    let address_data = match any_address_index.to_enum() {
        AnyAddressDataIndexEnum::Loaded(index) => stateful
            .loadedaddressindex_to_loadedaddressdata
            .iter()
            .unwrap_get_inner(index),
        AnyAddressDataIndexEnum::Empty(index) => stateful
            .emptyaddressindex_to_emptyaddressdata
            .iter()
            .unwrap_get_inner(index)
            .into(),
    };

    Ok(AddressStats {
        address: address.into(),
        chain_stats: AddressChainStats::default(),
        mempool_stats: AddressMempoolStats::default(),
    })

    // Ok(Address {
    //     address: address.to_string(),
    //     r#type: type_,
    //     type_index,
    //     utxo_count: address_data.utxo_count,
    //     total_sent: address_data.sent,
    //     total_received: address_data.received,
    //     balance,
    //     balance_usd: price.map(|p| p * Bitcoin::from(balance)),
    //     estimated_total_invested: price.map(|_| address_data.realized_cap),
    //     estimated_avg_entry_price: price.map(|_| address_data.realized_price()),
    // })
}
