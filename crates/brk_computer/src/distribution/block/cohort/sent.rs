use brk_cohort::{AmountBucket, ByAddrType};
use brk_error::Result;
use brk_types::{Age, Cents, CheckedSub, Height, Sats, Timestamp, TypeIndex};
use rustc_hash::FxHashSet;
use vecdb::VecIndex;

use crate::distribution::{
    addr::{AddrMetricsState, AddrSendPreState, HeightToAddrTypeToVec},
    cohorts::AddrCohorts,
    compute::PriceRangeMax,
};

use super::super::cache::AddrLookup;

/// Process sent UTXOs for address cohorts: age metrics, cohort membership,
/// and empty-address transitions.
///
/// Takes separate price/timestamp slices rather than `chain_state` so it can
/// run in parallel with UTXO cohort processing (which mutates `chain_state`).
/// `price_range_max` feeds peak-regret computation via max price during
/// each UTXO's holding period.
#[allow(clippy::too_many_arguments)]
pub(crate) fn process_sent(
    sent_data: HeightToAddrTypeToVec<(TypeIndex, Sats)>,
    cohorts: &mut AddrCohorts,
    lookup: &mut AddrLookup<'_>,
    current_price: Cents,
    price_range_max: &PriceRangeMax,
    state: &mut AddrMetricsState,
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
        let peak_price = price_range_max.max_between(receive_height, current_height);

        for (output_type, vec) in by_type.unwrap().into_iter() {
            let type_received = received_addrs.get(output_type);
            let type_seen = seen_senders.get_mut_unwrap(output_type);

            for (type_index, value) in vec {
                let addr_data = lookup.get_for_send(output_type, type_index);
                let pre = AddrSendPreState::capture(addr_data, output_type);

                let prev_balance = addr_data.balance();
                let new_balance = prev_balance.checked_sub(value).unwrap();
                let is_first_encounter = type_seen.insert(type_index);
                let also_received = type_received.is_some_and(|s| s.contains(&type_index));
                let will_be_empty = addr_data.has_1_utxos();

                let prev_bucket = AmountBucket::from(prev_balance);
                let new_bucket = AmountBucket::from(new_balance);
                let crossing_boundary = prev_bucket != new_bucket;

                let cohort_state = cohorts
                    .amount_range
                    .get_mut_by_bucket(prev_bucket)
                    .state
                    .as_mut()
                    .unwrap();

                // Mutates addr_data.spent_txo_count (+= 1). on_send_applied reads the post-spend view.
                cohort_state.send(addr_data, value, current_price, prev_price, peak_price, age)?;
                state.on_send_applied(
                    output_type,
                    addr_data,
                    &pre,
                    is_first_encounter,
                    also_received,
                    will_be_empty,
                );

                if will_be_empty || crossing_boundary {
                    cohort_state.subtract(addr_data);
                }
                if will_be_empty {
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
