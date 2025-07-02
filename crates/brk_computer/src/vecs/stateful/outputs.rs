use std::{collections::BTreeMap, ops::ControlFlow};

use brk_core::{
    CheckedSub, Dollars, GroupFilter, HalvingEpoch, Height, Result, Timestamp, UTXOGroups,
};
use brk_exit::Exit;
use brk_vec::StoredIndex;
use rayon::prelude::*;

use crate::vecs::{Indexes, stateful::utxo_cohort};

pub trait OutputCohorts {
    fn tick_tock_next_block(&mut self, chain_state: &[BlockState], timestamp: Timestamp);
    fn send(&mut self, height_to_sent: BTreeMap<Height, Transacted>, chain_state: &[BlockState]);
    fn receive(&mut self, received: Transacted, height: Height, price: Option<Dollars>);
    fn compute_overlapping_vecs(&mut self, starting_indexes: &Indexes, exit: &Exit) -> Result<()>;
}

impl OutputCohorts for UTXOGroups<(GroupFilter, utxo_cohort::Vecs)> {
    fn tick_tock_next_block(&mut self, chain_state: &[BlockState], timestamp: Timestamp) {
        if chain_state.is_empty() {
            return;
        }

        let prev_timestamp = chain_state.last().unwrap().timestamp;

        self.by_date_range
            .as_mut_vec()
            .into_par_iter()
            .for_each(|(filter, v)| {
                let state = &mut v.state;

                let _ = chain_state
                    .iter()
                    .try_for_each(|block_state| -> ControlFlow<()> {
                        let prev_days_old = block_state
                            .timestamp
                            .difference_in_days_between(prev_timestamp);
                        let days_old = block_state.timestamp.difference_in_days_between(timestamp);

                        if prev_days_old == days_old {
                            return ControlFlow::Continue(());
                        }

                        let is = filter.contains(days_old);
                        let was = filter.contains(prev_days_old);

                        if is && !was {
                            state.increment(&block_state.supply, block_state.price);
                        } else if was && !is {
                            state.decrement(&block_state.supply, block_state.price);
                        }

                        ControlFlow::Continue(())
                    });
            });
    }

    fn send(&mut self, height_to_sent: BTreeMap<Height, Transacted>, chain_state: &[BlockState]) {
        let mut time_based_vecs = self
            .by_date_range
            .as_mut_vec()
            .into_iter()
            .chain(self.by_epoch.as_mut_vec())
            .collect::<Vec<_>>();

        let last_timestamp = chain_state.last().unwrap().timestamp;
        let current_price = chain_state.last().unwrap().price;

        // dbg!(&height_to_sent);

        height_to_sent.into_iter().for_each(|(height, sent)| {
            let block_state = chain_state.get(height.unwrap_to_usize()).unwrap();
            let prev_price = block_state.price;

            let blocks_old = chain_state.len() - 1 - height.unwrap_to_usize();

            let days_old = block_state
                .timestamp
                .difference_in_days_between(last_timestamp);

            let days_old_foat = block_state
                .timestamp
                .difference_in_days_between_float(last_timestamp);

            let older_than_hour =
                jiff::Timestamp::from(last_timestamp.checked_sub(block_state.timestamp).unwrap())
                    .as_second()
                    >= 60 * 60;

            time_based_vecs
                .iter_mut()
                .filter(|(filter, _)| match filter {
                    GroupFilter::From(from) => *from <= days_old,
                    GroupFilter::To(to) => *to > days_old,
                    GroupFilter::Range(range) => range.contains(&days_old),
                    GroupFilter::Epoch(epoch) => *epoch == HalvingEpoch::from(height),
                    _ => unreachable!(),
                })
                .for_each(|(_, vecs)| {
                    vecs.state.send(
                        &sent.spendable_supply,
                        current_price,
                        prev_price,
                        blocks_old,
                        days_old_foat,
                        older_than_hour,
                    );
                });

            sent.by_type.spendable.as_typed_vec().into_iter().for_each(
                |(output_type, supply_state)| {
                    self.by_type.get_mut(output_type).1.state.send(
                        supply_state,
                        current_price,
                        prev_price,
                        blocks_old,
                        days_old_foat,
                        older_than_hour,
                    )
                },
            );

            sent.by_size_group
                .as_typed_vec()
                .into_iter()
                .for_each(|(group, supply_state)| {
                    self.by_size_range.get_mut(group).1.state.send(
                        &supply_state,
                        current_price,
                        prev_price,
                        blocks_old,
                        days_old_foat,
                        older_than_hour,
                    );
                });
        });
    }

