use brk_cohort::{AmountBucket, ByAddrType};
use brk_error::Result;
use brk_types::{Age, Cents, CheckedSub, Height, Sats, Timestamp, TypeIndex};
use rustc_hash::FxHashSet;
use vecdb::VecIndex;

use crate::distribution::{
    addr::{
        AddrTypeToActivityCounts, AddrTypeToExposedAddrCount, AddrTypeToExposedAddrSupply,
        AddrTypeToReusedAddrCount, HeightToAddrTypeToVec,
    },
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
    reused_addr_count: &mut AddrTypeToReusedAddrCount,
    exposed_addr_count: &mut AddrTypeToExposedAddrCount,
    total_exposed_addr_count: &mut AddrTypeToExposedAddrCount,
    exposed_addr_supply: &mut AddrTypeToExposedAddrSupply,
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
            let type_reused_count = reused_addr_count.get_mut(output_type).unwrap();
            let type_exposed_count = exposed_addr_count.get_mut(output_type).unwrap();
            let type_total_exposed_count = total_exposed_addr_count.get_mut(output_type).unwrap();
            let type_exposed_supply = exposed_addr_supply.get_mut(output_type).unwrap();
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

                // Capture exposed state BEFORE the spend mutates spent_txo_count.
                let was_pubkey_exposed = addr_data.is_pubkey_exposed(output_type);
                let exposed_contribution_before =
                    addr_data.exposed_supply_contribution(output_type);

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
                // addr_data.spent_txo_count is now incremented by 1.

                // Update exposed supply via post-spend contribution delta.
                let exposed_contribution_after =
                    addr_data.exposed_supply_contribution(output_type);
                if exposed_contribution_after >= exposed_contribution_before {
                    *type_exposed_supply += exposed_contribution_after - exposed_contribution_before;
                } else {
                    *type_exposed_supply -= exposed_contribution_before - exposed_contribution_after;
                }

                // Update exposed counts on first-ever pubkey exposure.
                // For non-pk-exposed types this fires on the first spend; for
                // pk-exposed types it never fires here (was_pubkey_exposed was
                // already true at first receive in process_received).
                if !was_pubkey_exposed {
                    *type_total_exposed_count += 1;
                    if !will_be_empty {
                        *type_exposed_count += 1;
                    }
                }

                // If crossing a bucket boundary, remove the (now-updated) address from old bucket
                if will_be_empty || crossing_boundary {
                    cohort_state.subtract(addr_data);
                }

                // Migrate address to new bucket or mark as empty
                if will_be_empty {
                    *type_addr_count -= 1;
                    *type_empty_count += 1;
                    // Reused addr leaving the funded reused set
                    if addr_data.is_reused() {
                        *type_reused_count -= 1;
                    }
                    // Exposed addr leaving the funded exposed set: was in set
                    // iff its pubkey was exposed pre-spend (since it was funded
                    // to be in process_sent in the first place), and now leaves
                    // because it's empty.
                    if was_pubkey_exposed {
                        *type_exposed_count -= 1;
                    }
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
