use brk_cohort::{AmountBucket, ByAddressType};
use brk_types::{Dollars, Sats, TypeIndex};
use rustc_hash::FxHashMap;

use crate::distribution::{
    address::{AddressTypeToActivityCounts, AddressTypeToVec},
    cohorts::AddressCohorts,
};

use super::super::cache::{AddressLookup, TrackingStatus};

#[allow(clippy::too_many_arguments)]
pub fn process_received(
    received_data: AddressTypeToVec<(TypeIndex, Sats)>,
    cohorts: &mut AddressCohorts,
    lookup: &mut AddressLookup<'_>,
    price: Option<Dollars>,
    addr_count: &mut ByAddressType<u64>,
    empty_addr_count: &mut ByAddressType<u64>,
    activity_counts: &mut AddressTypeToActivityCounts,
) {
    for (output_type, vec) in received_data.unwrap().into_iter() {
        if vec.is_empty() {
            continue;
        }

        // Cache mutable refs for this address type
        let type_addr_count = addr_count.get_mut(output_type).unwrap();
        let type_empty_count = empty_addr_count.get_mut(output_type).unwrap();
        let type_activity = activity_counts.get_mut_unwrap(output_type);

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

            // Track receiving activity - each address in receive aggregation
            type_activity.receiving += 1;

            match status {
                TrackingStatus::New => {
                    *type_addr_count += 1;
                }
                TrackingStatus::WasEmpty => {
                    *type_addr_count += 1;
                    *type_empty_count -= 1;
                    // Reactivated - was empty, now has funds
                    type_activity.reactivated += 1;
                }
                TrackingStatus::Tracked => {}
            }

            let is_new_entry = matches!(status, TrackingStatus::New | TrackingStatus::WasEmpty);

            if is_new_entry {
                // New/was-empty address - just add to cohort
                addr_data.receive_outputs(total_value, price, output_count);
                let new_bucket = AmountBucket::from(total_value);
                cohorts
                    .amount_range
                    .get_mut_by_bucket(new_bucket)
                    .state
                    .as_mut()
                    .unwrap()
                    .add(addr_data);
            } else {
                let prev_balance = addr_data.balance();
                let new_balance = prev_balance + total_value;
                let prev_bucket = AmountBucket::from(prev_balance);
                let new_bucket = AmountBucket::from(new_balance);

                if let Some((old_bucket, new_bucket)) = prev_bucket.transition_to(new_bucket) {
                    // Crossing cohort boundary - subtract from old, add to new
                    let cohort_state = cohorts
                        .amount_range
                        .get_mut_by_bucket(old_bucket)
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
                        .get_mut_by_bucket(new_bucket)
                        .state
                        .as_mut()
                        .unwrap()
                        .add(addr_data);
                } else {
                    // Staying in same cohort - just receive
                    cohorts
                        .amount_range
                        .get_mut_by_bucket(new_bucket)
                        .state
                        .as_mut()
                        .unwrap()
                        .receive_outputs(addr_data, total_value, price, output_count);
                }
            }
        }
    }
}
