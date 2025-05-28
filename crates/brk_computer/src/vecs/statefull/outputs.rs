use std::{collections::BTreeMap, ops::ControlFlow};

use brk_core::{CheckedSub, Dollars, HalvingEpoch, Height, Timestamp};
use brk_state::{BlockState, OutputFilter, Outputs, Transacted};
use brk_vec::StoredIndex;
use rayon::prelude::*;

use super::cohort;

pub trait OutputCohorts {
    fn tick_tock_next_block(&mut self, chain_state: &[BlockState], timestamp: Timestamp);
    fn send(&mut self, height_to_sent: BTreeMap<Height, Transacted>, chain_state: &[BlockState]);
    fn receive(&mut self, received: Transacted, height: Height, price: Option<Dollars>);
}

impl OutputCohorts for Outputs<(OutputFilter, cohort::Vecs)> {
    fn tick_tock_next_block(&mut self, chain_state: &[BlockState], timestamp: Timestamp) {
        if chain_state.is_empty() {
            return;
        }

        let prev_timestamp = chain_state.last().unwrap().timestamp;

        self.by_term
            .as_mut_vec()
            .into_par_iter()
            .chain(self.by_up_to.as_mut_vec())
            .chain(self.by_from.as_mut_vec())
            .chain(self.by_range.as_mut_vec())
            .for_each(|(filter, v)| {
                let state = &mut v.state;

                let mut check_days_old = |days_old: usize| -> bool {
                    match filter {
                        OutputFilter::From(from) => *from <= days_old,
                        OutputFilter::To(to) => *to > days_old,
                        OutputFilter::Range(range) => range.contains(&days_old),
                        OutputFilter::All
                        | OutputFilter::Epoch(_)
                        | OutputFilter::Size
                        | OutputFilter::Type(_) => unreachable!(),
                    }
                };

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

                        let is = check_days_old(days_old);
                        let was = check_days_old(prev_days_old);

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
            .by_term
            .as_mut_vec()
            .into_iter()
            .chain(self.by_up_to.as_mut_vec())
            .chain(self.by_from.as_mut_vec())
            .chain(self.by_range.as_mut_vec())
            .chain(self.by_epoch.as_mut_vec())
            .collect::<Vec<_>>();

        let last_timestamp = chain_state.last().unwrap().timestamp;
        let current_price = chain_state.last().unwrap().price;

        height_to_sent.into_iter().for_each(|(height, sent)| {
            let block_state = chain_state.get(height.unwrap_to_usize()).unwrap();
            let prev_price = block_state.price;

            let days_old = block_state
                .timestamp
                .difference_in_days_between(last_timestamp);

            let older_than_hour =
                jiff::Timestamp::from(last_timestamp.checked_sub(block_state.timestamp).unwrap())
                    .as_second()
                    >= 60 * 60;

            self.all.1.state.send(
                &sent.spendable_supply,
                current_price,
                prev_price,
                older_than_hour,
            );

            time_based_vecs
                .iter_mut()
                .filter(|(filter, _)| match filter {
                    OutputFilter::From(from) => *from <= days_old,
                    OutputFilter::To(to) => *to > days_old,
                    OutputFilter::Range(range) => range.contains(&days_old),
                    OutputFilter::Epoch(epoch) => *epoch == HalvingEpoch::from(height),
                    _ => unreachable!(),
                })
                .for_each(|(_, vecs)| {
                    vecs.state.send(
                        &sent.spendable_supply,
                        current_price,
                        prev_price,
                        older_than_hour,
                    );
                });

            sent.by_type.spendable.as_typed_vec().into_iter().for_each(
                |(output_type, supply_state)| {
                    self.by_type.get_mut(output_type).1.state.send(
                        supply_state,
                        current_price,
                        prev_price,
                        older_than_hour,
                    )
                },
            );

            sent.by_size.into_iter().for_each(|(group, supply_state)| {
                self.by_size.get_mut(group).1.state.send(
                    &supply_state,
                    current_price,
                    prev_price,
                    older_than_hour,
                );
            });
        });
    }

    fn receive(&mut self, received: Transacted, height: Height, price: Option<Dollars>) {
        let supply_state = received.spendable_supply;

        [
            &mut self.all.1,
            &mut self.by_term.short.1,
            &mut self.by_epoch.mut_vec_from_height(height).1,
            // Skip from and range as can't receive in the past
        ]
        .into_iter()
        .chain(self.by_up_to.as_mut_vec().map(|(_, v)| v))
        .for_each(|v| {
            v.state.receive(&supply_state, price);
        });

        self.by_type
            .as_mut_vec()
            .into_iter()
            .for_each(|(filter, vecs)| {
                let output_type = match filter {
                    OutputFilter::Type(output_type) => *output_type,
                    _ => unreachable!(),
                };
                vecs.state.receive(received.by_type.get(output_type), price)
            });

        received
            .by_size
            .into_iter()
            .for_each(|(group, supply_state)| {
                self.by_size
                    .get_mut(group)
                    .1
                    .state
                    .receive(&supply_state, price);
            });
    }
}
