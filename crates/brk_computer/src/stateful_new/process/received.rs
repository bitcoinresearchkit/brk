//! Process received outputs for address cohorts.
//!
//! Updates address cohort states when addresses receive funds:
//! - New addresses enter a cohort
//! - Existing addresses may cross cohort boundaries
//! - Empty addresses become non-empty again

use brk_grouper::{ByAddressType, Filtered};
use brk_types::{Dollars, OutputType, Sats, TypeIndex};

use super::super::address::AddressTypeToVec;
use super::super::cohorts::AddressCohorts;
use super::address_lookup::{AddressLookup, LoadedAddressDataWithSource};

/// Process received outputs for address cohorts.
///
/// For each received output:
/// 1. Look up or create address data
/// 2. Update address balance and cohort membership
/// 3. Update cohort states (add/subtract for boundary crossings, receive otherwise)
pub fn process_received<F>(
    received_data: AddressTypeToVec<(TypeIndex, Sats)>,
    cohorts: &mut AddressCohorts,
    lookup: &mut AddressLookup<F>,
    price: Option<Dollars>,
    addr_count: &mut ByAddressType<u64>,
    empty_addr_count: &mut ByAddressType<u64>,
) where
    F: FnMut(OutputType, TypeIndex) -> Option<LoadedAddressDataWithSource>,
{
    for (output_type, vec) in received_data.unwrap().into_iter() {
        if vec.is_empty() {
            continue;
        }

        for (type_index, value) in vec {
            let (addr_data, is_new, from_empty) =
                lookup.get_or_create_for_receive(output_type, type_index);

            // Update address counts
            if is_new || from_empty {
                *addr_count.get_mut(output_type).unwrap() += 1;
                if from_empty {
                    *empty_addr_count.get_mut(output_type).unwrap() -= 1;
                }
            }

            let prev_balance = addr_data.balance();
            let new_balance = prev_balance + value;

            // Check if crossing cohort boundary
            let prev_cohort = cohorts.amount_range.get(prev_balance);
            let new_cohort = cohorts.amount_range.get(new_balance);
            let filters_differ = prev_cohort.filter() != new_cohort.filter();

            if is_new || from_empty || filters_differ {
                // Address entering or changing cohorts
                if !is_new && !from_empty {
                    // Subtract from old cohort
                    cohorts
                        .amount_range
                        .get_mut(prev_balance)
                        .state
                        .as_mut()
                        .unwrap()
                        .subtract(addr_data);
                }

                // Update address data
                addr_data.receive(value, price);

                // Add to new cohort
                cohorts
                    .amount_range
                    .get_mut(new_balance)
                    .state
                    .as_mut()
                    .unwrap()
                    .add(addr_data);
            } else {
                // Address staying in same cohort - update in place
                cohorts
                    .amount_range
                    .get_mut(new_balance)
                    .state
                    .as_mut()
                    .unwrap()
                    .receive(addr_data, value, price);
            }
        }
    }
}
