use brk_cohort::{AmountBucket, ByAddressType};
use brk_error::Result;
use brk_types::{Age, CheckedSub, Dollars, Height, Sats, Timestamp, TypeIndex};
use rustc_hash::FxHashSet;
use vecdb::{unlikely, VecIndex};

use crate::distribution::{
    address::{AddressTypeToActivityCounts, HeightToAddressTypeToVec},
    cohorts::AddressCohorts,
};

use super::super::cache::AddressLookup;

/// Process sent outputs for address cohorts.
///
/// For each spent UTXO:
/// 1. Look up address data
/// 2. Calculate age metrics
/// 3. Update address balance and cohort membership
/// 4. Handle addresses becoming empty
///
/// Note: Takes separate price/timestamp slices instead of chain_state to allow
/// parallel execution with UTXO cohort processing (which mutates chain_state).
#[allow(clippy::too_many_arguments)]
pub fn process_sent(
    sent_data: HeightToAddressTypeToVec<(TypeIndex, Sats)>,
    cohorts: &mut AddressCohorts,
    lookup: &mut AddressLookup<'_>,
    current_price: Option<Dollars>,
    addr_count: &mut ByAddressType<u64>,
    empty_addr_count: &mut ByAddressType<u64>,
    activity_counts: &mut AddressTypeToActivityCounts,
    received_addresses: &ByAddressType<FxHashSet<TypeIndex>>,
    height_to_price: Option<&[Dollars]>,
    height_to_timestamp: &[Timestamp],
    current_height: Height,
    current_timestamp: Timestamp,
) -> Result<()> {
    // Track unique senders per address type (simple set, no extra data needed)
    let mut seen_senders: ByAddressType<FxHashSet<TypeIndex>> = ByAddressType::default();

    for (prev_height, by_type) in sent_data.into_iter() {
        let prev_price = height_to_price.map(|v| v[prev_height.to_usize()]);
        let prev_timestamp = height_to_timestamp[prev_height.to_usize()];
        let blocks_old = current_height.to_usize() - prev_height.to_usize();
        let age = Age::new(current_timestamp, prev_timestamp, blocks_old);

        for (output_type, vec) in by_type.unwrap().into_iter() {
            // Cache mutable refs for this address type
            let type_addr_count = addr_count.get_mut(output_type).unwrap();
            let type_empty_count = empty_addr_count.get_mut(output_type).unwrap();
            let type_activity = activity_counts.get_mut_unwrap(output_type);
            let type_received = received_addresses.get_unwrap(output_type);
            let type_seen = seen_senders.get_mut_unwrap(output_type);

            for (type_index, value) in vec {
                let addr_data = lookup.get_for_send(output_type, type_index);

                let prev_balance = addr_data.balance();
                let new_balance = prev_balance.checked_sub(value).unwrap();

                // On first encounter of this address this block, track activity
                if type_seen.insert(type_index) {
                    type_activity.sending += 1;

                    // Track "both" - addresses that sent AND received this block
                    if type_received.contains(&type_index) {
                        type_activity.both += 1;
                    }
                }

                let will_be_empty = addr_data.has_1_utxos();

                // Compute buckets once
                let prev_bucket = AmountBucket::from(prev_balance);
                let new_bucket = AmountBucket::from(new_balance);
                let crossing_boundary = prev_bucket != new_bucket;

                if will_be_empty || crossing_boundary {
                    // Subtract from old cohort
                    let cohort_state = cohorts
                        .amount_range
                        .get_mut_by_bucket(prev_bucket)
                        .state
                        .as_mut()
                        .unwrap();

                    // Debug info for tracking down underflow issues
                    if unlikely(
                        cohort_state.inner.supply.utxo_count < addr_data.utxo_count() as u64,
                    ) {
                        panic!(
                            "process_sent: cohort underflow detected!\n\
                            Block context: prev_height={:?}, output_type={:?}, type_index={:?}\n\
                            prev_balance={}, new_balance={}, value={}\n\
                            will_be_empty={}, crossing_boundary={}\n\
                            Address: {:?}",
                            prev_height,
                            output_type,
                            type_index,
                            prev_balance,
                            new_balance,
                            value,
                            will_be_empty,
                            crossing_boundary,
                            addr_data
                        );
                    }

                    cohort_state.subtract(addr_data);

                    // Update address data
                    addr_data.send(value, prev_price)?;

                    if will_be_empty {
                        // Address becoming empty - invariant check
                        if new_balance.is_not_zero() {
                            unreachable!()
                        }

                        *type_addr_count -= 1;
                        *type_empty_count += 1;

                        // Move from loaded to empty
                        lookup.move_to_empty(output_type, type_index);
                    } else {
                        // Add to new cohort
                        cohorts
                            .amount_range
                            .get_mut_by_bucket(new_bucket)
                            .state
                            .as_mut()
                            .unwrap()
                            .add(addr_data);
                    }
                } else {
                    // Address staying in same cohort - update in place
                    cohorts
                        .amount_range
                        .get_mut_by_bucket(new_bucket)
                        .state
                        .as_mut()
                        .unwrap()
                        .send(addr_data, value, current_price, prev_price, age)?;
                }
            }
        }
    }

    Ok(())
}