    fn receive(&mut self, received: Transacted, height: Height, price: Option<Dollars>) {
        let supply_state = received.spendable_supply;

        [
            &mut self.by_date_range.start_to_1d.1,
            &mut self.by_epoch.mut_vec_from_height(height).1,
        ]
        .into_iter()
        .for_each(|v| {
            v.state.receive(&supply_state, price);
        });

        self.by_type
            .as_mut_vec()
            .into_iter()
            .for_each(|(filter, vecs)| {
                let output_type = match filter {
                    GroupFilter::Type(output_type) => *output_type,
                    _ => unreachable!(),
                };
                vecs.state.receive(received.by_type.get(output_type), price)
            });

        received
            .by_size_group
            .as_typed_vec()
            .into_iter()
            .for_each(|(group, supply_state)| {
                self.by_size_range
                    .get_mut(group)
                    .1
                    .state
                    .receive(&supply_state, price);
            });
    }

    fn compute_overlapping_vecs(&mut self, starting_indexes: &Indexes, exit: &Exit) -> Result<()> {
        let by_date_range = self.by_date_range.as_vec();
        let by_size_range = self.by_size_range.as_vec();

        [
            vec![(&mut self.all.1, self.by_epoch.vecs().to_vec())],
            self.by_from_date
                .as_mut_vec()
                .into_iter()
                .map(|(filter, vecs)| {
                    (
                        vecs,
                        by_date_range
                            .into_iter()
                            .filter(|(other, _)| filter.includes(other))
                            .map(|(_, v)| v)
                            .collect::<Vec<_>>(),
                    )
                })
                .collect::<Vec<_>>(),
            self.by_up_to_date
                .as_mut_vec()
                .into_iter()
                .map(|(filter, vecs)| {
                    (
                        vecs,
                        by_date_range
                            .into_iter()
                            .filter(|(other, _)| filter.includes(other))
                            .map(|(_, v)| v)
                            .collect::<Vec<_>>(),
                    )
                })
                .collect::<Vec<_>>(),
            self.by_term
                .as_mut_vec()
                .into_iter()
                .map(|(filter, vecs)| {
                    (
                        vecs,
                        by_date_range
                            .into_iter()
                            .filter(|(other, _)| filter.includes(other))
                            .map(|(_, v)| v)
                            .collect::<Vec<_>>(),
                    )
                })
                .collect::<Vec<_>>(),
            self.by_from_size
                .as_mut_vec()
                .into_iter()
                .map(|(filter, vecs)| {
                    (
                        vecs,
                        by_size_range
                            .into_iter()
                            .filter(|(other, _)| filter.includes(other))
                            .map(|(_, v)| v)
                            .collect::<Vec<_>>(),
                    )
                })
                .collect::<Vec<_>>(),
            self.by_up_to_size
                .as_mut_vec()
                .into_iter()
                .map(|(filter, vecs)| {
                    (
                        vecs,
                        by_size_range
                            .into_iter()
                            .filter(|(other, _)| filter.includes(other))
                            .map(|(_, v)| v)
                            .collect::<Vec<_>>(),
                    )
                })
                .collect::<Vec<_>>(),
        ]
        .into_par_iter()
        .flatten()
        .try_for_each(|(vecs, stateful)| {
            vecs.compute_from_stateful(starting_indexes, &stateful, exit)
        })
    }
}
