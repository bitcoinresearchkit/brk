use brk_cohort::{AmountBucket, ByAddrType};
use brk_error::Result;
use brk_types::{Age, Cents, CheckedSub, Height, Sats, Timestamp, TypeIndex};
use rustc_hash::FxHashSet;
use vecdb::VecIndex;

use crate::distribution::{
    addr::{AddrTypeToActivityCounts, HeightToAddrTypeToVec},
    cohorts::AddrCohorts,
    compute::PriceRangeMax,
};

use super::super::cache::AddrLookup;

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
///
/// `price_range_max` is used to compute the peak price during each UTXO's holding period
/// for accurate peak regret calculation.
#[allow(clippy::too_many_arguments)]
pub(crate) fn process_sent(
    sent_data: HeightToAddrTypeToVec<(TypeIndex, Sats)>,
    cohorts: &mut AddrCohorts,
    lookup: &mut AddrLookup<'_>,
    current_price: Cents,
    price_range_max: &PriceRangeMax,
    addr_count: &mut ByAddrType<u64>,
    empty_addr_count: &mut ByAddrType<u64>,
    activity_counts: &mut AddrTypeToActivityCounts,
    received_addrs: &ByAddrType<FxHashSet<TypeIndex>>,
    height_to_price: &[Cents],
    height_to_timestamp: &[Timestamp],
    current_height: Height,
    current_timestamp: Timestamp,
    seen_senders: &mut ByAddrType<FxHashSet<TypeIndex>>,
) -> Result<()> {
    seen_senders.values_mut().for_each(|set| set.clear());

    for (receive_height, by_type) in sent_data.into_iter() {
        let prev_price = height_to_price[receive_height.to_usize()];
        let prev_timestamp = height_to_timestamp[receive_height.to_usize()];
        let age = Age::new(current_timestamp, prev_timestamp);

        // Compute peak spot price during holding period for peak regret
        let peak_price = price_range_max.max_between(receive_height, current_height);

        for (output_type, vec) in by_type.unwrap().into_iter() {
            // Cache mutable refs for this address type
            let type_addr_count = addr_count.get_mut(output_type).unwrap();
            let type_empty_count = empty_addr_count.get_mut(output_type).unwrap();
            let type_activity = activity_counts.get_mut_unwrap(output_type);
            let type_received = received_addrs.get(output_type);
            let type_seen = seen_senders.get_mut_unwrap(output_type);

            for (type_index, value) in vec {
                let addr_data = lookup.get_for_send(output_type, type_index);

                let prev_balance = addr_data.balance();
                let new_balance = prev_balance.checked_sub(value).unwrap();

                // On first encounter of this address this block, track activity
                if type_seen.insert(type_index) {
                    type_activity.sending += 1;

                    // Track "both" - addresses that sent AND received this block
                    if type_received.is_some_and(|s| s.contains(&type_index)) {
                        type_activity.both += 1;
                    }
                }

                let will_be_empty = addr_data.has_1_utxos();

                // Compute buckets once
                let prev_bucket = AmountBucket::from(prev_balance);
                let new_bucket = AmountBucket::from(new_balance);
                let crossing_boundary = prev_bucket != new_bucket;

                let cohort_state = cohorts
                    .amount_range
                    .get_mut_by_bucket(prev_bucket)
                    .state
                    .as_mut()
                    .unwrap();

                cohort_state.send(addr_data, value, current_price, prev_price, peak_price, age)?;

                // If crossing a bucket boundary, remove the (now-updated) address from old bucket
                if will_be_empty || crossing_boundary {
                    cohort_state.subtract(addr_data);
                }

                // Migrate address to new bucket or mark as empty
                if will_be_empty {
                    *type_addr_count -= 1;
                    *type_empty_count += 1;
                    lookup.move_to_empty(output_type, type_index);
                } else if crossing_boundary {
                    cohorts
                        .amount_range
                        .get_mut_by_bucket(new_bucket)
                        .state
                        .as_mut()
                        .unwrap()
                        .add(addr_data);
                }
            }
        }
    }

    Ok(())
}
