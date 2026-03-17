use brk_cohort::{AmountBucket, ByAddrType};
use brk_types::{Cents, Sats, TypeIndex};
use rustc_hash::FxHashMap;

use crate::distribution::{
    addr::{AddrTypeToActivityCounts, AddrTypeToVec},
    cohorts::AddrCohorts,
};

use super::super::cache::{AddrLookup, TrackingStatus};

/// Aggregated receive data for a single address within a block.
#[derive(Default)]
struct AggregatedReceive {
    total_value: Sats,
    output_count: u32,
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn process_received(
    received_data: AddrTypeToVec<(TypeIndex, Sats)>,
    cohorts: &mut AddrCohorts,
    lookup: &mut AddrLookup<'_>,
    price: Cents,
    addr_count: &mut ByAddrType<u64>,
    empty_addr_count: &mut ByAddrType<u64>,
    activity_counts: &mut AddrTypeToActivityCounts,
) {
    let max_type_len = received_data.iter().map(|(_, v)| v.len()).max().unwrap_or(0);
    let mut aggregated: FxHashMap<TypeIndex, AggregatedReceive> =
        FxHashMap::with_capacity_and_hasher(max_type_len, Default::default());

    for (output_type, vec) in received_data.unwrap().into_iter() {
        if vec.is_empty() {
            continue;
        }

        // Cache mutable refs for this address type
        let type_addr_count = addr_count.get_mut(output_type).unwrap();
        let type_empty_count = empty_addr_count.get_mut(output_type).unwrap();
        let type_activity = activity_counts.get_mut_unwrap(output_type);

        // Aggregate receives by address - each address processed exactly once
        for (type_index, value) in vec {
            let entry = aggregated.entry(type_index).or_default();
            entry.total_value += value;
            entry.output_count += 1;
        }

        for (type_index, recv) in aggregated.drain() {
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
                addr_data.receive_outputs(recv.total_value, price, recv.output_count);
                let new_bucket = AmountBucket::from(recv.total_value);
                cohorts
                    .amount_range
                    .get_mut_by_bucket(new_bucket)
                    .state
                    .as_mut()
                    .unwrap()
                    .add(addr_data);
            } else {
                let prev_balance = addr_data.balance();
                let new_balance = prev_balance + recv.total_value;
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
                            Addr: {:?}",
                            output_type,
                            type_index,
                            prev_balance,
                            new_balance,
                            recv.total_value,
                            addr_data
                        );
                    }

                    cohort_state.subtract(addr_data);
                    addr_data.receive_outputs(recv.total_value, price, recv.output_count);
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
                        .receive_outputs(addr_data, recv.total_value, price, recv.output_count);
                }
            }
        }
    }
}
