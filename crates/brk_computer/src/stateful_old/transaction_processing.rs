use brk_error::Result;
use brk_grouper::{ByAddressType, Filtered};
use brk_types::{
    CheckedSub, Dollars, EmptyAddressData, Height, LoadedAddressData, Sats, Timestamp, TypeIndex,
};
use vecdb::VecIndex;

use crate::utils::OptionExt;

use super::{
    address_cohorts,
    addresstype::{AddressTypeToTypeIndexMap, AddressTypeToVec, HeightToAddressTypeToVec},
    withaddressdatasource::WithAddressDataSource,
};

impl AddressTypeToVec<(TypeIndex, Sats)> {
    #[allow(clippy::too_many_arguments)]
    pub fn process_received(
        self,
        vecs: &mut address_cohorts::Vecs,
        addresstype_to_typeindex_to_loadedaddressdata: &mut AddressTypeToTypeIndexMap<
            WithAddressDataSource<LoadedAddressData>,
        >,
        addresstype_to_typeindex_to_emptyaddressdata: &mut AddressTypeToTypeIndexMap<
            WithAddressDataSource<EmptyAddressData>,
        >,
        price: Option<Dollars>,
        addresstype_to_addr_count: &mut ByAddressType<u64>,
        addresstype_to_empty_addr_count: &mut ByAddressType<u64>,
        stored_or_new_addresstype_to_typeindex_to_addressdatawithsource: &mut AddressTypeToTypeIndexMap<
            WithAddressDataSource<LoadedAddressData>,
        >,
    ) {
        self.unwrap().into_iter().for_each(|(_type, vec)| {
            vec.into_iter().for_each(|(type_index, value)| {
                let mut is_new = false;
                let mut from_any_empty = false;

                let addressdata_withsource = addresstype_to_typeindex_to_loadedaddressdata
                    .get_mut(_type)
                    .unwrap()
                    .entry(type_index)
                    .or_insert_with(|| {
                        addresstype_to_typeindex_to_emptyaddressdata
                            .get_mut(_type)
                            .unwrap()
                            .remove(&type_index)
                            .map(|ad| {
                                from_any_empty = true;
                                ad.into()
                            })
                            .unwrap_or_else(|| {
                                let addressdata =
                                    stored_or_new_addresstype_to_typeindex_to_addressdatawithsource
                                        .remove_for_type(_type, &type_index);
                                is_new = addressdata.is_new();
                                from_any_empty = addressdata.is_from_emptyaddressdata();
                                addressdata
                            })
                    });

                if is_new || from_any_empty {
                    (*addresstype_to_addr_count.get_mut(_type).unwrap()) += 1;
                    if from_any_empty {
                        (*addresstype_to_empty_addr_count.get_mut(_type).unwrap()) -= 1;
                    }
                }

                let addressdata = addressdata_withsource.deref_mut();

                let prev_amount = addressdata.balance();

                let amount = prev_amount + value;

                let filters_differ = vecs.amount_range.get(amount).filter()
                    != vecs.amount_range.get(prev_amount).filter();

                if is_new || from_any_empty || filters_differ {
                    if !is_new && !from_any_empty {
                        vecs.amount_range
                            .get_mut(prev_amount)
                            .state
                            .um()
                            .subtract(addressdata);
                    }

                    addressdata.receive(value, price);

                    vecs.amount_range
                        .get_mut(amount)
                        .state
                        .um()
                        .add(addressdata);
                } else {
                    vecs.amount_range
                        .get_mut(amount)
                        .state
                        .um()
                        .receive(addressdata, value, price);
                }
            });
        });
    }
}

impl HeightToAddressTypeToVec<(TypeIndex, Sats)> {
    #[allow(clippy::too_many_arguments)]
    pub fn process_sent(
        self,
        vecs: &mut address_cohorts::Vecs,
        addresstype_to_typeindex_to_loadedaddressdata: &mut AddressTypeToTypeIndexMap<
            WithAddressDataSource<LoadedAddressData>,
        >,
        addresstype_to_typeindex_to_emptyaddressdata: &mut AddressTypeToTypeIndexMap<
            WithAddressDataSource<EmptyAddressData>,
        >,
        price: Option<Dollars>,
        addresstype_to_addr_count: &mut ByAddressType<u64>,
        addresstype_to_empty_addr_count: &mut ByAddressType<u64>,
        height_to_price_close_vec: Option<&Vec<brk_types::Close<Dollars>>>,
        height_to_timestamp_fixed_vec: &[Timestamp],
        height: Height,
        timestamp: Timestamp,
        stored_or_new_addresstype_to_typeindex_to_addressdatawithsource: &mut AddressTypeToTypeIndexMap<
            WithAddressDataSource<LoadedAddressData>,
        >,
    ) -> Result<()> {
        self.0.into_iter().try_for_each(|(prev_height, v)| {
            let prev_price = height_to_price_close_vec
                .as_ref()
                .map(|v| **v.get(prev_height.to_usize()).unwrap());

            let prev_timestamp = *height_to_timestamp_fixed_vec
                .get(prev_height.to_usize())
                .unwrap();

            let blocks_old = height.to_usize() - prev_height.to_usize();

            let days_old = timestamp.difference_in_days_between_float(prev_timestamp);

            let older_than_hour = timestamp
                .checked_sub(prev_timestamp)
                .unwrap()
                .is_more_than_hour();

            v.unwrap().into_iter().try_for_each(|(_type, vec)| {
                vec.into_iter().try_for_each(|(type_index, value)| {
                    let typeindex_to_loadedaddressdata =
                        addresstype_to_typeindex_to_loadedaddressdata.get_mut_unwrap(_type);

                    let addressdata_withsource = typeindex_to_loadedaddressdata
                        .entry(type_index)
                        .or_insert_with(|| {
                            stored_or_new_addresstype_to_typeindex_to_addressdatawithsource
                                .remove_for_type(_type, &type_index)
                        });

                    let addressdata = addressdata_withsource.deref_mut();

                    let prev_amount = addressdata.balance();

                    let amount = prev_amount.checked_sub(value).unwrap();

                    let will_be_empty = addressdata.has_1_utxos();

                    let filters_differ = vecs.amount_range.get(amount).filter()
                        != vecs.amount_range.get(prev_amount).filter();

                    if will_be_empty || filters_differ {
                        vecs.amount_range
                            .get_mut(prev_amount)
                            .state
                            .um()
                            .subtract(addressdata);

                        addressdata.send(value, prev_price)?;

                        if will_be_empty {
                            if amount.is_not_zero() {
                                unreachable!()
                            }

                            (*addresstype_to_addr_count.get_mut(_type).unwrap()) -= 1;
                            (*addresstype_to_empty_addr_count.get_mut(_type).unwrap()) += 1;

                            let addressdata =
                                typeindex_to_loadedaddressdata.remove(&type_index).unwrap();

                            addresstype_to_typeindex_to_emptyaddressdata
                                .get_mut(_type)
                                .unwrap()
                                .insert(type_index, addressdata.into());
                        } else {
                            vecs.amount_range
                                .get_mut(amount)
                                .state
                                .um()
                                .add(addressdata);
                        }
                    } else {
                        vecs.amount_range.get_mut(amount).state.um().send(
                            addressdata,
                            value,
                            price,
                            prev_price,
                            blocks_old,
                            days_old,
                            older_than_hour,
                        )?;
                    }

                    Ok(())
                })
            })
        })
    }
}
