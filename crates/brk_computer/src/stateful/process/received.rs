//! Process received outputs for address cohorts.

use brk_grouper::{amounts_in_different_buckets, ByAddressType};
use brk_types::{Dollars, Sats, TypeIndex};
use rustc_hash::FxHashMap;

use super::super::address::AddressTypeToVec;
use super::super::cohorts::AddressCohorts;
use super::lookup::{AddressLookup, TrackingStatus};

pub fn process_received(
    received_data: AddressTypeToVec<(TypeIndex, Sats)>,
    cohorts: &mut AddressCohorts,
    lookup: &mut AddressLookup<'_>,
    price: Option<Dollars>,
    addr_count: &mut ByAddressType<u64>,
    empty_addr_count: &mut ByAddressType<u64>,
) {
    for (output_type, vec) in received_data.unwrap().into_iter() {
        if vec.is_empty() {
            continue;
        }

        // Aggregate receives by address - each address processed exactly once
        // Track (total_value, output_count) for correct UTXO counting
        let mut aggregated: FxHashMap<TypeIndex, (Sats, u32)> = FxHashMap::default();
        for (type_index, value) in vec {
            let entry = aggregated.entry(type_index).or_default();
            entry.0 += value;
            entry.1 += 1;
        }

        for (type_index, (total_value, output_count)) in aggregated {
            let (addr_data, status) = lookup.get_or_create_for_receive(output_type, type_index);

            match status {
                TrackingStatus::New => {
                    *addr_count.get_mut(output_type).unwrap() += 1;
                }
                TrackingStatus::WasEmpty => {
                    *addr_count.get_mut(output_type).unwrap() += 1;
                    *empty_addr_count.get_mut(output_type).unwrap() -= 1;
                }
                TrackingStatus::Tracked => {}
            }

            let is_new_entry = matches!(status, TrackingStatus::New | TrackingStatus::WasEmpty);

            if is_new_entry {
                // New/was-empty address - just add to cohort
                addr_data.receive_outputs(total_value, price, output_count);
                cohorts
                    .amount_range
                    .get_mut(total_value) // new_balance = 0 + total_value
                    .state
                    .as_mut()
                    .unwrap()
                    .add(addr_data);
            } else {
                let prev_balance = addr_data.balance();
                let new_balance = prev_balance + total_value;

                if amounts_in_different_buckets(prev_balance, new_balance) {
                    // Crossing cohort boundary - subtract from old, add to new
                    let cohort_state = cohorts
                        .amount_range
                        .get_mut(prev_balance)
                        .state
                        .as_mut()
                        .unwrap();

                    // Debug info for tracking down underflow issues
                    if cohort_state.inner.supply.utxo_count < addr_data.utxo_count() as u64 {
                        panic!(
                            "process_received: cohort underflow detected!\n\
                            output_type={:?}, type_index={:?}\n\
                            prev_balance={}, new_balance={}, total_value={}\n\
                            Address: {:?}",
                            output_type,
                            type_index,
                            prev_balance,
                            new_balance,
                            total_value,
                            addr_data
                        );
                    }

                    cohort_state.subtract(addr_data);
                    addr_data.receive_outputs(total_value, price, output_count);
                    cohorts
                        .amount_range
                        .get_mut(new_balance)
                        .state
                        .as_mut()
                        .unwrap()
                        .add(addr_data);
                } else {
                    // Staying in same cohort - just receive
                    cohorts
                        .amount_range
                        .get_mut(new_balance)
                        .state
                        .as_mut()
                        .unwrap()
                        .receive_outputs(addr_data, total_value, price, output_count);
                }
            }
        }
    }
}
